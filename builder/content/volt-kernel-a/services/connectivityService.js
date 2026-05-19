const { execFile } = require("node:child_process");
const { promisify } = require("node:util");
const fs = require("node:fs/promises");
const path = require("node:path");

const execFileAsync = promisify(execFile);

const AIRPORT_BIN = (() => {
  const candidates = [
    "/System/Library/PrivateFrameworks/Apple80211.framework/Versions/Current/Resources/airport",
    "/usr/local/bin/airport",
    "/opt/homebrew/bin/airport",
  ];
  for (const c of candidates) {
    try { require("fs").accessSync(c); return c; } catch {}
  }
  return "airport";
})();

function toId(v) { return String(v || "").trim().toLowerCase().replace(/\s+/g, "-").replace(/[^a-z0-9._-]/g, ""); }
function parseIfconfigAddress(out) { const m = /\binet\s+(\d+\.\d+\.\d+\.\d+)/.exec(out); return m ? m[1] : null; }
function parseAirportInfo(out) {
  const info = {};
  for (const l of String(out || "").split("\n")) { const m = /^\s*([^:]+):\s*(.+?)\s*$/.exec(l); if (m) info[m[1].trim()] = m[2].trim(); }
  return info;
}
function parseAirportScan(out, currentSsid) {
  const lines = String(out || "").split("\n").map((l) => l.trimEnd()).filter((l) => l && !/^warning:/i.test(l) && !/wireless diagnostics/i.test(l));
  if (lines.length <= 1) return [];
  return lines.slice(1).map((l) => {
    const signalMatch = l.match(/(-?\d+)\s+[\w:]{17}\s+/);
    const rssi = signalMatch ? parseInt(signalMatch[1], 10) : -90;
    const strength = Math.max(0, Math.min(100, 2 * (rssi + 100)));
    const pieces = l.split(/\s{2,}/).filter(Boolean);
    const [ _, ssid, ch ] = l.match(/^\s*\S+\s+\S+\s+(\S+)\s+(\S+)/) || [];
    const channel = ch || "";
    const band = parseInt(channel) > 48 ? "5 GHz" : "2.4 GHz";
    return { id: toId(pieces[0] || "Unknown"), label: pieces[0] || "Unknown", strength, secured: !/none/i.test(pieces[pieces.length - 1] || ""), connected: currentSsid === pieces[0], channel, band };
  });
}

const BLUEUTIL_BIN = (() => {
  const candidates = ["/opt/homebrew/bin/blueutil", "/usr/local/bin/blueutil", "/opt/local/bin/blueutil"];
  for (const c of candidates) {
    try { require("fs").accessSync(c); return c; } catch {}
  }
  return "blueutil";
})();
let _cachedBlueutil = null;

async function hasBlueutil(reset = false) {
  if (reset) _cachedBlueutil = null;
  if (_cachedBlueutil !== null) return _cachedBlueutil;
  try { require("fs").accessSync(BLUEUTIL_BIN); _cachedBlueutil = true; return true; } catch {}
  _cachedBlueutil = false;
  return false;
}

class ConnectivityService {
  constructor(logger, eventBus, userService, vaultService) {
    this.logger = logger;
    this.eventBus = eventBus;
    this.userService = userService;
    this.vault = vaultService;
    this.platform = process.platform;
    this._wifiInterfaceCache = null;
    this._wifiInterfaceCacheTime = 0;
    this._profilerCache = null;
    this._profilerCacheTime = 0;
  }

  async getStatus() {
    if (this.platform === "darwin") return this.readDarwinStatus();
    if (this.platform === "linux") return this.readLinuxStatus();
    return { platform: this.platform, wifiEnabled: false, bluetoothEnabled: false, currentNetworkId: "", currentNetworkLabel: "", currentAddress: "", networks: [], devices: [], capabilities: { wifiToggle: false, wifiConnect: false, bluetoothToggle: false, bluetoothConnect: false } };
  }

  // ── Wi-Fi ────────────────────────────────────────

  async toggleWifi(enabled) {
    if (this.platform === "linux") { await execFileAsync("nmcli", ["radio", "wifi", enabled ? "on" : "off"]); return this.getStatus(); }
    if (this.platform === "darwin") { const i = await this.detectDarwinWifiInterface(); await execFileAsync("/usr/sbin/networksetup", ["-setairportpower", i, enabled ? "on" : "off"]); return this.getStatus(); }
    throw Object.assign(new Error("unsupported_platform"), { code: "UNSUPPORTED_PLATFORM" });
  }

  async connectWifi(ssid, password) {
    if (!ssid) throw Object.assign(new Error("SSID required"), { code: "MAIL_AUTH_FAILED" });
    if (this.platform === "linux") {
      const args = ["device", "wifi", "connect", ssid];
      if (password) { args.push("password", password); }
      try { await execFileAsync("nmcli", args, { timeout: 15000 }); } catch (e) { throw this._normalizeError(e, "wifi"); }
      await this._saveNetwork(ssid, password || "");
      return this.getStatus();
    }
    if (this.platform === "darwin") {
      const wifiInterface = await this.detectDarwinWifiInterface();
      // Save password to macOS keychain first so networksetup can find it
      if (password) {
        try { await execFileAsync("security", ["add-generic-password", "-a", ssid, "-s", `airport-${ssid}`, "-w", password, "-U"], { timeout: 5000 }); } catch {}
      }
      try { await execFileAsync("/usr/sbin/networksetup", ["-setairportnetwork", wifiInterface, ssid], { timeout: 30000 }); } catch (e) { throw this._normalizeError(e, "wifi"); }
      // Save to local store for auto-connect
      await this._saveNetwork(ssid, password || "");
      return this.getStatus();
    }
    throw Object.assign(new Error("unsupported_platform"), { code: "UNSUPPORTED_PLATFORM" });
  }

  async disconnectWifi() {
    if (this.platform === "linux") {
      const i = await this._linuxWifiInterface();
      if (i) await execFileAsync("nmcli", ["device", "disconnect", i], { timeout: 10000 });
      return this.getStatus();
    }
    if (this.platform === "darwin") {
      await execFileAsync(AIRPORT_BIN, ["-z"], { timeout: 5000 }).catch(() => {});
      return this.getStatus();
    }
    throw Object.assign(new Error("unsupported_platform"), { code: "UNSUPPORTED_PLATFORM" });
  }

  async forgetNetwork(ssid) {
    const saved = await this._loadSavedNetworks();
    const next = saved.filter((n) => n.ssid !== ssid);
    const savedNetworksFile = this._savedNetworksFile();
    if (savedNetworksFile) {
      await fs.writeFile(savedNetworksFile, JSON.stringify({ networks: next }, null, 2), "utf8").catch(() => {});
    }
    if (this.platform === "darwin") {
      await execFileAsync("security", ["delete-generic-password", "-s", `airport-${ssid}`], { timeout: 5000 }).catch(() => {});
    }
    return { ok: true };
  }

  async getSavedNetworks() {
    const saved = await this._loadSavedNetworks();
    return saved.map((n) => ({ ssid: n.ssid }));
  }

  async autoConnect() {
    const saved = await this._loadSavedNetworks();
    if (saved.length === 0) return { ok: false };
    const status = await this.getStatus();
    if (status.wifiEnabled && status.currentNetworkId) return { ok: true, connected: status.currentNetworkId };
    for (const n of saved) {
      try {
        let pw = n.password || "";
        if (n._encrypted) pw = await this.vault.decryptForActiveUser(n._encrypted);
        await this.connectWifi(n.ssid, pw);
        return { ok: true, ssid: n.ssid };
      } catch {}
    }
    return { ok: false };
  }

  // ── Bluetooth ─────────────────────────────────────

  async toggleBluetooth(enabled) {
    if (this.platform === "darwin") {
      if (!(await hasBlueutil())) throw Object.assign(new Error("blueutil not installed"), { code: "UNSUPPORTED_PLATFORM" });
      await execFileAsync(BLUEUTIL_BIN, ["--power", enabled ? "1" : "0"], { timeout: 10000 });
      return this.getStatus();
    }
    if (this.platform === "linux") {
      await execFileAsync("bluetoothctl", ["power", enabled ? "on" : "off"], { timeout: 10000 });
      return this.getStatus();
    }
    throw Object.assign(new Error("unsupported_platform"), { code: "UNSUPPORTED_PLATFORM" });
  }

  async connectBluetooth(deviceId) {
    if (this.platform === "darwin") {
      if (!(await hasBlueutil())) throw Object.assign(new Error("blueutil not installed"), { code: "UNSUPPORTED_PLATFORM" });
      await execFileAsync(BLUEUTIL_BIN, ["--connect", deviceId], { timeout: 15000 });
      return this.getStatus();
    }
    if (this.platform === "linux") {
      await execFileAsync("bluetoothctl", ["connect", deviceId], { timeout: 15000 });
      return this.getStatus();
    }
    throw Object.assign(new Error("unsupported_platform"), { code: "UNSUPPORTED_PLATFORM" });
  }

  async disconnectBluetooth(deviceId) {
    if (this.platform === "darwin") {
      if (!(await hasBlueutil())) throw Object.assign(new Error("blueutil not installed"), { code: "UNSUPPORTED_PLATFORM" });
      await execFileAsync(BLUEUTIL_BIN, ["--disconnect", deviceId], { timeout: 10000 });
      return this.getStatus();
    }
    if (this.platform === "linux") {
      await execFileAsync("bluetoothctl", ["disconnect", deviceId], { timeout: 10000 });
      return this.getStatus();
    }
    throw Object.assign(new Error("unsupported_platform"), { code: "UNSUPPORTED_PLATFORM" });
  }

  async unpairBluetooth(deviceId) {
    if (this.platform === "darwin") {
      if (!(await hasBlueutil())) throw Object.assign(new Error("blueutil not installed"), { code: "UNSUPPORTED_PLATFORM" });
      await execFileAsync(BLUEUTIL_BIN, ["--unpair", deviceId], { timeout: 10000 });
      return this.getStatus();
    }
    if (this.platform === "linux") {
      await execFileAsync("bluetoothctl", ["remove", deviceId], { timeout: 10000 });
      return this.getStatus();
    }
    throw Object.assign(new Error("unsupported_platform"), { code: "UNSUPPORTED_PLATFORM" });
  }

  async scanBluetooth() {
    if (!(await hasBlueutil())) throw Object.assign(new Error("blueutil not installed"), { code: "UNSUPPORTED_PLATFORM" });
    if (this.platform === "darwin") {
      const { stdout } = await execFileAsync(BLUEUTIL_BIN, ["--inquiry", "--format", "json"], { timeout: 15000 });
      try { return JSON.parse(stdout); } catch { return []; }
    }
    if (this.platform === "linux") {
      await execFileAsync("bluetoothctl", ["scan", "on"], { timeout: 2000 }).catch(() => {});
      await new Promise((r) => setTimeout(r, 8000));
      const { stdout } = await execFileAsync("bluetoothctl", ["devices"], { timeout: 5000 });
      return stdout.split("\n").filter(Boolean).map((l) => {
        const parts = l.split(" ");
        return { address: parts[1] || "", name: parts.slice(2).join(" ") || "Unknown" };
      });
    }
    return [];
  }

  // ── Status readers ────────────────────────────────

  async readDarwinStatus() {
    const wifiInterface = await this.detectDarwinWifiInterface();

    // Use cached profiler output if recent
    let profResult;
    if (this._profilerCache && Date.now() - this._profilerCacheTime < 10000) {
      profResult = this._profilerCache;
    } else {
      profResult = await execFileAsync("/usr/sbin/system_profiler", ["SPAirPortDataType", "-json"], { timeout: 10000 }).catch(() => ({ stdout: "{}" }));
      this._profilerCache = profResult;
      this._profilerCacheTime = Date.now();
    }

    const [powerResult, netResult, ifResult, scanResult] = await Promise.all([
      execFileAsync("/usr/sbin/networksetup", ["-getairportpower", wifiInterface], { timeout: 5000 }).catch(() => ({ stdout: "" })),
      execFileAsync("/usr/sbin/networksetup", ["-getairportnetwork", wifiInterface], { timeout: 5000 }).catch(() => ({ stdout: "" })),
      execFileAsync("ifconfig", [wifiInterface], { timeout: 5000 }).catch(() => ({ stdout: "" })),
      execFileAsync(AIRPORT_BIN, ["-s"], { timeout: 10000 }).catch(() => ({ stdout: "" })),
    ]);

    const wifiEnabled = powerResult.stdout.toLowerCase().includes("on");
    let currentSsid = "";
    let rssi = 0;
    let channel = "";
    let networksFromProfiler = null;

    // Parse current network SSID
    if (wifiEnabled && netResult.stdout) {
      const m = netResult.stdout.match(/Current\s+Wi-Fi\s+Network:\s+(.+)/);
      if (m) currentSsid = m[1].trim();
    }

    // Parse signal/channel from system_profiler (correct JSON structure)
    if (wifiEnabled && profResult.stdout) {
      try {
        const prof = JSON.parse(profResult.stdout);
        const data = prof?.SPAirPortDataType?.[0]?.spairport_airport_interfaces?.[0];
        if (data) {
          // Current network info
          const current = data.spairport_current_network_information;
          if (current?._name) {
            currentSsid = current._name;
            const sn = (current.spairport_signal_noise || "").split("/")[0].trim();
            rssi = parseInt(sn) || 0;
            channel = (current.spairport_network_channel || "").split(" ")[0];
          }
          // Scan nearby networks
          const nearby = data.spairport_airport_other_local_wireless_networks;
          if (Array.isArray(nearby) && nearby.length > 0) {
            networksFromProfiler = nearby.map((n) => ({
              id: toId(n._name || ""),
              label: n._name || "Unknown",
              strength: Math.max(0, Math.min(100, 2 * ((parseInt((n.spairport_signal_noise || "").split("/")[0].trim()) || -90) + 100))),
              secured: !(n.spairport_security_mode || "").includes("none"),
              connected: currentSsid === n._name,
              channel: (n.spairport_network_channel || "").split(" ")[0],
              band: (n.spairport_network_channel || "").includes("5GHz") ? "5 GHz" : "2.4 GHz",
            }));
          }
        }
      } catch {}
    }

    // Parse IP address
    const address = parseIfconfigAddress(ifResult.stdout) || "";

    // Parse scan results
    let networks = networksFromProfiler || [];
    if (scanResult.stdout && !scanResult.stdout.includes("deprecated")) {
      networks = parseAirportScan(scanResult.stdout, currentSsid);
    }

    // Add current network to list if not already present
    if (currentSsid && !networks.some((n) => n.label === currentSsid)) {
      networks.unshift({
        id: toId(currentSsid),
        label: currentSsid,
        strength: Math.max(0, Math.min(100, 2 * (rssi + 100))),
        secured: true,
        connected: true,
        channel,
        band: parseInt(channel) > 48 ? "5 GHz" : "2.4 GHz",
      });
    }

    const btAvailable = await hasBlueutil(true);
    let bluetoothEnabled = false;
    let btDevices = [];
    if (btAvailable) {
      try {
        const { stdout: btPower } = await execFileAsync(BLUEUTIL_BIN, ["--power"], { timeout: 5000 });
        bluetoothEnabled = String(btPower).trim() === "1";
        const { stdout: pairedJson } = await execFileAsync(BLUEUTIL_BIN, ["--paired", "--format", "json"], { timeout: 5000 });
        btDevices = JSON.parse(pairedJson || "[]").map((d) => ({ id: d.address || d.identifier, label: d.name || d.address, connected: d.connected || false }));
      } catch {}
    }

    return {
      platform: "darwin",
      wifiEnabled,
      bluetoothEnabled,
      currentNetworkId: toId(currentSsid),
      currentNetworkLabel: currentSsid,
      currentAddress: address || "",
      networks,
      devices: btDevices,
      capabilities: { wifiToggle: true, wifiConnect: true, bluetoothToggle: btAvailable, bluetoothConnect: btAvailable },
      interfaceName: wifiInterface,
    };
  }

  async readLinuxStatus() {
    let wifiEnabled = false, currentSsid = "", address = "", networks = [];
    try {
      const { stdout: radio } = await execFileAsync("nmcli", ["-t", "-f", "WIFI", "general"], { timeout: 5000 });
      wifiEnabled = radio.trim().toLowerCase() === "enabled";
    } catch {}
    try {
      const { stdout: listRaw } = await execFileAsync("nmcli", ["-t", "-f", "ACTIVE,SSID,SIGNAL,SECURITY", "device", "wifi", "list"], { timeout: 10000 });
      networks = listRaw.split("\n").filter(Boolean).map((l) => {
        const [active, ssid, signal, security] = l.split(":");
        return { id: toId(ssid || ""), label: ssid || "Unknown", strength: parseInt(signal || "0") * 2, secured: security && !/^$/i.test(security), connected: active === "yes", channel: "", band: "" };
      }).filter((n) => n.label !== "Unknown" || n.strength > 0);
      const active = networks.find((n) => n.connected);
      if (active) currentSsid = active.label;
    } catch {}
    try {
      const { stdout: ipRaw } = await execFileAsync("nmcli", ["-t", "device", "show", "wifi"], { timeout: 5000 });
      const m = /IP4\.ADDRESS\[1\]:\s*([\d.]+)/.exec(ipRaw);
      if (m) address = m[1];
    } catch {}

    let btDevices = [];
    try {
      const { stdout: btRaw } = await execFileAsync("bluetoothctl", ["devices", "Paired"], { timeout: 5000 });
      btDevices = btRaw.split("\n").filter(Boolean).map((l) => {
        const parts = l.split(" ");
        return { id: parts[1] || "", label: parts.slice(2).join(" ") || "Unknown", connected: false };
      });
    } catch {}

    return { platform: "linux", wifiEnabled, currentNetworkId: toId(currentSsid), currentNetworkLabel: currentSsid, currentAddress: address, networks, devices: btDevices, capabilities: { wifiToggle: true, wifiConnect: true, bluetoothToggle: true, bluetoothConnect: true } };
  }

  // ── Helpers ───────────────────────────────────────

  async detectDarwinWifiInterface() {
    if (this._wifiInterfaceCache && Date.now() - this._wifiInterfaceCacheTime < 30000) {
      return this._wifiInterfaceCache;
    }
    try {
      const { stdout } = await execFileAsync("system_profiler", ["SPAirPortDataType", "-json"], { timeout: 10000 });
      const parsed = JSON.parse(stdout);
      const interfaces = parsed?.SPAirPortDataType?.[0]?.spairport_airport_interfaces;
      if (interfaces?.length > 0) {
        this._wifiInterfaceCache = interfaces[0]._name || "en0";
        this._wifiInterfaceCacheTime = Date.now();
        return this._wifiInterfaceCache;
      }
    } catch {}
    return "en0";
  }

  async _linuxWifiInterface() {
    try { const { stdout } = await execFileAsync("nmcli", ["-t", "-f", "DEVICE,TYPE", "device"], { timeout: 5000 }); const l = stdout.split("\n").find((l) => l.includes(":wifi")); return l ? l.split(":")[0] : null; } catch { return null; }
  }

  async _loadSavedNetworks() {
    const savedNetworksFile = this._savedNetworksFile();
    if (!savedNetworksFile) return [];
    try { const raw = await fs.readFile(savedNetworksFile, "utf8"); const p = JSON.parse(raw); return Array.isArray(p.networks) ? p.networks : []; } catch { return []; }
  }

  async _saveNetwork(ssid, password) {
    if (!this._savedNetworksFile()) return;
    const saved = await this._loadSavedNetworks();
    const existing = saved.findIndex((n) => n.ssid === ssid);
    const entry = { ssid, password: "", _encrypted: password ? await this.vault.encryptForActiveUser(password) : undefined, savedAt: new Date().toISOString() };
    if (existing >= 0) saved[existing] = entry;
    else saved.push(entry);
    const savedNetworksFile = this._savedNetworksFile();
    await fs.mkdir(path.dirname(savedNetworksFile), { recursive: true }).catch(() => {});
    await fs.writeFile(savedNetworksFile, JSON.stringify({ networks: saved }, null, 2), "utf8").catch(() => {});
  }

  _savedNetworksFile() {
    const active = this.userService?.getActiveUser?.();
    if (!active?.username) return null;
    const home = this.userService.resolveHome(active.username);
    return path.join(path.dirname(home), "wifi.json");
  }

  _normalizeError(error, channel) {
    const text = [error?.code, error?.message].filter(Boolean).join(" ").toLowerCase();
    const next = new Error(error?.message || "connection_failed");
    next.cause = error;
    if (text.includes("timeout")) { next.code = "TIMEOUT"; return next; }
    if (text.includes("auth") || text.includes("invalid") || text.includes("password")) { next.code = "AUTH_FAILED"; return next; }
    if (text.includes("enotfound") || text.includes("econnrefused") || text.includes("getaddrinfo")) { next.code = "UNREACHABLE"; return next; }
    next.code = "CONNECTION_FAILED"; return next;
  }
}

module.exports = ConnectivityService;
