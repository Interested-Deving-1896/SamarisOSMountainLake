import React, { useRef, useCallback, useState, useEffect } from "react";
import type { AppWindow } from "../../shell/windowing/types";
import { GenieRenderer } from "./GenieRenderer";
import { getDockTargetRect, getElementRect } from "./geometry";
import { createPhysics, type GeniePhysics } from "./GenieEngine";
import { computeShadow } from "./GenieShadow";
import { computePostFx } from "./GeniePostProcess";
import { getIntensityMultiplier, type GenieConfig } from "./GenieSettings";

type Snapshot = {
  bitmap: HTMLCanvasElement;
  sourceRect: { x: number; y: number; width: number; height: number };
  dockRect: { x: number; y: number; width: number; height: number };
};

export function useGenieManager(config: Partial<GenieConfig> = {}) {
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const rendererRef = useRef<GenieRenderer | null>(null);
  const frameRef = useRef<number | null>(null);
  const busyRef = useRef(false);
  const snapshotRef = useRef<Snapshot | null>(null);
  const [, forceRender] = useState(0);
  const lastPhysRef = useRef<GeniePhysics | null>(null);

  const cfg: GenieConfig = {
    style: config.style ?? "genie",
    intensity: config.intensity ?? "dramatic",
    soundEnabled: false,
    shadowEnabled: config.shadowEnabled ?? true,
    postFxEnabled: config.postFxEnabled ?? true,
  };
  const intensityMult = getIntensityMultiplier(cfg.intensity);

  const stopAnim = useCallback(() => {
    if (frameRef.current) { cancelAnimationFrame(frameRef.current); frameRef.current = null; }
  }, []);

  const runAnim = useCallback((snapshot: Snapshot, direction: 1 | -1, onDone: () => void) => {
    const canvas = canvasRef.current;
    if (!canvas) { onDone(); return; }

    let renderer = rendererRef.current;
    if (!renderer) {
      try {
        renderer = new GenieRenderer(canvas, snapshot.bitmap, { cols: 96, rows: 48 });
        renderer.resize(window.innerWidth, window.innerHeight);
        rendererRef.current = renderer;
      } catch {
        onDone();
        return;
      }
    }

    canvas.style.display = "block";
    canvas.style.filter = "drop-shadow(0 8px 18px rgba(0,0,0,0.45))";
    busyRef.current = true;
    forceRender((v) => v + 1);

    const start = performance.now();
    const src = snapshot.sourceRect;
    const dock = snapshot.dockRect;

    const tick = (now: number) => {
      const elapsed = now - start;
      const phys = createPhysics(elapsed, src.width, src.height, dock.width, dock.height, direction);

      let p: number;
      if (direction === 1) {
        p = phys.progress;
      } else {
        p = 1 - phys.progress;
      }
      p = Math.max(0, Math.min(1, p));

      const v = phys.velocity * intensityMult;

      if (cfg.postFxEnabled) {
        renderer!.render(p, src, dock, v, intensityMult);
      } else {
        renderer!.render(p, src, dock, 0, intensityMult);
      }

      // Update shadow on canvas CSS
      if (cfg.shadowEnabled) {
        const sh = computeShadow(src, p, direction);
        canvas!.style.filter = `drop-shadow(${sh.offsetX}px ${sh.offsetY + 6}px ${sh.blurRadius}px rgba(0,0,0,${sh.alpha})) blur(${v * 1.2}px)`;
      }

      lastPhysRef.current = phys;

      if (elapsed < phys.progress * 950 || phys.progress < 0.995) {
        frameRef.current = requestAnimationFrame(tick);
      } else {
        busyRef.current = false;
        canvas.style.display = "none";
        canvas.style.filter = "";
        renderer!.clear();
        forceRender((v) => v + 1);
        onDone();
      }
    };
    frameRef.current = requestAnimationFrame(tick);
  }, [cfg, intensityMult]);

  const minimize = useCallback(async (win: AppWindow) => {
    if (busyRef.current) return;

    const el = document.getElementById(win.id);
    const dockEl = document.getElementById(`dock-icon-${win.appId}`);

    if (!el || !dockEl) return;

    try {
      const bitmap = await import("html2canvas").then(m => m.default(el as HTMLElement, {
        backgroundColor: null,
        scale: Math.min(window.devicePixelRatio || 1, 2),
        useCORS: true,
        logging: false,
      }));

      const srcRect = getElementRect(el as HTMLElement);
      const dockRect = getDockTargetRect(getElementRect(dockEl));
      const snap: Snapshot = { bitmap, sourceRect: srcRect, dockRect };

      snapshotRef.current = { bitmap, sourceRect: srcRect, dockRect: { ...dockRect } };

      runAnim(snap, 1, () => {});
    } catch { /* fallback */ }
  }, [runAnim]);

  const restore = useCallback(async (win: AppWindow, appId: string) => {
    if (busyRef.current) return;

    const dockEl = document.getElementById(`dock-icon-${appId}`);
    if (!dockEl) return;

    const snap = snapshotRef.current;
    if (!snap) return;

    const dockRect = getDockTargetRect(getElementRect(dockEl));
    const srcRect = { x: win.x, y: win.y, width: win.w, height: win.h };

    runAnim({ bitmap: snap.bitmap, sourceRect: srcRect, dockRect }, -1, () => {});
  }, [runAnim]);

  const isAnimating = busyRef.current;

  useEffect(() => {
    const onResize = () => rendererRef.current?.resize(window.innerWidth, window.innerHeight);
    window.addEventListener("resize", onResize);
    return () => { window.removeEventListener("resize", onResize); stopAnim(); };
  }, [stopAnim]);

  return { canvasRef, minimize, restore, isAnimating };
}
