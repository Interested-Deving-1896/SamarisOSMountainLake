import React from "react";
import { BatteryCharging, BatteryFull, PlugZap, Zap } from "lucide-react";
import { batteryStore } from "../../system/battery/batteryStore";
import { useAirBar } from "../airbar/useAirBar";
import { SYSTEM_PANEL_CLASSES } from "./panel.styles";

export function BatteryPanel() {
  const air = useAirBar();
  const open = air.activePanel === "battery";
  const battery = React.useSyncExternalStore(
    (listener) => batteryStore.subscribe(listener),
    () => batteryStore.getState()
  );
  const style = air.getPanelStyle("battery", { width: 308, align: "end" });
  const isDesktopPower = !battery.available;
  const label = isDesktopPower
    ? battery.source === "AC Power"
      ? "AC Power"
      : "No Battery Detected"
    : battery.charging
      ? `${battery.percentage}% Charging`
      : battery.source === "Fully Charged" || battery.percentage >= 100
        ? "100% Fully Charged"
        : `${battery.percentage}%`;
  const meta = isDesktopPower ? "Desktop power source" : battery.source || "System power source";

  React.useEffect(() => {
    batteryStore.init();
    if (!open) return;
    void batteryStore.refresh();
  }, [open]);

  return (
    <section style={style} className={`airbar-panel airbar-system-panel ${open ? "open" : ""}`} role="dialog" aria-label="Power">
      <div className={SYSTEM_PANEL_CLASSES.panel}>
        <div className={SYSTEM_PANEL_CLASSES.section}>
          <div className={SYSTEM_PANEL_CLASSES.heading}>Power</div>
          <div className={SYSTEM_PANEL_CLASSES.row}>
            <span className={SYSTEM_PANEL_CLASSES.rowIcon}>
              {isDesktopPower ? (
                <PlugZap size={18} strokeWidth={2.2} />
              ) : battery.charging ? (
                <BatteryCharging size={18} strokeWidth={2.2} />
              ) : (
                <BatteryFull size={18} strokeWidth={2.2} />
              )}
            </span>
            <span className={SYSTEM_PANEL_CLASSES.rowText}>
              <span className={SYSTEM_PANEL_CLASSES.rowLabel}>{label}</span>
              <span className={SYSTEM_PANEL_CLASSES.rowMeta}>{meta}</span>
            </span>
            {battery.lowPower ? <Zap size={16} strokeWidth={2.2} /> : null}
          </div>
        </div>
      </div>
    </section>
  );
}
