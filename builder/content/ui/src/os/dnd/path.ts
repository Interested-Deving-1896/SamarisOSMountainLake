export function normalizeVirtualPath(input: string): string {
  const trimmed = String(input || "").trim();
  if (!trimmed) return "/";
  const prefixed = trimmed.startsWith("/") ? trimmed : `/${trimmed}`;
  return prefixed.replace(/\\/g, "/").replace(/\/+/g, "/").replace(/\/+$/, "") || "/";
}

export function basename(path: string): string {
  const parts = normalizeVirtualPath(path).split("/").filter(Boolean);
  return parts[parts.length - 1] || path;
}

export function dirname(path: string): string {
  const parts = normalizeVirtualPath(path).split("/").filter(Boolean);
  if (parts.length <= 1) return "/";
  return `/${parts.slice(0, -1).join("/")}`;
}

export function joinPath(base: string, name: string): string {
  return `${normalizeVirtualPath(base).replace(/\/+$/, "")}/${String(name).replace(/^\/+/, "")}`.replace(/\/+/g, "/");
}

export function extensionOf(name: string): string {
  const dot = name.lastIndexOf(".");
  return dot >= 0 ? name.slice(dot).toLowerCase() : "";
}

export function matchesAccept(name: string, accept?: string[]): boolean {
  if (!accept || accept.length === 0) return true;
  const lower = name.toLowerCase();
  return accept.some((entry) => {
    const token = entry.toLowerCase().trim();
    if (!token) return false;
    if (token.startsWith(".")) return lower.endsWith(token);
    if (token.endsWith("/*")) return false;
    return lower === token || lower.endsWith(`.${token}`);
  });
}

