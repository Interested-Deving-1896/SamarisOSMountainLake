const fs = require("node:fs/promises");
const path = require("node:path");
const os = require("node:os");
const { execFile } = require("node:child_process");
const { promisify } = require("node:util");

const execFileAsync = promisify(execFile);

const DEFAULT_STATE = {
  enabled: false,
  inboundPolicy: "allow",
  outboundPolicy: "allow",
  rules: [
    { id: "http", direction: "outbound", action: "allow", port: 80, label: "HTTP" },
    { id: "https", direction: "outbound", action: "allow", port: 443, label: "HTTPS" },
    { id: "ssh", direction: "inbound", action: "deny", port: 22, label: "SSH" }
  ]
};

class FirewallService {
  constructor(logger) {
    this.logger = logger;
    this.stateFile = path.resolve(__dirname, "../../.volt/system/firewall.json");
  }

  async load() {
    try {
      await fs.mkdir(path.dirname(this.stateFile), { recursive: true });
      const raw = await fs.readFile(this.stateFile, "utf8");
      return { ...DEFAULT_STATE, ...JSON.parse(raw) };
    } catch {
      return { ...DEFAULT_STATE };
    }
  }

  async save(state) {
    await fs.mkdir(path.dirname(this.stateFile), { recursive: true });
    await fs.writeFile(this.stateFile, JSON.stringify(state, null, 2), "utf8");
  }

  async readSystemStatus() {
    if (os.platform() === "darwin") {
      try {
        const { stdout } = await execFileAsync("/usr/libexec/ApplicationFirewall/socketfilterfw", ["--getglobalstate"], {
          timeout: 10000
        });
        return /enabled/i.test(stdout);
      } catch {
        return null;
      }
    }
    if (os.platform() === "linux") {
      try {
        const { stdout } = await execFileAsync("ufw", ["status"], { timeout: 10000 });
        return /Status:\s+active/i.test(stdout);
      } catch {
        return null;
      }
    }
    return null;
  }

  async list() {
    const state = await this.load();
    const systemEnabled = await this.readSystemStatus();
    return {
      ...state,
      systemEnabled,
      platform: os.platform()
    };
  }

  async setEnabled(enabled) {
    const state = await this.load();
    state.enabled = Boolean(enabled);
    await this.save(state);
    return this.list();
  }

  async addRule(rule = {}) {
    const state = await this.load();
    const entry = {
      id: rule.id || `rule-${Date.now()}`,
      direction: rule.direction || "inbound",
      action: rule.action || "allow",
      port: Number(rule.port || 0),
      label: rule.label || `Port ${rule.port || 0}`
    };
    state.rules = [...state.rules.filter((item) => item.id !== entry.id), entry];
    await this.save(state);
    return this.list();
  }

  async removeRule(ruleId) {
    const state = await this.load();
    state.rules = state.rules.filter((item) => item.id !== ruleId);
    await this.save(state);
    return this.list();
  }

  async setPolicy(direction, action) {
    const state = await this.load();
    if (direction === "inbound") state.inboundPolicy = action;
    if (direction === "outbound") state.outboundPolicy = action;
    await this.save(state);
    return this.list();
  }
}

module.exports = FirewallService;
