const fs = require("node:fs/promises");
const path = require("node:path");
const crypto = require("node:crypto");
const { ImapFlow } = require("imapflow");
const nodemailer = require("nodemailer");
const { simpleParser } = require("mailparser");

const PROVIDERS = [
  { id: "gmail", label: "Gmail", imap: { host: "imap.gmail.com", port: 993, secure: true }, smtp: { host: "smtp.gmail.com", port: 465, secure: true }, authHint: "Use your Gmail address and an app password." },
  { id: "outlook", label: "Outlook", imap: { host: "outlook.office365.com", port: 993, secure: true }, smtp: { host: "smtp.office365.com", port: 587, secure: false }, authHint: "Use your Outlook or Microsoft 365 credentials." },
  { id: "icloud", label: "iCloud", imap: { host: "imap.mail.me.com", port: 993, secure: true }, smtp: { host: "smtp.mail.me.com", port: 587, secure: false }, authHint: "Use your Apple ID email and an app-specific password." },
  { id: "yahoo", label: "Yahoo", imap: { host: "imap.mail.yahoo.com", port: 993, secure: true }, smtp: { host: "smtp.mail.yahoo.com", port: 465, secure: true }, authHint: "Use your Yahoo address and an app password." },
  { id: "custom", label: "Custom", imap: { host: "", port: 993, secure: true }, smtp: { host: "", port: 465, secure: true }, authHint: "Enter your provider IMAP and SMTP settings manually." },
];

class ImapConnectionPool {
  constructor(logger, eventBus) {
    this.logger = logger;
    this.eventBus = eventBus;
    this.connections = new Map();
    this.idleTimers = new Map();
  }

  async connect(account) {
    const existing = this.connections.get(account.id);
    if (existing && existing.client.usable) return existing.client;

    if (existing) {
      try { await existing.client.logout(); } catch {}
    }

    const client = new ImapFlow({
      host: account.imap.host,
      port: account.imap.port,
      secure: account.imap.secure,
      auth: { user: account.username, pass: account.password },
      logger: false,
    });

    await client.connect();
    this.logger.info("mail:pool:connected", { accountId: account.id });
    this.connections.set(account.id, { client, accountId: account.id });
    return client;
  }

  async startIdle(accountId, mailbox = "INBOX") {
    try {
      const entry = this.connections.get(accountId);
      if (!entry) return;
      const client = entry.client;
      if (!client.usable) return;

      await client.mailboxOpen(mailbox);
      const idle = client.idle();

      idle.addEventListener("mailboxOpen", () => {
        this.logger.info("mail:idle:started", { accountId, mailbox });
      });

      (async () => {
        for await (const event of idle) {
          if (event.type === "exists") {
            this.eventBus.emit("mail:new-message", { accountId, mailbox, count: event.count });
          }
        }
      })();

      // IMAP IDLE timeout is 30min, reconnect at 29min
      if (this.idleTimers.has(accountId)) clearTimeout(this.idleTimers.get(accountId));
      this.idleTimers.set(accountId, setTimeout(() => {
        this.restartIdle(accountId, mailbox).catch(() => {});
      }, 29 * 60 * 1000));
    } catch (err) {
      this.logger.warn("mail:idle:failed", { accountId, err: err.message });
    }
  }

  async restartIdle(accountId, mailbox) {
    try {
      const entry = this.connections.get(accountId);
      if (entry && entry.client.usable) {
        try { await entry.client.idleStop(); } catch {}
      }
      await this.startIdle(accountId, mailbox);
    } catch {}
  }

  async disconnect(accountId) {
    const entry = this.connections.get(accountId);
    if (!entry) return;
    if (this.idleTimers.has(accountId)) clearTimeout(this.idleTimers.get(accountId));
    try {
      if (entry.client.usable) {
        await entry.client.idleStop().catch(() => {});
        await entry.client.logout().catch(() => {});
      }
    } catch {}
    this.connections.delete(accountId);
    this.logger.info("mail:pool:disconnected", { accountId });
  }

  async disconnectAll() {
    for (const accountId of this.connections.keys()) {
      await this.disconnect(accountId).catch(() => {});
    }
  }

  getStatus(accountId) {
    const entry = this.connections.get(accountId);
    return { connected: entry?.client?.usable || false };
  }
}

class MessageCache {
  constructor() {
    this._cache = new Map();
  }

  key(accountId, folder) {
    return `${accountId}:${folder}`;
  }

  get(accountId, folder) {
    return this._cache.get(this.key(accountId, folder)) || null;
  }

  set(accountId, folder, data) {
    this._cache.set(this.key(accountId, folder), { data, ts: Date.now() });
  }

  invalidate(accountId, folder) {
    this._cache.delete(this.key(accountId, folder));
  }

  invalidateAll(accountId) {
    for (const key of this._cache.keys()) {
      if (key.startsWith(`${accountId}:`)) this._cache.delete(key);
    }
  }
}

class MailService {
  constructor(logger, eventBus, userService, vaultService) {
    this.logger = logger;
    this.eventBus = eventBus;
    this.userService = userService;
    this.vault = vaultService;
    this.pool = new ImapConnectionPool(logger, eventBus);
    this.cache = new MessageCache();
  }

  _accountsFile({ requireActive = true } = {}) {
    const active = this.userService?.getActiveUser?.();
    if (!active?.username) {
      if (!requireActive) return null;
      const error = new Error("mail_vault_locked");
      error.code = "MAIL_VAULT_LOCKED";
      throw error;
    }
    const home = this.userService.resolveHome(active.username);
    return path.join(path.dirname(home), "mail.json");
  }

  async ensureStorage(options) {
    const accountsFile = this._accountsFile(options);
    if (!accountsFile) return null;
    await fs.mkdir(path.dirname(accountsFile), { recursive: true });
    try { await fs.access(accountsFile); } catch {
      await fs.writeFile(accountsFile, JSON.stringify({ accounts: [] }, null, 2), "utf8");
    }
    return accountsFile;
  }

  providers() { return PROVIDERS; }

  async listAccounts() {
    const accounts = await this.readAccounts();
    return accounts.map((a) => this.sanitizeAccount(a));
  }

  async saveAccount(payload) {
    await this.ensureStorage();
    const nextAccount = this.normalizeAccount(payload);

    // Verify
    const verifyAccount = { ...nextAccount, password: nextAccount.password || String(payload.password || "") };
    await this.verifyAccount(verifyAccount);

    nextAccount._encrypted = await this.vault.encryptForActiveUser(nextAccount.password);
    nextAccount.password = "";

    const accounts = await this.readAccounts();
    const idx = accounts.findIndex((a) => a.id === nextAccount.id);
    if (idx >= 0) accounts[idx] = nextAccount;
    else accounts.push(nextAccount);
    await this.writeAccounts(accounts);

    // Connect pool + start IDLE
    try {
      await this.pool.connect(verifyAccount);
      await this.pool.startIdle(nextAccount.id);
    } catch {}

    return this.sanitizeAccount(nextAccount);
  }

  async deleteAccount(accountId) {
    await this.pool.disconnect(accountId);
    const accounts = await this.readAccounts();
    await this.writeAccounts(accounts.filter((a) => a.id !== accountId));
    return { ok: true };
  }

  async listFolders(accountId) {
    const account = await this.getAccount(accountId);
    const client = await this.pool.connect(account);
    const boxes = await client.list();
    return boxes.map((box) => ({
      path: box.path,
      name: box.name,
      specialUse: Array.isArray(box.specialUse) ? box.specialUse[0] || null : box.specialUse || null,
    }));
  }

  async listMessages(accountId, folder = "INBOX", limit = 50) {
    const account = await this.getAccount(accountId);

    const cached = this.cache.get(accountId, folder);
    if (cached) return { folder, messages: cached.data, cached: true };

    const client = await this.pool.connect(account);
    const mailbox = await client.mailboxOpen(folder, { readOnly: true });
    const total = mailbox.exists || 0;
    if (!total) return { folder, messages: [] };

    const start = Math.max(1, total - limit + 1);
    const messages = [];
    for await (const msg of client.fetch(`${start}:${total}`, {
      uid: true, envelope: true, flags: true, internalDate: true, bodyStructure: true,
    })) {
      messages.push({
        uid: msg.uid,
        subject: msg.envelope?.subject || "(No subject)",
        from: this.formatAddressList(msg.envelope?.from),
        to: this.formatAddressList(msg.envelope?.to),
        date: msg.internalDate || msg.envelope?.date || null,
        seen: Array.isArray(msg.flags) ? msg.flags.includes("\\Seen") : false,
        flagged: Array.isArray(msg.flags) ? msg.flags.includes("\\Flagged") : false,
        hasAttachments: Boolean(msg.bodyStructure?.childNodes?.some((n) => n.disposition === "attachment")),
        snippet: "",
      });
    }
    messages.sort((a, b) => new Date(b.date || 0).getTime() - new Date(a.date || 0).getTime());
    this.cache.set(accountId, folder, messages);
    return { folder, messages };
  }

  async getMessage(accountId, folder, uid) {
    const account = await this.getAccount(accountId);
    const client = await this.pool.connect(account);
    await client.mailboxOpen(folder, { readOnly: true });
    const msg = await client.fetchOne(Number(uid), {
      uid: true, envelope: true, source: true, flags: true, internalDate: true,
    });
    if (!msg) { const e = new Error("mail_message_not_found"); e.code = "ENOENT"; throw e; }

    const parsed = await simpleParser(msg.source);
    return {
      uid: msg.uid,
      subject: parsed.subject || msg.envelope?.subject || "(No subject)",
      from: this.formatAddressHeader(parsed.from?.value || msg.envelope?.from || []),
      to: this.formatAddressHeader(parsed.to?.value || msg.envelope?.to || []),
      cc: this.formatAddressHeader(parsed.cc?.value || []),
      date: parsed.date || msg.internalDate || msg.envelope?.date || null,
      text: parsed.text || "",
      html: parsed.html ? String(parsed.html) : "",
      attachments: (parsed.attachments || []).map((a) => ({
        filename: a.filename || "attachment", contentType: a.contentType, size: a.size,
        contentDisposition: a.contentDisposition,
      })),
    };
  }

  async sendMessage(accountId, payload) {
    const account = await this.getAccount(accountId);
    const transporter = nodemailer.createTransport({
      host: account.smtp.host, port: account.smtp.port, secure: account.smtp.secure,
      auth: { user: account.username, pass: account.password },
    });
    await transporter.verify().catch((e) => { throw this.normalizeMailError(e, "smtp"); });
    const result = await transporter.sendMail({
      from: payload.from || account.email,
      to: Array.isArray(payload.to) ? payload.to.join(", ") : payload.to,
      cc: Array.isArray(payload.cc) ? payload.cc.join(", ") : payload.cc,
      bcc: Array.isArray(payload.bcc) ? payload.bcc.join(", ") : payload.bcc,
      subject: payload.subject || "",
      text: payload.text || "",
      html: payload.html || undefined,
    });
    this.cache.invalidate(accountId, "INBOX");
    return { ok: true, messageId: result.messageId, accepted: result.accepted || [] };
  }

  async searchMessages(accountId, folder, query) {
    const account = await this.getAccount(accountId);
    const client = await this.pool.connect(account);
    await client.mailboxOpen(folder, { readOnly: true });

    const searchQuery = {};
    if (query.text) searchQuery.text = query.text;
    if (query.from) searchQuery.headerFrom = query.from;
    if (query.subject) searchQuery.headerSubject = query.subject;
    if (query.unseen) searchQuery.unseen = true;
    if (query.flagged) searchQuery.flagged = true;

    const uids = await client.search(searchQuery);
    if (!uids || uids.length === 0) return { folder, messages: [] };

    const limit = Math.min(uids.length, 50);
    const recent = uids.slice(-limit);
    const messages = [];
    for await (const msg of client.fetch(recent, {
      uid: true, envelope: true, flags: true, internalDate: true,
    })) {
      messages.push({
        uid: msg.uid, subject: msg.envelope?.subject || "(No subject)",
        from: this.formatAddressList(msg.envelope?.from),
        date: msg.internalDate || msg.envelope?.date || null,
        seen: Array.isArray(msg.flags) ? msg.flags.includes("\\Seen") : false,
        flagged: Array.isArray(msg.flags) ? msg.flags.includes("\\Flagged") : false,
      });
    }
    messages.sort((a, b) => new Date(b.date || 0).getTime() - new Date(a.date || 0).getTime());
    return { folder, messages };
  }

  async moveMessage(accountId, folder, uid, destFolder) {
    const account = await this.getAccount(accountId);
    const client = await this.pool.connect(account);
    await client.mailboxOpen(folder, { readOnly: false });
    await client.messageMove({ uid: Number(uid) }, destFolder);
    this.cache.invalidate(accountId, folder);
    this.cache.invalidate(accountId, destFolder);
    return { ok: true };
  }

  async deleteMessage(accountId, folder, uid) {
    return this.moveMessage(accountId, folder, uid, "[Gmail]/Trash");
  }

  async flagMessage(accountId, folder, uid, flagged) {
    const account = await this.getAccount(accountId);
    const client = await this.pool.connect(account);
    await client.mailboxOpen(folder, { readOnly: false });
    if (flagged) await client.messageFlagsAdd({ uid: Number(uid) }, ["\\Flagged"]);
    else await client.messageFlagsRemove({ uid: Number(uid) }, ["\\Flagged"]);
    this.cache.invalidate(accountId, folder);
    return { ok: true };
  }

  async markSeen(accountId, folder, uid, seen) {
    const account = await this.getAccount(accountId);
    const client = await this.pool.connect(account);
    await client.mailboxOpen(folder, { readOnly: false });
    if (seen) await client.messageFlagsAdd({ uid: Number(uid) }, ["\\Seen"]);
    else await client.messageFlagsRemove({ uid: Number(uid) }, ["\\Seen"]);
    this.cache.invalidate(accountId, folder);
    return { ok: true };
  }

  async getAttachment(accountId, folder, uid, partId) {
    const account = await this.getAccount(accountId);
    const client = await this.pool.connect(account);
    await client.mailboxOpen(folder, { readOnly: true });
    const msg = await client.fetchOne(Number(uid), { source: true });
    if (!msg) throw new Error("mail_message_not_found");
    const parsed = await simpleParser(msg.source);
    const att = (parsed.attachments || [])[Number(partId)];
    if (!att) throw new Error("mail_attachment_not_found");
    return { filename: att.filename || "attachment", contentType: att.contentType, content: att.content?.toString("base64") || "" };
  }

  async verifyAccount(account) {
    const imap = new ImapFlow({
      host: account.imap.host, port: account.imap.port, secure: account.imap.secure,
      auth: { user: account.username, pass: account.password }, logger: false,
    });
    await imap.connect().catch((e) => { throw this.normalizeMailError(e, "imap"); });
    await imap.logout().catch(() => {});
    const transporter = nodemailer.createTransport({
      host: account.smtp.host, port: account.smtp.port, secure: account.smtp.secure,
      auth: { user: account.username, pass: account.password },
    });
    await transporter.verify().catch((e) => { throw this.normalizeMailError(e, "smtp"); });
    return { ok: true };
  }

  async reconnectPool(accountId) {
    const account = await this.getAccount(accountId);
    await this.pool.connect(account);
    await this.pool.startIdle(accountId);
  }

  // ── Helpers ──

  normalizeMailError(error, channel) {
    const text = [error?.code, error?.response, error?.responseCode, error?.message].filter(Boolean).join(" ").toLowerCase();
    const next = new Error(error?.message || "mail_error");
    next.cause = error;
    if (text.includes("timeout") || text.includes("timed out") || error?.code === "ETIMEDOUT") { next.code = "MAIL_TIMEOUT"; return next; }
    if (text.includes("auth") || text.includes("invalid login") || text.includes("authentication")) { next.code = "MAIL_AUTH_FAILED"; return next; }
    if (text.includes("ssl") || text.includes("tls") || text.includes("certificate")) { next.code = "MAIL_TLS_ERROR"; return next; }
    if (error?.code === "ENOTFOUND" || error?.code === "ECONNREFUSED" || error?.code === "EHOSTUNREACH" || text.includes("getaddrinfo")) { next.code = channel === "smtp" ? "MAIL_SMTP_UNREACHABLE" : "MAIL_IMAP_UNREACHABLE"; return next; }
    next.code = channel === "smtp" ? "MAIL_SMTP_ERROR" : "MAIL_IMAP_ERROR";
    return next;
  }

  async readAccounts() {
    const accountsFile = await this.ensureStorage({ requireActive: false });
    if (!accountsFile) return [];
    const raw = await fs.readFile(accountsFile, "utf8");
    const parsed = JSON.parse(raw);
    return Array.isArray(parsed.accounts) ? parsed.accounts : [];
  }

  async writeAccounts(accounts) {
    const accountsFile = await this.ensureStorage();
    await fs.writeFile(accountsFile, JSON.stringify({ accounts }, null, 2), "utf8");
  }

  async getAccount(accountId) {
    const accounts = await this.readAccounts();
    const account = accounts.find((a) => a.id === accountId);
    if (!account) { const e = new Error("mail_account_not_found"); e.code = "ENOENT"; throw e; }
    // Decrypt password for connection use
    if (account._encrypted) { account.password = await this.vault.decryptForActiveUser(account._encrypted); }
    return account;
  }

  normalizeAccount(payload) {
    const id = payload.id || `mail-${crypto.randomUUID()}`;
    return {
      id, providerId: payload.providerId || "custom",
      label: String(payload.label || payload.email || "Mail account"),
      name: String(payload.name || payload.email || "Mail account"),
      email: String(payload.email || ""),
      username: String(payload.username || payload.email || ""),
      password: String(payload.password || ""),
      imap: { host: String(payload.imap?.host || ""), port: Number(payload.imap?.port || 993), secure: Boolean(payload.imap?.secure ?? true) },
      smtp: { host: String(payload.smtp?.host || ""), port: Number(payload.smtp?.port || 465), secure: Boolean(payload.smtp?.secure ?? true) },
    };
  }

  sanitizeAccount(account) {
    return { id: account.id, providerId: account.providerId, label: account.label, name: account.name, email: account.email, username: account.username, imap: account.imap, smtp: account.smtp };
  }

  formatAddressList(entries = []) { return entries.map((e) => this.formatSingleAddress(e)).join(", "); }
  formatAddressHeader(entries = []) { if (!Array.isArray(entries)) return ""; return entries.map((e) => this.formatSingleAddress(e)).join(", "); }
  formatSingleAddress(entry) {
    const name = entry.name ? String(entry.name).trim() : "";
    const address = entry.address ? String(entry.address).trim() : "";
    if (name && address) return `${name} <${address}>`;
    return address || name || "";
  }
}

module.exports = MailService;
