import type { OnboardingState, OnboardingStep } from "../../services/kernel/onboarding";

export type OnboardingDraft = {
  fullName: string;
  username: string;
  password: string;
  confirmPassword: string;
};

export type SetupStatus =
  | "idle"
  | "preparing"
  | "creating-account"
  | "securing-storage"
  | "finalizing"
  | "done"
  | "error";

export type OnboardingViewModel = {
  loaded: boolean;
  state: OnboardingState | null;
  activeStep: OnboardingStep;
  draft: OnboardingDraft;
  setupStatus: SetupStatus;
  setupMessage: string;
  error: string;
  entering: boolean;
};

export const ONBOARDING_STEP_ORDER: OnboardingStep[] = [
  "welcome",
  "intro",
  "license",
  "account",
  "final"
];

export function nextStep(step: OnboardingStep) {
  const index = ONBOARDING_STEP_ORDER.indexOf(step);
  return ONBOARDING_STEP_ORDER[Math.min(index + 1, ONBOARDING_STEP_ORDER.length - 1)];
}

export function previousStep(step: OnboardingStep) {
  const index = ONBOARDING_STEP_ORDER.indexOf(step);
  return ONBOARDING_STEP_ORDER[Math.max(index - 1, 0)];
}
