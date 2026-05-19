import type { SamarisIconName, SamarisIconPreset, SamarisIconProps, SamarisIconThemePreset } from "../types";
import { samarisIconPresets } from "../registry";
import { samarisIconThemes } from "../themes";

export function resolveIconPreset(name: SamarisIconName, props: SamarisIconProps): SamarisIconPreset {
  const base = samarisIconPresets[name] ?? samarisIconPresets.computer;
  return {
    tint: props.tint ?? base.tint,
    saturation: props.saturation ?? base.saturation,
    lightness: props.lightness ?? base.lightness,
    strength: props.strength ?? base.strength
  };
}

export function resolveThemePreset(theme: "light" | "dark", props: SamarisIconProps): SamarisIconThemePreset {
  const base = samarisIconThemes[theme];
  return { alpha: props.alpha ?? base.alpha };
}

