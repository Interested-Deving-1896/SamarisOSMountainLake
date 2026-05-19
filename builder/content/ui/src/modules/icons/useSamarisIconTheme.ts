import React from "react";
import { themeStore } from "../../system/theme/themeStore";
import type { SamarisIconTheme, SamarisIconThemePreset, SamarisIconVariant } from "./types";
import { samarisIconThemes } from "./themes";
import { useSamarisIconProvider } from "./SamarisIconProvider";

type ResolvedIconTheme = "light" | "dark";

function resolveAutoTheme(input: SamarisIconTheme | undefined, system: ResolvedIconTheme): ResolvedIconTheme {
  if (!input || input === "auto") return system;
  return input;
}

export function useSamarisIconTheme(explicit?: {
  theme?: SamarisIconTheme;
  variant?: SamarisIconVariant;
}): {
  theme: ResolvedIconTheme;
  variant: SamarisIconVariant;
  preset: SamarisIconThemePreset;
} {
  const provider = useSamarisIconProvider();
  const systemTheme = React.useSyncExternalStore(
    (listener) => themeStore.subscribe(listener),
    () => themeStore.getState()
  );

  const theme = resolveAutoTheme(explicit?.theme ?? provider.theme, systemTheme);
  const variant = explicit?.variant ?? provider.variant ?? "soft";
  const preset = samarisIconThemes[theme];

  return { theme, variant, preset };
}
