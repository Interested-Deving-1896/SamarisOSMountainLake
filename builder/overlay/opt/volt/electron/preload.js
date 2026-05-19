const { contextBridge, ipcRenderer } = require("electron");

contextBridge.exposeInMainWorld("electronAPI", {
  // ── Window Controls ──────────────────────────────────────
  window: {
    minimize: () => ipcRenderer.invoke("window:minimize"),
    maximize: () => ipcRenderer.invoke("window:maximize"),
    unmaximize: () => ipcRenderer.invoke("window:unmaximize"),
    close: () => ipcRenderer.invoke("window:close"),
    isMaximized: () => ipcRenderer.invoke("window:isMaximized"),
    onMaximizeChange: (callback) => {
      const handler = (_, value) => callback(value);
      ipcRenderer.on("window:maximize-change", handler);
      return () => ipcRenderer.removeListener("window:maximize-change", handler);
    },
  },

  // ── Clipboard ────────────────────────────────────────────
  clipboard: {
    readText: () => ipcRenderer.invoke("clipboard:readText"),
    writeText: (text) => ipcRenderer.invoke("clipboard:writeText", text),
  },

  // ── System Info ──────────────────────────────────────────
  system: {
    platform: process.platform,
    arch: process.arch,
    versions: process.versions,
    getMemoryInfo: () => ipcRenderer.invoke("system:memory"),
    getGPUInfo: () => ipcRenderer.invoke("system:gpu"),
  },

  // ── Peregrine Browser (main-process WebContentsView engine) ─────────
  browser: {
    createTab: (optionsOrUrl, bounds, opts) => ipcRenderer.invoke("browser:createTab", optionsOrUrl, bounds, opts),
    getSnapshot: () => ipcRenderer.invoke("browser:getSnapshot"),
    navigate: (tabId, url) => ipcRenderer.invoke("browser:navigate", tabId, url),
    command: (tabId, command, payload) => ipcRenderer.invoke("browser:command", tabId, command, payload),
    goBack: (tabId) => ipcRenderer.invoke("browser:goBack", tabId),
    goForward: (tabId) => ipcRenderer.invoke("browser:goForward", tabId),
    reload: (tabId) => ipcRenderer.invoke("browser:reload", tabId),
    stop: (tabId) => ipcRenderer.invoke("browser:stop", tabId),
    closeTab: (tabId) => ipcRenderer.invoke("browser:closeTab", tabId),
    getTabs: () => ipcRenderer.invoke("browser:getTabs"),
    setBounds: (tabId, bounds) => ipcRenderer.invoke("browser:setBounds", tabId, bounds),
    activateTab: (tabId) => ipcRenderer.invoke("browser:activateTab", tabId),
    savePage: (tabId) => ipcRenderer.invoke("browser:savePage", tabId),
    printPage: (tabId) => ipcRenderer.invoke("browser:printPage", tabId),
    createPrivateTab: (url, bounds) => ipcRenderer.invoke("browser:createPrivateTab", url, bounds),
    setZoom: (tabId, factor) => ipcRenderer.invoke("browser:setZoom", tabId, factor),
    showToolbarMenu: (point) => ipcRenderer.invoke("browser:showToolbarMenu", point),
    clearData: (options) => ipcRenderer.invoke("browser:clearData", options),
    openDevTools: (tabId) => ipcRenderer.invoke("browser:openDevTools", tabId),
    destroyAll: () => ipcRenderer.invoke("browser:destroyAll"),
    reorderTabs: (tabIds) => ipcRenderer.invoke("browser:reorderTabs", tabIds),
    onSnapshot: (callback) => {
      const handler = (_, data) => callback(data);
      ipcRenderer.on("browser:snapshot", handler);
      return () => ipcRenderer.removeListener("browser:snapshot", handler);
    },
    onTabUpdate: (callback) => {
      const handler = (_, data) => callback(data);
      ipcRenderer.on("browser:tab-updated", handler);
      return () => ipcRenderer.removeListener("browser:tab-updated", handler);
    },
    onPermissionRequest: (callback) => {
      const handler = (_, data) => callback(data);
      ipcRenderer.on("permission:request", handler);
      return () => ipcRenderer.removeListener("permission:request", handler);
    },
  },

  // ── Terminal (node-pty) ──────────────────────────────────
  terminal: {
    create: (id, options) => ipcRenderer.invoke("terminal:create", id, options),
    write: (id, data) => ipcRenderer.invoke("terminal:write", id, data),
    resize: (id, cols, rows) => ipcRenderer.invoke("terminal:resize", id, cols, rows),
    kill: (id) => ipcRenderer.invoke("terminal:kill", id),
    onData: (callback) => {
      const handler = (_, data) => callback(data);
      ipcRenderer.on("terminal:data", handler);
      return () => ipcRenderer.removeListener("terminal:data", handler);
    },
    onExit: (callback) => {
      const handler = (_, data) => callback(data);
      ipcRenderer.on("terminal:exit", handler);
      return () => ipcRenderer.removeListener("terminal:exit", handler);
    },
  },

  // ── Dialog ────────────────────────────────────────────
  dialog: {
    save: (defaultName) => ipcRenderer.invoke("dialog:save", defaultName),
    open: (options) => ipcRenderer.invoke("dialog:open", options),
  },

  // ── Downloads ────────────────────────────────────────────
  downloads: {
    onStarted: (callback) => {
      const handler = (_, data) => callback(data);
      ipcRenderer.on("download:started", handler);
      return () => { ipcRenderer.removeListener("download:started", handler); };
    },
    onProgress: (callback) => {
      const handler = (_, data) => callback(data);
      ipcRenderer.on("download:progress", handler);
      return () => { ipcRenderer.removeListener("download:progress", handler); };
    },
    onComplete: (callback) => {
      const handler = (_, data) => callback(data);
      ipcRenderer.on("download:complete", handler);
      return () => { ipcRenderer.removeListener("download:complete", handler); };
    },
    cancel: (itemId) => ipcRenderer.invoke("download:cancel", itemId),
  },

  // ── Shell ────────────────────────────────────────────────
  shell: {
    openExternal: (url) => ipcRenderer.invoke("shell:openExternal", url),
    openPath: (filePath) => ipcRenderer.invoke("shell:openPath", filePath),
    showItemInFolder: (filePath) => ipcRenderer.invoke("shell:showItemInFolder", filePath),
  },

  // ── App ──────────────────────────────────────────────────
  app: {
    quit: () => ipcRenderer.send("app:quit"),
    restart: () => ipcRenderer.send("app:restart"),
    getVersion: () => ipcRenderer.invoke("app:version"),
    isPackaged: () => ipcRenderer.invoke("app:isPackaged"),
  },

  // ── Cursor ──────────────────────────────────────────────
  cursor: {
    setCursor: (type, theme) => ipcRenderer.invoke("cursor:set", { type, theme }),
  },

  // ── Bench ─────────────────────────────────────────────────
  bench: {
    latest: () => ipcRenderer.invoke("bench:latest"),
    history: () => ipcRenderer.invoke("bench:history"),
    baselines: () => ipcRenderer.invoke("bench:baselines"),
    run: (mode) => ipcRenderer.invoke("bench:run", mode),
    importBaseline: (path) => ipcRenderer.invoke("bench:import-baseline", path),
    exportJson: () => ipcRenderer.invoke("bench:export-json"),
    exportCsv: () => ipcRenderer.invoke("bench:export-csv"),
    optimizerInput: () => ipcRenderer.invoke("bench:optimizer-input"),
    onProgress: (callback) => {
      const handler = (_, data) => callback(data);
      ipcRenderer.on("bench:progress", handler);
      return () => ipcRenderer.removeListener("bench:progress", handler);
    },
    onComplete: (callback) => {
      const handler = (_, data) => callback(data);
      ipcRenderer.on("bench:complete", handler);
      return () => ipcRenderer.removeListener("bench:complete", handler);
    },
  },
});
