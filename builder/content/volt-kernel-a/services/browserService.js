const fs = require("node:fs/promises");
const os = require("node:os");
const path = require("node:path");
const { execFile, spawn } = require("node:child_process");
const { promisify } = require("node:util");
const crypto = require("node:crypto");

const execFileAsync = promisify(execFile);

class BrowserService {
  constructor(logger, eventBus) {
    this.logger = logger;
    this.eventBus = eventBus;
    this.profileDir = path.join(os.homedir(), ".samaris", "peregrine", "profile", "default");
    this.binaryCandidates = [
      "chromium",
      "chromium-browser",
      "google-chrome",
      "google-chrome-stable",
      "/Applications/Chromium.app/Contents/MacOS/Chromium",
      "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome"
    ];
    this.sessions = new Map();
  }

  normalizeAddress(value = "") {
    const trimmed = String(value || "").trim();
    if (!trimmed) {
      return { url: "https://www.google.com", title: "google.com", startPage: false, query: false };
    }
    if (/^https?:\/\//i.test(trimmed)) {
      return { url: trimmed, title: this.titleFromUrl(trimmed), startPage: false, query: false };
    }
    if (/^[\w.-]+\.[a-z]{2,}/i.test(trimmed) || trimmed.startsWith("localhost")) {
      const protocol = trimmed.startsWith("localhost") ? "http" : "https";
      const url = `${protocol}://${trimmed.replace(/^\/+/, "")}`;
      return { url, title: this.titleFromUrl(url), startPage: false, query: false };
    }
    const url = `https://www.google.com/search?q=${encodeURIComponent(trimmed)}`;
    return { url, title: `Search: ${trimmed}`, startPage: false, query: true };
  }

  async ensureProfileDir() {
    await fs.mkdir(this.profileDir, { recursive: true });
    return this.profileDir;
  }

  async probeBinary(candidate, args = ["--version"]) {
    try {
      const { stdout, stderr } = await execFileAsync(candidate, args, {
        timeout: 4000,
        env: process.env
      });
      const version = String(stdout || stderr || "").trim() || null;
      return {
        available: true,
        executable: candidate,
        version
      };
    } catch {
      return null;
    }
  }

  async resolveBinary() {
    for (const candidate of this.binaryCandidates) {
      const result = await this.probeBinary(candidate);
      if (result) return result;
    }
    return {
      available: false,
      executable: null,
      version: null
    };
  }

  async checkAttachSupport() {
    if (process.platform !== "linux") {
      return {
        supported: false,
        reason: "native_attached_mode_requires_linux_x11"
      };
    }

    const xdotool = await this.probeBinary("xdotool", ["-v"]);
    if (!xdotool) {
      return {
        supported: false,
        reason: "xdotool_missing"
      };
    }

    return {
      supported: true,
      reason: null
    };
  }

  buildLaunchArgs(url, bounds) {
    const args = [
      `--user-data-dir=${this.profileDir}`,
      "--app=" + url,
      "--new-window",
      "--no-first-run",
      "--no-default-browser-check",
      "--disable-infobars",
      "--disable-session-crashed-bubble",
      "--disable-features=TranslateUI"
    ];

    if (bounds) {
      args.push(`--window-position=${Math.max(0, Math.round(bounds.left))},${Math.max(0, Math.round(bounds.top))}`);
      args.push(`--window-size=${Math.max(320, Math.round(bounds.width))},${Math.max(240, Math.round(bounds.height))}`);
    }

    return args;
  }

  async launch(urlValue = "") {
    const resolved = this.normalizeAddress(urlValue);
    const binary = await this.resolveBinary();
    await this.ensureProfileDir();

    if (!binary.available || !binary.executable) {
      return {
        ok: false,
        engine: "chromium",
        bridge: "native-window",
        installed: false,
        executable: null,
        version: null,
        profileDir: this.profileDir,
        url: resolved.url,
        title: resolved.title,
        startPage: resolved.startPage,
        message: "Chromium is not installed on this system."
      };
    }

    const child = spawn(binary.executable, this.buildLaunchArgs(resolved.url), {
      detached: true,
      stdio: "ignore",
      env: {
        ...process.env,
        CHROME_USER_DATA_DIR: this.profileDir
      }
    });
    child.unref();

    this.logger.info("browser:launch", JSON.stringify({ executable: binary.executable, url: resolved.url, pid: child.pid || null }));
    this.eventBus.emit("browser:navigated", {
      url: resolved.url,
      executable: binary.executable,
      pid: child.pid || null
    });

    return {
      ok: true,
      engine: "chromium",
      bridge: "native-window",
      installed: true,
      executable: binary.executable,
      version: binary.version,
      profileDir: this.profileDir,
      url: resolved.url,
      title: resolved.title,
      startPage: resolved.startPage,
      pid: child.pid || null
    };
  }

  async openAttached(options = {}) {
    const resolved = this.normalizeAddress(options.url || "");
    const binary = await this.resolveBinary();
    await this.ensureProfileDir();

    if (!binary.available || !binary.executable) {
      return {
        ok: false,
        engine: "chromium",
        bridge: "native-window",
        attached: false,
        installed: false,
        executable: null,
        version: null,
        profileDir: this.profileDir,
        url: resolved.url,
        title: resolved.title,
        startPage: resolved.startPage,
        message: "Chromium is not installed on this system."
      };
    }

    const support = await this.checkAttachSupport();
    if (!support.supported) {
      const detached = await this.launch(resolved.url);
      return {
        ...detached,
        attached: false,
        sessionId: null,
        attachReason: support.reason,
        message: "Attached mode is unavailable on this platform. Peregrine used native detached window mode."
      };
    }

    const existingSessionId = options.sessionId && this.sessions.has(options.sessionId) ? options.sessionId : null;
    if (existingSessionId) {
      await this.closeSession(existingSessionId);
    }

    const child = spawn(binary.executable, this.buildLaunchArgs(resolved.url, options.bounds), {
      detached: true,
      stdio: "ignore",
      env: {
        ...process.env,
        CHROME_USER_DATA_DIR: this.profileDir
      }
    });
    child.unref();

    const nativeWindowId = await this.waitForNativeWindow(child.pid);
    const sessionId = crypto.randomUUID();
    const session = {
      id: sessionId,
      windowId: options.windowId || null,
      nativeWindowId,
      pid: child.pid || null,
      url: resolved.url,
      title: resolved.title,
      state: "starting",
      bridge: "native-attached",
      executable: binary.executable
    };
    this.sessions.set(sessionId, session);

    child.once("exit", (code, signal) => {
      const current = this.sessions.get(sessionId);
      if (!current) return;
      this.sessions.set(sessionId, {
        ...current,
        state: "exited",
        exitCode: code ?? null,
        exitSignal: signal ?? null
      });
    });

    if (options.bounds) {
      await this.syncAttached({
        sessionId,
        bounds: options.bounds,
        focused: Boolean(options.focused),
        minimized: Boolean(options.minimized)
      });
    }

    this.logger.info(
      "browser:attach",
      JSON.stringify({
        executable: binary.executable,
        url: resolved.url,
        pid: child.pid || null,
        nativeWindowId,
        windowId: options.windowId || null
      })
    );

    return {
      ok: true,
      engine: "chromium",
      bridge: "native-attached",
      attached: true,
      installed: true,
      executable: binary.executable,
      version: binary.version,
      profileDir: this.profileDir,
      url: resolved.url,
      title: resolved.title,
      startPage: resolved.startPage,
      pid: child.pid || null,
      sessionId,
      nativeWindowId,
      state: this.sessions.get(sessionId)?.state || "running"
    };
  }

  async syncAttached(options = {}) {
    const session = options.sessionId ? this.sessions.get(options.sessionId) : null;
    if (!session) {
      return {
        ok: false,
        found: false,
        message: "Session not found."
      };
    }

    if (session.state === "exited") {
      return {
        ok: false,
        found: true,
        state: "exited",
        message: "Browser session has already exited."
      };
    }

    if (!session.nativeWindowId) {
      return {
        ok: false,
        found: true,
        state: session.state,
        message: "Native Chromium window was not detected."
      };
    }

    if (options.minimized || options.focused === false) {
      await this.execXdotool(["windowunmap", session.nativeWindowId]);
      this.sessions.set(session.id, { ...session, state: "hidden" });
      return {
        ok: true,
        found: true,
        state: "hidden"
      };
    }

    await this.execXdotool(["windowmap", session.nativeWindowId]);

    if (options.bounds) {
      await this.execXdotool([
        "windowmove",
        session.nativeWindowId,
        String(Math.max(0, Math.round(options.bounds.left))),
        String(Math.max(0, Math.round(options.bounds.top)))
      ]);
      await this.execXdotool([
        "windowsize",
        session.nativeWindowId,
        String(Math.max(320, Math.round(options.bounds.width))),
        String(Math.max(240, Math.round(options.bounds.height)))
      ]);
    }

    if (options.focused) {
      await this.execXdotool(["windowraise", session.nativeWindowId]);
      await this.execXdotool(["windowactivate", "--sync", session.nativeWindowId]);
    }

    this.sessions.set(session.id, { ...session, state: "running" });

    return {
      ok: true,
      found: true,
      state: "running"
    };
  }

  async closeSession(sessionId) {
    const session = this.sessions.get(sessionId);
    if (!session) {
      return {
        ok: false,
        found: false,
        message: "Session not found."
      };
    }

    if (session.nativeWindowId && process.platform === "linux") {
      await this.execXdotool(["windowclose", session.nativeWindowId]).catch(() => {});
    }

    if (typeof session.pid === "number") {
      try {
        process.kill(session.pid, "SIGTERM");
      } catch {}
    }

    this.sessions.delete(sessionId);
    return {
      ok: true,
      found: true
    };
  }

  async status() {
    const binary = await this.resolveBinary();
    const attach = await this.checkAttachSupport();
    await this.ensureProfileDir();
    return {
      connected: true,
      engine: "chromium",
      bridge: attach.supported ? "native-attached" : "native-window",
      installed: binary.available,
      executable: binary.executable,
      version: binary.version,
      profileDir: this.profileDir,
      attachedSupported: attach.supported,
      attachReason: attach.reason,
      platform: process.platform,
      activeSessions: [...this.sessions.values()].map((session) => ({
        id: session.id,
        windowId: session.windowId,
        pid: session.pid,
        url: session.url,
        state: session.state
      }))
    };
  }

  async waitForNativeWindow(pid) {
    if (!pid || process.platform !== "linux") {
      return null;
    }

    const startedAt = Date.now();
    while (Date.now() - startedAt < 5000) {
      try {
        const { stdout } = await execFileAsync("xdotool", ["search", "--onlyvisible", "--pid", String(pid)]);
        const match = String(stdout || "")
          .trim()
          .split(/\s+/)
          .find(Boolean);
        if (match) return match;
      } catch {}
      await new Promise((resolve) => setTimeout(resolve, 120));
    }
    return null;
  }

  async execXdotool(args) {
    if (process.platform !== "linux") return null;
    return await execFileAsync("xdotool", args, {
      timeout: 4000,
      env: process.env
    });
  }

  titleFromUrl(url) {
    try {
      const parsed = new URL(url);
      return parsed.hostname.replace(/^www\./, "") || parsed.host || url;
    } catch {
      return url;
    }
  }
}

module.exports = BrowserService;
