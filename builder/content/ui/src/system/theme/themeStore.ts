type ThemeMode = "light" | "dark";
const THEME_KEY = "samaris-os/theme";
type ThemeListener = () => void;

class ThemeStore {
  private mode: ThemeMode = "light";
  private listeners = new Set<ThemeListener>();
  private initialized = false;

  init() {
    if (this.initialized) return;
    this.initialized = true;
    const stored = window.localStorage.getItem(THEME_KEY);
    if (stored === "dark" || stored === "light") {
      this.mode = stored;
    } else if (window.matchMedia?.("(prefers-color-scheme: dark)").matches) {
      this.mode = "dark";
    }
    this.apply();
  }

  getState() { return this.mode; }

  subscribe(listener: ThemeListener) {
    this.listeners.add(listener);
    return () => { this.listeners.delete(listener); };
  }

  setMode(mode: ThemeMode) {
    this.mode = mode;
    window.localStorage.setItem(THEME_KEY, mode);
    this.apply();
    this.emit();
  }

  toggle() { this.setMode(this.mode === "light" ? "dark" : "light"); }

  private apply() {
    document.documentElement.dataset.theme = this.mode;
    document.documentElement.style.colorScheme = this.mode;
  }

  private emit() {
    for (const l of this.listeners) l();
  }
}

export const themeStore = new ThemeStore();
export type { ThemeMode };
