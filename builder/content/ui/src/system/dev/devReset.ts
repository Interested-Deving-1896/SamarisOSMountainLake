const DEV_RESET_STORAGE_TOKEN_KEY = "samaris-os/dev-reset-token";
const RESETTABLE_STORAGE_KEYS = [
  "samaris-os/onboarding-draft",
  "samaris-os/session",
  "samaris-os/restore-enabled",
  "samaris-os/window-preferences"
];

type DevResetState = {
  token?: string | null;
};

export async function applyDevResetIfNeeded() {
  try {
    const response = await fetch("http://127.0.0.1:9999/api/dev/reset-state", {
      cache: "no-store"
    });
    if (!response.ok) return false;

    const data = (await response.json()) as DevResetState;
    const token = typeof data.token === "string" ? data.token.trim() : "";
    if (!token) return false;

    const current = window.localStorage.getItem(DEV_RESET_STORAGE_TOKEN_KEY);
    if (current === token) return false;

    for (const key of RESETTABLE_STORAGE_KEYS) {
      window.localStorage.removeItem(key);
    }
    window.localStorage.setItem(DEV_RESET_STORAGE_TOKEN_KEY, token);
    return true;
  } catch {
    return false;
  }
}
