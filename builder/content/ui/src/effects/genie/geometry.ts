import type { Rect } from "./types";

export function getElementRect(element: HTMLElement): Rect {
  const rect = element.getBoundingClientRect();

  return {
    x: rect.left,
    y: rect.top,
    width: rect.width,
    height: rect.height,
  };
}

export function getDockTargetRect(dockRect: Rect): Rect {
  const size = Math.min(dockRect.width, dockRect.height);

  return {
    x: dockRect.x + dockRect.width / 2 - size / 2,
    y: dockRect.y + dockRect.height / 2 - size / 2,
    width: size,
    height: size,
  };
}
