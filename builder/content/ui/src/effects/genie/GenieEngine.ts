export type GeniePhysics = {
  progress: number;
  velocity: number;
  overshoot: number;
};

function distance(a: number, b: number, c: number, d: number) {
  return Math.sqrt((a - c) ** 2 + (b - d) ** 2);
}

export function createPhysics(
  elapsed: number,
  srcW: number, srcH: number,
  dockW: number, dockH: number,
  direction: 1 | -1
): GeniePhysics {
  const dist = distance(0, 0, 0, srcH + dockH * 0.5);

  let omega: number;
  let zeta: number;
  let duration: number;

  if (dist < 200) {
    omega = 8; zeta = 0.40; duration = 600;
  } else if (dist < 500) {
    omega = 6.5; zeta = 0.35; duration = 780;
  } else {
    omega = 5; zeta = 0.30; duration = 900;
  }

  const t = Math.max(0, Math.min(1, elapsed / duration));

  let progress: number;
  if (t >= 1) {
    progress = 1;
  } else {
    const wd = omega * Math.sqrt(1 - zeta * zeta);
    const decay = Math.exp(-zeta * omega * t);
    const sinusoid = Math.sin(wd * t);
    progress = 1 - decay * (Math.cos(wd * t) + (zeta / Math.sqrt(1 - zeta * zeta)) * sinusoid);
    progress = Math.max(0, Math.min(1, progress));
  }

  let velocity = 0;
  if (t < 0.98) {
    const dt = 0.001;
    const t2 = Math.min(t + dt, 1);
    const wd = omega * Math.sqrt(1 - zeta * zeta);
    const p2 = 1 - Math.exp(-zeta * omega * t2) * Math.cos(wd * t2);
    const p1 = 1 - Math.exp(-zeta * omega * t) * Math.cos(wd * t);
    velocity = Math.abs(p2 - p1) / dt / omega * 0.5;
  }

  let overshoot = 0;
  if (t > 0.9) {
    overshoot = Math.max(0, progress - 1) * 15;
  }

  if (direction === -1) {
    velocity *= 1.3;
  }

  return { progress, velocity, overshoot };
}
