export type GenieStyle = "genie" | "shatter" | "slide";
export type GenieIntensity = "subtle" | "normal" | "dramatic";

export type GenieConfig = {
  style: GenieStyle;
  intensity: GenieIntensity;
  soundEnabled: boolean;
  shadowEnabled: boolean;
  postFxEnabled: boolean;
};

export const DEFAULT_CONFIG: GenieConfig = {
  style: "genie",
  intensity: "dramatic",
  soundEnabled: false,
  shadowEnabled: true,
  postFxEnabled: true,
};

export function getIntensityMultiplier(intensity: GenieIntensity): number {
  switch (intensity) {
    case "subtle": return 0.55;
    case "normal": return 1.0;
    case "dramatic": return 1.55;
  }
}
