import type { OnboardingDraft } from "./onboarding.state";

const KEY = "samaris-os/onboarding-draft";

const EMPTY_DRAFT: OnboardingDraft = {
  fullName: "",
  username: "",
  password: "",
  confirmPassword: ""
};

export function loadOnboardingDraft(): OnboardingDraft {
  try {
    const raw = window.localStorage.getItem(KEY);
    if (!raw) return { ...EMPTY_DRAFT };
    const parsed = JSON.parse(raw) as Partial<OnboardingDraft>;
    return {
      fullName: String(parsed.fullName || ""),
      username: String(parsed.username || ""),
      password: "",
      confirmPassword: ""
    };
  } catch {
    return { ...EMPTY_DRAFT };
  }
}

export function saveOnboardingDraft(draft: OnboardingDraft) {
  window.localStorage.setItem(
    KEY,
    JSON.stringify({
      fullName: draft.fullName,
      username: draft.username
    })
  );
}

export function clearOnboardingDraft() {
  window.localStorage.removeItem(KEY);
}

