export function joinPath(basePath: string, entryName: string) {
  return `${basePath === "/" ? "" : basePath}/${entryName}`.replace(/\/+/g, "/") || "/";
}

export function splitPath(path: string) {
  const cleaned = path.replace(/^\/+|\/+$/g, "");
  return cleaned ? cleaned.split("/") : [];
}

export function looksTextLike(fileName: string) {
  return /\.(txt|md|json|js|ts|tsx|jsx|css|html|sh|env|yml|yaml|xml|toml|lock)$/i.test(fileName);
}

export function formatBytes(size?: number) {
  const value = size ?? 0;
  if (value < 1024) return `${value} B`;
  if (value < 1024 * 1024) return `${(value / 1024).toFixed(1)} KB`;
  if (value < 1024 * 1024 * 1024) return `${(value / (1024 * 1024)).toFixed(1)} MB`;
  return `${(value / (1024 * 1024 * 1024)).toFixed(1)} GB`;
}

export function formatDate(date?: number | string | null): string {
  if (!date) return "—";
  const d = new Date(date);
  const now = new Date();
  const diff = now.getTime() - d.getTime();
  if (diff < 86400000 && d.getDate() === now.getDate()) return d.toLocaleTimeString(undefined, { hour: "2-digit", minute: "2-digit" });
  if (diff < 604800000) return d.toLocaleDateString(undefined, { weekday: "short" });
  return d.toLocaleDateString(undefined, { month: "short", day: "numeric", year: "numeric" });
}

const FILE_KIND_MAP: Record<string, string> = {
  txt: "Text", md: "Markdown", json: "JSON", js: "JavaScript", ts: "TypeScript",
  tsx: "TypeScript", jsx: "React", css: "Stylesheet", html: "HTML", sh: "Shell",
  png: "Image", jpg: "Image", jpeg: "Image", gif: "Image", webp: "Image", svg: "Image",
  mp3: "Audio", wav: "Audio", m4a: "Audio", aac: "Audio", ogg: "Audio", flac: "Audio",
  mp4: "Video", webm: "Video", avi: "Video", mkv: "Video", mov: "Video",
  pdf: "PDF", doc: "Document", docx: "Document", xls: "Spreadsheet", xlsx: "Spreadsheet",
  zip: "Archive", tar: "Archive", gz: "Archive", rar: "Archive", "7z": "Archive",
  exe: "Executable", deb: "Package", dmg: "Package",
};

export function fileKind(fileName: string): string {
  const dot = fileName.lastIndexOf(".");
  if (dot === -1) return "File";
  const ext = fileName.slice(dot + 1).toLowerCase();
  return FILE_KIND_MAP[ext] || `${ext.toUpperCase()} File`;
}
