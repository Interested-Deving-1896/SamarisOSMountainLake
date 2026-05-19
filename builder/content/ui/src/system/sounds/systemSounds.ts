import { audioStore } from "../audio/audioStore";

export type SystemSoundName = "login" | "logout" | "shutdown" | "notification" | "error";

type Listener = () => void;

const SOUND_URLS: Record<SystemSoundName, string> = {
  login: "/sounds/system/login.mp3",
  logout: "/sounds/system/logout.mp3",
  shutdown: "/sounds/system/shutdown.mp3",
  notification: "/sounds/system/notification.mp3",
  error: "/sounds/system/error.mp3"
};

function clamp01(value: number) {
  if (Number.isNaN(value)) return 0;
  return Math.max(0, Math.min(1, value));
}

class SystemSounds {
  private initialized = false;
  private cache = new Map<SystemSoundName, HTMLAudioElement>();
  private state = { muted: false, volume: 0.5 };
  private listeners = new Set<Listener>();

  init() {
    if (this.initialized) return;
    this.initialized = true;
    this.syncFromAudioStore();
    audioStore.subscribe(() => this.syncFromAudioStore());
  }

  subscribe(listener: Listener) {
    this.listeners.add(listener);
    return () => this.listeners.delete(listener);
  }

  getState() {
    return this.state;
  }

  play(name: SystemSoundName) {
    if (typeof window === "undefined") return;
    if (!this.initialized) this.init();
    if (this.state.muted || this.state.volume <= 0.001) return;

    const audio = this.getAudio(name);
    audio.volume = clamp01(this.state.volume);
    try {
      audio.currentTime = 0;
    } catch {}
    // Fire-and-forget: browsers may block autoplay until user gesture; that's OK.
    void audio.play().catch(() => {});
  }

  private syncFromAudioStore() {
    const status = audioStore.getState();
    // Keep system sounds slightly under the master volume; they should be subtle.
    const volume = clamp01((status.volume ?? 50) / 100) * 0.85;
    const muted = Boolean(status.muted);
    this.state = { muted, volume };
    for (const listener of this.listeners) listener();
  }

  private getAudio(name: SystemSoundName) {
    const existing = this.cache.get(name);
    if (existing) return existing;
    const audio = new Audio(SOUND_URLS[name]);
    audio.preload = "auto";
    this.cache.set(name, audio);
    return audio;
  }
}

export const systemSounds = new SystemSounds();

