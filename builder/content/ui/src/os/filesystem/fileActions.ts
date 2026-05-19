import type { FsService } from "../../services/fs/types";
import { TrashIndex } from "../../apps/Trash/trashIndex";
import { appLoader } from "../apps/appLoader";
import { systemClipboard } from "./clipboard";

const CHUNK_SIZE = 65536;

function splitPath(path: string) {
  return path.replace(/\/+/g, "/").split("/").filter(Boolean);
}

function basename(path: string) {
  const parts = splitPath(path);
  return parts[parts.length - 1] || path;
}

function dirname(path: string) {
  const parts = splitPath(path);
  if (parts.length <= 1) return "/";
  return `/${parts.slice(0, -1).join("/")}`;
}

function joinPath(base: string, name: string) {
  return `${base.replace(/\/+$/, "")}/${name.replace(/^\/+/, "")}`.replace(/\/+/g, "/");
}

async function exists(fs: FsService, targetPath: string) {
  const parent = dirname(targetPath);
  const name = basename(targetPath);
  const result = await fs.list(parent);
  return result.nodes.some((node) => node.name === name);
}

export async function uniquePath(fs: FsService, targetPath: string) {
  let pathExists = false;
  try {
    pathExists = await exists(fs, targetPath);
  } catch {
    return targetPath;
  }

  if (!pathExists) {
    return targetPath;
  }

  const name = basename(targetPath);
  const parent = dirname(targetPath);
  const dotIndex = name.lastIndexOf(".");
  const stem = dotIndex > 0 ? name.slice(0, dotIndex) : name;
  const ext = dotIndex > 0 ? name.slice(dotIndex) : "";

  for (let index = 2; index < 1000; index += 1) {
    const candidate = joinPath(parent, `${stem} ${index}${ext}`);
    try {
      if (!(await exists(fs, candidate))) {
        return candidate;
      }
    } catch {
      return candidate;
    }
  }

  return joinPath(parent, `${stem}-${Date.now()}${ext}`);
}

export async function moveToTrash(fs: FsService, targetPath: string) {
  const destination = await uniquePath(fs, joinPath("/User/Trash", basename(targetPath)));
  const originalPath = targetPath;
  const nodeName = basename(targetPath);

  let size = 0;
  let kind: "file" | "dir" = "file";
  try {
    const parent = dirname(targetPath);
    const list = await fs.list(parent);
    const node = list.nodes.find((n) => n.name === nodeName);
    if (node) {
      size = node.size || 0;
      kind = node.kind;
    }
  } catch {}

  await fs.rename(targetPath, destination);

  try {
    const index = new TrashIndex(fs);
    await index.add(basename(destination), originalPath, size, kind);
  } catch {}

  return destination;
}

export async function restoreFromTrash(fs: FsService, targetPath: string, destinationFolder?: string) {
  const nodeName = basename(targetPath);
  let originalPath = destinationFolder || "/User/Desktop";

  try {
    const index = new TrashIndex(fs);
    const entry = await index.get(nodeName);
    if (entry) {
      originalPath = entry.originalPath;
      await index.remove(nodeName);
    }
  } catch {}

  const destination = await uniquePath(fs, joinPath(originalPath, nodeName));
  await fs.rename(targetPath, destination);
  return destination;
}

export async function permanentlyDelete(fs: FsService, targetPath: string, secure = false) {
  const nodeName = basename(targetPath);

  if (secure) {
    try {
      const result = await fs.read(targetPath);
      const contentLength = result.content.length;
      const zeros = "\0".repeat(CHUNK_SIZE);
      let written = 0;
      while (written < contentLength) {
        const chunk = zeros.slice(0, Math.min(CHUNK_SIZE, contentLength - written));
        await fs.write(targetPath, chunk);
        written += chunk.length;
      }
    } catch {}
  }

  let kind: "file" | "dir" = "file";
  try {
    const parent = dirname(targetPath);
    const list = await fs.list(parent);
    const node = list.nodes.find((n) => n.name === nodeName);
    if (node) kind = node.kind;
  } catch {}

  await fs.delete(targetPath, { recursive: kind === "dir" });

  try {
    const index = new TrashIndex(fs);
    await index.remove(nodeName);
  } catch {}
}

export async function secureEmptyTrash(fs: FsService) {
  const nodes = await fs.list("/User/Trash");
  const entries = nodes.nodes.filter((n) => n.name !== ".trash_index.json");
  for (const node of entries) {
    await permanentlyDelete(fs, `/User/Trash/${node.name}`, true);
  }
  try {
    const index = new TrashIndex(fs);
    await index.clear();
  } catch {}
}

export async function sendTo(fs: FsService, sourcePath: string, destinationFolder: string) {
  const destination = await uniquePath(fs, joinPath(destinationFolder, basename(sourcePath)));
  await fs.rename(sourcePath, destination);
  return destination;
}

export async function pasteFromClipboard(fs: FsService, destinationFolder: string) {
  const item = systemClipboard.readFiles();
  if (!item) return false;
  const destination = await uniquePath(fs, joinPath(destinationFolder, basename(item.path)));
  if (item.action === "move") {
    await fs.rename(item.path, destination);
    systemClipboard.clearFiles();
  } else {
    await fs.copy(item.path, destination);
  }
  return true;
}

export async function openPathInApp(targetPath: string, kind: "dir" | "file") {
  if (kind === "dir") {
    await appLoader.openApp("finder", { windowParams: { path: targetPath } });
    return;
  }

  const lower = targetPath.toLowerCase();
  if (lower.endsWith(".md")) {
    await appLoader.openApp("notes", { windowParams: { path: targetPath } });
    return;
  }
  if (lower.endsWith(".txt")) {
    await appLoader.openApp("textedit", { windowParams: { path: targetPath } });
    return;
  }
  if (lower.match(/\.(png|jpg|jpeg|webp|gif|svg|bmp|tiff|tif|heic|heif)$/)) {
    await appLoader.openApp("photos", { windowParams: { path: targetPath } });
    return;
  }
  if (lower.match(/\.(mp3|wav|m4a|aac|ogg|flac)$/)) {
    await appLoader.openApp("music", { windowParams: { path: targetPath } });
    return;
  }
  if (lower.match(/\.(mp4|webm|avi|mkv|mov|m4v|ogv|3gp|wmv|flv)$/)) {
    await appLoader.openApp("videos", { windowParams: { path: targetPath } });
    return;
  }
  if (lower.match(/\.(zip|tar|gz|tgz|bz2|7z)$/)) {
    await appLoader.openApp("archive", { windowParams: { path: targetPath } });
    return;
  }
  if (lower.endsWith(".pdf")) {
    await appLoader.openApp("pdf-viewer", { windowParams: { path: targetPath } });
    return;
  }
  if (lower.endsWith(".exe")) {
    await appLoader.openApp("wine", { windowParams: { path: targetPath, autoLaunch: true } });
    return;
  }

  await appLoader.openApp("finder", { windowParams: { path: dirname(targetPath) } });
}
