import { SAMARIS_DOCK, SAMARIS_TOPBAR, SAMARIS_WINDOW_GEOMETRY } from "../constants";
import type { SnapTarget, WindowGeometry } from "../types";

type Viewport = {
  width: number;
  height: number;
};

function getViewport(): Viewport {
  if (typeof window === "undefined") {
    return { width: 1440, height: 900 };
  }

  return {
    width: window.innerWidth,
    height: window.innerHeight
  };
}

export function createDefaultGeometry(index = 0): WindowGeometry {
  const viewport = getViewport();
  const offset = index * 24;
  const maxWidth = viewport.width - SAMARIS_WINDOW_GEOMETRY.sideMargin * 2;
  const maxHeight = viewport.height - SAMARIS_WINDOW_GEOMETRY.bottomReserved;

  return clampWindowGeometry({
    left: SAMARIS_WINDOW_GEOMETRY.sideMargin + 56 + offset,
    top: SAMARIS_WINDOW_GEOMETRY.topOffset + offset,
    width: Math.min(SAMARIS_WINDOW_GEOMETRY.defaultWidth, maxWidth),
    height: Math.min(SAMARIS_WINDOW_GEOMETRY.defaultHeight, maxHeight)
  }, viewport);
}

export function clampWindowGeometry(geometry: WindowGeometry, viewport = getViewport()): WindowGeometry {
  const width = Math.max(
    SAMARIS_WINDOW_GEOMETRY.mobileMinWidth,
    Math.min(geometry.width, viewport.width - SAMARIS_WINDOW_GEOMETRY.sideMargin * 2)
  );
  const height = Math.max(
    SAMARIS_WINDOW_GEOMETRY.minHeight,
    Math.min(geometry.height, viewport.height - SAMARIS_WINDOW_GEOMETRY.bottomReserved + 20)
  );
  const maxLeft = Math.max(SAMARIS_WINDOW_GEOMETRY.sideMargin, viewport.width - width - SAMARIS_WINDOW_GEOMETRY.sideMargin);
  const maxTop = Math.max(SAMARIS_WINDOW_GEOMETRY.topOffset, viewport.height - height - (SAMARIS_DOCK.minHeight + 64));

  return {
    left: Math.min(Math.max(SAMARIS_WINDOW_GEOMETRY.sideMargin, geometry.left), maxLeft),
    top: Math.min(Math.max(SAMARIS_WINDOW_GEOMETRY.topOffset, geometry.top), maxTop),
    width,
    height
  };
}

export function getMaximizedGeometry(viewport = getViewport()): WindowGeometry {
  return {
    left: SAMARIS_TOPBAR.left,
    top: SAMARIS_WINDOW_GEOMETRY.topOffset,
    width: viewport.width - SAMARIS_TOPBAR.left - SAMARIS_TOPBAR.right,
    height: viewport.height - SAMARIS_WINDOW_GEOMETRY.bottomReserved
  };
}

export function getSnappedGeometry(target: SnapTarget, viewport = getViewport()): WindowGeometry {
  if (target === "top") {
    return getMaximizedGeometry(viewport);
  }

  if (target === "left") {
    return {
      left: SAMARIS_WINDOW_GEOMETRY.sideMargin,
      top: SAMARIS_WINDOW_GEOMETRY.topOffset,
      width: viewport.width / 2 - SAMARIS_WINDOW_GEOMETRY.snapHalfWidthOffset,
      height: viewport.height - SAMARIS_WINDOW_GEOMETRY.bottomReserved
    };
  }

  if (target === "right") {
    return {
      left: viewport.width / 2 + SAMARIS_WINDOW_GEOMETRY.snapGap,
      top: SAMARIS_WINDOW_GEOMETRY.topOffset,
      width: viewport.width / 2 - SAMARIS_WINDOW_GEOMETRY.snapHalfWidthOffset,
      height: viewport.height - SAMARIS_WINDOW_GEOMETRY.bottomReserved
    };
  }

  return createDefaultGeometry();
}

export function serializeWindowGeometry(geometry: WindowGeometry) {
  return JSON.stringify(geometry);
}

export function restoreWindowGeometry(input?: string | null): WindowGeometry | null {
  if (!input) return null;

  try {
    const parsed = JSON.parse(input) as Partial<WindowGeometry>;
    if (
      typeof parsed.left !== "number" ||
      typeof parsed.top !== "number" ||
      typeof parsed.width !== "number" ||
      typeof parsed.height !== "number"
    ) {
      return null;
    }

    return parsed as WindowGeometry;
  } catch {
    return null;
  }
}
