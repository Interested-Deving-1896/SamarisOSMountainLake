const { execFile } = require("node:child_process");
const { promisify } = require("node:util");

const execFileAsync = promisify(execFile);

function parseMacVolume(value) {
  const number = Number.parseInt(String(value || "").trim(), 10);
  return Number.isFinite(number) ? Math.max(0, Math.min(100, number)) : 0;
}

function parseWpctlVolume(output) {
  const match = /Volume:\s+([0-9.]+)/i.exec(String(output || ""));
  if (!match) return 50;
  return Math.max(0, Math.min(100, Math.round(Number.parseFloat(match[1]) * 100)));
}

class AudioService {
  constructor(logger) {
    this.logger = logger;
  }

  async getStatus() {
    if (process.platform === "darwin") {
      const [volumeRaw, muteRaw] = await Promise.all([
        execFileAsync("osascript", ["-e", "output volume of (get volume settings)"]).catch(() => ({ stdout: "50" })),
        execFileAsync("osascript", ["-e", "output muted of (get volume settings)"]).catch(() => ({ stdout: "false" }))
      ]);
      return {
        volume: parseMacVolume(volumeRaw.stdout),
        muted: /true/i.test(String(muteRaw.stdout || "")),
        outputs: [{ id: "system-default", label: "System Default", active: true }],
        activeOutputId: "system-default"
      };
    }

    if (process.platform === "linux") {
      const raw = await execFileAsync("wpctl", ["get-volume", "@DEFAULT_AUDIO_SINK@"]).catch(() => ({ stdout: "" }));
      return {
        volume: parseWpctlVolume(raw.stdout),
        muted: /\bMUTED\b/i.test(String(raw.stdout || "")),
        outputs: [{ id: "default-sink", label: "Default Output", active: true }],
        activeOutputId: "default-sink"
      };
    }

    return {
      volume: 50,
      muted: false,
      outputs: [{ id: "default", label: "Default Output", active: true }],
      activeOutputId: "default"
    };
  }

  async setVolume(input = {}) {
    const volume = Math.max(0, Math.min(100, Number.parseInt(String(input.volume || 0), 10) || 0));

    if (process.platform === "darwin") {
      await execFileAsync("osascript", ["-e", `set volume output volume ${volume}`]).catch(() => {});
      return this.getStatus();
    }

    if (process.platform === "linux") {
      await execFileAsync("wpctl", ["set-volume", "@DEFAULT_AUDIO_SINK@", `${volume}%`]).catch(async () => {
        await execFileAsync("amixer", ["set", "Master", `${volume}%`]).catch(() => {});
      });
      return this.getStatus();
    }

    return {
      ...(await this.getStatus()),
      volume
    };
  }

  async setOutput(input = {}) {
    const outputId = String(input.outputId || "");
    if (!outputId) {
      throw Object.assign(new Error("missing_output"), { code: "EINVAL" });
    }

    if (process.platform === "linux") {
      await execFileAsync("wpctl", ["set-default", outputId]).catch(() => {});
    }

    const status = await this.getStatus();
    return {
      ...status,
      activeOutputId: outputId || status.activeOutputId
    };
  }
}

module.exports = AudioService;
