import React from "react";
import type { SamarisIconName, SamarisIconProps, SamarisIconVariant } from "./types";
import { iconRegistry, samarisIconColors } from "./registry";
import { createCssVars } from "./utils/createCssVars";
import { resolveIconPreset, resolveThemePreset } from "./utils/resolveIconPreset";
import { useSamarisIconTheme } from "./useSamarisIconTheme";
import "./styles/samaris-icon.css";

function resolveName(name: SamarisIconName): SamarisIconName {
  if (iconRegistry[name]) return name;
  if (import.meta.env?.DEV) {
    // eslint-disable-next-line no-console
    console.warn("[SamarisIcon] Unknown icon name:", name);
  }
  return "computer";
}

function isBareByDefault(variant: SamarisIconVariant) {
  return variant === "mono" || variant === "outline";
}

export const SamarisIcon = React.memo(function SamarisIcon(rawProps: SamarisIconProps) {
  const name = resolveName(rawProps.name);
  const { theme, variant } = useSamarisIconTheme({ theme: rawProps.theme, variant: rawProps.variant });
  const preset = resolveIconPreset(name, rawProps);
  const themePreset = resolveThemePreset(theme, rawProps);
  const surface = rawProps.surface ?? (isBareByDefault(variant) ? "bare" : "tile");
  const Glyph = iconRegistry[name] ?? iconRegistry.computer;
  const iconColor = samarisIconColors[name] ?? samarisIconColors.computer;

  const size =
    typeof rawProps.size === "number"
      ? `${rawProps.size}px`
      : rawProps.size
        ? rawProps.size
        : "64px";

  const decorative = !rawProps.title;

  const cssVars = React.useMemo(
    () =>
      createCssVars({
        "--tile-h": preset.tint,
        "--tile-s": `${preset.saturation}%`,
        "--tile-l": `${preset.lightness}%`,
        "--tile-strength": preset.strength,
        "--icon-color": iconColor,
        "--glass-alpha": themePreset.alpha,
        "--icon-size": size
      }),
    [iconColor, preset.tint, preset.saturation, preset.lightness, preset.strength, themePreset.alpha, size]
  );

  const className = [
    "samaris-icon",
    `samaris-icon--${variant}`,
    `samaris-icon--${surface}`,
    rawProps.className ?? ""
  ]
    .filter(Boolean)
    .join(" ");

  const icon = (
    <Glyph
      className="samaris-icon__glyph"
      strokeWidth={variant === "mono" || variant === "outline" ? 2.25 : 2.3}
      strokeLinecap="round"
      strokeLinejoin="round"
      vectorEffect="non-scaling-stroke"
      absoluteStrokeWidth
      aria-hidden="true"
    />
  );

  return (
    <div
      className={className}
      style={cssVars}
      aria-hidden={decorative ? "true" : undefined}
      role={!decorative ? "img" : undefined}
      aria-label={!decorative ? rawProps.title : undefined}
    >
      {surface === "tile" ? <div className="samaris-icon__tile">{icon}</div> : icon}
    </div>
  );
});
