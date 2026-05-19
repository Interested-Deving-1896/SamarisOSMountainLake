import { getIntensityMultiplier, type GenieIntensity } from "./GenieSettings";

export type PostFxResult = {
  canvasOpacity: number;
  canvasFilter: string;
  backdropOpacity: number;
  backdropBlur: string;
};

export function computePostFx(
  progress: number,
  velocity: number,
  direction: 1 | -1,
  intensity: GenieIntensity
): PostFxResult {
  const mult = getIntensityMultiplier(intensity);
  const p = direction === 1 ? progress : 1 - progress;

  const chroma = Math.min(velocity * 1.8 * mult, 3);
  const blur = Math.min(velocity * 2.5 * mult, 6);

  let canvasOpacity = 1;
  if (p > 0.94) {
    canvasOpacity = 1 - Math.pow((p - 0.94) / 0.06, 3);
  }

  return {
    canvasOpacity: canvasOpacity,
    canvasFilter: `blur(${blur.toFixed(1)}px) ${chroma > 0.3 ? "url(#genie-chroma)" : ""}`,
    backdropOpacity: Math.min(p * 0.22 * mult, 0.22),
    backdropBlur: `blur(${Math.floor(p * 4 * mult)}px)`,
  };
}
