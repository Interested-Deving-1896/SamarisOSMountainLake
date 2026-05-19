import React from "react";
import { SAMARIS_RESIZE } from "../constants";
import type { ResizeDirection, WindowGeometry } from "../types";

export function useWindowResize(params: {
  id: string;
  geometry: WindowGeometry;
  disabled?: boolean;
  minWidth?: number;
  minHeight?: number;
  onResizeStart?: (id: string, direction: ResizeDirection, geometry: WindowGeometry) => void;
  onResizeMove?: (id: string, direction: ResizeDirection, geometry: WindowGeometry) => void;
  onResizeEnd?: (id: string, direction: ResizeDirection, geometry: WindowGeometry) => void;
}) {
  const [direction, setDirection] = React.useState<ResizeDirection | null>(null);

  function bindResizeHandle(nextDirection: ResizeDirection) {
    return {
      onPointerDown: () => {
        if (params.disabled) return;
        setDirection(nextDirection);
        params.onResizeStart?.(params.id, nextDirection, params.geometry);
      }
    };
  }

  function stopResize() {
    if (!direction) return;

    params.onResizeEnd?.(params.id, direction, params.geometry);
    setDirection(null);
  }

  return {
    direction,
    minWidth: params.minWidth ?? SAMARIS_RESIZE.minWidth,
    minHeight: params.minHeight ?? SAMARIS_RESIZE.minHeight,
    bindResizeHandle,
    stopResize
  };
}
