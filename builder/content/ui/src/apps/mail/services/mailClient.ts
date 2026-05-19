import { kernelClient } from "../../../os/kernel/kernelClient";
import type { MailAccount, MailComposerState, MailFolder, MailMessageDetail, MailMessageSummary, MailProvider, MailAttachment } from "../types";

export const mailClient = {
  async providers() {
    const r = await kernelClient.request<MailProvider[]>({ type: "mail.providers", data: {} }, { timeoutMs: 20000 });
    return r.data || [];
  },

  async accounts() {
    const r = await kernelClient.request<MailAccount[]>({ type: "mail.accounts", data: {} }, { timeoutMs: 20000 });
    return r.data || [];
  },

  async saveAccount(account: Record<string, unknown>) {
    const r = await kernelClient.request<MailAccount>({ type: "mail.account.save", data: account }, { timeoutMs: 40000 });
    return r.data as MailAccount;
  },

  async deleteAccount(accountId: string) {
    await kernelClient.request({ type: "mail.account.delete", data: { accountId } }, { timeoutMs: 20000 });
  },

  async verifyAccount(account: Record<string, unknown>) {
    const r = await kernelClient.request<{ ok: boolean }>({ type: "mail.account.verify", data: account }, { timeoutMs: 40000 });
    return r.data;
  },

  async folders(accountId: string) {
    const r = await kernelClient.request<MailFolder[]>({ type: "mail.folders", data: { accountId } }, { timeoutMs: 30000 });
    return r.data || [];
  },

  async messages(accountId: string, folder: string) {
    const r = await kernelClient.request<{ folder: string; messages: MailMessageSummary[] }>(
      { type: "mail.messages", data: { accountId, folder, limit: 80 } }, { timeoutMs: 40000 }
    );
    return r.data?.messages || [];
  },

  async message(accountId: string, folder: string, uid: number) {
    const r = await kernelClient.request<MailMessageDetail>(
      { type: "mail.message", data: { accountId, folder, uid } }, { timeoutMs: 40000 }
    );
    return r.data as MailMessageDetail;
  },

  async send(accountId: string, payload: MailComposerState) {
    const r = await kernelClient.request(
      { type: "mail.send", data: { accountId, ...payload, to: payload.to.split(",").map((s: string) => s.trim()).filter(Boolean), cc: payload.cc.split(",").map((s: string) => s.trim()).filter(Boolean), bcc: payload.bcc.split(",").map((s: string) => s.trim()).filter(Boolean) } },
      { timeoutMs: 40000 }
    );
    return r.data;
  },

  async search(accountId: string, folder: string, query: { text?: string; from?: string; subject?: string }) {
    const r = await kernelClient.request<{ folder: string; messages: MailMessageSummary[] }>(
      { type: "mail.search", data: { accountId, folder, query } }, { timeoutMs: 40000 }
    );
    return r.data?.messages || [];
  },

  async move(accountId: string, folder: string, uid: number, destFolder: string) {
    const r = await kernelClient.request<{ ok: boolean }>({ type: "mail.move", data: { accountId, folder, uid, destFolder } }, { timeoutMs: 30000 });
    return r.data;
  },

  async delete(accountId: string, folder: string, uid: number) {
    const r = await kernelClient.request<{ ok: boolean }>({ type: "mail.delete", data: { accountId, folder, uid } }, { timeoutMs: 30000 });
    return r.data;
  },

  async flag(accountId: string, folder: string, uid: number, flagged: boolean) {
    const r = await kernelClient.request<{ ok: boolean }>({ type: "mail.flag", data: { accountId, folder, uid, flagged } }, { timeoutMs: 30000 });
    return r.data;
  },

  async seen(accountId: string, folder: string, uid: number, seen: boolean) {
    const r = await kernelClient.request<{ ok: boolean }>({ type: "mail.seen", data: { accountId, folder, uid, seen } }, { timeoutMs: 30000 });
    return r.data;
  },

  async attachment(accountId: string, folder: string, uid: number, partId: number) {
    const r = await kernelClient.request<MailAttachment>({ type: "mail.attachment", data: { accountId, folder, uid, partId } }, { timeoutMs: 40000 });
    return r.data as MailAttachment;
  },
};
