import type { SamarisIconGlyph, SamarisIconName, SamarisIconPreset } from "./types";
import {
  Monitor,
  Mail,
  Music,
  Clapperboard,
  Image,
  Orbit,
  Bird,
  Settings,
  Trash2,
  Wrench,
  Gamepad2,
  Notebook,
  Store,
  Brain,
  Folder,
  Wine,
  Network
} from "lucide-react";

export const samarisIconPresets: Record<SamarisIconName, SamarisIconPreset> = {
  computer: { tint: 212, saturation: 88, lightness: 66, strength: 0.52 },
  mail: { tint: 205, saturation: 88, lightness: 66, strength: 0.48 },
  music: { tint: 286, saturation: 78, lightness: 68, strength: 0.5 },
  videos: { tint: 198, saturation: 90, lightness: 64, strength: 0.5 },
  brain: { tint: 260, saturation: 76, lightness: 70, strength: 0.52 },
  peregrine: { tint: 215, saturation: 78, lightness: 65, strength: 0.48 },
  settings: { tint: 240, saturation: 78, lightness: 66, strength: 0.5 },
  photos: { tint: 188, saturation: 88, lightness: 68, strength: 0.48 },
  games: { tint: 235, saturation: 82, lightness: 65, strength: 0.5 },
  trash: { tint: 0, saturation: 82, lightness: 65, strength: 0.48 },
  tools: { tint: 210, saturation: 68, lightness: 66, strength: 0.42 },
  network: { tint: 230, saturation: 82, lightness: 62, strength: 0.48 },
  orbit: { tint: 222, saturation: 94, lightness: 66, strength: 0.54 },
  notes: { tint: 34, saturation: 78, lightness: 68, strength: 0.44 },
  appstore: { tint: 268, saturation: 82, lightness: 66, strength: 0.5 },
  folder: { tint: 212, saturation: 88, lightness: 66, strength: 0.4 },
  compat: { tint: 220, saturation: 72, lightness: 66, strength: 0.44 }
};

export const samarisIconColors: Record<SamarisIconName, string> = {
  computer: "#3B82F6",
  mail: "#0EA5E9",
  peregrine: "#14B8A6",
  orbit: "#6366F1",
  photos: "#10B981",
  music: "#A855F7",
  videos: "#06B6D4",
  appstore: "#9333EA",
  notes: "#F59E0B",
  games: "#3B5BDB",
  tools: "#64748B",
  settings: "#6366F1",
  trash: "#EF4444",
  brain: "#6366F1",
  folder: "#3B82F6",
  compat: "#2563EB",
  network: "#3B82F6"
};

export const iconRegistry = {
  computer: Monitor,
  mail: Mail,
  music: Music,
  videos: Clapperboard,
  photos: Image,
  orbit: Orbit,
  peregrine: Bird,
  settings: Settings,
  trash: Trash2,
  tools: Wrench,
  games: Gamepad2,
  notes: Notebook,
  appstore: Store,
  brain: Brain,
  folder: Folder,
  compat: Wine,
  network: Network
} as const satisfies Record<SamarisIconName, SamarisIconGlyph>;
