const fs = require("node:fs/promises");
const fsSync = require("node:fs");
const path = require("node:path");

class FileSystemService {
  constructor(logger, eventBus, userService, kernelB = null) {
    this.logger = logger;
    this.eventBus = eventBus;
    this.userService = userService || null;
    this.kernelB = kernelB;
    this.rootPath = path.resolve(__dirname, "../..");
    this.userRootPath = path.join(this.rootPath, ".volt", "user");
    this.externalVolumesRoot = path.join(this.rootPath, ".volt", "volumes");
    this.externalRoots = {};
    this.virtualRoots = {
      "/User": this.userRootPath,
      "/Volumes": this.externalVolumesRoot
    };
    this.userFolders = ["Desktop", "Documents", "Downloads", "Music", "Photos", "Pictures", "Videos", "Applications", "AppData", "Trash", ".samaris"];
    this.initialized = null;
    this.watchers = new Map();
  }

  /** Switch /User/ to a specific user's home directory */
  setActiveUser(username) {
    if (this.userService && username) {
      const home = this.userService.resolveHome(username);
      this.setUserRoot(home);
    }
  }

  toVirtualPath(targetPath) {
    const normalized = path.posix.normalize(`/${String(targetPath || "/").replace(/\\/g, "/").replace(/^\/+/, "")}`);
    return normalized === "." ? "/" : normalized;
  }

  async ensureVirtualRoots() {
    if (!this.initialized) {
      this.initialized = Promise.all([
        fs.mkdir(this.userRootPath, { recursive: true }),
        fs.mkdir(this.externalVolumesRoot, { recursive: true }),
        ...this.userFolders.map((folder) => fs.mkdir(path.join(this.userRootPath, folder), { recursive: true }))
      ]);
    }
    await this.initialized;
  }

  setUserRoot(nextRoot) {
    const resolved = path.resolve(String(nextRoot || this.userRootPath));
    this.userRootPath = resolved;
    this.virtualRoots["/User"] = resolved;
    this.initialized = null;
  }

  setExternalRoots(entries = []) {
    this.externalRoots = {};
    for (const entry of entries) {
      const id = String(entry?.id || "").trim();
      const actualPath = String(entry?.actualPath || "").trim();
      if (!id || !actualPath) continue;
      this.externalRoots[`/Volumes/${id}`] = path.resolve(actualPath);
    }
  }

  resolveRoot(virtualPath) {
    const roots = Object.entries({
      ...this.virtualRoots,
      ...this.externalRoots
    }).sort((left, right) => right[0].length - left[0].length);
    for (const [prefix, actualRoot] of roots) {
      if (virtualPath === prefix || virtualPath.startsWith(`${prefix}/`)) {
        return { prefix, actualRoot };
      }
    }
    return null;
  }

  toActualPath(targetPath = "/") {
    const virtualPath = this.toVirtualPath(targetPath);
    if (virtualPath === "/") {
      return {
        actualPath: null,
        virtualPath,
        root: null
      };
    }

    const resolvedRoot = this.resolveRoot(virtualPath);
    if (!resolvedRoot) {
      const error = new Error("not_found");
      error.code = "ENOENT";
      throw error;
    }

    const suffix = virtualPath.slice(resolvedRoot.prefix.length).replace(/^\/+/, "");
    const actualPath = path.resolve(resolvedRoot.actualRoot, suffix);
    if (actualPath !== resolvedRoot.actualRoot && !actualPath.startsWith(`${resolvedRoot.actualRoot}${path.sep}`)) {
      const error = new Error("permission_denied");
      error.code = "EACCES";
      throw error;
    }
    return {
      actualPath,
      virtualPath,
      root: resolvedRoot.prefix
    };
  }

  async list(targetPath = "/") {
    await this.ensureVirtualRoots();
    const { actualPath, virtualPath } = this.toActualPath(targetPath);
    this.logger.info("fs:list", virtualPath);
    if (virtualPath === "/") {
      const nodes = [{ name: "User", kind: "dir" }];
      if (Object.keys(this.externalRoots).length > 0) {
        nodes.push({ name: "Volumes", kind: "dir" });
      }
      return { path: "/", nodes };
    }
    if (virtualPath === "/Volumes") {
      const nodes = Object.keys(this.externalRoots)
        .map((entryPath) => ({
          name: entryPath.split("/").pop() || "Volume",
          kind: "dir"
        }))
        .sort((left, right) => left.name.localeCompare(right.name));
      return { path: "/Volumes", nodes };
    }
    const entries = await fs.readdir(actualPath, { withFileTypes: true });
    const nodes = await Promise.all(
      entries.map(async (entry) => {
        if (entry.isDirectory()) {
          return { name: entry.name, kind: "dir" };
        }
        const fullPath = path.join(actualPath, entry.name);
        const stat = await fs.stat(fullPath);
        if (entry.isFile()) {
          return { name: entry.name, kind: "file", size: stat.size, modifiedAt: stat.mtime.toISOString() };
        }
        return { name: entry.name, kind: "file", modifiedAt: stat.mtime.toISOString() };
      })
    );
    const visibleNodes = nodes.filter((node) => !node.name.startsWith("."));
    visibleNodes.sort((left, right) => {
      if (left.kind !== right.kind) {
        return left.kind === "dir" ? -1 : 1;
      }
      return left.name.localeCompare(right.name);
    });
    return { path: virtualPath, nodes: visibleNodes };
  }

  async read(targetPath) {
    await this.ensureVirtualRoots();
    const { actualPath, virtualPath } = this.toActualPath(targetPath);
    this.logger.info("fs:read", virtualPath);
    const content = await fs.readFile(actualPath, "utf8");
    return { path: virtualPath, content };
  }

  async readDataUrl(targetPath) {
    await this.ensureVirtualRoots();
    const { actualPath, virtualPath } = this.toActualPath(targetPath);
    this.logger.info("fs:readDataUrl", virtualPath);
    const buffer = await fs.readFile(actualPath);
    const extension = path.extname(actualPath).toLowerCase();
    const mime =
      extension === ".png"
        ? "image/png"
        : extension === ".jpg" || extension === ".jpeg"
          ? "image/jpeg"
          : extension === ".webp"
            ? "image/webp"
            : extension === ".gif"
              ? "image/gif"
              : extension === ".mp3"
                ? "audio/mpeg"
                : extension === ".wav"
                  ? "audio/wav"
                  : extension === ".ogg"
                    ? "audio/ogg"
                    : extension === ".m4a"
                      ? "audio/mp4"
                      : extension === ".aac"
                        ? "audio/aac"
                        : extension === ".flac"
                          ? "audio/flac"
              : "application/octet-stream";
    return {
      path: virtualPath,
      dataUrl: `data:${mime};base64,${buffer.toString("base64")}`
    };
  }

  async write(targetPath, content = "") {
    await this.ensureVirtualRoots();
    const { actualPath, virtualPath } = this.toActualPath(targetPath);
    this.logger.info("fs:write", virtualPath);
    await fs.mkdir(path.dirname(actualPath), { recursive: true });
    await fs.writeFile(actualPath, content, "utf8");
    this.eventBus.emit("file:changed", { action: "write", path: virtualPath });
    return { ok: true };
  }

  async writeBase64(targetPath, base64 = "") {
    await this.ensureVirtualRoots();
    const { actualPath, virtualPath } = this.toActualPath(targetPath);
    this.logger.info("fs:writeBase64", virtualPath);
    const normalized = String(base64)
      .replace(/^data:[^;]+;base64,/, "")
      .replace(/\s+/g, "");
    const buffer = Buffer.from(normalized, "base64");
    await fs.mkdir(path.dirname(actualPath), { recursive: true });
    await fs.writeFile(actualPath, buffer);
    this.eventBus.emit("file:changed", { action: "write", path: virtualPath });
    return { ok: true };
  }

  async mkdir(targetPath) {
    await this.ensureVirtualRoots();
    const { actualPath, virtualPath } = this.toActualPath(targetPath);
    this.logger.info("fs:mkdir", virtualPath);
    await fs.mkdir(actualPath, { recursive: true });
    this.eventBus.emit("file:changed", { action: "mkdir", path: virtualPath });
    return { ok: true };
  }

  async rename(from, to) {
    await this.ensureVirtualRoots();
    const source = this.toActualPath(from);
    const destination = this.toActualPath(to);
    if (source.root !== destination.root) {
      const error = new Error("permission_denied");
      error.code = "EACCES";
      throw error;
    }
    this.logger.info("fs:rename", { from: source.virtualPath, to: destination.virtualPath });
    if (!(await this._kernelB("fs.move", { from: source.actualPath, to: destination.actualPath }, { timeoutMs: 60000 }))) {
      await fs.rename(source.actualPath, destination.actualPath);
    }
    this.eventBus.emit("file:changed", {
      action: "rename",
      from: source.virtualPath,
      to: destination.virtualPath
    });
    return { ok: true };
  }

  async copy(from, to) {
    await this.ensureVirtualRoots();
    const source = this.toActualPath(from);
    const destination = this.toActualPath(to);
    if (source.root !== destination.root) {
      const error = new Error("permission_denied");
      error.code = "EACCES";
      throw error;
    }
    this.logger.info("fs:copy", { from: source.virtualPath, to: destination.virtualPath });
    if (!(await this._kernelB("fs.copy", { from: source.actualPath, to: destination.actualPath }, { timeoutMs: 120000 }))) {
      await fs.cp(source.actualPath, destination.actualPath, {
        recursive: true,
        force: true,
        errorOnExist: false
      });
    }
    this.eventBus.emit("file:changed", {
      action: "copy",
      from: source.virtualPath,
      to: destination.virtualPath
    });
    return { ok: true };
  }

  async remove(targetPath, recursive = false) {
    await this.ensureVirtualRoots();
    const { actualPath, virtualPath } = this.toActualPath(targetPath);
    this.logger.info("fs:delete", { path: virtualPath, recursive });
    if (!(await this._kernelB("fs.delete", { path: actualPath, recursive }, { timeoutMs: 120000 }))) {
      await fs.rm(actualPath, { recursive, force: true });
    }
    this.eventBus.emit("file:changed", { action: "delete", path: virtualPath });
    return { ok: true };
  }

  async watch(targetPath, context = {}) {
    await this.ensureVirtualRoots();
    const { actualPath, virtualPath } = this.toActualPath(targetPath);
    const id = `watch:${virtualPath}`;
    if (this.watchers.has(id)) return { ok: true, id, path: virtualPath, reused: true };

    const sendChange = (eventType, filename) => {
      const child = filename ? `${virtualPath}/${String(filename).replace(/^\/+/, "")}`.replace(/\/+/g, "/") : virtualPath;
      const payload = { id, path: child, root: virtualPath, eventType };
      this.eventBus.emit("file:changed", payload);
      context.send?.({ type: "fs.watch.event", data: payload });
    };

    const watcher = fsSync.watch(actualPath, { persistent: false }, sendChange);
    const entry = { close: () => watcher.close() };
    this.watchers.set(id, entry);
    watcher.on("error", (error) => {
      this.logger.warn("fs:watch:error", { path: virtualPath, error: error.message });
      context.send?.({ type: "fs.watch.error", data: { id, path: virtualPath, error: error.message } });
      entry.close();
      this.watchers.set(id, this._createPollingWatcher(actualPath, virtualPath, id, sendChange));
    });
    return { ok: true, id, path: virtualPath };
  }

  async unwatch(targetPath) {
    const virtualPath = this.toVirtualPath(targetPath);
    const id = `watch:${virtualPath}`;
    const watcher = this.watchers.get(id);
    if (watcher) {
      watcher.close();
      this.watchers.delete(id);
    }
    return { ok: true, id };
  }

  _createPollingWatcher(actualPath, virtualPath, id, sendChange) {
    this.logger.warn("fs:watch:fallback-polling", { path: virtualPath });
    let previous = "";
    const snapshot = () => {
      try {
        return fsSync.readdirSync(actualPath, { withFileTypes: true })
          .map((entry) => {
            const stat = fsSync.statSync(path.join(actualPath, entry.name));
            return `${entry.name}:${entry.isDirectory() ? "d" : "f"}:${stat.size}:${stat.mtimeMs}`;
          })
          .sort()
          .join("|");
      } catch {
        return "";
      }
    };
    previous = snapshot();
    const timer = setInterval(() => {
      const next = snapshot();
      if (next !== previous) {
        previous = next;
        sendChange("change", "");
      }
    }, 1000);
    if (typeof timer.unref === "function") timer.unref();
    return { close: () => clearInterval(timer), id };
  }

  async _kernelB(method, params, options) {
    if (!this.kernelB?.available?.()) return false;
    try {
      await this.kernelB.call(method, params, options);
      return true;
    } catch (error) {
      this.logger?.warn?.("fs:kernel-b-fallback", { method, error: error.message });
      return false;
    }
  }
}

module.exports = FileSystemService;
