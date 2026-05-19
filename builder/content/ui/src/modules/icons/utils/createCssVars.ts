import type React from "react";

type CSSVars = { [key: `--${string}`]: string | number };

export function createCssVars(vars: Record<string, string | number | undefined>): CSSVars {
  const style: CSSVars = {};
  for (const [key, value] of Object.entries(vars)) {
    if (value === undefined) continue;
    if (key.startsWith("--")) {
      (style as Record<string, string | number>)[key] = value;
    }
  }
  return style;
}

