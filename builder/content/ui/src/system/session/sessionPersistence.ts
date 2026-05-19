import { osStore } from "../../os/core/osStore";
import { sessionManager } from "../../os/core/sessionManager";
import { securityStore } from "./securityStore";
import { clampWindowGeometry, getMaximizedGeometry, getSnappedGeometry } from "../../modules/window-system";
import type { AppWindow } from "../../shell/windowing/types";

const SESSION_KEY = "samaris-os/session";
const RESTORE_KEY = "samaris-os/restore-enabled";

class SessionPersistence {
  private restored = false;

  restore() {
    if (this.restored) return;
    this.restored = true;

    const raw = window.localStorage.getItem(SESSION_KEY);
    if (!raw) return;

    try {
      const snapshot = JSON.parse(raw) as ReturnType<typeof osStore.getState>;
      const restoreEnabled = window.localStorage.getItem(RESTORE_KEY) === "1";
      osStore.patch({
        windows: restoreEnabled ? this.normalizeWindows(snapshot.windows || []) : [],
        processes: restoreEnabled ? snapshot.processes || [] : [],
        runtimes: restoreEnabled ? snapshot.runtimes || [] : [],
        session: snapshot.session || {}
      });
    } catch {
      window.localStorage.removeItem(SESSION_KEY);
    }
  }

  private normalizeWindows(windows: AppWindow[]) {
    return windows.map((window) => {
      const previousBounds = window.previousBounds
        ? {
            x: clampWindowGeometry({
              left: window.previousBounds.x,
              top: window.previousBounds.y,
              width: window.previousBounds.w,
              height: window.previousBounds.h
            }).left,
            y: clampWindowGeometry({
              left: window.previousBounds.x,
              top: window.previousBounds.y,
              width: window.previousBounds.w,
              height: window.previousBounds.h
            }).top,
            w: clampWindowGeometry({
              left: window.previousBounds.x,
              top: window.previousBounds.y,
              width: window.previousBounds.w,
              height: window.previousBounds.h
            }).width,
            h: clampWindowGeometry({
              left: window.previousBounds.x,
              top: window.previousBounds.y,
              width: window.previousBounds.w,
              height: window.previousBounds.h
            }).height
          }
        : undefined;

      if (window.maximized) {
        const geometry = getMaximizedGeometry();
        return {
          ...window,
          x: geometry.left,
          y: geometry.top,
          w: geometry.width,
          h: geometry.height,
          previousBounds
        };
      }

      if (window.snapTarget === "left" || window.snapTarget === "right") {
        const geometry = getSnappedGeometry(window.snapTarget);
        return {
          ...window,
          x: geometry.left,
          y: geometry.top,
          w: geometry.width,
          h: geometry.height,
          previousBounds
        };
      }

      const geometry = clampWindowGeometry({
        left: window.x,
        top: window.y,
        width: window.w,
        height: window.h
      });

      return {
        ...window,
        x: geometry.left,
        y: geometry.top,
        w: geometry.width,
        h: geometry.height,
        previousBounds
      };
    });
  }

  save() {
    if (securityStore.getState().guestMode) {
      window.localStorage.removeItem(SESSION_KEY);
      return;
    }
    try {
      const snapshot = osStore.getState();
      const payload = {
        windows: Array.isArray(snapshot.windows) ? snapshot.windows.slice(0, 50) : [],
        processes: Array.isArray(snapshot.processes) ? snapshot.processes : [],
        runtimes: Array.isArray(snapshot.runtimes) ? snapshot.runtimes : [],
        session: snapshot.session && typeof snapshot.session === "object" ? { lastBootAt: "lastBootAt" in snapshot.session ? String(snapshot.session.lastBootAt) : null } : {}
      };
      const json = JSON.stringify(payload);
      if (json.length < 1024 * 1024) { // 1MB sanity limit
        window.localStorage.setItem(SESSION_KEY, json);
      }
    } catch {
      // If save fails (corrupted state, quota exceeded), clear and continue
      window.localStorage.removeItem(SESSION_KEY);
    }
    void sessionManager.saveState().catch(() => {});
  }

  restoreEnabled() {
    if (securityStore.getState().guestMode) return false;
    return window.localStorage.getItem(RESTORE_KEY) === "1";
  }

  setRestoreEnabled(enabled: boolean) {
    window.localStorage.setItem(RESTORE_KEY, enabled ? "1" : "0");
  }
}

export const sessionPersistence = new SessionPersistence();
