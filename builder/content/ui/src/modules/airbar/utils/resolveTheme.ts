import { themeStore } from "../../../system/theme/themeStore";
import type { AirBarTheme } from "../types";

export function resolveTheme(theme: AirBarTheme): "day" | "night" {
  if (theme === "day") return "day";
  if (theme === "night") return "night";
  const system = themeStore.getState();
  return system === "dark" ? "night" : "day";
}

