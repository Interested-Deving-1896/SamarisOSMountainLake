import type React from "react";
import type { LucideIcon } from "lucide-react";

export type SamarisIconName =
  | "computer"
  | "mail"
  | "music"
  | "videos"
  | "photos"
  | "orbit"
  | "peregrine"
  | "settings"
  | "trash"
  | "tools"
  | "games"
  | "notes"
  | "appstore"
  | "brain"
  | "folder"
  | "compat"
  | "network";

export type SamarisIconTheme = "light" | "dark" | "auto";

export type SamarisIconVariant = "soft" | "glass" | "mono" | "outline";

export type SamarisIconSurface = "tile" | "bare";

export type SamarisIconPreset = {
  tint: number;
  saturation: number;
  lightness: number;
  strength: number;
};

export type SamarisIconThemePreset = {
  alpha: number;
};

export type SamarisIconGlyph = LucideIcon;

export type SamarisIconProps = {
  name: SamarisIconName;
  size?: number | string;
  theme?: SamarisIconTheme;
  variant?: SamarisIconVariant;
  surface?: SamarisIconSurface;
  interactive?: boolean;
  tint?: number;
  saturation?: number;
  lightness?: number;
  strength?: number;
  alpha?: number;
  className?: string;
  title?: string;
};
