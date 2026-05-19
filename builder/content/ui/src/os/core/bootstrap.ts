import type { AppWindow } from "../../shell/windowing/types";
import { kernelClient } from "../kernel/kernelClient";
import { eventBus } from "../kernel/eventBus";
import { osStore, type OSDevice, type OSProcess, type OSRuntime } from "./osStore";
import { processManager } from "./processManager";
import { runtimeManager } from "./runtimeManager";
import { sessionManager } from "./sessionManager";
import { windowManager } from "./windowManager";
import { sessionPersistence } from "../../system/session/sessionPersistence";
import { permissionManager } from "../kernel/permissionManager";
import { appRegistry } from "../apps/appRegistry";

const SYSTEM_NAMESPACES = [
  "system.*", "user.*", "fs.*", "window.*", "event.*",
  "device.*", "audio.*", "battery.*", "session.*", "runtime.*",
  "process.*", "app.*", "search.*", "storage.*", "network.*",
  "power.*", "permission.*", "mail.*", "media.*", "print.*",
  "wine.*", "orbit.*", "encryption.*", "onboarding.*", "firewall.*", "browser.*"
];

const APP_BASE_PERMISSIONS = ["window.open", "window.focus", "event.emit"];

type KernelState = {
  windows: AppWindow[];
  processes: OSProcess[];
  runtimes: OSRuntime[];
  devices: OSDevice[];
  session: Record<string, unknown>;
};

class OSBootstrap {
  private initialized = false;

  async init() {
    if (this.initialized) return;
    this.initialized = true;

    permissionManager.seed("volt.desktop", SYSTEM_NAMESPACES);

    for (const [appId, appDef] of Object.entries(appRegistry)) {
      const extras: string[] = [];
      if (appDef.runtime === "app") extras.push("fs.read", "fs.write");
      permissionManager.seed(appId, [...APP_BASE_PERMISSIONS, ...extras]);
    }

    permissionManager.seed("app-store", ["network"]);
    permissionManager.seed("wine", ["fs.read"]);

    sessionPersistence.restore();
    const restoredState = osStore.getState();

    await kernelClient.connect();
    const response = await kernelClient.request<KernelState>({
      type: "system.state",
      data: {}
    });

    if (!response.data) {
      throw new Error("kernel_state_missing");
    }

    osStore.setState({
      windows: response.data.windows.length ? response.data.windows : restoredState.windows,
      processes: response.data.processes.length ? response.data.processes : restoredState.processes,
      runtimes: response.data.runtimes.length ? response.data.runtimes : restoredState.runtimes,
      devices: response.data.devices,
      session: response.data.session
    });

    processManager.sync(response.data.processes.length ? response.data.processes : restoredState.processes);
    runtimeManager.sync(response.data.runtimes.length ? response.data.runtimes : restoredState.runtimes);
    sessionManager.sync(response.data.session);
    windowManager.sync(response.data.windows.length ? response.data.windows : restoredState.windows);

    eventBus.emit("os:ready", response.data);
    sessionPersistence.save();
  }
}

export const osBootstrap = new OSBootstrap();
