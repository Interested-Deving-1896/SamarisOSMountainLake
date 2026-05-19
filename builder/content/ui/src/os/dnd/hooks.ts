import React from "react";
import { cursorStore } from "../../system/cursor/cursorStore";
import { clearActiveDndSource, readDndFiles, writeDndPayload } from "./payload";
import { nativeDndBridge } from "./nativeDndBridge";
import type { DragFilePayload, FileDropOptions } from "./types";

export function useDndSource(params: {
  files: DragFilePayload[];
  getDragImage?: () => HTMLElement | null;
  nativeExport?: boolean;
}) {
  return React.useMemo(() => ({
    draggable: true,
    onDragStart(event: React.DragEvent) {
      const source = writeDndPayload(event.dataTransfer, params.files);
      event.dataTransfer.effectAllowed = "copyMove";
      const img = params.getDragImage?.();
      if (img) event.dataTransfer.setDragImage(img, 2, 2);
      if (params.nativeExport && event.altKey) {
        void nativeDndBridge.startNativeDrag(params.files);
      }
      const cleanup = () => clearActiveDndSource(source.id);
      document.addEventListener("dragend", cleanup, { once: true });
      window.setTimeout(cleanup, 5000);
    },
    onDragEnd() {
      clearActiveDndSource();
    }
  }), [params]);
}

export function useDropZone(_options: FileDropOptions) {
  const [isDragging, setIsDragging] = React.useState(false);
  return {
    isDragging,
    setIsDragging
  };
}

export function getFilesFromDataTransfer(dataTransfer: DataTransfer) {
  return readDndFiles(dataTransfer);
}

export function usePointerDrag(params: {
  kind: string;
  threshold?: number;
  disabled?: boolean;
  onStart?: (event: PointerEvent | React.PointerEvent) => void;
  onMove?: (delta: { dx: number; dy: number; x: number; y: number }, event: PointerEvent) => void;
  onCommit?: (delta: { dx: number; dy: number; x: number; y: number }, event: PointerEvent) => void;
  onCancel?: () => void;
}) {
  const stateRef = React.useRef<{ startX: number; startY: number; active: boolean } | null>(null);

  const bind = React.useMemo(() => ({
    onPointerDown(event: React.PointerEvent<HTMLElement>) {
      if (params.disabled || event.button !== 0) return;
      stateRef.current = { startX: event.clientX, startY: event.clientY, active: false };
      params.onStart?.(event);
      const threshold = params.threshold ?? 3;

      const handleMove = (moveEvent: PointerEvent) => {
        const state = stateRef.current;
        if (!state) return;
        const dx = moveEvent.clientX - state.startX;
        const dy = moveEvent.clientY - state.startY;
        if (!state.active && Math.hypot(dx, dy) < threshold) return;
        state.active = true;
        cursorStore.setType("move");
        params.onMove?.({ dx, dy, x: moveEvent.clientX, y: moveEvent.clientY }, moveEvent);
      };

      const finish = (upEvent: PointerEvent) => {
        const state = stateRef.current;
        if (state?.active) {
          params.onCommit?.({
            dx: upEvent.clientX - state.startX,
            dy: upEvent.clientY - state.startY,
            x: upEvent.clientX,
            y: upEvent.clientY
          }, upEvent);
        } else {
          params.onCancel?.();
        }
        cursorStore.setType("default");
        stateRef.current = null;
        window.removeEventListener("pointermove", handleMove);
        window.removeEventListener("pointerup", finish);
        window.removeEventListener("pointercancel", cancel);
      };

      const cancel = () => {
        stateRef.current = null;
        cursorStore.setType("default");
        params.onCancel?.();
        window.removeEventListener("pointermove", handleMove);
        window.removeEventListener("pointerup", finish);
        window.removeEventListener("pointercancel", cancel);
      };

      window.addEventListener("pointermove", handleMove);
      window.addEventListener("pointerup", finish, { once: true });
      window.addEventListener("pointercancel", cancel, { once: true });
    }
  }), [params]);

  return { bindPointerDrag: bind };
}

export function useWindowDragSnap<T>(value: T) {
  return value;
}
