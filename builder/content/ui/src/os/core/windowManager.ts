import type { AppWindow } from "../../shell/windowing/types";
import { eventBus } from "../kernel/eventBus";
import { permissionManager } from "../kernel/permissionManager";
import { osStore } from "./osStore";
import { appRegistry } from "../apps/appRegistry";
import type { SnapTarget } from "../../modules/window-system";
import { clampWindowGeometry, getMaximizedGeometry, getSnappedGeometry, nextZIndex } from "../../modules/window-system";
import { loadWindowPreference, saveWindowPreference } from "../../system/windowing/windowPreferences";
import { computeWindowGeometry, getSizingConfig } from "../../system/windowing/windowSizingEngine";

function getWindowPreferenceKey(appId: string, params?: Record<string, unknown>) {
  const candidate = params?.windowPreferenceKey;
  return typeof candidate === "string" && candidate.trim() ? candidate : appId;
}

function getPreferredDimension(value: unknown, fallback: number, minimum: number) {
  if (typeof value !== "number" || !Number.isFinite(value)) return Math.max(minimum, fallback);
  return Math.max(minimum, Math.round(value));
}

class OSWindowManager {
  sync(windows: AppWindow[]) {
    osStore.setWindows(windows);
  }

  private getTopZ() {
    return nextZIndex(Math.max(...osStore.getState().windows.map((window) => window.z), 0));
  }

  openWindow(appId: string, processId?: number, params?: Record<string, unknown>) {
    const app = appRegistry[appId];
    if (!app) {
      throw new Error(`unknown_app:${appId}`);
    }

    if (!permissionManager.can(appId, "window.open")) {
      throw new Error("permission_denied");
    }

    const windowId = `${appId}-${Date.now()}`;
    const current = osStore.getState().windows;
    const preferred = loadWindowPreference(getWindowPreferenceKey(appId, params));
    const cascadeCount = current.filter((w) => w.appId === appId).length;
    const sizing = computeWindowGeometry(appId, cascadeCount);
    const geometry = clampWindowGeometry(
      preferred?.geometry ?? {
        left: sizing.left,
        top: sizing.top,
        width: sizing.width,
        height: sizing.height,
      }
    );
    const nextWindow: AppWindow = {
      id: windowId,
      appId,
      title: typeof params?.title === "string" ? params.title : app.title,
      subtitle: typeof params?.subtitle === "string" ? params.subtitle : app.subtitle,
      accent: typeof params?.accent === "number" ? params.accent : app.accent,
      x: geometry.left,
      y: geometry.top,
      w: geometry.width,
      h: geometry.height,
      z: nextZIndex(Math.max(...current.map((window) => window.z), 0)),
      focused: true,
      processId,
      minimized: false,
      minimizing: false,
      maximized: preferred?.maximized ?? false,
      snapTarget: preferred?.snapTarget ?? null,
      params: params || undefined
    };
    if (preferred?.previousBounds) {
      nextWindow.previousBounds = {
        x: preferred.previousBounds.left,
        y: preferred.previousBounds.top,
        w: preferred.previousBounds.width,
        h: preferred.previousBounds.height
      };
    }
    nextWindow.opening = true;

    osStore.update((state) => ({
      ...state,
      windows: [...state.windows.map((window) => ({ ...window, focused: false })), nextWindow]
    }));
    eventBus.emit("window:opened", nextWindow);
    window.setTimeout(() => {
      osStore.update((state) => ({
        ...state,
        windows: state.windows.map((window) => (window.id === windowId ? { ...window, opening: false } : window))
      }));
    }, 160);
    saveWindowPreference(nextWindow);
    return nextWindow.id;
  }

  focus(id: string) {
    const next = osStore.getState().windows.find((window) => window.id === id) || null;
    if (!next) return null;

    const topZ = this.getTopZ();
    osStore.update((state) => ({
      ...state,
      windows: state.windows.map((window) =>
        window.id === id
          ? { ...window, focused: true, z: topZ, minimized: false, minimizing: false }
          : { ...window, focused: false }
      )
    }));
    eventBus.emit("window:focused", { ...next, focused: true, z: topZ });
    return { ...next, focused: true, z: topZ };
  }

  close(id: string) {
    const exists = osStore.getState().windows.some((window) => window.id === id);
    if (!exists) return;
    osStore.update((state) => ({
      ...state,
      windows: state.windows.map((window) => (window.id === id ? { ...window, closing: true, focused: false } : window))
    }));
    window.setTimeout(() => {
      osStore.update((state) => ({
        ...state,
        windows: state.windows.filter((window) => window.id !== id)
      }));
      eventBus.emit("window:closed", { id });
    }, 170);
  }

  minimize(id: string) {
    const target = osStore.getState().windows.find((window) => window.id === id) || null;
    if (!target) return null;

    osStore.update((state) => ({
      ...state,
      windows: state.windows.map((window) =>
        window.id === id
          ? {
              ...window,
              minimizing: true,
              focused: false,
              minimizeTarget: {
                x: globalThis.window.innerWidth / 2,
                y: globalThis.window.innerHeight - 38
              }
            }
          : { ...window, focused: false }
      )
    }));

    window.setTimeout(() => {
      osStore.update((state) => ({
        ...state,
        windows: state.windows.map((window) =>
          window.id === id
            ? { ...window, minimizing: false, minimized: true, focused: false, minimizeTarget: undefined }
            : window
        )
      }));
    }, 220);

    eventBus.emit("window:minimized", { id });
    return id;
  }

  restore(id: string) {
    const target = osStore.getState().windows.find((window) => window.id === id) || null;
    if (!target) return null;
    const topZ = this.getTopZ();
    osStore.update((state) => ({
      ...state,
      windows: state.windows.map((window) =>
        window.id === id
          ? { ...window, minimized: false, minimizing: false, opening: true, focused: true, z: topZ }
          : { ...window, focused: false }
      )
    }));
    window.setTimeout(() => {
      osStore.update((state) => ({
        ...state,
        windows: state.windows.map((window) => (window.id === id ? { ...window, opening: false } : window))
      }));
    }, 180);
    eventBus.emit("window:restored", { id });
    return id;
  }

  restoreWindowState(id: string) {
    const target = osStore.getState().windows.find((window) => window.id === id) || null;
    if (!target) return null;
    if (!target.previousBounds) {
      return this.focus(id);
    }

    const topZ = this.getTopZ();
    osStore.update((state) => ({
      ...state,
      windows: state.windows.map((window) =>
        window.id === id
          ? {
              ...window,
              ...window.previousBounds,
              previousBounds: undefined,
              maximized: false,
              snapTarget: null,
              focused: true,
              minimized: false,
              minimizing: false,
              z: topZ
            }
          : { ...window, focused: false }
      )
    }));
    const nextWindow = osStore.getState().windows.find((window) => window.id === id) || null;
    if (nextWindow) saveWindowPreference(nextWindow);
    return id;
  }

  toggleMaximize(id: string) {
    const target = osStore.getState().windows.find((window) => window.id === id) || null;
    if (!target) return null;

    const topZ = this.getTopZ();

    osStore.update((state) => ({
      ...state,
      windows: state.windows.map((appWindow) => {
        if (appWindow.id !== id) {
          return { ...appWindow, focused: false };
        }

        if (appWindow.maximized && appWindow.previousBounds) {
          return {
            ...appWindow,
            ...appWindow.previousBounds,
            maximized: false,
            previousBounds: undefined,
            snapTarget: null,
            focused: true,
            minimized: false,
            minimizing: false,
            z: topZ
          };
        }

        const maximizedGeometry = getMaximizedGeometry();

        return {
          ...appWindow,
          x: maximizedGeometry.left,
          y: maximizedGeometry.top,
          w: maximizedGeometry.width,
          h: maximizedGeometry.height,
          maximized: true,
          snapTarget: null,
          previousBounds:
            appWindow.previousBounds ??
            (appWindow.snapTarget
              ? appWindow.previousBounds
              : {
                  x: appWindow.x,
                  y: appWindow.y,
                  w: appWindow.w,
                  h: appWindow.h
                }),
          focused: true,
          minimized: false,
          minimizing: false,
          z: topZ
        };
      })
    }));
    const nextWindow = osStore.getState().windows.find((window) => window.id === id) || null;
    if (nextWindow) saveWindowPreference(nextWindow);

    eventBus.emit("window:maximized", { id });
    return id;
  }

  snap(id: string, target: Exclude<SnapTarget, null>) {
    const currentWindow = osStore.getState().windows.find((window) => window.id === id) || null;
    if (!currentWindow) return null;

    const topZ = this.getTopZ();
    const snappedGeometry = getSnappedGeometry(target);

    osStore.update((state) => ({
      ...state,
      windows: state.windows.map((window) => {
        if (window.id !== id) {
          return { ...window, focused: false };
        }

        const previousBounds =
          window.previousBounds ??
          (window.maximized || window.snapTarget
            ? window.previousBounds
            : {
                x: window.x,
                y: window.y,
                w: window.w,
                h: window.h
              });

        return {
          ...window,
          x: snappedGeometry.left,
          y: snappedGeometry.top,
          w: snappedGeometry.width,
          h: snappedGeometry.height,
          maximized: false,
          snapTarget: target,
          previousBounds,
          focused: true,
          minimized: false,
          minimizing: false,
          z: topZ
        };
      })
    }));
    const nextWindow = osStore.getState().windows.find((window) => window.id === id) || null;
    if (nextWindow) saveWindowPreference(nextWindow);

    eventBus.emit("window:snapped", { id, target });
    return id;
  }

  updateLocal(id: string, partial: Partial<AppWindow>) {
    osStore.update((state) => ({
      ...state,
      windows: state.windows.map((window) => (window.id === id ? { ...window, ...partial } : window))
    }));
    const nextWindow = osStore.getState().windows.find((window) => window.id === id) || null;
    if (nextWindow && ("x" in partial || "y" in partial || "w" in partial || "h" in partial || "maximized" in partial || "snapTarget" in partial || "previousBounds" in partial)) {
      saveWindowPreference(nextWindow);
    }
  }
}

export const windowManager = new OSWindowManager();
