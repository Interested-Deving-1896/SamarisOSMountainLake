const { ipcMain, app } = require("electron");
const os = require("node:os");

function registerSystem() {
  ipcMain.handle("system:memory", () => {
    return {
      total: os.totalmem(),
      free: os.freemem(),
      used: os.totalmem() - os.freemem(),
      usagePercent: ((1 - os.freemem() / os.totalmem()) * 100).toFixed(1),
    };
  });

  ipcMain.handle("system:gpu", async () => {
    const { app } = require("electron");
    const gpuInfo = app.getGPUInfo("basic");
    return gpuInfo;
  });
}

module.exports = { registerSystem };
