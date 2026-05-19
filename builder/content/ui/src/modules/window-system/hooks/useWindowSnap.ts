import React from "react";
import { getSnapTarget, getSnapViewport, getSnapZoneClass } from "../utils/snap";
import { getSnappedGeometry } from "../utils/geometry";
import type { SnapTarget } from "../types";

export function useWindowSnap() {
  const [activeTarget, setActiveTarget] = React.useState<SnapTarget>(null);

  function getSnapTargetAt(x: number, y: number) {
    return getSnapTarget(x, y, getSnapViewport());
  }

  function getPreviewGeometry(target: SnapTarget) {
    return getSnappedGeometry(target);
  }

  return {
    activeTarget,
    setActiveTarget,
    getSnapTargetAt,
    getPreviewGeometry,
    getZoneClass: getSnapZoneClass
  };
}
