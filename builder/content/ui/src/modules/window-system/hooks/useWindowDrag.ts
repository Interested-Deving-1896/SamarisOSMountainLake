import React from "react";
import type { WindowGeometry } from "../types";

export function useWindowDrag(params: {
  id: string;
  geometry: WindowGeometry;
  disabled?: boolean;
  onDragStart?: (id: string, geometry: WindowGeometry) => void;
  onDragMove?: (id: string, geometry: WindowGeometry) => void;
  onDragEnd?: (id: string, geometry: WindowGeometry) => void;
}) {
  const [dragging, setDragging] = React.useState(false);
  const stateRef = React.useRef<{ startX: number; startY: number; origin: WindowGeometry } | null>(null);

  function handlePointerDown(event: React.PointerEvent<HTMLElement>) {
    if (params.disabled) return;
    stateRef.current = {
      startX: event.clientX,
      startY: event.clientY,
      origin: params.geometry
    };
    setDragging(true);
    params.onDragStart?.(params.id, params.geometry);
  }

  function handlePointerMove(event: React.PointerEvent<HTMLElement>) {
    if (!dragging || !stateRef.current) return;
    const dx = event.clientX - stateRef.current.startX;
    const dy = event.clientY - stateRef.current.startY;
    params.onDragMove?.(params.id, {
      ...stateRef.current.origin,
      left: stateRef.current.origin.left + dx,
      top: stateRef.current.origin.top + dy
    });
  }

  function handlePointerUp() {
    if (!dragging || !stateRef.current) return;
    const finalGeometry = stateRef.current.origin;
    stateRef.current = null;
    setDragging(false);
    params.onDragEnd?.(params.id, finalGeometry);
  }

  return {
    dragging,
    bindDragHandle: {
      onPointerDown: handlePointerDown,
      onPointerMove: handlePointerMove,
      onPointerUp: handlePointerUp
    },
    cancelDrag() {
      stateRef.current = null;
      setDragging(false);
    }
  };
}
