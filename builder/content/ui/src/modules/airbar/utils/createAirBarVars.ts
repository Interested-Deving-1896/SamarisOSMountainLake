import type React from "react";

type CSSVars = { [key: `--${string}`]: string };

export type AirBarVarsInput = {
  accentH: number;
  accent2H: number;
  accentRgb: { r: number; g: number; b: number };
  glassRgb: { r: number; g: number; b: number };
  x: string;
  y: string;
  theme: "day" | "night";
};

export function createAirBarVars(input: AirBarVarsInput): React.CSSProperties {
  const isNight = input.theme === "night";

  const style: CSSVars = {
    "--air-accent": String(input.accentH),
    "--air-accent-2": String(input.accent2H),
    "--air-accent-r": String(input.accentRgb.r),
    "--air-accent-g": String(input.accentRgb.g),
    "--air-accent-b": String(input.accentRgb.b),

    "--air-text": isNight ? "rgba(238, 245, 255, 0.96)" : "rgba(10, 26, 46, 0.94)",
    "--air-text-soft": isNight ? "rgba(238, 245, 255, 0.68)" : "rgba(10, 26, 46, 0.62)",
    "--air-text-faint": isNight ? "rgba(238, 245, 255, 0.44)" : "rgba(10, 26, 46, 0.42)",

    "--air-glass-r": String(input.glassRgb.r),
    "--air-glass-g": String(input.glassRgb.g),
    "--air-glass-b": String(input.glassRgb.b),
    "--air-glass-alpha": isNight ? "0.20" : "0.34",
    "--air-glass-alpha-strong": isNight ? "0.32" : "0.48",
    "--air-glass-border": isNight ? "rgba(255,255,255,0.13)" : "rgba(255,255,255,0.64)",
    "--air-glass-border-soft": isNight ? "rgba(255,255,255,0.07)" : "rgba(255,255,255,0.24)",

    "--air-panel-alpha": isNight ? "0.18" : "0.38",
    "--air-blur": "24px",
    "--air-panel-blur": "22px",
    "--air-x": input.x,
    "--air-y": input.y,

    "--air-shadow": isNight
      ? "inset 0 1px 0 rgba(255,255,255,0.10), inset 18px 16px 42px rgba(255,255,255,0.028), inset -22px -22px 48px rgba(0,0,0,0.18), 0 18px 42px rgba(0,0,0,0.42)"
      : "inset 0 1px 0 rgba(255,255,255,0.74), inset 22px 18px 52px rgba(255,255,255,0.26), inset -24px -26px 58px rgba(var(--air-accent-r), var(--air-accent-g), var(--air-accent-b), 0.12), 0 26px 78px rgba(26, 74, 135, 0.28), 0 0 54px rgba(var(--air-accent-r), var(--air-accent-g), var(--air-accent-b), 0.12)",

    "--air-panel-shadow": isNight
      ? "inset 0 1px 0 rgba(255,255,255,0.09), inset 18px 16px 40px rgba(255,255,255,0.026), inset -22px -24px 48px rgba(0,0,0,0.18), 0 24px 64px rgba(0,0,0,0.44)"
      : "inset 0 1px 0 rgba(255,255,255,0.48), inset 20px 18px 48px rgba(255,255,255,0.16), inset -22px -24px 52px rgba(var(--air-accent-r), var(--air-accent-g), var(--air-accent-b), 0.10), 0 32px 96px rgba(20, 70, 130, 0.30)"
  };

  return style as React.CSSProperties;
}
