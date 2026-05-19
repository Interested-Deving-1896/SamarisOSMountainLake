import dayWallpaperUrl from "../../assets/wallpapers/samaris-day-wallpaper.png";
import auroraWallpaperUrl from "../../assets/wallpapers/volt-os-aurora.png";

export type WallpaperPreset = {
  id: string;
  label: string;
  kind: "image" | "gradient";
  preview: string;
  imageUrl: string;
  background: string;
};

const WALLPAPER_KEY = "samaris-os/wallpaper";
const PREFS_KEY = "samaris-os/settings-prefs";

function bgSizeForFit(fit?: string): string {
  if (fit === "fit") return "contain";
  if (fit === "stretch") return "100% 100%";
  if (fit === "center") return "auto";
  return "cover";
}

function loadFit(): string {
  try { const p = JSON.parse(localStorage.getItem(PREFS_KEY) || "{}"); return p.wallpaperFit || "fill"; } catch { return "fill"; }
}

function makeBackground(imageUrl: string, gradient: string, fit?: string): string {
  const bg = bgSizeForFit(fit);
  return `${gradient}, url("${imageUrl}") center/${bg} no-repeat fixed`;
}

const GRADIENT_DAY = "linear-gradient(180deg, rgba(255, 255, 255, 0.08), rgba(231, 240, 255, 0.12))";
const GRADIENT_AURORA = "linear-gradient(180deg, rgba(255, 255, 255, 0.04), rgba(18, 35, 64, 0.1))";

const PRESETS: WallpaperPreset[] = [
  {
    id: "mountain-lake",
    label: "Mountain Lake",
    kind: "image",
    preview: dayWallpaperUrl,
    imageUrl: dayWallpaperUrl,
    background: makeBackground(dayWallpaperUrl, GRADIENT_DAY, "fill")
  },
  {
    id: "aurora",
    label: "Aurora",
    kind: "image",
    preview: auroraWallpaperUrl,
    imageUrl: auroraWallpaperUrl,
    background: makeBackground(auroraWallpaperUrl, GRADIENT_AURORA, "fill")
  }
];

type WallpaperListener = () => void;

class WallpaperStore {
  private currentId = PRESETS[0].id;
  private listeners = new Set<WallpaperListener>();
  private initialized = false;

  init() {
    if (this.initialized) return;
    this.initialized = true;
    const stored = window.localStorage.getItem(WALLPAPER_KEY);
    if (stored && PRESETS.some((preset) => preset.id === stored)) {
      this.currentId = stored;
    }
    this.apply();
  }

  list() {
    return PRESETS;
  }

  getState() {
    return this.currentId;
  }

  getCurrent() {
    return PRESETS.find((preset) => preset.id === this.currentId) || PRESETS[0];
  }

  subscribe(listener: WallpaperListener) {
    this.listeners.add(listener);
    return () => {
      this.listeners.delete(listener);
    };
  }

  setWallpaper(id: string) {
    if (!PRESETS.some((preset) => preset.id === id)) return;
    this.currentId = id;
    window.localStorage.setItem(WALLPAPER_KEY, id);
    this.apply();
    this.emit();
  }

  refreshFit() {
    this.apply();
  }

  private apply() {
    const preset = this.getCurrent();
    const fit = loadFit();
    const gradient = preset.id === "aurora" ? GRADIENT_AURORA : GRADIENT_DAY;
    document.documentElement.style.setProperty("--volt-wallpaper-scene", makeBackground(preset.imageUrl, gradient, fit));
  }

  private emit() {
    for (const listener of this.listeners) {
      listener();
    }
  }
}

export const wallpaperStore = new WallpaperStore();
