const { spawn } = require("node:child_process");
const path = require("node:path");
const os = require("node:os");

class TerminalManager {
  constructor() {
    this.sessions = new Map();
    this._helperPath = path.join(__dirname, "pty-helper.js");
  }

  create(id, options = {}) {
    const safeId = String(id).replace(/[^a-zA-Z0-9_-]/g, "");
    const cols = Math.min(Math.max(parseInt(options.cols, 10) || 80, 10), 500);
    const rows = Math.min(Math.max(parseInt(options.rows, 10) || 24, 5), 300);
    let cwd = options.cwd || path.join(os.homedir(), ".volt", "user");
    const allowedDirs = [path.join(os.homedir(), ".volt")];
    const resolvedCwd = path.resolve(cwd);
    if (!allowedDirs.some((d) => resolvedCwd.startsWith(d))) {
      cwd = path.join(os.homedir(), ".volt", "user");
    }

    const child = spawn(process.execPath, [this._helperPath, safeId], {
      stdio: ["pipe", "pipe", "pipe"],
      env: {
        ...process.env,
        COLS: String(cols),
        ROWS: String(rows),
        HOME: os.homedir(),
        TERM: "xterm-256color",
        COLORTERM: "truecolor",
      },
    });

    const session = { id, child, cols, rows, cwd, buffer: "", started: Date.now(), pid: null };

    child.stdout.on("data", (chunk) => {
      session.buffer += chunk.toString();
      const lines = session.buffer.split("\n");
      session.buffer = lines.pop() || "";
      for (const line of lines) {
        if (!line.trim()) continue;
        try {
          const msg = JSON.parse(line);
          if (msg.type === "started") {
            session.pid = msg.pid;
          }
          if (msg.type === "data") {
            this._emit("terminal:data", { id, data: msg.data });
          }
          if (msg.type === "exit") {
            this._emit("terminal:exit", { id, exitCode: msg.exitCode, signal: msg.signal });
            this.sessions.delete(id);
          }
        } catch {}
      }
    });

    child.stderr.on("data", (chunk) => {
      console.error("[pty-helper]", chunk.toString().trim());
    });

    child.on("exit", () => {
      if (this.sessions.has(id)) {
        this.sessions.delete(id);
        this._emit("terminal:exit", { id, exitCode: 0, signal: null });
      }
    });

    child.on("error", () => {
      this.sessions.delete(id);
      this._emit("terminal:exit", { id, exitCode: 1, signal: null });
    });

    this.sessions.set(id, session);
    return { id, pid: child.pid, shell: "bash", cols, rows };
  }

  _send(id, msg) {
    const session = this.sessions.get(id);
    if (session && session.child.stdin.writable) {
      session.child.stdin.write(JSON.stringify(msg) + "\n");
    }
  }

  write(id, data) {
    if (typeof data !== "string") return;
    data = data.slice(0, 1024 * 100);
    this._send(id, { type: "write", data });
  }

  resize(id, cols, rows) {
    const session = this.sessions.get(id);
    if (session) { session.cols = cols; session.rows = rows; }
    this._send(id, { type: "resize", cols, rows });
  }

  kill(id) {
    this._send(id, { type: "kill" });
    const session = this.sessions.get(id);
    if (session) {
      try { session.child.kill("SIGTERM"); } catch {}
      setTimeout(() => { try { session.child.kill("SIGKILL"); } catch {} }, 1000);
      this.sessions.delete(id);
    }
  }

  killAll() {
    for (const id of this.sessions.keys()) this.kill(id);
  }

  _emit(event, data) {
    try {
      const { BrowserWindow } = require("electron");
      const wins = BrowserWindow.getAllWindows();
      for (const win of wins) {
        if (!win.isDestroyed()) {
          try { win.webContents.send(event, data); } catch {}
        }
      }
    } catch {}
  }
}

module.exports = { TerminalManager };
