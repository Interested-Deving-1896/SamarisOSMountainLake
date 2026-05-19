import { appRegistry } from "./appRegistry";
import { processManager } from "../core/processManager";
import { osStore } from "../core/osStore";
import { windowManager } from "../core/windowManager";

class AppLoader {
  listApps() {
    return Object.values(appRegistry);
  }

  async openApp(appId: string, opts?: { windowParams?: Record<string, unknown>; forceNewWindow?: boolean }) {
    const app = appRegistry[appId];
    if (!app) {
      console.error("[SAMARIS] App not found:", appId);
      return null;
    }

    const existingWindow = opts?.forceNewWindow
      ? null
      : osStore
          .getState()
          .windows
          .filter((window) => window.appId === appId)
          .sort((a, b) => b.z - a.z)[0] || null;

    if (existingWindow) {
      if (existingWindow.minimized) {
        windowManager.restore(existingWindow.id);
      } else {
        windowManager.focus(existingWindow.id);
      }
      if (opts?.windowParams) {
        windowManager.updateLocal(existingWindow.id, { params: opts.windowParams });
      }
      return osStore.getState().processes.find((process) => process.windowId === existingWindow.id) || null;
    }

    const process = processManager.createProcess(
      app.id,
      app.runtime === "browser" ? "chromium" : app.runtime,
      opts?.windowParams
    );
    console.log("[SAMARIS] App launched:", appId);
    return process;
  }
}

export const appLoader = new AppLoader();
