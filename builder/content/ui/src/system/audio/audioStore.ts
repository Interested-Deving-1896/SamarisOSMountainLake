import { audioKernel, type AudioStatus } from "../../services/kernel/audio";

type Listener = () => void;

const DEFAULT_STATE: AudioStatus = {
  volume: 50,
  muted: false,
  outputs: [{ id: "default", label: "Default Output", active: true }],
  activeOutputId: "default"
};

class AudioStore {
  private state: AudioStatus = DEFAULT_STATE;
  private listeners = new Set<Listener>();
  private initialized = false;
  private pollTimer: number | null = null;
  private volumeCommitTimer: number | null = null;

  init() {
    if (this.initialized) return;
    this.initialized = true;
    void this.refresh();
    this.pollTimer = window.setInterval(() => void this.refresh(), 12000);
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
      this.patch(await audioKernel.status());
    } catch {}
  }

  setVolume(volume: number) {
    const nextVolume = Math.max(0, Math.min(100, Math.round(volume)));
    this.patch({
      ...this.state,
      volume: nextVolume,
      muted: nextVolume === 0
    });

    if (this.volumeCommitTimer !== null) {
      window.clearTimeout(this.volumeCommitTimer);
    }

    this.volumeCommitTimer = window.setTimeout(() => {
      this.volumeCommitTimer = null;
      void audioKernel
        .setVolume(nextVolume)
        .then((status) => this.patch(status))
        .catch(() => void this.refresh());
    }, 30);
  }

  setOutput(outputId: string) {
    void audioKernel.setOutput(outputId).then((status) => this.patch(status)).catch(() => {});
  }

  private patch(next: AudioStatus) {
    this.state = next;
    for (const listener of this.listeners) listener();
  }
}

export const audioStore = new AudioStore();
