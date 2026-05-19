import React from "react";
import { BatteryCharging, BatteryFull } from "lucide-react";
import { useBatteryStatus } from "../useBatteryStatus";
import { batteryStore } from "../../../system/battery/batteryStore";
import { useAirBar } from "../useAirBar";
import { AirBarButton } from "./AirBarButton";

export const AirBarBattery = React.memo(function AirBarBattery(props: { showPercentage?: boolean }) {
  const battery = useBatteryStatus();
  const air = useAirBar();
  const ref = React.useRef<HTMLButtonElement | null>(null);
  const isDesktopPower = !battery.available;

  React.useEffect(() => { batteryStore.refresh(); }, []);
  React.useLayoutEffect(() => { air.registerAnchor("battery", ref.current); });

  if (isDesktopPower) return null;

  const Icon = battery.charging ? BatteryCharging : BatteryFull;
  const label = battery.charging
    ? `${battery.percentage}% Charging`
    : battery.source === "Fully Charged" || battery.percentage >= 100
      ? "100% Fully Charged"
      : `${battery.percentage}%`;

  return (
    <AirBarButton
      ref={ref}
      className="air-status battery"
      ariaLabel="Power"
      active={air.activePanel === "battery"}
      ariaExpanded={air.activePanel === "battery"}
      onClick={() => air.setActivePanel(air.activePanel === "battery" ? "none" : "battery")}
    >
      <span className="icon">
        <Icon size={16} strokeWidth={2.2} absoluteStrokeWidth aria-hidden="true" />
      </span>
      {props.showPercentage !== false ? <span className="label">{label}</span> : null}
    </AirBarButton>
  );
});
