import { getElementRect } from "./geometry";

export type ShadowConfig = {
  offsetX: number;
  offsetY: number;
  blurRadius: number;
  alpha: number;
};

export function computeShadow(
  sourceRect: { x: number; y: number; width: number; height: number },
  progress: number,
  direction: 1 | -1
): ShadowConfig {
  const p = direction === 1 ? progress : 1 - progress;
  const strength = Math.pow(p, 2.5);

  return {
    offsetX: 0,
    offsetY: 8 * (1 - p) + 4,
    blurRadius: 24 * strength + 8,
    alpha: 0.35 * strength,
  };
}
