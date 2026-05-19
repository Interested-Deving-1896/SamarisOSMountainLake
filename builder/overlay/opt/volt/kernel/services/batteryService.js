const { execFile } = require("node:child_process");
const { promisify } = require("node:util");

const execFileAsync = promisify(execFile);

class BatteryService {
  constructor(logger) {
    this.logger = logger;
  }

  async getStatus() {
    if (process.platform === "darwin") {
      const { stdout } = await execFileAsync("pmset", ["-g", "batt"]).catch(() => ({ stdout: "" }));
      const output = String(stdout || "");
      const percentMatch = /(\d+)%/.exec(output);
      const sourceMatch = /Now drawing from '([^']+)'/i.exec(output);
      const source = sourceMatch?.[1] || "Unknown";
      const charging = /\bcharging\b/i.test(output);
      return {
        available: Boolean(percentMatch),
        percentage: percentMatch ? Number.parseInt(percentMatch[1], 10) : 0,
        charging,
        lowPower: percentMatch ? Number.parseInt(percentMatch[1], 10) < 15 : false,
        source: percentMatch ? source : source === "AC Power" ? "AC Power" : "No Battery Detected"
      };
    }

    if (process.platform === "linux") {
      const deviceList = await execFileAsync("upower", ["-e"]).catch(() => ({ stdout: "" }));
      const batteryDevice = String(deviceList.stdout || "")
        .split("\n")
        .find((line) => /battery/i.test(line));
      if (!batteryDevice) {
        const acDevice = String(deviceList.stdout || "")
          .split("\n")
          .find((line) => /line_power|ac/i.test(line));
        return { available: false, percentage: 0, charging: false, lowPower: false, source: acDevice ? "AC Power" : "No Battery Detected" };
      }
      const { stdout } = await execFileAsync("upower", ["-i", batteryDevice.trim()]).catch(() => ({ stdout: "" }));
      const output = String(stdout || "");
      const percentMatch = /percentage:\s+(\d+)%/i.exec(output);
      const charging = /state:\s+charging/i.test(output);
      const fullyCharged = /state:\s+fully-charged/i.test(output);
      return {
        available: Boolean(percentMatch),
        percentage: percentMatch ? Number.parseInt(percentMatch[1], 10) : 0,
        charging,
        lowPower: percentMatch ? Number.parseInt(percentMatch[1], 10) < 15 : false,
        source: fullyCharged ? "Fully Charged" : charging ? "AC Power" : "Battery"
      };
    }

    return { available: false, percentage: 0, charging: false, lowPower: false, source: "No Battery Detected" };
  }
}

module.exports = BatteryService;
