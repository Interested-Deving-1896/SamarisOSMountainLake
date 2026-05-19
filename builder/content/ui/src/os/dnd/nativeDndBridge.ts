import type { ConflictStrategy, DragFilePayload } from "./types";

type NativeFileToken = {
  token: string;
  name: string;
  kind: "file" | "dir";
  size: number;
  mime?: string;
  lastModified?: number;
};

export const nativeDndBridge = {
  async resolveExternalFiles(files: File[]): Promise<DragFilePayload[]> {
    if (files.length === 0) return [];
    const api = window.electronAPI?.dnd;
    if (api?.resolveFiles) {
      const resolved = await api.resolveFiles(files);
      return resolved.map((entry: NativeFileToken) => ({
        name: entry.name,
        path: `host://${entry.token}`,
        kind: entry.kind,
        size: entry.size || 0,
        source: "host",
        token: entry.token,
        mime: entry.mime,
        lastModified: entry.lastModified
      }));
    }

    return files.map((file, index) => ({
      name: file.name || `Dropped file ${index + 1}`,
      path: `browser://${index}:${file.name}`,
      kind: "file",
      size: file.size || 0,
      source: "browser",
      token: `${index}:${file.name}`,
      mime: file.type,
      lastModified: file.lastModified
    }));
  },

  async importHostFiles(files: DragFilePayload[], destinationPath: string, conflictStrategy: ConflictStrategy) {
    const tokens = files.filter((file) => file.source === "host" && file.token).map((file) => file.token!) ;
    if (tokens.length === 0) return { ok: true, imported: [] as string[] };
    const api = window.electronAPI?.dnd;
    if (!api?.importHostFiles) throw new Error("Native host import is unavailable");
    return api.importHostFiles(tokens, destinationPath, { conflictStrategy });
  },

  async startNativeDrag(files: DragFilePayload[]) {
    const api = window.electronAPI?.dnd;
    if (!api?.startDragVirtualFiles) return { ok: false, error: "native_drag_unavailable" };
    return api.startDragVirtualFiles(files);
  }
};

