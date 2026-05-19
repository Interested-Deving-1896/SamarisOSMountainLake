function p(msg) { return msg.payload || msg.data || {}; }

async function handle(message, kernel) {
  const payload = p(message);
  const mail = kernel.mail;

  switch (message.type) {
    case "mail.providers": return { type: "mail.providers.result", data: mail.providers() };
    case "mail.accounts": return { type: "mail.accounts.result", data: await mail.listAccounts() };
    case "mail.account.save": return { type: "mail.account.save.result", data: await mail.saveAccount(payload) };
    case "mail.account.delete": return { type: "mail.account.delete.result", data: await mail.deleteAccount(payload.accountId) };
    case "mail.account.verify": return { type: "mail.account.verify.result", data: await mail.verifyAccount(payload) };
    case "mail.folders": return { type: "mail.folders.result", data: await mail.listFolders(payload.accountId) };
    case "mail.messages": return { type: "mail.messages.result", data: await mail.listMessages(payload.accountId, payload.folder, payload.limit) };
    case "mail.message": return { type: "mail.message.result", data: await mail.getMessage(payload.accountId, payload.folder, payload.uid) };
    case "mail.send": return { type: "mail.send.result", data: await mail.sendMessage(payload.accountId, payload) };
    case "mail.search": return { type: "mail.search.result", data: await mail.searchMessages(payload.accountId, payload.folder, payload.query || {}) };
    case "mail.move": return { type: "mail.move.result", data: await mail.moveMessage(payload.accountId, payload.folder, payload.uid, payload.destFolder) };
    case "mail.delete": return { type: "mail.delete.result", data: await mail.deleteMessage(payload.accountId, payload.folder, payload.uid) };
    case "mail.flag": return { type: "mail.flag.result", data: await mail.flagMessage(payload.accountId, payload.folder, payload.uid, payload.flagged) };
    case "mail.seen": return { type: "mail.seen.result", data: await mail.markSeen(payload.accountId, payload.folder, payload.uid, payload.seen) };
    case "mail.attachment": return { type: "mail.attachment.result", data: await mail.getAttachment(payload.accountId, payload.folder, payload.uid, payload.partId) };
    case "mail.reconnect": return { type: "mail.reconnect.result", data: await mail.reconnectPool(payload.accountId) };
    case "mail.idle.status": return { type: "mail.idle.status.result", data: mail.pool.getStatus(payload.accountId) };
    default: return { type: "error", data: "unknown_type" };
  }
}

module.exports = { handle };
