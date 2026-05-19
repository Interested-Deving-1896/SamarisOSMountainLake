import React from "react";
import { sampleWallpaperColor } from "./utils/sampleWallpaperColor";

export function useAdaptiveGlass(enabled: boolean | undefined) {
  return React.useMemo(() => {
    if (!enabled) return null;
    return sampleWallpaperColor();
  }, [enabled]);
}

