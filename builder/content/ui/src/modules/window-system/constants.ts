export const SAMARIS_TOPBAR = {
  top: 14,
  left: 14,
  right: 14,
  height: 44,
  radius: 999,
  zIndex: 100000
} as const;

export const SAMARIS_DOCK = {
  bottom: 18,
  minHeight: 68,
  paddingY: 9,
  paddingX: 12,
  gap: 10,
  radius: 26,
  zIndex: 100000,
  maxWidthOffset: 36
} as const;

export const SAMARIS_WINDOW_GEOMETRY = {
  defaultWidth: 720,
  defaultHeight: 460,
  minWidth: 380,
  minHeight: 260,
  mobileMinWidth: 310,
  topOffset: 70,
  bottomReserved: 164,
  sideMargin: 14,
  snapGap: 8,
  snapHalfWidthOffset: 22
} as const;

export const SAMARIS_RESIZE = {
  edgeInset: 18,
  edgeSize: 10,
  cornerSize: 22,
  gripSize: 22,
  gripOffset: 8,
  gripMarkSize: 12,
  minWidth: SAMARIS_WINDOW_GEOMETRY.minWidth,
  minHeight: SAMARIS_WINDOW_GEOMETRY.minHeight,
} as const;

export const SAMARIS_SNAP = {
  edgeThreshold: 30,
  topThreshold: 74
} as const;

export const SAMARIS_Z_INDEX = {
  baseWindow: 100,
  snapOverlay: 90000,
  dock: 100000,
  topbar: 100000,
  toast: 100001
} as const;
