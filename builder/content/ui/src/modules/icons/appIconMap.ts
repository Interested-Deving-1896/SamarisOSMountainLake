import type { SamarisIconName } from "./types";

export const appIconMap = {
  finder: "computer",
  files: "computer",
  computer: "computer",
  mail: "mail",
  peregrine: "peregrine",
  browser: "peregrine",
  orbit: "orbit",
  photos: "photos",
  music: "music",
  videos: "videos",
  notes: "notes",
  games: "games",
  settings: "settings",
  trash: "trash",
  appstore: "appstore",
  utilities: "tools",
  tools: "tools",
  wine: "compat",
  network: "network",
  bench: "brain"
} as const;

const compatIconMap: Record<string, SamarisIconName> = {
  "app-store": "appstore",
  doom: "games",
  "task-manager": "tools",
  "disk-utility": "tools",
  "system-monitor": "tools",
  "pdf-viewer": "notes",
  "permissions-manager": "settings",
  firewall: "settings",
  encryption: "settings",
  print: "tools",
  terminal: "tools",
  textedit: "notes",
  downloads: "folder"
};

export function resolveAppIconName(appId: string): SamarisIconName {
  const key = appId.toLowerCase();
  if (key in appIconMap) return appIconMap[key as keyof typeof appIconMap];
  return compatIconMap[key] ?? "computer";
}
