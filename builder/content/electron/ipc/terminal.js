const { ipcMain } = require("electron");

function registerTerminal(terminalManager) {
  if (!terminalManager) return;

  ipcMain.handle("terminal:create", async (_, id, options) => {
    return terminalManager.create(id, options);
  });

  ipcMain.handle("terminal:write", async (_, id, data) => {
    terminalManager.write(id, data);
  });

  ipcMain.handle("terminal:resize", async (_, id, cols, rows) => {
    terminalManager.resize(id, cols, rows);
  });

  ipcMain.handle("terminal:kill", async (_, id) => {
    terminalManager.kill(id);
  });
}

module.exports = { registerTerminal };
