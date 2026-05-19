import React from "react";
import { AirBarProvider, useAirBarContext } from "./AirBarProvider";
import type { AirBarPanel, AirBarProps, AirBarTheme } from "./types";
import { useAdaptiveGlass } from "./useAdaptiveGlass";
import { createAirBarVars } from "./utils/createAirBarVars";
import { AirBarButton } from "./components/AirBarButton";
import { AirBarAppsButton } from "./components/AirBarAppsButton";
import { AirBarSearchButton } from "./components/AirBarSearchButton";
import { AirBarThemeToggle } from "./components/AirBarThemeToggle";
import { AirBarStatusCluster } from "./components/AirBarStatusCluster";
import { AirBarDateTime } from "./components/AirBarDateTime";
import { AppsPanel } from "./components/AppsPanel";
import { UserMenu } from "../user-menu";
import { SearchPanel } from "./components/SearchPanel";
import { SidebarPanel } from "./components/SidebarPanel";
import { BatteryPanel, BluetoothPanel, SoundPanel, WifiPanel } from "../system-panels";
import "./styles/airbar.css";

function AirBarInner(
  props: Required<Pick<AirBarProps, "adaptive" | "showLabels" | "showBatteryPercentage">> &
    Pick<AirBarProps, "className" | "onOpenApps" | "onOpenSearch" | "onOpenSidebar" | "onOpenSamarisMenu" | "onToggleTheme">
) {
  const air = useAirBarContext();
  const adaptive = useAdaptiveGlass(props.adaptive);
  const rootRef = React.useRef<HTMLDivElement | null>(null);
  const samarisRef = React.useRef<HTMLButtonElement | null>(null);

  const vars = React.useMemo(() => {
    const sampled = adaptive || { r: 108, g: 171, b: 232, x: "72%", y: "12%" };
    return createAirBarVars({
      accentH: 210,
      accent2H: 265,
      accentRgb: { r: sampled.r, g: sampled.g, b: sampled.b },
      glassRgb: { r: 255, g: 255, b: 255 },
      x: sampled.x,
      y: sampled.y,
      theme: air.resolvedTheme
    });
  }, [adaptive, air.resolvedTheme]);

  React.useEffect(() => {
    air.registerAnchor("samaris", samarisRef.current);
  }, [air.registerAnchor, air]);

  React.useEffect(() => {
    if (air.activePanel === "none") return;

    function handlePointerDown(event: PointerEvent) {
      const target = event.target as Node | null;
      if (!target) return;
      if (rootRef.current?.contains(target)) return;
      air.closePanels();
    }

    function handleKeyDown(event: KeyboardEvent) {
      if (event.key === "Escape") air.closePanels();
    }

    window.addEventListener("pointerdown", handlePointerDown);
    window.addEventListener("keydown", handleKeyDown);
    return () => {
      window.removeEventListener("pointerdown", handlePointerDown);
      window.removeEventListener("keydown", handleKeyDown);
    };
  }, [air]);

  function toggle(panel: AirBarPanel, cb?: () => void) {
    const next = air.activePanel === panel ? "none" : panel;
    air.setActivePanel(next);
    if (next !== "none") cb?.();
  }

  return (
    <div ref={rootRef} style={vars} className={`samaris-airbar-shell ${props.className || ""}`.trim()}>
      <header className="samaris-airbar" role="banner" aria-label="AirBar">
        <div className="air-left">
          <AirBarButton
            ref={samarisRef}
            className="air-brand"
            ariaLabel="Samaris menu"
            active={air.activePanel === "samaris"}
            ariaExpanded={air.activePanel === "samaris"}
            onClick={() => toggle("samaris", props.onOpenSamarisMenu)}
          >
            <span className="air-orb" aria-hidden="true">
              <img className="air-orb__logo" src="brand/samaris-logo.png" alt="" />
            </span>
          </AirBarButton>
          <AirBarAppsButton onOpenApps={props.onOpenApps} />
          <AirBarSearchButton onOpenSearch={props.onOpenSearch} />
        </div>

        <div className="air-center">
          <AirBarDateTime />
        </div>

        <div className="air-right">
          <AirBarThemeToggle onToggleTheme={props.onToggleTheme} />
          <AirBarStatusCluster
            showBatteryPercentage={props.showBatteryPercentage}
            onOpenSidebar={props.onOpenSidebar}
          />
        </div>
      </header>

      <UserMenu />
      <AppsPanel />
      <SearchPanel />
      <SidebarPanel />
      <SoundPanel />
      <WifiPanel />
      <BluetoothPanel />
      <BatteryPanel />
    </div>
  );
}

export function AirBar(props: AirBarProps) {
  const theme: AirBarTheme = props.theme ?? "auto";
  return (
    <AirBarProvider theme={theme}>
      <AirBarInner
        adaptive={props.adaptive ?? true}
        showLabels={props.showLabels ?? true}
        showBatteryPercentage={props.showBatteryPercentage ?? true}
        className={props.className}
        onOpenApps={props.onOpenApps}
        onOpenSearch={props.onOpenSearch}
        onOpenSidebar={props.onOpenSidebar}
        onOpenSamarisMenu={props.onOpenSamarisMenu}
        onToggleTheme={props.onToggleTheme}
      />
    </AirBarProvider>
  );
}
