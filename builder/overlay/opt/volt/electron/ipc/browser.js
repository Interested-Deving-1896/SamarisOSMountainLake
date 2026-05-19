const { ipcMain } = require("electron");

function normalizeCreateArgs(urlOrOptions, _bounds, opts) {
  if (urlOrOptions && typeof urlOrOptions === "object") return urlOrOptions;
  return { url: urlOrOptions || "about:blank", private: Boolean(opts?.private), activate: true };
}

function registerBrowser(mainWindow, browserManager) {
  if (!browserManager) return;

  browserManager.setMainWindow(mainWindow);

  ipcMain.handle("browser:createTab", async (_, urlOrOptions, bounds, opts) => {
    return browserManager.createTab(normalizeCreateArgs(urlOrOptions, bounds, opts));
  });

  ipcMain.handle("browser:createPrivateTab", async (_, url) => {
    return browserManager.createTab({ url: url || "about:blank", private: true, activate: true });
  });

  ipcMain.handle("browser:getSnapshot", async () => {
    return browserManager.getSnapshot();
  });

  ipcMain.handle("browser:getTabs", async () => {
    return browserManager.getTabs();
  });

  ipcMain.handle("browser:navigate", async (_, tabId, url) => {
    return browserManager.navigate(tabId, url);
  });

  ipcMain.handle("browser:activateTab", async (_, tabId) => {
    return browserManager.activateTab(tabId);
  });

  ipcMain.handle("browser:closeTab", async (_, tabId) => {
    return browserManager.closeTab(tabId);
  });

  ipcMain.handle("browser:setBounds", async (_, tabId, bounds) => {
    browserManager.setBounds(tabId, bounds);
    return { ok: true };
  });

  ipcMain.handle("browser:command", async (_, tabId, command, payload) => {
    return browserManager.command(tabId, command, payload || {});
  });

  ipcMain.handle("browser:goBack", async (_, tabId) => browserManager.command(tabId, "back"));
  ipcMain.handle("browser:goForward", async (_, tabId) => browserManager.command(tabId, "forward"));
  ipcMain.handle("browser:reload", async (_, tabId) => browserManager.command(tabId, "reload"));
  ipcMain.handle("browser:stop", async (_, tabId) => browserManager.command(tabId, "stop"));
  ipcMain.handle("browser:savePage", async (_, tabId) => browserManager.command(tabId, "savePage"));
  ipcMain.handle("browser:printPage", async (_, tabId) => browserManager.command(tabId, "print"));
  ipcMain.handle("browser:openDevTools", async (_, tabId) => browserManager.command(tabId, "openDevTools"));

  ipcMain.handle("browser:setZoom", async (_, tabId, factor) => {
    return browserManager.setZoom(tabId, factor);
  });

  ipcMain.handle("browser:showToolbarMenu", async (_, point) => {
    return browserManager.showToolbarMenu(point || {});
  });

  ipcMain.handle("browser:clearData", async (_, options) => {
    return browserManager.clearData(options || { scope: "all" });
  });

  ipcMain.handle("browser:reorderTabs", async (_, tabIds) => {
    return browserManager.reorderTabs(tabIds);
  });

  ipcMain.handle("browser:destroyAll", async () => {
    browserManager.destroyAll();
    return { ok: true };
  });

  ipcMain.handle("cursor:set", async (_, { type, theme }) => {
    browserManager.setCursor(type, theme);
    return { ok: true };
  });
}

module.exports = { registerBrowser };
