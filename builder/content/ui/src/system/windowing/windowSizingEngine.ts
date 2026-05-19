type Viewport = { width: number; height: number };

type SizingBehavior = "centered" | "maximized" | "cascade";

export type WindowSizingConfig = {
  widthFraction: number;
  heightFraction: number;
  minWidth: number;
  minHeight: number;
  maxWidth?: number;
  maxHeight?: number;
  behavior: SizingBehavior;
};

const APP_SIZING: Record<string, WindowSizingConfig> = {
  peregrine:     { widthFraction: 0.80, heightFraction: 0.85, minWidth: 800, minHeight: 600, behavior: "maximized" },
  terminal:      { widthFraction: 0.55, heightFraction: 0.50, minWidth: 600, minHeight: 400, behavior: "centered" },
  finder:        { widthFraction: 0.50, heightFraction: 0.65, minWidth: 600, minHeight: 400, behavior: "centered" },
  "pdf-viewer":  { widthFraction: 0.60, heightFraction: 0.70, minWidth: 700, minHeight: 500, behavior: "maximized" },
  settings:      { widthFraction: 0.45, heightFraction: 0.50, minWidth: 500, minHeight: 400, behavior: "centered" },
  mail:          { widthFraction: 0.60, heightFraction: 0.70, minWidth: 700, minHeight: 500, behavior: "centered" },
  music:         { widthFraction: 0.45, heightFraction: 0.55, minWidth: 500, minHeight: 400, behavior: "centered" },
  notes:         { widthFraction: 0.50, heightFraction: 0.55, minWidth: 500, minHeight: 400, behavior: "centered" },
  "app-store":   { widthFraction: 0.55, heightFraction: 0.65, minWidth: 700, minHeight: 500, behavior: "centered" },
  photos:        { widthFraction: 0.80, heightFraction: 0.82, minWidth: 1040, minHeight: 680, behavior: "centered" },
  videos:        { widthFraction: 0.60, heightFraction: 0.70, minWidth: 700, minHeight: 500, behavior: "centered" },
  downloads:     { widthFraction: 0.50, heightFraction: 0.55, minWidth: 600, minHeight: 400, behavior: "centered" },
  wine:          { widthFraction: 0.50, heightFraction: 0.55, minWidth: 600, minHeight: 400, behavior: "centered" },
  orbit:         { widthFraction: 0.60, heightFraction: 0.70, minWidth: 700, minHeight: 500, behavior: "centered" },
  textedit:      { widthFraction: 0.50, heightFraction: 0.55, minWidth: 500, minHeight: 400, behavior: "centered" },
  network:       { widthFraction: 0.50, heightFraction: 0.55, minWidth: 600, minHeight: 400, behavior: "centered" },
  trash:         { widthFraction: 0.55, heightFraction: 0.62, minWidth: 800, minHeight: 520, behavior: "centered" },
};

const DEFAULT: WindowSizingConfig = { widthFraction: 0.50, heightFraction: 0.55, minWidth: 500, minHeight: 400, behavior: "centered" };
const topBarHeight = 70;
const bottomReserved = 164;
const sideMargin = 14;
const cascadeStep = 28;
const cascadeMax = 5;

function getViewportSize(): Viewport {
  if (typeof window === "undefined") return { width: 1440, height: 900 };
  return { width: window.innerWidth, height: window.innerHeight };
}

export function getSizingConfig(appId: string): WindowSizingConfig {
  return APP_SIZING[appId] || DEFAULT;
}

export function computeWindowGeometry(appId: string, cascadeCount = 0) {
  const vp = getViewportSize();
  const config = getSizingConfig(appId);

  const availW = vp.width - sideMargin * 2;
  const availH = vp.height - topBarHeight - bottomReserved;

  let w = Math.round(availW * config.widthFraction);
  let h = Math.round(availH * config.heightFraction);
  if (config.maxWidth) w = Math.min(w, config.maxWidth);
  if (config.maxHeight) h = Math.min(h, config.maxHeight);
  w = Math.max(w, config.minWidth);
  h = Math.max(h, config.minHeight);

  const cascadeX = (cascadeCount % cascadeMax) * cascadeStep;
  const cascadeY = (cascadeCount % cascadeMax) * cascadeStep;

  let left: number, top: number;
  if (config.behavior === "maximized") {
    left = sideMargin;
    top = topBarHeight;
    w = availW;
    h = availH;
  } else if (config.behavior === "cascade") {
    left = sideMargin + cascadeX;
    top = topBarHeight + cascadeY;
  } else {
    left = sideMargin + Math.round((availW - w) / 2) + cascadeX;
    top = topBarHeight + Math.round((availH - h) / 2) + cascadeY;
  }

  left = Math.max(sideMargin, Math.min(left, vp.width - w - sideMargin));
  top = Math.max(topBarHeight, Math.min(top, vp.height - h - bottomReserved));

  return { left, top, width: w, height: h };
}

export function getCascadeOffset(count: number) {
  const step = (count % cascadeMax) * cascadeStep;
  return { x: step, y: step };
}
