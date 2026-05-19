import { eventBus } from "../kernel/eventBus";
import { osStore, type OSProcess } from "./osStore";
import { runtimeManager } from "./runtimeManager";
import { windowManager } from "./windowManager";

let pidCounter = 1;

class ProcessManager {
  sync(processes: OSProcess[]) {
    const windows = osStore.getState().windows;
    const normalized = processes.map((process) => {
      const boundWindow = windows.find((window) => window.appId === process.appId && !window.processId);
      return {
        ...process,
        windowId: process.windowId || boundWindow?.id
      };
    });
    pidCounter = Math.max(pidCounter, ...normalized.map((process) => process.pid + 1), 1);
    osStore.setProcesses(normalized);
  }

  createProcess(
    appId: string,
    runtime: "app" | "chromium" | "sandbox" = "app",
    windowParams?: Record<string, unknown>
  ) {
    const pid = pidCounter++;
    const baseProcess: OSProcess = {
      pid,
      appId,
      runtime,
      state: "running",
      cpu: Math.round(Math.random() * 10 * 10) / 10,
      memory: Math.round(Math.random() * 100 * 10) / 10
    };

    const current = osStore.getState().processes;
    osStore.setProcesses([...current, baseProcess]);

    const runtimeEntry = runtimeManager.launchRuntime(baseProcess);
    osStore.setProcesses(
      osStore.getState().processes.map((process) =>
        process.pid === pid ? { ...process, runtimeId: runtimeEntry.id } : process
      )
    );

    const windowId = windowManager.openWindow(appId, pid, windowParams);
    this.bindWindow(pid, windowId);

    const next = osStore.getState().processes.find((entry) => entry.pid === pid) || {
      ...baseProcess,
      runtimeId: runtimeEntry.id
    };
    console.log("[SAMARIS] Process created", next);
    eventBus.emit("process:created", next);
    return next;
  }

  killProcess(pid: number) {
    const process = osStore.getState().processes.find((entry) => entry.pid === pid) || null;
    if (!process) return null;

    if (process.runtimeId) {
      runtimeManager.stopRuntime(process.runtimeId);
    }

    if (process.windowId) {
      windowManager.close(process.windowId);
    }

    osStore.setProcesses(osStore.getState().processes.filter((entry) => entry.pid !== pid));
    console.log("[SAMARIS] Process killed", pid);
    eventBus.emit("process:killed", pid);
    return pid;
  }

  pauseProcess(pid: number) {
    osStore.setProcesses(
      osStore.getState().processes.map((process) =>
        process.pid === pid ? { ...process, state: "paused" } : process
      )
    );
    return osStore.getState().processes.find((process) => process.pid === pid) || null;
  }

  resumeProcess(pid: number) {
    osStore.setProcesses(
      osStore.getState().processes.map((process) =>
        process.pid === pid ? { ...process, state: "running" } : process
      )
    );
    return osStore.getState().processes.find((process) => process.pid === pid) || null;
  }

  bindWindow(pid: number, windowId: string) {
    osStore.setProcesses(
      osStore.getState().processes.map((process) =>
        process.pid === pid ? { ...process, windowId } : process
      )
    );

    windowManager.updateLocal(windowId, { processId: pid });
    return windowId;
  }

  killProcessByWindow(windowId: string) {
    const process = osStore.getState().processes.find((entry) => entry.windowId === windowId);
    if (!process) {
      windowManager.close(windowId);
      return null;
    }
    return this.killProcess(process.pid);
  }
}

export const processManager = new ProcessManager();
