export type SamarisTheme = "light" | "dark";

export type SnapTarget = "left" | "right" | "top" | null;

export type ResizeDirection =
  | "n"
  | "s"
  | "e"
  | "w"
  | "ne"
  | "nw"
  | "se"
  | "sw";

export type WindowGeometry = {
  left: number;
  top: number;
  width: number;
  height: number;
};

export type SamarisWindowVisualState =
  | "focused"
  | "inactive"
  | "minimized"
  | "hidden"
  | "maximized"
  | "snapped-left"
  | "snapped-right";

export type WindowAction =
  | "close"
  | "minimize"
  | "maximize"
  | "restore"
  | "snap-left"
  | "snap-right"
  | "duplicate";

export type WindowSidebarItem = {
  id: string;
  label: string;
  active?: boolean;
  accent?: number;
};
