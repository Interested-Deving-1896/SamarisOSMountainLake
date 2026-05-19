function payloadOf(message) {
  return message.payload || message.data || {};
}

async function handle(message, kernel) {
  switch (message.type) {
    case "encryption.status":
      return { type: "encryption.status.result", data: await kernel.encryption.status() };
    case "encryption.luksChangePassphrase":
      return { type: "encryption.luksChangePassphrase.result", data: await kernel.encryption.luksChangePassphrase() };
    case "encryption.backupRecoveryPhrase":
      return { type: "encryption.backupRecoveryPhrase.result", data: await kernel.encryption.backupRecoveryPhrase() };
    case "encryption.integrityCheck":
      return { type: "encryption.integrityCheck.result", data: await kernel.encryption.integrityCheck() };
    default:
      return { type: "error", data: "unknown_type" };
  }
}

module.exports = { handle };
