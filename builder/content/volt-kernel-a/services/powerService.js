const { execFile } = require("node:child_process");
const { promisify } = require("node:util");

const execFileAsync = promisify(execFile);

class PowerService {
  constructor(logger, eventBus) {
    this.logger = logger;
    this.eventBus = eventBus;
    this.platform = process.platform;
  }

  async shutdown() {
    this.logger.info("power:shutdown", "Initiating system shutdown");
    try {
      if (this.platform === "linux") {
        await execFileAsync("systemctl", ["poweroff"], { timeout: 5000 });
      } else if (this.platform === "darwin") {
        await execFileAsync("shutdown", ["-h", "now"], { timeout: 5000 });
      }
      return { ok: true };
    } catch (err) {
      this.logger.error("power:shutdown", err.message);
      return { ok: false, error: err.message };
    }
  }

  async restart() {
    this.logger.info("power:restart", "Initiating system restart");
    try {
      if (this.platform === "linux") {
        await execFileAsync("systemctl", ["reboot"], { timeout: 5000 });
      } else if (this.platform === "darwin") {
        await execFileAsync("shutdown", ["-r", "now"], { timeout: 5000 });
      }
      return { ok: true };
    } catch (err) {
      this.logger.error("power:restart", err.message);
      return { ok: false, error: err.message };
    }
  }

  async sleep() {
    this.logger.info("power:sleep", "Initiating system suspend");
    try {
      if (this.platform === "linux") {
        await execFileAsync("systemctl", ["suspend"], { timeout: 5000 });
      } else if (this.platform === "darwin") {
        await execFileAsync("pmset", ["sleepnow"], { timeout: 5000 });
      }
      return { ok: true };
    } catch (err) {
      this.logger.error("power:sleep", err.message);
      return { ok: false, error: err.message };
    }
  }

  async lock() {
    this.logger.info("power:lock", "Locking session");
    try {
      if (this.platform === "linux") {
        await execFileAsync("loginctl", ["lock-session"], { timeout: 5000 });
      } else if (this.platform === "darwin") {
        await execFileAsync("pmset", ["displaysleepnow"], { timeout: 5000 });
      }
      return { ok: true };
    } catch (err) {
      this.logger.error("power:lock", err.message);
      return { ok: false, error: err.message };
    }
  }
}

module.exports = PowerService;
