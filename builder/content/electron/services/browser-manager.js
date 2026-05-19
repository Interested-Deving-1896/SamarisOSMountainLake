const { WebContentsView, session, dialog, Menu, clipboard, shell } = require("electron");
const path = require("node:path");
const fs = require("node:fs");
const os = require("node:os");
const crypto = require("node:crypto");

const DISCARD_AFTER_MS = 10 * 60 * 1000;
const DISCARD_SWEEP_MS = 60 * 1000;
const DEFAULT_HOME = "about:blank";

const ALLOWED_FILE_ROOTS = [
  path.join(os.homedir(), ".volt"),
  path.join(os.homedir(), "Downloads"),
];

function isPathInside(candidate, root) {
  const resolved = path.resolve(candidate);
  const base = path.resolve(root);
  const relative = path.relative(base, resolved);
  return relative === "" || (!relative.startsWith("..") && !path.isAbsolute(relative));
}

function normalizeUrl(rawUrl, options = {}) {
  const raw = String(rawUrl || "").trim();
  const fallbackSearch = options.search ?? true;
  if (!raw) return DEFAULT_HOME;
  if (raw === "about:blank") return raw;

  if (raw.startsWith("view-source:")) {
    const inner = normalizeUrl(raw.slice("view-source:".length), { search: false });
    return inner ? `view-source:${inner}` : null;
  }

  if (/^[\w.-]+\.[a-z]{2,}/i.test(raw) || raw.startsWith("localhost")) {
    const protocol = raw.startsWith("localhost") ? "http" : "https";
    try { return new URL(`${protocol}://${raw.replace(/^\/+/, "")}`).href; } catch {}
  }

  try {
    const parsed = new URL(raw);
    if (parsed.protocol === "http:" || parsed.protocol === "https:") return parsed.href;
    if (parsed.protocol === "file:") {
      const filePath = decodeURIComponent(parsed.pathname);
      return ALLOWED_FILE_ROOTS.some((root) => isPathInside(filePath, root)) ? parsed.href : null;
    }
    return null;
  } catch {}

  return fallbackSearch ? `https://www.google.com/search?q=${encodeURIComponent(raw)}` : null;
}

function isHomeUrl(url) {
  return !url || url === "about:blank";
}

function toPublicTab(tab) {
  return {
    id: tab.id,
    url: tab.url,
    title: tab.title,
    loading: tab.loading,
    canGoBack: tab.canGoBack,
    canGoForward: tab.canGoForward,
    favicon: tab.favicon,
    active: Boolean(tab.active),
    private: Boolean(tab.private),
    crashed: Boolean(tab.crashed),
    discarded: Boolean(tab.discarded),
    zoom: tab.zoom,
  };
}

class BrowserManager {
  constructor(downloadManager) {
    if (!WebContentsView) {
      throw new Error("WebContentsView is unavailable. Upgrade Electron to 30 or newer.");
    }

    this.tabs = new Map();
    this.tabCounter = 0;
    this.activeTabId = null;
    this.mainWindow = null;
    this.downloadManager = downloadManager || null;
    this._updateListeners = new Set();
    this._sessionListeners = new Map();
    this._viewportBounds = { x: 0, y: 76, width: 800, height: 524 };
    this._discardTimer = setInterval(() => this.discardInactiveTabs(), DISCARD_SWEEP_MS);
    this._discardTimer.unref?.();

    this._cursorCache = { light: new Map(), dark: new Map() };
    this._currentCursorType = "default";
    this._currentCursorTheme = "light";
    this._loadCursorCache();

    this._configureSession(session.fromPartition("persist:peregrine"));
  }

  setMainWindow(win) {
    this.mainWindow = win;
  }

  _resolveCursorsDir() {
    const isDev = process.env.NODE_ENV === "development";
    if (isDev) {
      return path.join(__dirname, "..", "..", "ui", "public", "cursors");
    }
    const prodDir = fs.existsSync("/opt/volt/desktop/app")
      ? "/opt/volt/desktop/app"
      : path.join(__dirname, "..", "..", "..", "overlay", "opt", "volt", "desktop", "app");
    return path.join(prodDir, "cursors");
  }

  _loadCursorCache() {
    const cursorsDir = this._resolveCursorsDir();
    if (!fs.existsSync(cursorsDir)) {
      console.warn("[peregrine] cursor directory not found:", cursorsDir);
      return;
    }
    const types = {
      arrow: "default", hand: "pointer", ibeam: "text", crosshair: "crosshair",
      help: "help", no: "not-allowed", sizeall: "move", sizens: "ns-resize",
      sizewe: "ew-resize", sizenwse: "nwse-resize", sizenesw: "nesw-resize",
      nwpen: "writing", person: "person", pin: "pin",
    };
    for (const theme of ["light", "dark"]) {
      const themeDir = path.join(cursorsDir, theme);
      if (!fs.existsSync(themeDir)) continue;
      const files = fs.readdirSync(themeDir).filter((f) => f.endsWith(".cur"));
      for (const file of files) {
        const key = file.replace(".cur", "");
        const mapped = types[key];
        if (!mapped) continue;
        try {
          const data = fs.readFileSync(path.join(themeDir, file));
          const b64 = data.toString("base64");
          this._cursorCache[theme].set(mapped, `data:image/x-cur;base64,${b64}`);
        } catch (e) {
          console.warn(`[peregrine] failed to load cursor ${file}:`, e.message);
        }
      }
      this._cursorCache[theme].set("grab", this._cursorCache[theme].get("pointer") || "");
      this._cursorCache[theme].set("grabbing", this._cursorCache[theme].get("pointer") || "");
      this._cursorCache[theme].set("col-resize", this._cursorCache[theme].get("ew-resize") || "");
      this._cursorCache[theme].set("row-resize", this._cursorCache[theme].get("ns-resize") || "");
    }
  }

  _injectCursorCSS(view, type, theme) {
    if (!view?.webContents || view.webContents.isDestroyed()) return;
    const cache = this._cursorCache[theme || "light"];
    if (!cache || cache.size === 0) return;
    const dataUri = cache.get(type || "default");
    if (!dataUri) return;
    const css = `html,body{cursor:${dataUri},auto!important}html *,body *{cursor:inherit!important}`;
    try {
      view.webContents.insertCSS(css);
    } catch (e) {
      console.warn("[peregrine] cursor CSS injection failed:", e.message);
    }
  }

  setCursor(type, theme) {
    this._currentCursorType = type;
    this._currentCursorTheme = theme;
    for (const tab of this.tabs.values()) {
      if (tab.view) this._injectCursorCSS(tab.view, type, theme);
    }
  }

  onUpdate(callback) {
    this._updateListeners.add(callback);
    return () => this._updateListeners.delete(callback);
  }

  getSnapshot() {
    return {
      activeTabId: this.activeTabId,
      tabs: [...this.tabs.values()].map(toPublicTab),
    };
  }

  _emit(event, data) {
    for (const cb of this._updateListeners) cb(event, data);
    if (this.mainWindow && !this.mainWindow.isDestroyed()) {
      try { this.mainWindow.webContents.send(event, data); } catch {}
    }
  }

  _emitTab(tab, patch = {}) {
    const publicTab = { ...toPublicTab(tab), ...patch };
    this._emit("browser:tab-updated", publicTab);
    this._emit("browser:snapshot", this.getSnapshot());
  }

  _emitSnapshot() {
    this._emit("browser:snapshot", this.getSnapshot());
  }

  _configureSession(sess) {
    if (!sess || this._sessionListeners.has(sess)) return;

    const onWillDownload = (event, item) => {
      this.downloadManager?.onWillDownload(event, item);
    };

    const onPermission = (webContents, permission, callback, details) => {
      const tab = this._findTabByWebContents(webContents);
      this._emit("permission:request", {
        tabId: tab?.id || null,
        url: details?.requestingUrl || webContents?.getURL?.() || "",
        permission,
      });
      callback(false);
    };

    if (this.downloadManager) sess.on("will-download", onWillDownload);
    sess.setPermissionRequestHandler(onPermission);
    this._sessionListeners.set(sess, { onWillDownload, onPermission });
  }

  _releaseSession(sess) {
    const handlers = this._sessionListeners.get(sess);
    if (!sess || !handlers) return;
    if (handlers.onWillDownload) sess.removeListener("will-download", handlers.onWillDownload);
    try { sess.setPermissionRequestHandler(null); } catch {}
    this._sessionListeners.delete(sess);
  }

  _releasePrivateSessionForTab(tab) {
    if (!tab?.private) return;
    const sess = tab.view?.webContents?.session;
    if (!sess) return;
    this._releaseSession(sess);
  }

  _partitionForTab(privateMode) {
    return privateMode ? `peregrine-private-${crypto.randomUUID()}` : "persist:peregrine";
  }

  _createView(tab) {
    const view = new WebContentsView({
      webPreferences: {
        partition: tab.partition,
        contextIsolation: true,
        nodeIntegration: false,
        sandbox: true,
        backgroundThrottling: true,
        devTools: true,
      },
    });

    tab.view = view;
    tab.discarded = false;
    this._configureSession(view.webContents.session);

    const wc = view.webContents;
    wc.setWindowOpenHandler(({ url }) => {
      const normalized = normalizeUrl(url, { search: false });
      if (normalized) {
        this.createTab({ url: normalized, private: tab.private, activate: true });
      }
      return { action: "deny" };
    });

    wc.on("page-title-updated", (_, title) => {
      tab.title = title || tab.title;
      this._emitTab(tab);
    });

    const onNav = (_, navUrl) => {
      tab.url = navUrl || tab.url;
      tab.canGoBack = wc.canGoBack();
      tab.canGoForward = wc.canGoForward();
      tab.crashed = false;
      this._emitTab(tab);
    };
    wc.on("did-navigate", onNav);
    wc.on("did-navigate-in-page", onNav);

    wc.on("did-start-loading", () => {
      tab.loading = true;
      tab.crashed = false;
      this._emitTab(tab);
    });

    wc.on("did-stop-loading", () => {
      tab.loading = false;
      tab.canGoBack = wc.canGoBack();
      tab.canGoForward = wc.canGoForward();
      this._emitTab(tab);
    });

    wc.on("did-finish-load", () => {
      this._injectCursorCSS(view, this._currentCursorType, this._currentCursorTheme);
    });

    wc.on("did-fail-load", (_, errorCode, errorDescription, validatedURL, isMainFrame) => {
      if (!isMainFrame || errorCode === -3) return;
      tab.loading = false;
      tab.lastError = errorDescription;
      this._emitTab(tab, { error: errorDescription, url: validatedURL || tab.url });
    });

    wc.on("page-favicon-updated", (_, favicons) => {
      if (favicons?.length) {
        tab.favicon = favicons[0];
        this._emitTab(tab);
      }
    });

    wc.on("render-process-gone", (_, details) => {
      tab.loading = false;
      tab.crashed = true;
      this._emitTab(tab, { error: details?.reason || "render-process-gone" });
    });

    wc.on("context-menu", (_, params) => {
      if (tab.id !== this.activeTabId) return;
      this._showContextMenu(tab, params);
    });

    return view;
  }

  _contextMenuPoint(params) {
    const rawX = Math.round(params?.x ?? 0);
    const rawY = Math.round(params?.y ?? 0);
    const contentBounds = this.mainWindow?.getContentBounds?.() || { width: 1920, height: 1080 };

    if (rawX >= 0 && rawX < contentBounds.width && rawY >= 0 && rawY < contentBounds.height) {
      return { x: Math.min(rawX, contentBounds.width - 1), y: Math.min(rawY, contentBounds.height - 1) };
    }

    const bounds = this._viewportBounds || { x: 0, y: 0, width: 800, height: 600 };
    return {
      x: Math.max(0, Math.min(rawX + (bounds.x || 0), contentBounds.width - 1)),
      y: Math.max(0, Math.min(rawY + (bounds.y || 0), contentBounds.height - 1)),
    };
  }

  _downloadURL(tab, url) {
    if (!url) return;
    const wc = tab.view?.webContents;
    if (!wc || wc.isDestroyed()) return;
    try { wc.downloadURL(url); } catch (e) { console.error("[peregrine] download url failed:", e.message); }
  }

  _openInNewTab(url, privateMode = false) {
    const normalized = normalizeUrl(url, { search: false });
    if (normalized) this.createTab({ url: normalized, private: privateMode, activate: true });
  }

  _openExternal(url) {
    try {
      const parsed = new URL(url);
      if (parsed.protocol === "http:" || parsed.protocol === "https:") shell.openExternal(parsed.href);
    } catch {}
  }

  _showContextMenu(tab, params) {
    const wc = tab.view?.webContents;
    if (!wc || wc.isDestroyed()) return;

    const editFlags = params.editFlags || {};
    const pageUrl = params.pageURL || wc.getURL() || tab.url;
    const canViewSource = Boolean(pageUrl) && !pageUrl.startsWith("view-source:");
    const run = (command, payload) => () => { void this.command(tab.id, command, payload); };
    const linkURL = params.linkURL || "";
    const srcURL = params.srcURL || "";
    const hasLink = /^https?:\/\//i.test(linkURL);
    const hasMedia = params.mediaType && params.mediaType !== "none" && Boolean(srcURL);
    const srcKnown = /^https?:\/\//i.test(srcURL);
    const mediaLabel = params.mediaType && params.mediaType !== "none"
      ? params.mediaType.charAt(0).toUpperCase() + params.mediaType.slice(1)
      : "Media";
    const hasSelection = Boolean(params.selectionText?.trim());
    const hasSuggestions = Array.isArray(params.dictionarySuggestions) && params.dictionarySuggestions.length > 0;
    const template = [];

    // ── Link section ──
    if (hasLink) {
      template.push(
        { label: "Open Link in New Tab", click: () => this._openInNewTab(linkURL, tab.private) },
        { label: "Open Link in Private Tab", click: () => this._openInNewTab(linkURL, true) },
        { label: "Open Link in System Browser", click: () => this._openExternal(linkURL) },
        { type: "separator" },
        { label: "Copy Link Address", click: () => clipboard.writeText(linkURL) },
        { label: "Save Link As...", click: () => this._downloadURL(tab, linkURL) },
        { type: "separator" },
      );
    }

    // ── Media section ──
    if (hasMedia) {
      if (srcKnown) {
        template.push(
          { label: `Open ${mediaLabel} in New Tab`, click: () => this._openInNewTab(srcURL, tab.private) },
          { label: `Open ${mediaLabel} in Private Tab`, click: () => this._openInNewTab(srcURL, true) },
          { label: `Copy ${mediaLabel} Address`, click: () => clipboard.writeText(srcURL) },
        );
      }
      template.push(
        { label: `Save ${mediaLabel} As...`, click: () => this._downloadURL(tab, srcURL) },
      );
      if (srcKnown) {
        template.push({ type: "separator" });
        if (params.mediaType === "image") {
          template.push(
            { label: "Search Image with Google", click: () => { const url = `https://www.google.com/searchbyimage?image_url=${encodeURIComponent(srcURL)}`; try { wc.loadURL(url); } catch {} } },
          );
        }
      }
      template.push({ type: "separator" });
    }

    // ── Spelling suggestions ──
    if (hasSuggestions) {
      for (const suggestion of params.dictionarySuggestions) {
        template.push({
          label: `Fix spelling: "${suggestion}"`,
          click: () => { try { wc.replaceMisspelling(suggestion); } catch {} },
        });
      }
      template.push(
        { type: "separator" },
        { label: "Add to Dictionary", click: () => { try { if (wc.session?.addWordToSpellCheckerDictionary) wc.session.addWordToSpellCheckerDictionary(params.selectionText || ""); } catch {} } },
        { type: "separator" },
      );
    }

    // ── Search / Selection section ──
    if (hasSelection) {
      const q = encodeURIComponent(params.selectionText.trim());
      template.push(
        { label: `Search Google for "${params.selectionText.trim().slice(0, 40)}"`, click: () => { try { wc.loadURL(`https://www.google.com/search?q=${q}`); } catch {} } },
      );
      template.push({ type: "separator" });
    }

    // ── Navigation section ──
    template.push(
      { label: "Back", enabled: wc.canGoBack(), click: run("back") },
      { label: "Forward", enabled: wc.canGoForward(), click: run("forward") },
      { label: "Reload", click: run("reload") },
      { label: "Stop", enabled: Boolean(tab.loading), click: run("stop") },
      { type: "separator" },
      { label: "Save Page As...", click: run("savePage") },
      { label: "Print...", click: run("print") },
      { label: "Copy Page Address", enabled: Boolean(pageUrl), click: () => clipboard.writeText(pageUrl) },
      { type: "separator" },
      { label: "Undo", accelerator: "CmdOrCtrl+Z", enabled: Boolean(editFlags.canUndo), click: run("undo") },
      { label: "Redo", accelerator: "Shift+CmdOrCtrl+Z", enabled: Boolean(editFlags.canRedo), click: run("redo") },
      { type: "separator" },
      { label: "Copy", accelerator: "CmdOrCtrl+C", enabled: Boolean(editFlags.canCopy || params.selectionText), click: run("copy") },
      { label: "Cut", accelerator: "CmdOrCtrl+X", enabled: Boolean(editFlags.canCut), click: run("cut") },
      { label: "Paste", accelerator: "CmdOrCtrl+V", enabled: Boolean(editFlags.canPaste), click: run("paste") },
      { label: "Paste and Match Style", enabled: Boolean(editFlags.canPaste && editFlags.canEditRichly), click: run("pasteAndMatchStyle") },
      { label: "Delete", enabled: Boolean(editFlags.canDelete), click: run("delete") },
      { label: "Select All", accelerator: "CmdOrCtrl+A", enabled: editFlags.canSelectAll !== false, click: run("selectAll") },
      { type: "separator" },
    );

    // ── Spell check toggle ──
    template.push({
      label: "Check Spelling While Typing",
      type: "checkbox",
      checked: wc.session?.isSpellCheckerEnabled?.() !== false,
      click: () => {
        try {
          if (wc.session?.isSpellCheckerEnabled?.()) wc.session.setSpellCheckerEnabled(false);
          else wc.session?.setSpellCheckerEnabled?.(true);
        } catch {}
      },
    });
    template.push({ type: "separator" });

    // ── Translate section ──
    template.push({
      label: "Translate Page...",
      enabled: Boolean(pageUrl) && /^https?:\/\//i.test(pageUrl),
      click: () => {
        if (pageUrl) try { wc.loadURL(`https://translate.google.com/translate?hl=&sl=auto&tl=en&u=${encodeURIComponent(pageUrl)}`); } catch {}
      },
    });

    // ── Developer section ──
    template.push(
      { type: "separator" },
      {
        label: "View Page Source",
        enabled: canViewSource,
        click: () => { if (canViewSource) this.navigate(tab.id, `view-source:${pageUrl}`); },
      },
      {
        label: "Inspect Element",
        click: () => {
          try { wc.inspectElement(params.x || 0, params.y || 0); } catch {}
          try { if (!wc.isDevToolsOpened()) wc.openDevTools({ mode: "detach" }); } catch {}
        },
      },
      {
        label: "Developer Tools",
        click: () => { try { wc.openDevTools({ mode: "detach" }); } catch {} },
      },
    );

    const menu = Menu.buildFromTemplate(template);
    const popupOptions = {
      window: this.mainWindow || undefined,
      frame: params.frame || undefined,
      sourceType: params.menuSourceType,
      ...this._contextMenuPoint(params),
    };

    try {
      menu.popup(popupOptions);
    } catch (e) {
      console.error("[peregrine] context menu failed:", e.message);
    }
  }

  showToolbarMenu(point = {}) {
    const tab = this.tabs.get(this.activeTabId);
    const wc = tab?.view?.webContents;
    const pageUrl = wc && !wc.isDestroyed() ? wc.getURL() : tab?.url || "";
    const isRealPage = Boolean(tab && !isHomeUrl(pageUrl));
    const zoom = tab?.zoom || 1;
    const template = [
      { label: "New Tab", accelerator: "CmdOrCtrl+T", click: () => this.createTab({ url: "about:blank", activate: true }) },
      { label: "New Private Tab", click: () => this.createTab({ url: "about:blank", private: true, activate: true }) },
      { type: "separator" },
      { label: "Back", enabled: Boolean(wc && !wc.isDestroyed() && wc.canGoBack()), click: () => { if (tab) void this.command(tab.id, "back"); } },
      { label: "Forward", enabled: Boolean(wc && !wc.isDestroyed() && wc.canGoForward()), click: () => { if (tab) void this.command(tab.id, "forward"); } },
      { label: "Reload", enabled: Boolean(tab), click: () => { if (tab) void this.command(tab.id, "reload"); } },
      { label: "Stop", enabled: Boolean(tab?.loading), click: () => { if (tab) void this.command(tab.id, "stop"); } },
      { type: "separator" },
      { label: "Save Page As...", enabled: isRealPage, click: () => { if (tab) void this.command(tab.id, "savePage"); } },
      { label: "Print...", enabled: isRealPage, click: () => { if (tab) void this.command(tab.id, "print"); } },
      { label: "Open in System Browser", enabled: /^https?:\/\//i.test(pageUrl), click: () => this._openExternal(pageUrl) },
      { type: "separator" },
      {
        label: `Zoom (${Math.round(zoom * 100)}%)`,
        submenu: [
          { label: "Zoom In", accelerator: "CmdOrCtrl+Plus", enabled: Boolean(tab && zoom < 5), click: () => { if (tab) this.setZoom(tab.id, +(zoom + 0.25).toFixed(2)); } },
          { label: "Zoom Out", accelerator: "CmdOrCtrl+-", enabled: Boolean(tab && zoom > 0.25), click: () => { if (tab) this.setZoom(tab.id, +(zoom - 0.25).toFixed(2)); } },
          { label: "Reset Zoom", accelerator: "CmdOrCtrl+0", enabled: Boolean(tab && zoom !== 1), click: () => { if (tab) this.setZoom(tab.id, 1); } },
        ],
      },
      { type: "separator" },
      { label: "Clear Browsing Data", click: () => { void this.clearData({ scope: "all", data: ["cache", "cookies", "storage", "serviceWorkers"] }); } },
      { label: "Developer Tools", enabled: Boolean(wc && !wc.isDestroyed()), click: () => { if (tab) void this.command(tab.id, "openDevTools"); } },
    ];
    const menu = Menu.buildFromTemplate(template);
    const contentBounds = this.mainWindow?.getContentBounds?.() || { width: 800, height: 600 };
    const x = Math.max(0, Math.min(Math.round(Number(point.x) || contentBounds.width - 24), Math.max(0, contentBounds.width - 1)));
    const y = Math.max(0, Math.min(Math.round(Number(point.y) || 72), Math.max(0, contentBounds.height - 1)));
    try {
      menu.popup({ window: this.mainWindow || undefined, x, y });
    } catch (e) {
      console.error("[peregrine] toolbar menu failed:", e.message);
    }
    return { ok: true };
  }

  _findTabByWebContents(wc) {
    for (const tab of this.tabs.values()) {
      if (tab.view?.webContents?.id === wc?.id) return tab;
    }
    return null;
  }

  _attachActiveView() {
    if (!this.mainWindow || this.mainWindow.isDestroyed()) return;
    const active = this.tabs.get(this.activeTabId);

    for (const tab of this.tabs.values()) {
      if (!tab.view) continue;
      try {
        this.mainWindow.contentView.removeChildView(tab.view);
      } catch {}
      tab.active = false;
    }

    if (!active) {
      this._emitSnapshot();
      return;
    }

    active.active = true;
    active.lastActiveAt = Date.now();
    if (isHomeUrl(active.url)) {
      this._emitTab(active);
      return;
    }
    const view = active.view || this._createView(active);
    try {
      this.mainWindow.contentView.addChildView(view);
      view.setBounds(this._viewportBounds);
    } catch (e) {
      console.error("[peregrine] attach view failed:", e.message);
    }
    this._emitTab(active);
  }

  _loadTab(tab, rawUrl) {
    const normalized = normalizeUrl(rawUrl);
    if (!normalized) return { ok: false, error: "unsupported_url" };
    tab.url = normalized;
    if (isHomeUrl(normalized)) {
      if (tab.view) {
        try { this.mainWindow?.contentView?.removeChildView(tab.view); } catch {}
        try { tab.view.webContents.destroy(); } catch {}
        tab.view = null;
      }
      tab.title = tab.private ? "Private Tab" : "Peregrine";
      tab.loading = false;
      tab.canGoBack = false;
      tab.canGoForward = false;
      tab.favicon = null;
      this._emitTab(tab);
      return { ok: true };
    }
    tab.loading = true;
    const view = tab.view || this._createView(tab);
    try {
      view.webContents.loadURL(normalized);
      if (tab.id === this.activeTabId) this._attachActiveView();
      else this._emitTab(tab);
      return { ok: true };
    } catch (e) {
      tab.loading = false;
      this._emitTab(tab, { error: e.message });
      return { ok: false, error: e.message };
    }
  }

  createTab(input = {}) {
    const options = typeof input === "string" ? { url: input } : input;
    const url = options.url || "about:blank";
    const privateMode = Boolean(options.private);
    const activate = options.activate !== false;
    const id = `tab-${++this.tabCounter}`;
    const tab = {
      id,
      url: normalizeUrl(url) || "about:blank",
      title: privateMode ? "Private Tab" : "New Tab",
      loading: false,
      canGoBack: false,
      canGoForward: false,
      favicon: null,
      active: false,
      private: privateMode,
      crashed: false,
      discarded: false,
      zoom: 1,
      created: Date.now(),
      lastActiveAt: Date.now(),
      partition: this._partitionForTab(privateMode),
      view: null,
    };

    this.tabs.set(id, tab);
    this._loadTab(tab, url);

    if (activate) this.activateTab(id);
    else this._emitSnapshot();

    return toPublicTab(tab);
  }

  activateTab(id) {
    const tab = this.tabs.get(id);
    if (!tab) return null;
    this.activeTabId = id;
    if (!tab.view) this._loadTab(tab, tab.url || "about:blank");
    this._attachActiveView();
    if (tab.view) this._injectCursorCSS(tab.view, this._currentCursorType, this._currentCursorTheme);
    return toPublicTab(tab);
  }

  setBounds(_id, bounds) {
    if (!bounds) return;
    const next = {
      x: Math.max(0, Math.round(bounds.x || 0)),
      y: Math.max(0, Math.round(bounds.y || 0)),
      width: Math.max(1, Math.round(bounds.width || 1)),
      height: Math.max(1, Math.round(bounds.height || 1)),
    };
    this._viewportBounds = next;
    const active = this.tabs.get(this.activeTabId);
    if (active?.view) {
      try { active.view.setBounds(next); } catch {}
    }
  }

  navigate(id, url) {
    const tab = this.tabs.get(id);
    if (!tab) return { ok: false, error: "tab_not_found" };
    return this._loadTab(tab, url);
  }

  command(id, command, payload = {}) {
    const tab = this.tabs.get(id);
    if (!tab) return Promise.resolve({ ok: false, error: "tab_not_found" });
    const wc = tab.view?.webContents;
    if (!wc || wc.isDestroyed()) return Promise.resolve({ ok: false, error: "webcontents_unavailable" });

    try {
      switch (command) {
        case "undo": wc.undo(); break;
        case "redo": wc.redo(); break;
        case "back": if (wc.canGoBack()) wc.goBack(); break;
        case "forward": if (wc.canGoForward()) wc.goForward(); break;
        case "reload": wc.reload(); break;
        case "stop": wc.stop(); break;
        case "copy": wc.copy(); break;
        case "cut": wc.cut(); break;
        case "paste": wc.paste(); break;
        case "pasteAndMatchStyle": wc.pasteAndMatchStyle(); break;
        case "delete": wc.delete(); break;
        case "selectAll": wc.selectAll(); break;
        case "print": wc.print({ printBackground: true }); break;
        case "openDevTools": wc.openDevTools(); break;
        case "find":
          if (payload.text) wc.findInPage(String(payload.text), payload.options || {});
          break;
        case "stopFind": wc.stopFindInPage(payload.action || "clearSelection"); break;
        case "savePage": return this.savePage(id);
        default: return Promise.resolve({ ok: false, error: "unknown_command" });
      }
      return Promise.resolve({ ok: true });
    } catch (e) {
      return Promise.resolve({ ok: false, error: e.message });
    }
  }

  savePage(id) {
    const tab = this.tabs.get(id);
    const wc = tab?.view?.webContents;
    if (!tab || !wc || wc.isDestroyed()) return Promise.resolve({ ok: false, error: "tab_not_found" });
    const pageTitle = wc.getTitle?.() || tab.title || "page";
    const safeTitle = pageTitle.replace(/[^a-z0-9]/gi, "_") || "page";
    return dialog.showSaveDialog(this.mainWindow || undefined, {
      defaultPath: `${safeTitle}.html`,
      filters: [{ name: "Web Page", extensions: ["html"] }],
    }).then(async (result) => {
      if (result.canceled || !result.filePath) return { ok: false, canceled: true };
      await wc.savePage(result.filePath, "HTMLComplete");
      return { ok: true, path: result.filePath };
    }).catch((e) => ({ ok: false, error: e.message }));
  }

  setZoom(id, factor) {
    const tab = this.tabs.get(id);
    if (!tab) return { ok: false, error: "tab_not_found" };
    const zoom = Math.max(0.25, Math.min(5, Number(factor) || 1));
    tab.zoom = zoom;
    if (tab.view?.webContents && !tab.view.webContents.isDestroyed()) {
      tab.view.webContents.setZoomFactor(zoom);
    }
    this._emitTab(tab);
    return { ok: true };
  }

  clearData(options = {}) {
    const scope = options.scope || (typeof options === "string" ? "active" : "all");
    const requested = options.data || ["cache", "cookies", "storage", "serviceWorkers"];
    const targetTabs = scope === "active"
      ? [this.tabs.get(this.activeTabId)].filter(Boolean)
      : [...this.tabs.values()];
    const sessions = new Set(targetTabs.map((tab) => tab.view?.webContents?.session).filter(Boolean));

    const promises = [...sessions].map(async (sess) => {
      if (requested.includes("cache")) await sess.clearCache();
      const storages = [];
      if (requested.includes("cookies")) storages.push("cookies");
      if (requested.includes("storage")) storages.push("localstorage", "indexdb", "websql", "cachestorage");
      if (requested.includes("serviceWorkers")) storages.push("serviceworkers");
      if (storages.length) await sess.clearStorageData({ storages });
    });

    return Promise.allSettled(promises).then(() => ({ ok: true }));
  }

  reorderTabs(tabIds = []) {
    if (!Array.isArray(tabIds)) return { ok: false, error: "invalid_order" };
    const next = new Map();
    for (const id of tabIds) {
      const tab = this.tabs.get(id);
      if (tab) next.set(id, tab);
    }
    for (const [id, tab] of this.tabs) {
      if (!next.has(id)) next.set(id, tab);
    }
    this.tabs = next;
    this._emitSnapshot();
    return { ok: true };
  }

  closeTab(id) {
    const tab = this.tabs.get(id);
    if (!tab) return { ok: false, error: "tab_not_found" };
    const wasActive = this.activeTabId === id;
    this._releasePrivateSessionForTab(tab);
    if (tab.view) {
      try { this.mainWindow?.contentView?.removeChildView(tab.view); } catch {}
      try { tab.view.webContents.destroy(); } catch {}
    }
    this.tabs.delete(id);

    if (wasActive) {
      const remaining = [...this.tabs.keys()];
      this.activeTabId = remaining.length ? remaining[remaining.length - 1] : null;
      this._attachActiveView();
    } else {
      this._emitSnapshot();
    }
    return { ok: true };
  }

  getTabs() {
    return this.getSnapshot().tabs;
  }

  discardInactiveTabs() {
    const now = Date.now();
    for (const tab of this.tabs.values()) {
      if (tab.id === this.activeTabId || !tab.view || tab.private) continue;
      if (now - tab.lastActiveAt < DISCARD_AFTER_MS) continue;
      try { this.mainWindow?.contentView?.removeChildView(tab.view); } catch {}
      try { tab.view.webContents.destroy(); } catch {}
      tab.view = null;
      tab.discarded = true;
      tab.loading = false;
      this._emitTab(tab);
    }
  }

  destroyAll() {
    for (const tab of this.tabs.values()) {
      this._releasePrivateSessionForTab(tab);
      if (!tab.view) continue;
      try { this.mainWindow?.contentView?.removeChildView(tab.view); } catch {}
      try { tab.view.webContents.destroy(); } catch {}
    }
    this.tabs.clear();
    this.activeTabId = null;
    this._emitSnapshot();
  }

  destroy() {
    if (this._discardTimer) clearInterval(this._discardTimer);
    this.destroyAll();
    this._updateListeners.clear();
    for (const [sess, handlers] of this._sessionListeners) {
      if (handlers.onWillDownload) sess.removeListener("will-download", handlers.onWillDownload);
      sess.setPermissionRequestHandler(null);
    }
    this._sessionListeners.clear();
  }
}

module.exports = { BrowserManager, normalizeUrl, isPathInside };
