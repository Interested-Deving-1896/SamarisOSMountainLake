export type Rect = {
  x: number;
  y: number;
  width: number;
  height: number;
};

export type GeniePhase =
  | "open"
  | "capturing"
  | "minimizing"
  | "minimized"
  | "restoring";

export type GenieDirection = 1 | -1;

export type GenieRendererOptions = {
  cols?: number;
  rows?: number;
};

export type GenieEffectOptions = {
  dockSelector: string;
  durationMinimize?: number;
  durationRestore?: number;
  maxCaptureScale?: number;
  onMinimized?: () => void;
  onRestored?: () => void;
};
