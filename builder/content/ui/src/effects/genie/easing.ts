export function clamp(value: number, min: number, max: number) {
  return Math.max(min, Math.min(max, value));
}

export function easeOutExpo(t: number) {
  return t === 1 ? 1 : 1 - Math.pow(2, -10 * t);
}

export function easeInOutCubic(t: number) {
  return t < 0.5
    ? 4 * t * t * t
    : 1 - Math.pow(-2 * t + 2, 3) / 2;
}

export function mix(a: number, b: number, t: number) {
  return a + (b - a) * t;
}
