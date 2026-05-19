#!/usr/bin/env node
const { app, BrowserWindow, screen } = require("electron");
const path = require("node:path");

/**
 * Launcher helpers for the native Electron desktop shell.
 * Runs before the BrowserWindow is created — sets up the environment.
 */

const isDev = process.env.NODE_ENV === "development";

async function chooseBestScreenMode() {
  const displays = screen.getAllDisplays();
  const preferred = [1920, 1080, 1600, 900, 1366, 768, 1280, 720, 1024, 768];
  const primary = displays.find((d) => d.bounds.x === 0 && d.bounds.y === 0) || displays[0];
  if (!primary) return { width: 1280, height: 720 };

  const { width, height } = primary.workAreaSize;
  let bestW = 1280, bestH = 720;
  for (let i = 0; i < preferred.length; i += 2) {
    const pw = preferred[i], ph = preferred[i + 1];
    if (pw <= width && ph <= height) {
      bestW = pw;
      bestH = ph;
    }
  }
  return { width: bestW, height: bestH, x: primary.bounds.x, y: primary.bounds.y };
}

function getUIPath() {
  if (isDev) {
    const devPath = path.join(__dirname, "..", "ui", "dist", "index.html");
    const builtPath = path.join(__dirname, "..", "..", "overlay", "opt", "volt", "desktop", "app", "index.html");
    const { existsSync } = require("node:fs");
    if (existsSync(devPath)) return devPath;
    if (existsSync(builtPath)) return builtPath;
    return null;
  }
  return "/opt/volt/desktop/app/index.html";
}

module.exports = { chooseBestScreenMode, getUIPath, isDev };
