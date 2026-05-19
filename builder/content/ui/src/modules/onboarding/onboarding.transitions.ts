export const ONBOARDING_MOTION_MS = {
  enter: 280,
  exit: 220
} as const;

export function onboardingPhaseClass(entering: boolean) {
  return entering ? "samaris-onboarding--entering" : "samaris-onboarding--ready";
}

