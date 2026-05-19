import { eventBus } from "../kernel/eventBus";
import { osStore, type OSRuntime } from "./osStore";

let runtimeCounter = 1;

class RuntimeManager {
  sync(runtimes: Array<OSRuntime & { kind?: string; state?: string; target?: string | null }>) {
    const normalized = runtimes.map((runtime) => ({
      id: runtime.id,
      processId: runtime.processId ?? 0,
      type:
        runtime.type ||
        (runtime.kind === "chromium" ? "browser" : runtime.kind === "sandbox" ? "sandbox" : "app"),
      status: runtime.status || (runtime.state === "stopped" ? "stopped" : "running")
    }));
    const nextCounter =
      normalized.reduce((maxValue, runtime) => {
        const match = /^rt-(\d+)$/.exec(runtime.id);
        if (!match) return maxValue;
        return Math.max(maxValue, Number.parseInt(match[1], 10) + 1);
      }, 1) || 1;
    runtimeCounter = Math.max(runtimeCounter, nextCounter);
    osStore.setRuntimes(normalized);
  }

  launchRuntime(process: { pid: number; runtime: "app" | "chromium" | "sandbox" }) {
    const runtime: OSRuntime = {
      id: `rt-${runtimeCounter++}`,
      processId: process.pid,
      type: process.runtime === "chromium" ? "browser" : process.runtime,
      status: "running"
    };

    osStore.setRuntimes([...osStore.getState().runtimes, runtime]);
    console.log("[SAMARIS] Runtime launched", runtime);
    eventBus.emit("runtime:started", runtime);
    return runtime;
  }

  attachRuntime(processId: number) {
    return osStore.getState().runtimes.find((runtime) => runtime.processId === processId) || null;
  }

  stopRuntime(runtimeId: string) {
    const runtime = osStore.getState().runtimes.find((entry) => entry.id === runtimeId) || null;
    if (!runtime) {
      return null;
    }

    osStore.setRuntimes(osStore.getState().runtimes.filter((entry) => entry.id !== runtimeId));
    console.log("[SAMARIS] Runtime stopped", runtimeId);
    eventBus.emit("runtime:stopped", runtimeId);
    return runtimeId;
  }
}

export const runtimeManager = new RuntimeManager();
