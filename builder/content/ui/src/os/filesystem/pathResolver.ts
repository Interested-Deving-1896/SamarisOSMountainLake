export function resolvePath(input: string, homePath = "/") {
  const trimmed = input.trim();
  if (!trimmed) return "/";
  if (trimmed === "~") return homePath;
  if (trimmed.startsWith("~/")) return `${homePath}/${trimmed.slice(2)}`.replace(/\/+/g, "/");
  if (trimmed.startsWith("/")) return trimmed.replace(/\/+/g, "/");
  return `/${trimmed}`.replace(/\/+/g, "/");
}

export const resolveSystemPath = resolvePath;
