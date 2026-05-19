const { app, BrowserWindow, session, screen, ipcMain, globalShortcut, nativeTheme } = require("electron");
const path = require("path");
const { spawn } = require("node:child_process");
const fs = require("node:fs");
const { registerIpcHandlers } = require("./ipc");
const { BrowserManager } = require("./services/browser-manager");
const { TerminalManager } = require("./services/terminal-manager");
const { DownloadManager } = require("./services/download-manager");

let mainWindow = null;
let browserManager = null;
let terminalManager = null;
let downloadManager = null;

const isDev = process.env.NODE_ENV === "development";
const KERNEL_PORT = 9999;
const KERNEL_URL = `http://127.0.0.1:${KERNEL_PORT}`;
const KERNEL_DIR = isDev
  ? path.join(__dirname, "..", "volt-kernel-a")
  : (fs.existsSync("/opt/volt/kernel") ? "/opt/volt/kernel" : path.join(process.resourcesPath, "kernel"));
const UI_DEV_URL = "http://localhost:5173";
const UI_PROD_DIR = fs.existsSync("/opt/volt/desktop/app")
  ? "/opt/volt/desktop/app"
  : path.join(__dirname, "..", "..", "overlay", "opt", "volt", "desktop", "app");

function log(...args) {
  console.log("[samaris]", ...args);
}

function findVdmEventPath() {
  const candidates = [
    process.env.VOLT_DISPLAY_EVENT_PATH,
    process.env.XDG_RUNTIME_DIR ? `${process.env.XDG_RUNTIME_DIR}/samaris/display.event.json` : null,
    "/home/user/.local/state/samaris/display.event.json",
    "/run/samaris/display.event.json",
    "/tmp/samaris/display.event.json",
  ];
  for (const p of candidates) {
    if (p && fs.existsSync(p)) return p;
  }
  return null;
}

function findVdmConfigPath() {
  const candidates = [
    process.env.VOLT_DISPLAY_CONFIG_PATH,
    process.env.XDG_RUNTIME_DIR ? `${process.env.XDG_RUNTIME_DIR}/samaris/display.generated.toml` : null,
    "/home/user/.local/state/samaris/display.generated.toml",
    "/run/samaris/display.generated.toml",
    "/tmp/samaris/display.generated.toml",
  ];
  for (const p of candidates) {
    if (p && fs.existsSync(p)) return p;
  }
  return "/home/user/.local/state/samaris/display.generated.toml";
}

function findNodeBinary() {
  // Try known locations for the system node binary (NOT Electron's process.execPath)
  const candidates = ["/usr/local/bin/node", "/opt/homebrew/bin/node", "/usr/bin/node"];
  for (const c of candidates) {
    if (fs.existsSync(c)) return c;
  }
  // Use which on PATH as last resort
  try {
    const { execFileSync } = require("node:child_process");
    return execFileSync("which", ["node"], { encoding: "utf8" }).trim();
  } catch {
    return process.execPath; // fallback (likely electron binary, may fail)
  }
}

async function waitForKernel(retries = 30, delay = 200) {
  const http = require("node:http");
  for (let i = 0; i < retries; i++) {
    try {
      await new Promise((resolve, reject) => {
        const req = http.get(`${KERNEL_URL}/health`, (res) => {
          resolve(res.statusCode === 200);
        });
        req.on("error", reject);
        req.setTimeout(1000, () => { req.destroy(); reject(new Error("timeout")); });
      });
      log("kernel bridge ready");
      return true;
    } catch {
      await new Promise((r) => setTimeout(r, delay));
    }
  }
  log("kernel bridge not reachable after", retries * delay, "ms");
  return false;
}

function startKernel() {
  const kernelMain = path.join(KERNEL_DIR, "server.js");
  if (!fs.existsSync(kernelMain)) {
    log("kernel server.js not found at", kernelMain);
    return null;
  }
  log("starting kernel bridge...");
  const nodeBin = findNodeBinary();
  const child = spawn(nodeBin, [kernelMain], {
    cwd: KERNEL_DIR,
    stdio: ["ignore", "pipe", "pipe"],
    env: { ...process.env, NODE_ENV: isDev ? "development" : "production", PORT: String(KERNEL_PORT) },
  });
  child.stdout.on("data", (d) => process.stdout.write(`[kernel] ${d}`));
  child.stderr.on("data", (d) => process.stderr.write(`[kernel:err] ${d}`));
  child.on("exit", (code) => log("kernel exited with code", code));
  return child;
}

function createWindow() {
  const displays = screen.getAllDisplays();

  // Try VDM primary config first, fall back to Electron screen detection
  let vdmPrimary = null;
  try {
    const vdmEventPath = findVdmEventPath();
    if (vdmEventPath && fs.existsSync(vdmEventPath)) {
      const raw = fs.readFileSync(vdmEventPath, "utf8");
      const evt = JSON.parse(raw);
      if (evt.safe_mode) {
        log("VDM safe mode active — using 1280x720 window");
        mainWindow = new BrowserWindow({
          width: 1280, height: 720,
          fullscreen: false, frame: true,
          backgroundColor: "#000000", show: false,
          webPreferences: {
            preload: path.join(__dirname, "preload.js"),
            contextIsolation: true, nodeIntegration: false,
            sandbox: process.argv.includes("--no-sandbox") ? false : true, webviewTag: false,
          },
        });
        mainWindow.loadURL(isDev ? UI_DEV_URL : `file://${path.join(UI_PROD_DIR, "index.html")}`);
        mainWindow.once("ready-to-show", () => mainWindow.show());
        return;
      }
      vdmPrimary = displays.find((d) => d.bounds.x === 0 && d.bounds.y === 0) || displays[0];
    }
  } catch {}

  const primary = vdmPrimary || displays.find((d) => d.bounds.x === 0 && d.bounds.y === 0) || displays[0];
  const { width, height } = primary ? primary.workAreaSize : { width: 1280, height: 720 };

  mainWindow = new BrowserWindow({
    x: primary ? primary.bounds.x : 0,
    y: primary ? primary.bounds.y : 0,
    width,
    height,
    fullscreen: !isDev,
    frame: false,
    resizable: true,
    backgroundColor: "#000000",
    show: false,
    webPreferences: {
      preload: path.join(__dirname, "preload.js"),
      contextIsolation: true,
      nodeIntegration: false,
      sandbox: process.argv.includes("--no-sandbox") ? false : true,
      webviewTag: false,
    },
  });

  if (isDev) {
    mainWindow.loadURL(UI_DEV_URL);
    mainWindow.webContents.openDevTools({ mode: "detach" });
  } else {
    const indexPath = path.join(UI_PROD_DIR, "index.html");
    if (fs.existsSync(indexPath)) {
      mainWindow.loadFile(indexPath);
    } else if (fs.existsSync(path.join(UI_PROD_DIR, "..", "app", "index.html"))) {
      mainWindow.loadFile(path.join(UI_PROD_DIR, "..", "app", "index.html"));
    } else {
      log("UI not found, falling back to dev URL");
      mainWindow.loadURL(UI_DEV_URL);
    }
  }

  mainWindow.once("ready-to-show", () => {
    mainWindow.show();
  });

  mainWindow.on("closed", () => {
    mainWindow = null;
  });

  mainWindow.on("maximize", () => {
    mainWindow?.webContents.send("window:maximize-change", true);
  });

  mainWindow.on("unmaximize", () => {
    mainWindow?.webContents.send("window:maximize-change", false);
  });

  mainWindow.webContents.setWindowOpenHandler(({ url }) => {
    if (browserManager) {
      browserManager.createTab({ url, activate: true });
    }
    return { action: "deny" };
  });

  session.defaultSession.webRequest.onHeadersReceived(
    { urls: ["http://127.0.0.1:*/*", "http://localhost:*/*"] },
    (details, callback) => {
      const headers = Object.fromEntries(
        Object.entries(details.responseHeaders || {}).filter(
          ([key]) => !/^(content-security-policy|x-frame-options)$/i.test(key)
        )
      );
      callback({ responseHeaders: headers });
    }
  );
}

function registerDownloadHandlers(dm) {
  const { ipcMain } = require("electron");
  ipcMain.handle("download:cancel", async (_, itemId) => {
    return dm.cancel(itemId);
  });
}

app.whenReady().then(async () => {
  log("starting electron shell v1.0.0-alpha");

  app.on("web-contents-created", (_, contents) => {
    contents.on("will-attach-webview", (event) => {
      event.preventDefault();
    });
  });

  // In dev mode, START-ALL.sh already starts the kernel. Check if it's running first.
  let kernelReady = await waitForKernel(3, 150);
  if (!kernelReady) {
    log("kernel not detected, starting it...");
    startKernel();
    kernelReady = await waitForKernel();
  }

  terminalManager = new TerminalManager();
  downloadManager = new DownloadManager(null);
  browserManager = new BrowserManager(downloadManager);

  createWindow();
  downloadManager.setMainWindow(mainWindow);
  registerIpcHandlers(mainWindow, browserManager, terminalManager);
  registerDownloadHandlers(downloadManager);

  // Dev mode: spawn VDM if not already running (Xorg is guaranteed ready now)
  if (isDev || !findVdmEventPath()) {
    const currentArch = process.arch === "arm64" ? "aarch64" : "x86_64";
    const vdmPaths = [
      "/usr/local/bin/start-display-manager.sh",
      `/opt/volt/bin/volt-display-manager-${currentArch}`,
      path.join(__dirname, "..", "volt-display-manager", "target", "release", "volt-display-manager"),
      "/opt/volt/bin/volt-display-manager",
      "/usr/local/bin/volt-display-manager",
    ];
    const vdmBin = vdmPaths.find((p) => fs.existsSync(p));
    if (vdmBin) {
      const stateDir = "/home/user/.local/state/samaris";
      log(`VDM: spawning ${vdmBin}...`);
      const vdmProc = spawn(vdmBin, [
        "--apply",
        "--config-path", `${stateDir}/display.generated.toml`,
        "--event-path", `${stateDir}/display.event.json`,
      ], {
        env: { ...process.env, DISPLAY: ":0" },
        stdio: "pipe",
      });
      vdmProc.on("exit", (code) => log("VDM exited with code", code));
      vdmProc.on("error", (err) => log("VDM spawn failed:", err.message));
      vdmProc.stderr.on("data", (d) => process.stderr.write(`[vdm:err] ${d}`));
    } else {
      log("VDM binary not found — skipping auto-detection");
    }
  }

  app.on("activate", () => {
    if (BrowserWindow.getAllWindows().length === 0) createWindow();
  });
});

app.on("window-all-closed", () => {
  if (process.platform !== "darwin") app.quit();
});

app.on("before-quit", () => {
  if (terminalManager) terminalManager.killAll();
  if (browserManager) browserManager.destroy();
});
