import React from "react";
import { mailClient } from "../services/mailClient";
import { kernelClient } from "../../../os/kernel/kernelClient";
import { eventBus } from "../../../os/kernel/eventBus";
import type { MailAccount, MailComposerState, MailFolder, MailMessageDetail, MailMessageSummary, MailProvider } from "../types";

const DEFAULT_COMPOSER: MailComposerState = { to: "", cc: "", bcc: "", subject: "", text: "", attachments: [] };

function humanizeMailError(error: unknown) {
  const raw = error instanceof Error ? error.message : String(error || "");
  const m = raw.toLowerCase();
  if (m.includes("mail_auth_failed")) return "Authentication failed. Check your credentials. Gmail/Outlook require an app password, not your regular password.";
  if (m.includes("mail_imap_unreachable")) return "Cannot reach IMAP server. Verify the host and port.";
  if (m.includes("mail_smtp_unreachable")) return "Cannot reach SMTP server. Verify the host and port.";
  if (m.includes("mail_tls_error")) return "TLS/SSL configuration error. Try disabling TLS or changing the port.";
  if (m.includes("mail_invalid_security_port")) return "Invalid port/security combination. Port 465 requires TLS.";
  if (m.includes("mail_timeout")) return "Connection timed out. The server may be slow or unreachable.";
  if (m.includes("mail_account_not_found")) return "Mail account not found.";
  if (m.includes("kernel_connection")) return "Mail service is unavailable.";
  return raw || "Mail is unavailable.";
}

export function useMailApp() {
  const [providers, setProviders] = React.useState<MailProvider[]>([]);
  const [accounts, setAccounts] = React.useState<MailAccount[]>([]);
  const [selectedAccountId, setSelectedAccountId] = React.useState<string | null>(null);
  const [folders, setFolders] = React.useState<MailFolder[]>([]);
  const [selectedFolder, setSelectedFolder] = React.useState<string>("INBOX");
  const [messages, setMessages] = React.useState<MailMessageSummary[]>([]);
  const [selectedMessageUid, setSelectedMessageUid] = React.useState<number | null>(null);
  const [messageDetail, setMessageDetail] = React.useState<MailMessageDetail | null>(null);
  const [loading, setLoading] = React.useState(true);
  const [busy, setBusy] = React.useState(false);
  const [error, setError] = React.useState<string | null>(null);
  const [composerOpen, setComposerOpen] = React.useState(false);
  const [searchOpen, setSearchOpen] = React.useState(false);
  const [searchQuery, setSearchQuery] = React.useState("");
  const [composer, setComposer] = React.useState<MailComposerState>(DEFAULT_COMPOSER);

  const selectedAccount = accounts.find((a) => a.id === selectedAccountId) || null;

  const refreshAccounts = React.useCallback(async () => {
    const [p, a] = await Promise.all([mailClient.providers(), mailClient.accounts()]);
    setProviders(p); setAccounts(a);
    setSelectedAccountId((c) => c || a[0]?.id || null);
  }, []);

  const refreshFolders = React.useCallback(async (id: string) => {
    const f = await mailClient.folders(id);
    setFolders(f);
    setSelectedFolder((c) => f.some((x: MailFolder) => x.path === c) ? c : (f.find((x: MailFolder) => x.path.toUpperCase() === "INBOX")?.path || f[0]?.path || "INBOX"));
  }, []);

  const refreshMessages = React.useCallback(async (id: string, folder: string) => {
    const m = await mailClient.messages(id, folder);
    setMessages(m);
    setSelectedMessageUid((c) => c && m.some((x: MailMessageSummary) => x.uid === c) ? c : m[0]?.uid || null);
  }, []);

  const refreshDetail = React.useCallback(async (id: string, folder: string, uid: number) => {
    setMessageDetail(await mailClient.message(id, folder, uid));
  }, []);

  // Initial load
  React.useEffect(() => {
    let cancelled = false;
    setLoading(true);
    refreshAccounts().catch((e) => { if (!cancelled) setError(humanizeMailError(e)); }).finally(() => { if (!cancelled) setLoading(false); });
    return () => { cancelled = true; };
  }, [refreshAccounts]);

  // Load folders when account changes
  React.useEffect(() => {
    if (!selectedAccountId) return;
    setBusy(true);
    refreshFolders(selectedAccountId).catch((e) => setError(humanizeMailError(e))).finally(() => setBusy(false));
  }, [refreshFolders, selectedAccountId]);

  // Load messages when folder changes
  React.useEffect(() => {
    if (!selectedAccountId || !selectedFolder) return;
    setBusy(true);
    refreshMessages(selectedAccountId, selectedFolder).catch((e) => setError(humanizeMailError(e))).finally(() => setBusy(false));
  }, [refreshMessages, selectedAccountId, selectedFolder]);

  // Load detail when message changes
  React.useEffect(() => {
    if (!selectedAccountId || !selectedFolder || !selectedMessageUid) { setMessageDetail(null); return; }
    setBusy(true);
    refreshDetail(selectedAccountId, selectedFolder, selectedMessageUid).catch((e) => setError(humanizeMailError(e))).finally(() => setBusy(false));
  }, [refreshDetail, selectedAccountId, selectedFolder, selectedMessageUid]);

  // Listen for real-time new message events from kernel (IDLE push)
  React.useEffect(() => {
    const unsub = eventBus.on("mail:new-message", () => {
      if (selectedAccountId && selectedFolder) refreshMessages(selectedAccountId, selectedFolder).catch(() => {});
    });
    return () => unsub();
  }, [selectedAccountId, selectedFolder, refreshMessages]);

  // Keyboard shortcuts
  React.useEffect(() => {
    const handleKey = (e: KeyboardEvent) => {
      const target = e.target as HTMLElement;
      if (target.tagName === "INPUT" || target.tagName === "TEXTAREA" || target.isContentEditable) return;
      if (composerOpen) return;

      switch (e.key) {
        case "c": if (!e.metaKey) { e.preventDefault(); setComposerOpen(true); setComposer(DEFAULT_COMPOSER); } break;
        case "j": case "ArrowDown": e.preventDefault(); navigateList(1); break;
        case "k": case "ArrowUp": e.preventDefault(); navigateList(-1); break;
        case "Enter": e.preventDefault(); if (selectedMessageUid) setSelectedMessageUid(selectedMessageUid); break;
        case "r": if (!e.metaKey && selectedMessageUid) { e.preventDefault(); setComposerOpen(true); setComposer((c) => ({ ...c, subject: messageDetail ? `Re: ${messageDetail.subject}` : "", to: messageDetail?.from || "" })); } break;
        case "s": if (selectedMessageUid && selectedAccountId && selectedFolder) { e.preventDefault(); mailClient.flag(selectedAccountId, selectedFolder, selectedMessageUid, true).catch(() => {}); } break;
        case "#": if (selectedMessageUid && selectedAccountId && selectedFolder) { e.preventDefault(); mailClient.delete(selectedAccountId, selectedFolder, selectedMessageUid).then(() => refreshMessages(selectedAccountId, selectedFolder)).catch(() => {}); } break;
        case "/": e.preventDefault(); setSearchOpen(true); break;
        case "u": case "Escape": if (searchOpen) { setSearchOpen(false); setSearchQuery(""); } else if (selectedMessageUid) { setSelectedMessageUid(null); } break;
      }
    };
    window.addEventListener("keydown", handleKey);
    return () => window.removeEventListener("keydown", handleKey);
  }, [selectedMessageUid, selectedAccountId, selectedFolder, messageDetail, refreshMessages, composerOpen, searchOpen]);

  function navigateList(direction: number) {
    const idx = messages.findIndex((m) => m.uid === selectedMessageUid);
    const next = idx + direction;
    if (next >= 0 && next < messages.length) setSelectedMessageUid(messages[next].uid);
  }

  const saveAccount = React.useCallback(async (payload: Record<string, unknown>) => {
    setBusy(true); setError(null);
    try {
      const account = await mailClient.saveAccount(payload);
      await refreshAccounts();
      setSelectedAccountId(account.id);
    } catch (err) { setError(humanizeMailError(err)); } finally { setBusy(false); }
  }, [refreshAccounts]);

  const send = React.useCallback(async () => {
    if (!selectedAccountId) return;
    setBusy(true); setError(null);
    try {
      await mailClient.send(selectedAccountId, composer);
      setComposer(DEFAULT_COMPOSER); setComposerOpen(false);
      await refreshMessages(selectedAccountId, selectedFolder);
    } catch (err) { setError(humanizeMailError(err)); } finally { setBusy(false); }
  }, [composer, refreshMessages, selectedAccountId, selectedFolder]);

  const fullRefresh = React.useCallback(async () => {
    if (!selectedAccountId) return;
    setBusy(true); setError(null);
    try {
      await refreshFolders(selectedAccountId);
      await refreshMessages(selectedAccountId, selectedFolder);
    } catch (err) { setError(humanizeMailError(err)); } finally { setBusy(false); }
  }, [selectedAccountId, selectedFolder, refreshFolders, refreshMessages]);

  return {
    providers, accounts, selectedAccount, selectedAccountId, setSelectedAccountId,
    folders, selectedFolder, setSelectedFolder,
    messages, selectedMessageUid, setSelectedMessageUid, messageDetail,
    loading, busy, error, setError,
    composerOpen, setComposerOpen, composer, setComposer, saveAccount, send,
    searchOpen, setSearchOpen, searchQuery, setSearchQuery,
    refreshAccounts, fullRefresh,
  };
}
