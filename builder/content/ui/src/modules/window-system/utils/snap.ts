import { SAMARIS_SNAP, SAMARIS_WINDOW_GEOMETRY } from "../constants";
import type { SnapTarget } from "../types";

type Viewport = {
  width: number;
  height: number;
};

export function getSnapTarget(x: number, y: number, viewport: Viewport): SnapTarget {
  if (y <= SAMARIS_SNAP.topThreshold) return "top";
  if (x <= SAMARIS_SNAP.edgeThreshold) return "left";
  if (x >= viewport.width - SAMARIS_SNAP.edgeThreshold) return "right";
  return null;
}

export function getSnapZoneClass(target: SnapTarget) {
  if (target === "left") return "samaris-snap-left";
  if (target === "right") return "samaris-snap-right";
  if (target === "top") return "samaris-snap-top";
  return "";
}

export function getSnapViewport() {
  if (typeof window === "undefined") {
    return { width: 1440, height: 900 };
  }

  return { width: window.innerWidth, height: window.innerHeight };
}

export function getSnapZoneGeometry(target: SnapTarget, viewport = getSnapViewport()) {
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
      right: SAMARIS_WINDOW_GEOMETRY.sideMargin,
      top: SAMARIS_WINDOW_GEOMETRY.topOffset,
      width: viewport.width / 2 - SAMARIS_WINDOW_GEOMETRY.snapHalfWidthOffset,
      height: viewport.height - SAMARIS_WINDOW_GEOMETRY.bottomReserved
    };
  }

  if (target === "top") {
    return {
      left: SAMARIS_WINDOW_GEOMETRY.sideMargin,
      top: SAMARIS_WINDOW_GEOMETRY.topOffset,
      width: viewport.width - SAMARIS_WINDOW_GEOMETRY.sideMargin * 2,
      height: viewport.height - SAMARIS_WINDOW_GEOMETRY.bottomReserved
    };
  }

  return null;
}
