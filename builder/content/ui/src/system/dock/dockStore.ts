import { osStore } from "../../os/core/osStore";

type Listener = () => void;

const DEFAULT_PINNED_APPS = [
  "finder",
  "mail",
  "peregrine",
  "orbit",
  "notes",
  "photos",
  "music",
  "app-store",
  "settings",
  "utilities",
  "trash",
];

const LEGACY_DEFAULT_PINNED_APPS = ["finder", "mail", "music", "notes", "photos", "videos", "peregrine", "settings", "terminal"];

function isLegacyDefaultPinned(value: string[]) {
  return value.length === LEGACY_DEFAULT_PINNED_APPS.length && value.every((id, index) => id === LEGACY_DEFAULT_PINNED_APPS[index]);
}

export class DockStore {
  private state: { pinned: string[] };
  private listeners = new Set<Listener>();

  constructor() {
    this.state = { pinned: this.load() };
  }

  private load(): string[] {
    try {
      const raw = localStorage.getItem("samaris-dock/pinned");
      if (raw) {
        const parsed = JSON.parse(raw);
        if (Array.isArray(parsed)) {
          if (isLegacyDefaultPinned(parsed)) return DEFAULT_PINNED_APPS;
          return parsed;
        }
      }
    } catch {}
    return DEFAULT_PINNED_APPS;
  }

  private save() {
    localStorage.setItem("samaris-dock/pinned", JSON.stringify(this.state.pinned));
  }

  private emit() {
    for (const fn of this.listeners) fn();
  }

  getState() {
    return this.state;
  }

  subscribe(fn: Listener) {
    this.listeners.add(fn);
    return () => this.listeners.delete(fn);
  }

  isPinned(appId: string): boolean {
    return this.state.pinned.includes(appId);
  }

  isRunning(appId: string): boolean {
    return osStore.getState().processes.some((p) => p.appId === appId && p.state === "running");
  }

  pin(appId: string) {
    if (this.isPinned(appId)) return;
    this.state = { pinned: [...this.state.pinned, appId] };
    this.save();
    this.emit();
  }

  unpin(appId: string) {
    this.state = { pinned: this.state.pinned.filter((id) => id !== appId) };
    this.save();
    this.emit();
  }

  togglePin(appId: string) {
    if (this.isPinned(appId)) this.unpin(appId);
    else this.pin(appId);
  }

  reorder(fromIndex: number, toIndex: number) {
    const next = [...this.state.pinned];
    if (fromIndex < 0 || fromIndex >= next.length || toIndex < 0 || toIndex >= next.length) return;
    const [removed] = next.splice(fromIndex, 1);
    next.splice(toIndex, 0, removed);
    this.state = { pinned: next };
    this.save();
    this.emit();
  }
}

export const dockStore = new DockStore();
