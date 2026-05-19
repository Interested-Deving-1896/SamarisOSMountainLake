const path = require("node:path");
const os = require("node:os");
const crypto = require("node:crypto");
const fs = require("node:fs");
const { dialog } = require("electron");

class DownloadManager {
  constructor(mainWindow) {
    this.mainWindow = mainWindow;
    this.activeDownloads = new Map();
  }

  setMainWindow(mainWindow) {
    this.mainWindow = mainWindow;
  }

  onWillDownload(event, item) {
    event.preventDefault();

    const id = `dl-${crypto.randomUUID()}`;
    const rawFilename = item.getFilename();
    const filename = path.basename(rawFilename).replace(/[/\\:*?"<>|]/g, "_") || "download";
    const url = item.getURL();
    const totalBytes = item.getTotalBytes();

    const downloadsDir = path.join(os.homedir(), ".volt", "user", "Downloads");
    try { fs.mkdirSync(downloadsDir, { recursive: true }); } catch (e) { console.error("[downloads] mkdir failed:", e.message); }

    const defaultPath = path.join(downloadsDir, filename);
    const win = this.mainWindow && !this.mainWindow.isDestroyed() ? this.mainWindow : null;

    const dialogOpts = {
      defaultPath,
      title: "Save As",
      buttonLabel: "Save",
      filters: [
        { name: "All Files", extensions: ["*"] },
      ],
    };

    const run = win
      ? async () => { return await dialog.showSaveDialog(win, dialogOpts); }
      : async () => { return await dialog.showSaveDialog(dialogOpts); };

    const track = (resolvedPath) => {
      const entry = { id, item, filename: path.basename(resolvedPath), url, totalBytes, startedAt: Date.now() };
      this.activeDownloads.set(id, entry);

      this._send("download:started", { id, filename: entry.filename, totalBytes, url });

      item.on("updated", (_, state) => {
        if (state !== "progressing") return;
        const received = item.getReceivedBytes();
        const total = item.getTotalBytes();
        this._send("download:progress", { id, filename: entry.filename, received, total });
      });

      item.on("done", (_, state) => {
        if (!this.activeDownloads.has(id)) return;
        const savePath = item.getSavePath();
        const finalFilename = savePath ? path.basename(savePath) : entry.filename;
        if (state === "completed") {
          this._send("download:complete", { id, filename: finalFilename, state: "completed", savePath, success: true });
        } else {
          this._send("download:complete", { id, filename: finalFilename, state, success: false, error: state });
        }
        this.activeDownloads.delete(id);
      });

      item.setSavePath(resolvedPath);
    };

    run().then((result) => {
      if (result.canceled || !result.filePath) {
        item.cancel();
        return;
      }
      track(path.resolve(result.filePath));
    }).catch((err) => {
      console.error("[downloads] dialog error:", err.message);
      item.cancel();
    });
  }

  cancel(identifier) {
    const byId = this.activeDownloads.get(identifier);
    if (byId && byId.item) {
      try { byId.item.cancel(); } catch {}
      this.activeDownloads.delete(identifier);
      this._send("download:complete", { id: byId.id, filename: byId.filename, state: "cancelled", success: false, error: "cancelled" });
      return { ok: true };
    }
    for (const [id, entry] of this.activeDownloads) {
      if (entry.filename === identifier) {
        try { entry.item.cancel(); } catch {}
        this.activeDownloads.delete(id);
        this._send("download:complete", { id: entry.id, filename: entry.filename, state: "cancelled", success: false, error: "cancelled" });
        return { ok: true };
      }
    }
    return { ok: false, error: "not_found" };
  }

  _send(event, data) {
    if (this.mainWindow && !this.mainWindow.isDestroyed()) {
      try { this.mainWindow.webContents.send(event, data); } catch {}
    }
  }
}

module.exports = { DownloadManager };
