import React from "react";
import { kernelClient } from "../../../os/kernel/kernelClient";
import { osStore } from "../../../os/core/osStore";

export function useMonitorSeries() {
  const [cpuSeries, setCpuSeries] = React.useState<number[]>([]);
  const [memorySeries, setMemorySeries] = React.useState<number[]>([]);
  const [ready, setReady] = React.useState(false);
  const [snapshot, setSnapshot] = React.useState(() => ({
    cpu: 0, memory: 0, processes: 0, windows: 0, runtimes: 0,
  }));

  React.useEffect(() => {
    let first = true;
    let cancelled = false;

    async function tick() {
      try {
        const result = await kernelClient.request<{
          cpu?: { usagePercent?: string; cores?: number };
          memory?: { total?: number; used?: number; usagePercent?: string };
          disk?: { usagePercent?: string };
        }>({ type: "system.metrics", data: {} }, { timeoutMs: 5000 });

        const metrics = result.data || {};
        const cpu = parseFloat(String(metrics.cpu?.usagePercent ?? "0")) || 0;
        const memPct = parseFloat(String(metrics.memory?.usagePercent ?? "0")) || 0;
        const memTotalMB = metrics.memory?.total ? Math.round(metrics.memory.total / (1024 * 1024)) : 0;
        const memUsedMB = metrics.memory?.used ? Math.round(metrics.memory.used / (1024 * 1024)) : 0;
        const mem = memTotalMB > 0 ? memUsedMB : memPct;

        const next = {
          cpu,
          memory: mem,
          processes: osStore.getState().processes.length,
          windows: osStore.getState().windows.length,
          runtimes: osStore.getState().runtimes.length,
        };

        if (!cancelled) {
          setSnapshot(next);
          setCpuSeries((c) => [...c.slice(-29), next.cpu]);
          setMemorySeries((c) => [...c.slice(-29), next.memory]);
          if (first) { setReady(true); first = false; }
        }
      } catch {
        if (!cancelled && first) { setReady(true); first = false; }
      }
    }

    tick();
    const interval = window.setInterval(tick, 2000);
    return () => { cancelled = true; window.clearInterval(interval); };
  }, []);

  return { snapshot, cpuSeries, memorySeries, ready };
}
