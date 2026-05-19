const os = require("node:os");
const { execFile } = require("node:child_process");
const { promisify } = require("node:util");

const execFileAsync = promisify(execFile);

class SystemMetricsService {
  constructor(logger, eventBus) {
    this.logger = logger;
    this.eventBus = eventBus;
    this.platform = process.platform;
    this._interval = null;
    this._prevCpu = null;
  }

  start(intervalMs = 15000) {
    if (this._interval) return;
    // First emit after 1s, then at interval
    setTimeout(() => this._tick(), 1000);
    this._interval = setInterval(() => this._tick(), intervalMs);
  }

  _tick() {
    this.getMetrics().then((metrics) => {
      if (metrics.cpu || metrics.memory || metrics.disk) {
        this.eventBus.emit("system:metrics", metrics);
      }
    }).catch(() => {});
  }

  stop() {
    if (this._interval) {
      clearInterval(this._interval);
      this._interval = null;
    }
  }

  async getMetrics() {
    const [cpu, memory, disk] = await Promise.all([
      this._getCPU(),
      this._getMemory(),
      this._getDisk(),
    ]);
    return { cpu, memory, disk };
  }

  async _getCPU() {
    try {
      if (this.platform === "linux") {
        const { stdout } = await execFileAsync("sh", ["-c", "grep '^cpu ' /proc/stat | awk '{print ($2+$3+$4+$5+$6+$7+$8+$9+$10)}'"], { timeout: 2000 });
        const idle = await execFileAsync("sh", ["-c", "grep '^cpu ' /proc/stat | awk '{print $5}'"], { timeout: 2000 });
        return { raw: stdout.trim(), idle: idle.stdout.trim() };
      }
      if (this.platform === "darwin") {
        const cpus = os.cpus();
        const total = cpus.reduce((s, c) => s + Object.values(c.times).reduce((a, b) => a + b, 0), 0);
        const idle = cpus.reduce((s, c) => s + c.times.idle, 0);
        const now = { total, idle };
        if (this._prevCpu) {
          const dTotal = now.total - this._prevCpu.total;
          const dIdle = now.idle - this._prevCpu.idle;
          const usage = dTotal > 0 ? ((1 - dIdle / dTotal) * 100).toFixed(1) : "0";
          this._prevCpu = now;
          return { usagePercent: usage, cores: cpus.length };
        }
        this._prevCpu = now;
        return { usagePercent: "0", cores: cpus.length };
      }
      return null;
    } catch { return null; }
  }

  async _getMemory() {
    try {
      const total = os.totalmem();
      const free = os.freemem();
      const used = total - free;
      return { total, used, free, usagePercent: total > 0 ? ((used / total) * 100).toFixed(1) : "0" };
    } catch { return null; }
  }

  async _getDisk() {
    try {
      if (this.platform === "linux") {
        const { stdout } = await execFileAsync("df", ["-B1", "/"], { timeout: 2000 });
        const parts = stdout.trim().split("\n").filter(Boolean);
        if (parts.length >= 2) {
          const cols = parts[1].split(/\s+/);
          return { mount: "/", total: parseInt(cols[1], 10), used: parseInt(cols[2], 10), free: parseInt(cols[3], 10), usagePercent: parseInt(cols[1], 10) > 0 ? ((parseInt(cols[2], 10) / parseInt(cols[1], 10)) * 100).toFixed(1) : "0" };
        }
      }
      if (this.platform === "darwin") {
        const { stdout } = await execFileAsync("df", ["-k", "/"], { timeout: 2000 });
        const parts = stdout.trim().split("\n").filter(Boolean);
        if (parts.length >= 2) {
          const cols = parts[1].split(/\s+/);
          const total = parseInt(cols[1], 10) * 1024;
          const used = parseInt(cols[2], 10) * 1024;
          const free = parseInt(cols[3], 10) * 1024;
          return { mount: "/", total, used, free, usagePercent: total > 0 ? ((used / total) * 100).toFixed(1) : "0" };
        }
      }
      return null;
    } catch { return null; }
  }
}

module.exports = SystemMetricsService;
