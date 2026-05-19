import { batteryKernel, type BatteryStatus } from "../../services/kernel/battery";

type Listener = () => void;

const DEFAULT_STATE: BatteryStatus = {
  available: false,
  percentage: 0,
  charging: false,
  lowPower: false,
  source: "Unknown"
};

class BatteryStore {
  private state: BatteryStatus = DEFAULT_STATE;
  private listeners = new Set<Listener>();
  private initialized = false;
  private pollTimer: number | null = null;

  init() {
    if (this.initialized) return;
    this.initialized = true;
    void this.refresh();
    this.pollTimer = window.setInterval(() => void this.refresh(), 30000);
  }

  getState() {
    return this.state;
  }

  subscribe(listener: Listener) {
    this.listeners.add(listener);
    return () => this.listeners.delete(listener);
  }

  async refresh() {
    try {
      this.patch(await batteryKernel.status());
    } catch {}
  }

  private patch(next: BatteryStatus) {
    this.state = next;
    for (const listener of this.listeners) listener();
  }
}

export const batteryStore = new BatteryStore();
