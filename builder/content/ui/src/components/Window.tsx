import React from "react";
import type { AppWindow } from "../shell/windowing/types";
import { appRegistry } from "../os/apps/appRegistry";
import { cursorStore, type CursorType } from "../system/cursor/cursorStore";
import {
  SAMARIS_WINDOW_GEOMETRY,
  SnapOverlay,
  WindowFrame,
  clampWindowGeometry,
  getSnapTarget,
  type ResizeDirection,
  type SnapTarget,
  type WindowGeometry
} from "../modules/window-system";

function renderApp(window: AppWindow) {
  const app = appRegistry[window.appId];

  if (!app) {
    return <div className="win__fallback">App not found</div>;
  }

  const Component = app.component;
  return (
    <React.Suspense fallback={<div className="win__fallback">Loading app…</div>}>
      <Component windowId={window.id} />
    </React.Suspense>
  );
}

function toGeometry(window: AppWindow): WindowGeometry {
  return {
    left: window.x,
    top: window.y,
    width: window.w,
    height: window.h
  };
}

function clampHeaderTop(top: number) {
  return Math.max(64, top);
}

function getVisualState(window: AppWindow) {
  if (window.minimized && !window.minimizing) return "hidden" as const;
  if (window.minimizing) return "minimized" as const;
  if (window.maximized) return "maximized" as const;
  if (window.snapTarget === "left") return "snapped-left" as const;
  if (window.snapTarget === "right") return "snapped-right" as const;
  return window.focused ? ("focused" as const) : ("inactive" as const);
}

type DragState = {
  pointerId: number;
  offsetX: number;
  offsetY: number;
};

type ResizeState = {
  pointerId: number;
  direction: ResizeDirection;
  startX: number;
  startY: number;
  origin: WindowGeometry;
};

function getResizeCursor(dir: ResizeDirection): CursorType {
  const map: Record<ResizeDirection, CursorType> = {
    n: "ns-resize", s: "ns-resize",
    e: "ew-resize", w: "ew-resize",
    ne: "nesw-resize", sw: "nesw-resize",
    nw: "nwse-resize", se: "nwse-resize",
  };
  return map[dir] || "move";
}

export function Window(props: {
  window: AppWindow;
  onFocus: (id: string) => void;
  onMinimize: (id: string) => void;
  onMaximize: (id: string) => void;
  onClose: (id: string) => void;
  onUpdate: (id: string, partial: Partial<AppWindow>) => void;
  onSnap: (id: string, target: Exclude<SnapTarget, null>) => void;
  onRestoreWindow: (id: string) => void;
  onDuplicate?: (appId: string, params?: Record<string, unknown>) => void;
}) {
  const app = appRegistry[props.window.appId];
  const frameRef = React.useRef<HTMLElement | null>(null);
  const dragState = React.useRef<DragState | null>(null);
  const resizeState = React.useRef<ResizeState | null>(null);
  const interactingRef = React.useRef<false | "dragging" | "resizing">(false);
  const [interacting, setInteracting] = React.useState<false | "dragging" | "resizing">(false);
  const [activeSnapTarget, setActiveSnapTarget] = React.useState<SnapTarget>(null);
  const [geometry, setGeometry] = React.useState<WindowGeometry>(() => toGeometry(props.window));
  const geometryRef = React.useRef<WindowGeometry>(geometry);
  const appNode = React.useMemo(() => renderApp(props.window), [props.window.appId, props.window.id, props.window.params]);
  const minWidth = Math.max(app?.minWindow?.w ?? 0, SAMARIS_WINDOW_GEOMETRY.minWidth);
  const minHeight = Math.max(app?.minWindow?.h ?? 0, SAMARIS_WINDOW_GEOMETRY.minHeight);

  const snapTargetFromPoint = React.useCallback((clientX: number, clientY: number) => getSnapTarget(clientX, clientY, {
    width: window.innerWidth,
    height: window.innerHeight
  }), []);

  React.useEffect(() => {
    geometryRef.current = geometry;
  }, [geometry]);

  React.useEffect(() => {
    if (interactingRef.current) return;
    setGeometry(toGeometry(props.window));
  }, [props.window.x, props.window.y, props.window.w, props.window.h]);

  const commitGeometry = React.useCallback(
    (nextGeometry: WindowGeometry, partial: Partial<AppWindow> = {}) => {
      props.onUpdate(props.window.id, {
        x: nextGeometry.left,
        y: nextGeometry.top,
        w: nextGeometry.width,
        h: nextGeometry.height,
        ...partial
      });
    },
    [props]
  );

  const finishInteraction = React.useCallback(() => {
    dragState.current = null;
    resizeState.current = null;
    interactingRef.current = false;
    setInteracting(false);
    setActiveSnapTarget(null);
    cursorStore.setType("default");
  }, []);

  const handleHeaderPointerDown = React.useCallback(
    (event: React.PointerEvent<HTMLDivElement>) => {
      const target = event.target as HTMLElement;
      if (target.closest("button, input, textarea, select, a, [role='button']")) return;

      props.onFocus(props.window.id);

      let nextGeometry = geometryRef.current;
      let offsetX = event.clientX - geometryRef.current.left;
      let offsetY = event.clientY - geometryRef.current.top;

      if ((props.window.maximized || props.window.snapTarget) && props.window.previousBounds) {
        const ratioX = props.window.w > 0 ? (event.clientX - props.window.x) / props.window.w : 0.5;
        nextGeometry = clampWindowGeometry({
          left: event.clientX - props.window.previousBounds.w * Math.min(Math.max(ratioX, 0.16), 0.84),
          top: clampHeaderTop(event.clientY - 20),
          width: props.window.previousBounds.w,
          height: props.window.previousBounds.h
        });
        props.onRestoreWindow(props.window.id);
        setGeometry(nextGeometry);
        offsetX = event.clientX - nextGeometry.left;
        offsetY = event.clientY - nextGeometry.top;
      }

      dragState.current = {
        pointerId: event.pointerId,
        offsetX,
        offsetY
      };
      interactingRef.current = "dragging";
      setInteracting("dragging");
      cursorStore.setType("pointer");
      event.preventDefault();
      event.currentTarget.setPointerCapture(event.pointerId);
    },
    [props]
  );

  const handleHeaderPointerMove = React.useCallback((event: React.PointerEvent<HTMLDivElement>) => {
    const state = dragState.current;
    if (!state) return;

    const nextTarget = snapTargetFromPoint(event.clientX, event.clientY);
    setActiveSnapTarget(nextTarget);

    setGeometry((current) =>
      clampWindowGeometry({
        left: event.clientX - state.offsetX,
        top: clampHeaderTop(event.clientY - state.offsetY),
        width: current.width,
        height: current.height
      })
    );
  }, [snapTargetFromPoint]);

  const handleHeaderPointerUp = React.useCallback(
    (event: React.PointerEvent<HTMLDivElement>) => {
      if (!dragState.current) return;

      const snapTarget = snapTargetFromPoint(event.clientX, event.clientY);
      if (snapTarget === "top") {
        props.onMaximize(props.window.id);
      } else if (snapTarget) {
        props.onSnap(props.window.id, snapTarget);
      } else {
        commitGeometry(geometryRef.current, {
          maximized: false,
          snapTarget: null
        });
      }

      try {
        event.currentTarget.releasePointerCapture(dragState.current.pointerId);
      } catch {}
      finishInteraction();
    },
    [commitGeometry, finishInteraction, props, snapTargetFromPoint]
  );

  const handleHeaderPointerCancel = React.useCallback(
    (event: React.PointerEvent<HTMLDivElement>) => {
      if (!dragState.current) return;
      try {
        event.currentTarget.releasePointerCapture(dragState.current.pointerId);
      } catch {}
      setGeometry(toGeometry(props.window));
      finishInteraction();
    },
    [finishInteraction, props.window]
  );

  const handleResizePointerDown = React.useCallback(
    (direction: ResizeDirection, event: React.PointerEvent<HTMLDivElement>) => {
      if (props.window.maximized || props.window.snapTarget) return;

      event.stopPropagation();
      props.onFocus(props.window.id);
      resizeState.current = {
        pointerId: event.pointerId,
        direction,
        startX: event.clientX,
        startY: event.clientY,
        origin: geometryRef.current
      };
      interactingRef.current = "resizing";
      setInteracting("resizing");
      cursorStore.setType(getResizeCursor(direction));
      event.currentTarget.setPointerCapture(event.pointerId);

      const handlePointerMove = (moveEvent: PointerEvent) => {
        const state = resizeState.current;
        if (!state) return;

        const dx = moveEvent.clientX - state.startX;
        const dy = moveEvent.clientY - state.startY;
        const right = state.origin.left + state.origin.width;
        const bottom = state.origin.top + state.origin.height;
        const viewportWidth = window.innerWidth;
        const viewportHeight = window.innerHeight;
        let nextLeft = state.origin.left;
        let nextTop = state.origin.top;
        let nextWidth = state.origin.width;
        let nextHeight = state.origin.height;

        if (state.direction.includes("e")) {
          nextWidth = Math.min(
            Math.max(minWidth, state.origin.width + dx),
            viewportWidth - state.origin.left - SAMARIS_WINDOW_GEOMETRY.sideMargin
          );
        }

        if (state.direction.includes("s")) {
          nextHeight = Math.min(
            Math.max(minHeight, state.origin.height + dy),
            viewportHeight - state.origin.top - SAMARIS_WINDOW_GEOMETRY.bottomReserved
          );
        }

        if (state.direction.includes("w")) {
          nextLeft = Math.min(
            Math.max(SAMARIS_WINDOW_GEOMETRY.sideMargin, state.origin.left + dx),
            right - minWidth
          );
          nextWidth = right - nextLeft;
        }

        if (state.direction.includes("n")) {
          nextTop = Math.min(
            Math.max(64, state.origin.top + dy),
            bottom - minHeight
          );
          nextHeight = bottom - nextTop;
        }

        setGeometry({
          left: nextLeft,
          top: nextTop,
          width: nextWidth,
          height: nextHeight
        });
      };

      const cleanupResize = (commit: boolean) => {
        const state = resizeState.current;
        if (state) {
          if (commit) {
            commitGeometry(geometryRef.current);
          } else {
            setGeometry(toGeometry(props.window));
          }
          try {
            (event.currentTarget as HTMLElement).releasePointerCapture(state.pointerId);
          } catch {}
        }
        window.removeEventListener("pointermove", handlePointerMove);
        window.removeEventListener("pointerup", handlePointerUp);
        window.removeEventListener("pointercancel", handlePointerCancel);
        finishInteraction();
      };

      const handlePointerUp = () => cleanupResize(true);
      const handlePointerCancel = () => cleanupResize(false);

      window.addEventListener("pointermove", handlePointerMove);
      window.addEventListener("pointerup", handlePointerUp, { once: true });
      window.addEventListener("pointercancel", handlePointerCancel, { once: true });
    },
    [commitGeometry, finishInteraction, minHeight, minWidth, props]
  );

  const visualState = getVisualState(props.window);
  const className = [
    "app-window",
    `app-window--${props.window.appId}`,
    props.window.opening ? "opening" : "",
    props.window.closing ? "closing" : "",
    interacting === "dragging" ? "dragging" : "",
    interacting === "resizing" ? "resizing" : ""
  ]
    .filter(Boolean)
    .join(" ");

  return (
    <>
      <WindowFrame
        id={props.window.id}
        frameRef={frameRef}
        title={props.window.title}
        subtitle={props.window.subtitle}
        accent={props.window.accent}
        focused={props.window.focused}
        state={visualState}
        className={className}
        contentClassName="samaris-window-content--flush"
        style={{
          left: geometry.left,
          top: geometry.top,
          width: geometry.width,
          height: geometry.height,
          zIndex: props.window.z,
          ["--win-minimize-x" as string]: `${props.window.minimizeTarget?.x ?? window.innerWidth / 2}px`,
          ["--win-minimize-y" as string]: `${props.window.minimizeTarget?.y ?? window.innerHeight - 40}px`
        }}
        onFocus={() => props.onFocus(props.window.id)}
        onMinimize={() => props.onMinimize(props.window.id)}
        onMaximize={() => props.onMaximize(props.window.id)}
        onClose={() => props.onClose(props.window.id)}
        onSnapLeft={() => props.onSnap(props.window.id, "left")}
        onSnapRight={() => props.onSnap(props.window.id, "right")}
        onDuplicate={
          app?.supportsDuplicate && props.onDuplicate
            ? () => props.onDuplicate?.(props.window.appId, props.window.params)
            : undefined
        }
        onHeaderPointerDown={handleHeaderPointerDown}
        onHeaderPointerMove={handleHeaderPointerMove}
        onHeaderPointerUp={handleHeaderPointerUp}
        onHeaderPointerCancel={handleHeaderPointerCancel}
        onHeaderDoubleClick={(event) => {
          const target = event.target as HTMLElement;
          if (target.closest("button")) return;
          props.onMaximize(props.window.id);
        }}
        onResizePointerDown={handleResizePointerDown}
      >
        {appNode}
      </WindowFrame>
      {interacting === "dragging" ? <SnapOverlay activeTarget={activeSnapTarget} /> : null}
    </>
  );
}
