import React from "react";
import { eventBus } from "../../../os/kernel/eventBus";

interface Metrics { cpu?: { usagePercent?: string; cores?: number }; memory?: { total?: number; used?: number; usagePercent?: string }; disk?: { mount?: string; total?: number; used?: number; usagePercent?: string } }

let lastMetrics: Metrics = {};
const listeners = new Set<(m: Metrics) => void>();
eventBus.on("system:metrics", (m: unknown) => { lastMetrics = m as Metrics; for (const l of listeners) l(lastMetrics); });

function useMetrics() {
  const [m, setM] = React.useState<Metrics>(lastMetrics);
  React.useEffect(() => { listeners.add(setM); return () => { listeners.delete(setM); }; }, []);
  return m;
}

export const AirBarMetrics = React.memo(function AirBarMetrics() {
  const m = useMetrics();
  const cpu = m.cpu?.usagePercent ? `${m.cpu.usagePercent}%` : null;
  const ram = m.memory?.usagePercent ? `${m.memory.usagePercent}%` : null;
  const disk = m.disk?.usagePercent ? `${m.disk.usagePercent}%` : null;

  return (
    <div className="air-metrics" title="CPU • RAM • Disk">
      {cpu && <span className="air-metric air-metric--cpu"><span className="air-metric__dot" />{cpu}</span>}
      {ram && <span className="air-metric air-metric--ram"><span className="air-metric__dot" />{ram}</span>}
      {disk && <span className="air-metric air-metric--disk"><span className="air-metric__dot" />{disk}</span>}
    </div>
  );
});
