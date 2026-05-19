const os = require("node:os");
const { execFile } = require("node:child_process");
const { promisify } = require("node:util");

const execFileAsync = promisify(execFile);

class EncryptionService {
  constructor(logger) {
    this.logger = logger;
  }

  async status() {
    if (os.platform() !== "linux") {
      return {
        available: false,
        encrypted: false,
        platform: os.platform(),
        note: "LUKS management is only available on Linux builds."
      };
    }

    try {
      const { stdout } = await execFileAsync("cryptsetup", ["status", "samaris-user"], { timeout: 10000 });
      return {
        available: true,
        encrypted: /is active/i.test(stdout),
        platform: os.platform(),
        note: stdout.trim()
      };
    } catch {
      return {
        available: true,
        encrypted: false,
        platform: os.platform(),
        note: "No active LUKS mapping detected."
      };
    }
  }

  async luksChangePassphrase() {
    return {
      ok: false,
      note: "Passphrase changes require privileged Linux execution and are not enabled in the local dev shell."
    };
  }

  async backupRecoveryPhrase() {
    return {
      ok: false,
      note: "Recovery phrase export is not enabled in the local dev shell."
    };
  }

  async integrityCheck() {
    return {
      ok: false,
      note: "Integrity checks are available only in the booted Linux environment."
    };
  }
}

module.exports = EncryptionService;
