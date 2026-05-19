import React from "react";
import ReactDOM from "react-dom/client";
import { Desktop } from "./components/Desktop";
import { RootErrorBoundary } from "./components/RootErrorBoundary";
import "./styles/global.css";
import "./styles/cursors.css";
import "./modules/window-system";
import { kernelClient } from "./os/kernel/kernelClient";
import { BootProvider } from "./system/boot/BootProvider";
import { bootSync } from "./system/boot/BootSync";
import { audioStore } from "./system/audio/audioStore";
import { batteryStore } from "./system/battery/batteryStore";
import { CursorEngine } from "./system/cursor/CursorEngine";
import { connectivityStore } from "./system/connectivity/connectivityStore";
import { securityStore } from "./system/session/securityStore";
import { themeStore } from "./system/theme/themeStore";
import { wallpaperStore } from "./system/wallpaper/wallpaperStore";
import { systemSounds } from "./system/sounds/systemSounds";
import { installSamarisSystemApi } from "./system/systemApi";
import { SamarisIconProvider } from "./modules/icons";
import { initScaleEngine } from "./system/theme/scaleEngine";
import { DndProvider } from "./os/dnd";

if (!window.crypto?.randomUUID) {
  window.crypto.randomUUID = () => {
    const tail = Date.now().toString(16).slice(-12).padStart(12, "0");
    return `00000000-0000-4000-8000-${tail}` as ReturnType<Crypto["randomUUID"]>;
  };
}

const isElectron = typeof window !== "undefined" && !!window.electronAPI;

void kernelClient.connect();
themeStore.init();
wallpaperStore.init();
connectivityStore.init();
audioStore.init();
batteryStore.init();
securityStore.init();
systemSounds.init();
installSamarisSystemApi();
initScaleEngine();

import { downloadStore } from "./system/downloads/downloadStore";

// Electron: set up download IPC listeners
if (isElectron) {
  downloadStore.init();

  window.electronAPI!.browser.onTabUpdate(() => {
    // Tab state updated in peregrine hook
  });

  window.electronAPI!.downloads.onStarted((data) => {
    downloadStore.addPending(data.filename, data.url, data.totalBytes, data.id);
  });

  window.electronAPI!.downloads.onProgress((data) => {
    downloadStore.updateProgress(data.id, data.received, data.total || data.totalBytes || 0);
  });

  window.electronAPI!.downloads.onComplete((data) => {
    if (data.success) {
      downloadStore.complete(data.id, data.savePath);
    } else if (data.state === "cancelled" || data.error === "cancelled") {
      downloadStore.cancel(data.id);
    } else {
      downloadStore.fail(data.id, data.error || "Download failed");
    }
  });
} else {
  downloadStore.init();
}

ReactDOM.createRoot(document.getElementById("root")!).render(
  <RootErrorBoundary>
    <BootProvider>
      <CursorEngine />
      <SamarisIconProvider theme="auto" variant="soft">
        <DndProvider>
          <Desktop />
        </DndProvider>
      </SamarisIconProvider>
    </BootProvider>
  </RootErrorBoundary>
);

// Electron: no iframe parent to signal, just mark ready immediately
if (isElectron) {
  bootSync.markReady();
} else {
  // Browser dev mode: still signal the loader iframe if it exists
  requestAnimationFrame(() => {
    requestAnimationFrame(() => {
      bootSync.markReady();
    });
  });
}
