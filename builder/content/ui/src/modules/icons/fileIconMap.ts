import type { SamarisIconName } from "./types";

export function iconNameForFile(name: string, kind: "file" | "dir"): SamarisIconName {
  if (kind === "dir") return "folder";

  const lower = name.toLowerCase();
  if (lower.endsWith(".mp3") || lower.endsWith(".wav") || lower.endsWith(".flac") || lower.endsWith(".ogg")) return "music";
  if (lower.endsWith(".mp4") || lower.endsWith(".mov") || lower.endsWith(".webm") || lower.endsWith(".mkv") || lower.endsWith(".avi")) {
    return "videos";
  }
  if (lower.endsWith(".png") || lower.endsWith(".jpg") || lower.endsWith(".jpeg") || lower.endsWith(".webp") || lower.endsWith(".gif")) {
    return "photos";
  }
  if (lower.endsWith(".pdf")) return "notes";
  if (lower.endsWith(".md") || lower.endsWith(".txt") || lower.endsWith(".rtf") || lower.endsWith(".json")) return "notes";
  if (lower.endsWith(".jsdos") || lower.endsWith(".exe")) return "games";

  return "notes";
}

