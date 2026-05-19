const { ipcMain, clipboard, nativeImage, shell, app, dialog } = require("electron");
const path = require("node:path");
const os = require("node:os");
const { registerWindowControls } = require("./window-controls");
const { registerClipboard } = require("./clipboard");
const { registerSystem } = require("./system");
const { registerBrowser } = require("./browser");
const { registerTerminal } = require("./terminal");
const { registerBench } = require("./bench");

const allowedPathRoots = [
  path.join(os.homedir(), ".volt"),
  path.join(os.homedir(), "Downloads"),
];

function isAllowedPath(filePath) {
  const resolved = path.resolve(filePath);
  return allowedPathRoots.some((root) => {
    const relative = path.relative(path.resolve(root), resolved);
    return relative === "" || (!relative.startsWith("..") && !path.isAbsolute(relative));
  });
}

function registerIpcHandlers(mainWindow, browserManager, terminalManager) {
  registerWindowControls(mainWindow);
  registerClipboard();
  registerSystem();
  registerBrowser(mainWindow, browserManager);
  registerTerminal(terminalManager);
  registerBench(mainWindow);

  // ── Shell ──────────────────────────────────────────────
  ipcMain.handle("shell:openExternal", async (_, url) => {
    const parsed = new URL(url);
    const allowedProtocols = ["http:", "https:", "mailto:"];
    if (!allowedProtocols.includes(parsed.protocol)) {
      throw new Error(`Protocol "${parsed.protocol}" not allowed`);
    }
    return shell.openExternal(url);
  });

  ipcMain.handle("shell:openPath", async (_, filePath) => {
    const resolved = path.resolve(filePath);
    if (!isAllowedPath(resolved)) {
      throw new Error("Path not allowed");
    }
    return shell.openPath(resolved);
  });

  ipcMain.handle("shell:showItemInFolder", async (_, filePath) => {
    const resolved = path.resolve(filePath);
    if (!isAllowedPath(resolved)) {
      throw new Error("Path not allowed");
    }
    return shell.showItemInFolder(resolved);
  });

  // ── Download / Save Dialog ──────────────────────────────
  ipcMain.handle("dialog:save", async (_, defaultName) => {
    const result = await dialog.showSaveDialog({
      defaultPath: defaultName,
      filters: [
        { name: "All Files", extensions: ["*"] },
        { name: "Documents", extensions: ["pdf", "txt", "md", "doc", "docx"] },
        { name: "Images", extensions: ["png", "jpg", "jpeg", "gif", "webp", "svg"] },
        { name: "Audio", extensions: ["mp3", "wav", "m4a", "aac", "ogg", "flac"] },
        { name: "Video", extensions: ["mp4", "webm", "avi", "mkv", "mov"] },
      ],
    });
    return result;
  });

  ipcMain.handle("dialog:open", async (_, options) => {
    const result = await dialog.showOpenDialog(options || {});
    return result;
  });

  // ── App ────────────────────────────────────────────────
  ipcMain.on("app:quit", () => app.quit());
  ipcMain.on("app:restart", () => app.relaunch({ args: process.argv.slice(1).concat(["--relaunch"]) }) && app.exit(0));
  ipcMain.handle("app:version", () => app.getVersion());
  ipcMain.handle("app:isPackaged", () => app.isPackaged);
}

module.exports = { registerIpcHandlers };
