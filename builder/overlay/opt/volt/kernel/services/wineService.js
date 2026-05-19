const fs = require("node:fs/promises");
const path = require("node:path");
const os = require("node:os");
const { spawn, execFile } = require("node:child_process");
const { promisify } = require("node:util");

const execFileAsync = promisify(execFile);
const MAX_LOG_LINES = 400;
const ALLOWED_COMMANDS = new Set(["wine", "wine64", "winecfg"]);
const DEFAULT_PREFIX_NAME = "default";
const VERSION_CACHE_TTL = 60000;

class WineService {
  constructor(logger, fileSystem) {
    this.logger = logger;
    this.fileSystem = fileSystem;
    this.stateFile = path.join(os.homedir(), ".volt", "system", "wine-state.json");
    this.prefixRoot = path.join(os.homedir(), ".samaris", "wine", "prefixes");
    this.sessions = new Map();
    this._versionCache = { version: null, timestamp: 0 };
  }

  async checkWineInstalled() {
    return (await this.resolveCommand()) !== null;
  }

  async getWineVersion() {
    if (this._versionCache.version && Date.now() - this._versionCache.timestamp < VERSION_CACHE_TTL) {
      return this._versionCache.version;
    }
    const command = await this.resolveCommand();
    if (!command) {
      this._versionCache = { version: null, timestamp: Date.now() };
      return null;
    }
    const result = await this.runNativeCommand(command, ["--version"], { timeoutMs: 8000 });
    const version = result.ok ? String(result.stdout || "").trim() || null : null;
    this._versionCache = { version, timestamp: Date.now() };
    return version;
  }

  defaultPrefixPath(prefix = DEFAULT_PREFIX_NAME) {
    return path.join(this.prefixRoot, prefix);
  }

  async status() {
    const state = await this.loadState();
    const installed = await this.checkWineInstalled();
    const version = installed ? await this.getWineVersion() : null;
    return {
      installed,
      version,
      prefixPath: this.defaultPrefixPath(),
      recentExecutables: Array.isArray(state.recentExecutables) ? state.recentExecutables : [],
      sessions: Array.from(this.sessions.values()).map((session) => this.serializeSession(session))
    };
  }

  async launchExe(exePath, options = {}, stream) {
    const command = await this.resolveCommand();
    if (!command) {
      const error = new Error("wine_not_installed");
      error.code = "wine_not_installed";
      throw error;
    }

    const resolved = await this.validateExePath(exePath);
    const prefixName = this.validatePrefixName(options.prefix || DEFAULT_PREFIX_NAME);
    const prefixPath = await this.ensurePrefix(prefixName);

    const session = this.createSession({
      command,
      exePath: resolved.virtualPath,
      actualPath: resolved.actualPath,
      prefixName,
      prefixPath,
      kind: "exe"
    });

    this.sessions.set(session.sessionId, session);
    await this.pushRecentExecutable(resolved.virtualPath);
    this.emitUpdate(stream, session);

    const child = spawn(command, [resolved.actualPath], {
      cwd: path.dirname(resolved.actualPath),
      env: {
        ...process.env,
        WINEPREFIX: prefixPath
      },
      stdio: ["ignore", "pipe", "pipe"]
    });

    session.child = child;
    session.pid = child.pid || null;
    session.status = "running";
    this.emitUpdate(stream, session);

    child.stdout.on("data", (chunk) => {
      this.appendLog(session, "stdout", chunk, stream);
    });
    child.stderr.on("data", (chunk) => {
      this.appendLog(session, "stderr", chunk, stream);
    });
    child.on("error", (error) => {
      session.status = "failed";
      session.endedAt = new Date().toISOString();
      session.lastMessage = error.message || "wine_launch_failed";
      this.appendLog(session, "stderr", error.message || "wine_launch_failed", stream);
      this.emitUpdate(stream, session);
    });
    child.on("exit", (code, signal) => {
      if (session.status === "failed") return;
      session.child = null;
      session.exitCode = typeof code === "number" ? code : null;
      session.endedAt = new Date().toISOString();
      if (session.status === "stopping") {
        session.status = "exited";
        session.lastMessage = signal ? `stopped:${signal}` : "stopped";
      } else {
        session.status = code === 0 ? "exited" : "failed";
        session.lastMessage = signal ? `signal:${signal}` : `exit:${code ?? "unknown"}`;
      }
      this.emitUpdate(stream, session);
    });

    this.logger.info("wine:launch", {
      sessionId: session.sessionId,
      exePath: session.exePath,
      command
    });

    return this.serializeSession(session);
  }

  async launchConfig(options = {}, stream) {
    const command = await this.resolveCommand();
    if (!command) {
      const error = new Error("wine_not_installed");
      error.code = "wine_not_installed";
      throw error;
    }

    const prefixName = this.validatePrefixName(options.prefix || DEFAULT_PREFIX_NAME);
    const prefixPath = await this.ensurePrefix(prefixName);

    const session = this.createSession({
      command: "winecfg",
      exePath: "winecfg",
      actualPath: "",
      prefixName,
      prefixPath,
      kind: "config"
    });

    this.sessions.set(session.sessionId, session);
    this.emitUpdate(stream, session);

    const child = spawn(command, ["winecfg"], {
      cwd: os.homedir(),
      env: {
        ...process.env,
        WINEPREFIX: prefixPath
      },
      stdio: ["ignore", "pipe", "pipe"]
    });

    session.child = child;
    session.pid = child.pid || null;
    session.status = "running";
    this.emitUpdate(stream, session);

    child.stdout.on("data", (chunk) => {
      this.appendLog(session, "stdout", chunk, stream);
    });
    child.stderr.on("data", (chunk) => {
      this.appendLog(session, "stderr", chunk, stream);
    });
    child.on("error", (error) => {
      session.status = "failed";
      session.endedAt = new Date().toISOString();
      session.lastMessage = error.message || "winecfg_failed";
      this.appendLog(session, "stderr", error.message || "winecfg_failed", stream);
      this.emitUpdate(stream, session);
    });
    child.on("exit", (code, signal) => {
      session.child = null;
      session.exitCode = typeof code === "number" ? code : null;
      session.endedAt = new Date().toISOString();
      session.status = code === 0 ? "exited" : "failed";
      session.lastMessage = signal ? `signal:${signal}` : `exit:${code ?? "unknown"}`;
      this.emitUpdate(stream, session);
    });

    this.logger.info("wine:config", {
      sessionId: session.sessionId,
      command: "winecfg"
    });

    return this.serializeSession(session);
  }

  async stopSession(sessionId) {
    const session = this.sessions.get(String(sessionId || ""));
    if (!session) {
      const error = new Error("wine_session_not_found");
      error.code = "wine_session_not_found";
      throw error;
    }

    if (!session.child || session.child.exitCode !== null) {
      return {
        ok: true,
        session: this.serializeSession(session)
      };
    }

    session.status = "stopping";
    try {
      session.child.kill("SIGTERM");
    } catch {}

    return {
      ok: true,
      session: this.serializeSession(session)
    };
  }

  async getSessionLogs(sessionId) {
    const session = this.sessions.get(String(sessionId || ""));
    if (!session) {
      const error = new Error("wine_session_not_found");
      error.code = "wine_session_not_found";
      throw error;
    }
    return {
      sessionId: session.sessionId,
      logs: [...session.logs]
    };
  }

  async runNativeCommand(command, args = [], options = {}) {
    if (!ALLOWED_COMMANDS.has(command)) {
      const error = new Error("wine_command_not_allowed");
      error.code = "wine_command_not_allowed";
      throw error;
    }

    try {
      const { stdout, stderr } = await execFileAsync(command, args, {
        env: options.env || process.env,
        timeout: options.timeoutMs || 15000
      });
      return {
        ok: true,
        stdout: stdout || "",
        stderr: stderr || ""
      };
    } catch (error) {
      if (error && error.code === "ENOENT") {
        return {
          ok: false,
          stdout: "",
          stderr: "",
          message: "command_not_found"
        };
      }
      return {
        ok: false,
        stdout: error.stdout || "",
        stderr: error.stderr || "",
        message: error.message || `${command}_failed`
      };
    }
  }

  async resolveCommand() {
    for (const command of ["wine", "wine64"]) {
      const result = await this.runNativeCommand(command, ["--version"], { timeoutMs: 8000 });
      if (result.ok) return command;
    }
    return null;
  }

  async validateExePath(targetPath) {
    const normalized = this.fileSystem.toVirtualPath(String(targetPath || ""));
    if (!normalized.startsWith("/User/")) {
      const error = new Error("wine_path_not_allowed");
      error.code = "wine_path_not_allowed";
      throw error;
    }
    if (!normalized.toLowerCase().endsWith(".exe")) {
      const error = new Error("wine_invalid_extension");
      error.code = "wine_invalid_extension";
      throw error;
    }
    const resolved = this.fileSystem.toActualPath(normalized);
    try {
      const stat = await fs.stat(resolved.actualPath);
      if (!stat.isFile()) {
        const error = new Error("wine_target_not_file");
        error.code = "wine_target_not_file";
        throw error;
      }
    } catch (err) {
      if (err.code === "ENOENT") {
        const error = new Error("wine_target_not_found");
        error.code = "wine_target_not_found";
        throw error;
      }
      throw err;
    }
    return resolved;
  }

  validatePrefixName(name) {
    const normalized = String(name || DEFAULT_PREFIX_NAME).trim().toLowerCase();
    if (!/^[a-z0-9_-]+$/.test(normalized)) {
      const error = new Error("wine_invalid_prefix");
      error.code = "wine_invalid_prefix";
      throw error;
    }
    return normalized || DEFAULT_PREFIX_NAME;
  }

  async ensurePrefix(prefixName) {
    const prefixPath = this.defaultPrefixPath(prefixName);
    await fs.mkdir(prefixPath, { recursive: true });
    return prefixPath;
  }

  createSession(input) {
    return {
      sessionId: `wine-${Date.now()}-${Math.random().toString(36).slice(2, 8)}`,
      command: input.command,
      exePath: input.exePath,
      actualPath: input.actualPath,
      prefixName: input.prefixName,
      prefixPath: input.prefixPath,
      kind: input.kind,
      status: "starting",
      startedAt: new Date().toISOString(),
      endedAt: null,
      exitCode: null,
      pid: null,
      lastMessage: "starting",
      logs: [],
      child: null
    };
  }

  serializeSession(session) {
    return {
      sessionId: session.sessionId,
      command: session.command,
      exePath: session.exePath,
      prefixName: session.prefixName,
      prefixPath: session.prefixPath,
      kind: session.kind,
      status: session.status,
      startedAt: session.startedAt,
      endedAt: session.endedAt,
      exitCode: session.exitCode,
      pid: session.pid,
      lastMessage: session.lastMessage
    };
  }

  appendLog(session, streamName, chunk, stream) {
    const lines = String(chunk || "")
      .replace(/\r/g, "")
      .split("\n")
      .filter(Boolean);

    for (const line of lines) {
      session.logs.push(`[${streamName}] ${line}`);
      if (session.logs.length > MAX_LOG_LINES) {
        session.logs.splice(0, session.logs.length - MAX_LOG_LINES);
      }
      session.lastMessage = line;
      stream?.send?.({
        type: "wine.session.log",
        data: {
          sessionId: session.sessionId,
          stream: streamName,
          chunk: line
        }
      });
    }
  }

  emitUpdate(stream, session) {
    stream?.send?.({
      type: "wine.session.update",
      data: {
        session: this.serializeSession(session)
      }
    });
  }

  async loadState() {
    try {
      await fs.mkdir(path.dirname(this.stateFile), { recursive: true });
      const raw = await fs.readFile(this.stateFile, "utf8");
      return JSON.parse(raw);
    } catch {
      return { recentExecutables: [] };
    }
  }

  async saveState(state) {
    await fs.mkdir(path.dirname(this.stateFile), { recursive: true });
    await fs.writeFile(this.stateFile, JSON.stringify(state, null, 2), "utf8");
  }

  async pushRecentExecutable(exePath) {
    const state = await this.loadState();
    const next = [exePath, ...(state.recentExecutables || []).filter((entry) => entry !== exePath)].slice(0, 12);
    state.recentExecutables = next;
    await this.saveState(state);
  }
}

module.exports = WineService;
