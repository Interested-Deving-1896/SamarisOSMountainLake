// Prototype supports wallpaper sampling. In this codebase we keep it lightweight and offline-safe.
// If sampling becomes necessary later, implement a non-blocking sampling strategy.

export type SampledColor = { r: number; g: number; b: number; x: string; y: string };

export function sampleWallpaperColor(): SampledColor {
  // Mountain Lake preset (fallback).
  return { r: 108, g: 171, b: 232, x: "72%", y: "12%" };
}

