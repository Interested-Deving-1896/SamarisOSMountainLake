const fs = require("node:fs/promises");
const os = require("node:os");
const path = require("node:path");
const { execFile } = require("node:child_process");
const { promisify } = require("node:util");

const execFileAsync = promisify(execFile);

function detectType(name) {
  const lower = String(name || "").toLowerCase();
  if (lower === "lo" || lower.startsWith("lo")) return "loopback";
  if (lower.includes("wl") || lower.includes("wi-fi") || lower.includes("wifi") || lower === "en0") return "wifi";
  return "ethernet";
}

class NetworkService {
  constructor(logger, connectivity, fileSystem) {
    this.logger = logger;
    this.connectivity = connectivity;
    this.fileSystem = fileSystem;
  }

  async configFilePath() {
    await this.fileSystem.ensureVirtualRoots();
    const root = path.join(this.fileSystem.rootPath, ".volt", "system");
    await fs.mkdir(root, { recursive: true });
    return path.join(root, "network-config.json");
  }

  async readConfig() {
    try {
      const raw = await fs.readFile(await this.configFilePath(), "utf8");
      return JSON.parse(raw);
    } catch {
      return {};
    }
  }

  async writeConfig(config) {
    await fs.writeFile(await this.configFilePath(), JSON.stringify(config, null, 2), "utf8");
  }

  async list() {
    const interfaces = os.networkInterfaces();
    const saved = await this.readConfig();
    const connectivity = await this.connectivity.getStatus().catch(() => null);

    return Object.entries(interfaces).map(([name, entries]) => {
      const ipv4 = (entries || []).find((entry) => entry.family === "IPv4") || null;
      const type = detectType(name);
      return {
        id: name,
        name,
        type,
        mac: ipv4?.mac || "",
        address: ipv4?.address || "",
        netmask: ipv4?.netmask || "",
        internal: Boolean(ipv4?.internal),
        mode: saved[name]?.mode || "dhcp",
        gateway: saved[name]?.gateway || "",
        dnsPrimary: saved[name]?.dnsPrimary || "",
        dnsSecondary: saved[name]?.dnsSecondary || "",
        connected:
          type === "wifi"
            ? Boolean(connectivity?.wifiEnabled && connectivity?.currentAddress && (connectivity.wifiInterface === name || !connectivity.wifiInterface))
            : Boolean(ipv4 && !ipv4.internal),
        label:
          type === "wifi"
            ? connectivity?.currentNetworkLabel || name
            : type === "loopback"
              ? "Loopback"
              : "Ethernet"
      };
    });
  }

  async setConfig(input = {}) {
    const interfaceId = String(input.interfaceId || input.id || "");
    if (!interfaceId) {
      throw Object.assign(new Error("missing_interface"), { code: "EINVAL" });
    }

    const saved = await this.readConfig();
    saved[interfaceId] = {
      mode: input.mode === "manual" ? "manual" : "dhcp",
      address: String(input.address || ""),
      netmask: String(input.netmask || ""),
      gateway: String(input.gateway || ""),
      dnsPrimary: String(input.dnsPrimary || ""),
      dnsSecondary: String(input.dnsSecondary || ""),
      updatedAt: new Date().toISOString()
    };
    await this.writeConfig(saved);

    let applied = false;
    let note = "Configuration saved locally.";

    if (process.platform === "linux") {
      try {
        if (saved[interfaceId].mode === "dhcp") {
          await execFileAsync("nmcli", ["device", "set", interfaceId, "managed", "yes"]);
        }
        applied = true;
        note = "Configuration saved. Live application support depends on NetworkManager profile availability.";
      } catch {
        applied = false;
        note = "Configuration saved, but live apply could not be completed on this host.";
      }
    }

    return {
      applied,
      note,
      interfaces: await this.list()
    };
  }
}

module.exports = NetworkService;
