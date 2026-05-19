import React from "react";
import { AppWindowMac } from "lucide-react";
import { useAirBar } from "../useAirBar";
import { AirBarButton } from "./AirBarButton";

export const AirBarAppsButton = React.memo(function AirBarAppsButton(props: { onOpenApps?: () => void }) {
  const air = useAirBar();
  const active = air.activePanel === "apps";
  const ref = React.useRef<HTMLButtonElement | null>(null);

  React.useEffect(() => {
    air.registerAnchor("apps", ref.current);
  });

  return (
    <AirBarButton
      ref={ref}
      className="air-button air-pill apps"
      ariaLabel="Apps"
      active={active}
      ariaExpanded={active}
      onClick={() => {
        const next = active ? "none" : "apps";
        air.setActivePanel(next);
        if (!active) props.onOpenApps?.();
      }}
    >
      <span className="icon">
        <AppWindowMac size={16} strokeWidth={2.2} absoluteStrokeWidth aria-hidden="true" />
      </span>
    </AirBarButton>
  );
});
