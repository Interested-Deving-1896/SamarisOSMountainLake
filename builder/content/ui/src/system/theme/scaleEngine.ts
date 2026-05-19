// Scale Engine — Samaris OS UI Scaling
//
// Two sources of truth:
//   1. VDM (Volt Display Manager) — hardware-detected DisplayProfile
//   2. Viewport fallback — window size + devicePixelRatio
//
// VDM profile takes priority when available.

type ScaleListener = (scale: number) => void;

let currentScale = 1;
let currentDensity = 1;
let currentFontScale = 1;
let currentSpacing = 1;
const listeners = new Set<ScaleListener>();

const VDM_PROFILE_PATH = "/run/samaris/display.generated.toml";
const VDM_EVENT_PATH = "/run/samaris/display.event.json";

function calculateScale(): number {
  const dpr = window.devicePixelRatio || 1;
  const w = window.innerWidth;
  const h = window.innerHeight;
  const scaleX = w / 1920;
  const scaleY = h / 1080;
  const base = Math.min(scaleX, scaleY);
  const scale = base * Math.min(dpr, 2) / 2;
  return Math.max(0.75, Math.min(scale, 1.5));
}

function applyScale(scale: number) {
  currentScale = scale;
  document.documentElement.style.setProperty("--scale", String(scale));
  for (const cb of listeners) cb(scale);
}

function applyVdmProfile(profile: { scale_factor?: number; recommended_ui_density?: number; recommended_font_scale?: number; recommended_spacing_scale?: number }) {
  if (profile.scale_factor && profile.scale_factor > 0.5) {
    applyScale(profile.scale_factor);
    currentDensity = profile.recommended_ui_density ?? 1;
    currentFontScale = profile.recommended_font_scale ?? 1;
    currentSpacing = profile.recommended_spacing_scale ?? 1;
    document.documentElement.dataset.vdmActive = "true";
  }
}

async function tryVdmProfile() {
  try {
    const resp = await fetch(VDM_PROFILE_PATH);
    if (!resp.ok) return;
    const text = await resp.text();
    // Minimal TOML value extraction for scale_factor
    const sfMatch = text.match(/scale_factor\s*=\s*([\d.]+)/);
    const densityMatch = text.match(/recommended_ui_density\s*=\s*([\d.]+)/);
    const fontMatch = text.match(/recommended_font_scale\s*=\s*([\d.]+)/);
    const spacingMatch = text.match(/recommended_spacing_scale\s*=\s*([\d.]+)/);
    if (sfMatch) {
      applyVdmProfile({
        scale_factor: parseFloat(sfMatch[1]),
        recommended_ui_density: densityMatch ? parseFloat(densityMatch[1]) : 1,
        recommended_font_scale: fontMatch ? parseFloat(fontMatch[1]) : 1,
        recommended_spacing_scale: spacingMatch ? parseFloat(spacingMatch[1]) : 1,
      });
    }
  } catch {
    // VDM profile not available — fall through to viewport-based scaling
  }
}

export function initScaleEngine() {
  // Try VDM first, fall back to viewport
  void tryVdmProfile().then(() => {
    if (!document.documentElement.dataset.vdmActive) {
      applyScale(calculateScale());
    }
  });

  let resizeTimer: number | null = null;
  const onResize = () => {
    if (resizeTimer) cancelAnimationFrame(resizeTimer);
    resizeTimer = requestAnimationFrame(() => {
      if (!document.documentElement.dataset.vdmActive) {
        applyScale(calculateScale());
      }
    });
  };

  window.addEventListener("resize", onResize);

  if (window.screen?.orientation) {
    window.screen.orientation.addEventListener("change", onResize);
  }

  if (window.matchMedia) {
    const dprQuery = window.matchMedia("(min-resolution: 2dppx)");
    dprQuery.addEventListener("change", onResize);
  }

  return () => {
    window.removeEventListener("resize", onResize);
    if (window.screen?.orientation) {
      window.screen.orientation.removeEventListener("change", onResize);
    }
  };
}

export function getScale(): number {
  return currentScale;
}

export function subscribeScale(cb: ScaleListener): () => void {
  listeners.add(cb);
  return () => listeners.delete(cb);
}

export function getVdmDensity(): number { return currentDensity; }
export function getVdmFontScale(): number { return currentFontScale; }
export function getVdmSpacing(): number { return currentSpacing; }
