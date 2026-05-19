const { ipcMain, BrowserWindow } = require("electron");

function registerWindowControls(mainWindow) {
  ipcMain.handle("window:minimize", () => {
    mainWindow?.minimize();
  });

  ipcMain.handle("window:maximize", () => {
    if (mainWindow?.isMaximized()) {
      mainWindow.unmaximize();
    } else {
      mainWindow?.maximize();
    }
  });

  ipcMain.handle("window:unmaximize", () => {
    mainWindow?.unmaximize();
  });

  ipcMain.handle("window:close", () => {
    mainWindow?.close();
  });

  ipcMain.handle("window:isMaximized", () => {
    return mainWindow?.isMaximized() || false;
  });
}

module.exports = { registerWindowControls };
