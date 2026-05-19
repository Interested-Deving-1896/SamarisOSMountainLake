export type DesktopIconPosition = {
  x: number;
  y: number;
};

const KEY = "samaris-os/desktop-layout";
const CELL_W = 110;
const CELL_H = 108;

export function loadDesktopLayout() {
  const raw = window.localStorage.getItem(KEY);
  if (!raw) return {} as Record<string, DesktopIconPosition>;
  try {
    return JSON.parse(raw) as Record<string, DesktopIconPosition>;
  } catch {
    return {};
  }
}

export function saveDesktopLayout(layout: Record<string, DesktopIconPosition>) {
  window.localStorage.setItem(KEY, JSON.stringify(layout));
}

export function defaultDesktopPosition(index: number) {
  const column = Math.floor(index / 7);
  const row = index % 7;
  return {
    x: 18 + column * CELL_W,
    y: 18 + row * CELL_H
  };
}

export function normalizeDesktopLayout(names: string[]) {
  const current = loadDesktopLayout();
  const next: Record<string, DesktopIconPosition> = {};
  names.forEach((name, index) => {
    next[name] = current[name] || defaultDesktopPosition(index);
  });
  saveDesktopLayout(next);
  return next;
}

export function resetDesktopLayout(names: string[]) {
  const next: Record<string, DesktopIconPosition> = {};
  names.forEach((name, index) => {
    next[name] = defaultDesktopPosition(index);
  });
  saveDesktopLayout(next);
  return next;
}
