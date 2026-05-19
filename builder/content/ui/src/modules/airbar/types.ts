import type React from "react";
export type AirBarTheme = "day" | "night" | "auto";

export type AirBarPanel =
  | "none"
  | "samaris"
  | "apps"
  | "search"
  | "sound"
  | "wifi"
  | "bluetooth"
  | "battery"
  | "sidebar";

export type AirBarProps = {
  theme?: AirBarTheme;
  adaptive?: boolean;
  showLabels?: boolean;
  showBatteryPercentage?: boolean;
  className?: string;

  onOpenSamarisMenu?: () => void;
  onOpenApps?: () => void;
  onOpenSearch?: () => void;
  onOpenSidebar?: () => void;
  onToggleTheme?: (theme: "day" | "night") => void;
};

export type AirBarContextValue = {
  theme: AirBarTheme;
  resolvedTheme: "day" | "night";
  activePanel: AirBarPanel;
  setActivePanel: (panel: AirBarPanel) => void;
  closePanels: () => void;
  registerAnchor: (panel: Exclude<AirBarPanel, "none">, element: HTMLElement | null) => void;
  getPanelStyle: (
    panel: Exclude<AirBarPanel, "none">,
    options?: {
      width?: number;
      maxWidth?: number;
      align?: "start" | "center" | "end";
      gap?: number;
      arrow?: boolean;
    }
  ) => React.CSSProperties;
};
