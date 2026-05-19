import React from "react";
import { PanelRightOpen } from "lucide-react";
import { useAirBar } from "../useAirBar";
import { AirBarButton } from "./AirBarButton";

export const AirBarSidebarButton = React.memo(function AirBarSidebarButton(props: { onOpenSidebar?: () => void }) {
  const air = useAirBar();
  const active = air.activePanel === "sidebar";
  const ref = React.useRef<HTMLButtonElement | null>(null);

  React.useEffect(() => {
    air.registerAnchor("sidebar", ref.current);
  });

  return (
    <AirBarButton
      ref={ref}
      className="air-button sidebar-toggle"
      ariaLabel="Open sidebar"
      active={active}
      ariaExpanded={active}
      onClick={() => {
        const next = active ? "none" : "sidebar";
        air.setActivePanel(next);
        if (!active) props.onOpenSidebar?.();
      }}
    >
      <span className="icon" aria-hidden="true">
        <PanelRightOpen size={16} strokeWidth={2.2} absoluteStrokeWidth aria-hidden="true" />
      </span>
    </AirBarButton>
  );
});
