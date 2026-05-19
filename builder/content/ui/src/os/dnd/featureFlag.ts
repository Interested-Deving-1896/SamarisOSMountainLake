import { DND_FEATURE_FLAG } from "./constants";

const SETTINGS_KEY = "samaris-os/settings-prefs";

export function isDndV2Enabled(): boolean {
  try {
    const prefs = JSON.parse(window.localStorage.getItem(SETTINGS_KEY) || "{}") as {
      featureFlags?: Record<string, boolean>;
    };
    return prefs.featureFlags?.[DND_FEATURE_FLAG] !== false;
  } catch {
    return true;
  }
}
