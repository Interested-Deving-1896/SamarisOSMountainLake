const fs = require("node:fs/promises");
const path = require("node:path");
const os = require("node:os");
const { execFile } = require("node:child_process");
const { promisify } = require("node:util");

const execFileAsync = promisify(execFile);

class PrintService {
  constructor(logger, fileSystem) {
    this.logger = logger;
    this.fileSystem = fileSystem;
    this.stateFile = path.resolve(__dirname, "../../.volt/system/print-state.json");
  }

  async loadState() {
    try {
      await fs.mkdir(path.dirname(this.stateFile), { recursive: true });
      const raw = await fs.readFile(this.stateFile, "utf8");
      return JSON.parse(raw);
    } catch {
      return { printers: [] };
    }
  }

  async saveState(state) {
    await fs.mkdir(path.dirname(this.stateFile), { recursive: true });
    await fs.writeFile(this.stateFile, JSON.stringify(state, null, 2), "utf8");
  }

  async safeExec(command, args = []) {
    try {
      const { stdout, stderr } = await execFileAsync(command, args, { timeout: 15000 });
      return { ok: true, stdout: stdout || "", stderr: stderr || "" };
    } catch (error) {
      return {
        ok: false,
        stdout: error.stdout || "",
        stderr: error.stderr || "",
        message: error.message || `${command}_failed`
      };
    }
  }

  async list() {
    const state = await this.loadState();
    const queue = [];
    const detected = [];

    if (os.platform() === "darwin" || os.platform() === "linux") {
      const printers = await this.safeExec("lpstat", ["-p", "-d"]);
      const jobs = await this.safeExec("lpstat", ["-o"]);

      if (printers.ok) {
        for (const line of printers.stdout.split("\n")) {
          const match = line.match(/^printer\s+(\S+)\s+/);
          if (match) {
            detected.push({
              id: match[1],
              name: match[1],
              status: /disabled/i.test(line) ? "disabled" : "ready",
              source: "system"
            });
          }
        }
      }

      if (jobs.ok) {
        for (const line of jobs.stdout.split("\n")) {
          const match = line.match(/^(\S+)-(\d+)\s+/);
          if (match) {
            queue.push({
              printerId: match[1],
              jobId: `${match[1]}-${match[2]}`,
              summary: line.trim()
            });
          }
        }
      }
    }

    const merged = [...detected];
    for (const printer of state.printers || []) {
      if (!merged.some((entry) => entry.id === printer.id)) {
        merged.push(printer);
      }
    }

    return {
      printers: merged,
      queue
    };
  }

  async add(config = {}) {
    const state = await this.loadState();
    const printer = {
      id: config.id || `printer-${Date.now()}`,
      name: config.name || "Custom Printer",
      uri: config.uri || "",
      protocol: config.protocol || "ipp",
      status: "saved",
      source: "custom"
    };
    state.printers = [...(state.printers || []).filter((entry) => entry.id !== printer.id), printer];
    await this.saveState(state);
    return {
      ok: true,
      note: os.platform() === "darwin" ? "Printer saved. System-level add may require admin rights." : "Printer saved.",
      printer
    };
  }

  async remove(printerId) {
    const state = await this.loadState();
    state.printers = (state.printers || []).filter((entry) => entry.id !== printerId);
    await this.saveState(state);
    return { ok: true };
  }

  async submit(payload = {}) {
    const targetPath = payload.path || "";
    const printerId = payload.printerId || "";
    const options = payload.options || {};
    const result = { ok: false, note: "", log: "" };

    try {
      const resolved = this.fileSystem.toActualPath(targetPath);
      if (resolved.virtualPath && (os.platform() === "darwin" || os.platform() === "linux")) {
        const args = ["-d", printerId];
        if (options.paper) args.push("-o", `media=${options.paper}`);
        if (options.quality) args.push("-o", `print-quality=${options.quality}`);
        if (options.colorMode) args.push("-o", `ColorModel=${options.colorMode}`);
        args.push(resolved.actualPath);
        const response = await this.safeExec("lp", args);
        result.ok = response.ok;
        result.note = response.ok ? "Print job submitted." : "Could not submit print job.";
        result.log = [response.stdout, response.stderr, response.message].filter(Boolean).join("\n");
        return result;
      }
    } catch (error) {
      result.log = error.message || String(error);
    }

    result.note = "Printing is not available on this platform in the local dev shell.";
    return result;
  }
}

module.exports = PrintService;
