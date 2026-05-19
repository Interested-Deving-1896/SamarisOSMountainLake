const { ipcMain, clipboard } = require("electron");

function registerClipboard() {
  ipcMain.handle("clipboard:readText", () => {
    return clipboard.readText();
  });

  ipcMain.handle("clipboard:writeText", (_, text) => {
    clipboard.writeText(String(text));
  });
}

module.exports = { registerClipboard };
