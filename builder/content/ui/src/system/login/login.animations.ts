export type LoginPhase = "LOCKED" | "ENTERING_PASSWORD" | "AUTHENTICATING" | "UNLOCKING" | "UNLOCKED";

export const LOGIN_ANIMATION_MS = {
  fadeIn: 520,
  unlock: 860,
  feedback: 260,
  wallpaperBreath: 24000,
  lightDrift: 30000
} as const;

export function phaseClass(phase: LoginPhase) {
  switch (phase) {
    case "ENTERING_PASSWORD":
      return "is-entering";
    case "AUTHENTICATING":
      return "is-authenticating";
    case "UNLOCKING":
      return "is-unlocking";
    case "UNLOCKED":
      return "is-unlocked";
    default:
      return "is-locked";
  }
}
