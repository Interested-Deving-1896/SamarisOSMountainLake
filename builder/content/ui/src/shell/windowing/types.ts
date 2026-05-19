export type WindowId = string;

export type AppId = string;

export type AppWindow = {
  id: WindowId;
  appId: AppId;
  title: string;
  subtitle?: string;
  accent?: number;
  x: number;
  y: number;
  w: number;
  h: number;
  z: number;
  focused?: boolean;
  processId?: number;
  minimized?: boolean;
  minimizing?: boolean;
  maximized?: boolean;
  snapTarget?: "left" | "right" | "top" | null;
  params?: Record<string, unknown>;
  opening?: boolean;
  closing?: boolean;
  previousBounds?: {
    x: number;
    y: number;
    w: number;
    h: number;
  };
  minimizeTarget?: {
    x: number;
    y: number;
  };
};
