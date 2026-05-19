import type { AppWindow } from "../../shell/windowing/types";
import { clampWindowGeometry, getMaximizedGeometry, getSnappedGeometry, type SnapTarget } from "../../modules/window-system";

const KEY = "samaris-os/window-preferences";

type StoredBounds = {
  x: number;
  y: number;
  w: number;
  h: number;
};

type StoredWindowPreference = StoredBounds & {
  maximized?: boolean;
  snapTarget?: SnapTarget;
  previousBounds?: StoredBounds;
};

type PreferenceMap = Record<string, StoredWindowPreference>;

function readAll(): PreferenceMap {
  try {
    const raw = window.localStorage.getItem(KEY);
    if (!raw) return {};
    return JSON.parse(raw) as PreferenceMap;
  } catch {
    return {};
  }
}

function writeAll(next: PreferenceMap) {
  window.localStorage.setItem(KEY, JSON.stringify(next));
}

function getPreferenceKey(windowState: Pick<AppWindow, "appId" | "params">) {
  const candidate = windowState.params?.windowPreferenceKey;
  return typeof candidate === "string" && candidate.trim() ? candidate : windowState.appId;
}

function toBounds(input?: StoredBounds | null) {
  if (!input) return null;
  return clampWindowGeometry({
    left: input.x,
    top: input.y,
    width: input.w,
    height: input.h
  });
}

export function loadWindowPreference(appId: string) {
  const stored = readAll()[appId];
  if (!stored) return null;

  const previousBounds = toBounds(stored.previousBounds);
  const baseBounds = toBounds(stored) ?? previousBounds;
  if (!baseBounds) return null;

  if (stored.maximized) {
    const geometry = getMaximizedGeometry();
    return {
      geometry,
      maximized: true,
      snapTarget: null as SnapTarget,
      previousBounds
    };
  }

  if (stored.snapTarget === "left" || stored.snapTarget === "right") {
    return {
      geometry: getSnappedGeometry(stored.snapTarget),
      maximized: false,
      snapTarget: stored.snapTarget,
      previousBounds: previousBounds ?? baseBounds
    };
  }

  return {
    geometry: baseBounds,
    maximized: false,
    snapTarget: null as SnapTarget,
    previousBounds: undefined
  };
}

export function saveWindowPreference(windowState: AppWindow) {
  if (windowState.minimized || windowState.minimizing) return;

  const next = readAll();
  next[getPreferenceKey(windowState)] = {
    x: windowState.x,
    y: windowState.y,
    w: windowState.w,
    h: windowState.h,
    maximized: Boolean(windowState.maximized),
    snapTarget: windowState.snapTarget ?? null,
    previousBounds: windowState.previousBounds
      ? {
          x: windowState.previousBounds.x,
          y: windowState.previousBounds.y,
          w: windowState.previousBounds.w,
          h: windowState.previousBounds.h
        }
      : undefined
  };
  writeAll(next);
}
