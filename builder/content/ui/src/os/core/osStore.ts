import type { AppWindow } from "../../shell/windowing/types";

export type OSProcess = {
  pid: number;
  appId: string;
  runtime: "app" | "chromium" | "sandbox";
  state: "running" | "paused" | "terminated";
  windowId?: string;
  runtimeId?: string;
  priority?: string;
  cpu: number;
  memory: number;
  permissions?: string[];
};

export type OSRuntime = {
  id: string;
  processId: number;
  type: "app" | "sandbox" | "browser";
  status: "running" | "stopped";
};

export type OSDevice = {
  id: string;
  type: string;
  state?: string;
  meta?: Record<string, unknown>;
};

export type OSSession = Record<string, unknown>;

export type OSState = {
  windows: AppWindow[];
  processes: OSProcess[];
  runtimes: OSRuntime[];
  devices: OSDevice[];
  session: OSSession;
};

const initialState: OSState = {
  windows: [],
  processes: [],
  runtimes: [],
  devices: [],
  session: {}
};

type StoreListener = () => void;

class OSStore {
  private state: OSState = initialState;
  private listeners = new Set<StoreListener>();
  private versions = { windows: 0, processes: 0, runtimes: 0, devices: 0, session: 0 };

  getState() {
    return this.state;
  }

  subscribe(listener: StoreListener) {
    this.listeners.add(listener);
    return () => { this.listeners.delete(listener); };
  }

  subscribeTo(slice: keyof OSState, listener: StoreListener) {
    let lastVersion = this.versions[slice];
    const wrapper = () => {
      if (this.versions[slice] !== lastVersion) {
        lastVersion = this.versions[slice];
        listener();
      }
    };
    this.listeners.add(wrapper);
    return () => { this.listeners.delete(wrapper); };
  }

  setState(nextState: OSState) {
    const prev = this.state;
    this.state = nextState;
    if (nextState.windows !== prev.windows) this.versions.windows++;
    if (nextState.processes !== prev.processes) this.versions.processes++;
    if (nextState.runtimes !== prev.runtimes) this.versions.runtimes++;
    if (nextState.devices !== prev.devices) this.versions.devices++;
    if (nextState.session !== prev.session) this.versions.session++;
    this.emit();
  }

  patch(partial: Partial<OSState>) {
    const prev = this.state;
    this.state = { ...prev, ...partial };
    for (const key of Object.keys(partial) as (keyof OSState)[]) {
      if (this.state[key] !== prev[key]) this.versions[key]++;
    }
    this.emit();
  }

  update(updater: (state: OSState) => OSState) {
    const prev = this.state;
    this.state = updater(this.state);
    for (const key of Object.keys(this.state) as (keyof OSState)[]) {
      if (this.state[key] !== prev[key]) this.versions[key]++;
    }
    this.emit();
  }

  private emit() {
    for (const listener of this.listeners) listener();
  }

  setProcesses(processes: OSProcess[]) {
    this.patch({ processes });
  }

  setRuntimes(runtimes: OSRuntime[]) {
    this.patch({ runtimes });
  }

  setWindows(windows: AppWindow[]) {
    this.patch({ windows });
  }
}

export const osStore = new OSStore();
