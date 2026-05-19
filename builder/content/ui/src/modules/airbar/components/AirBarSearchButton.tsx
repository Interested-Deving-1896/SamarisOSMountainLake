import React from "react";
import { Search } from "lucide-react";
import { useAirBar } from "../useAirBar";
import { AirBarButton } from "./AirBarButton";

export const AirBarSearchButton = React.memo(function AirBarSearchButton(props: { onOpenSearch?: () => void }) {
  const air = useAirBar();
  const active = air.activePanel === "search";
  const ref = React.useRef<HTMLButtonElement | null>(null);

  React.useEffect(() => {
    air.registerAnchor("search", ref.current);
  });

  return (
    <AirBarButton
      ref={ref}
      className="air-button air-pill air-search"
      ariaLabel="Search"
      active={active}
      ariaExpanded={active}
      onClick={() => {
        const next = active ? "none" : "search";
        air.setActivePanel(next);
        if (!active) props.onOpenSearch?.();
      }}
    >
      <span className="icon search-icon">
        <Search size={16} strokeWidth={2.2} absoluteStrokeWidth aria-hidden="true" />
      </span>
    </AirBarButton>
  );
});
