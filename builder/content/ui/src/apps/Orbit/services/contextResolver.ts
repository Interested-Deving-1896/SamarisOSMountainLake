import type { OrbitModeId } from "../types";

export function suggestModeFromApp(appId: string | undefined): OrbitModeId {
  switch (appId) {
    case "textedit":
    case "settings":
      return "smart";
    default:
      return "fast";
  }
}
