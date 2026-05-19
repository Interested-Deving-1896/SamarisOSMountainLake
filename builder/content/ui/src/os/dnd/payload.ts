import {
  MIME_PLAIN,
  MIME_SAMARIS_DND_V2,
  MIME_SAMARIS_LEGACY_FILES,
  MIME_URI_LIST
} from "./constants";
import type { DndEntity, DndSource, DragFilePayload } from "./types";
import { basename, normalizeVirtualPath } from "./path";

type SamarisDndPayloadV2 = {
  version: 2;
  source: DndSource;
};

let activeDndSource: DndSource | null = null;

export function setActiveDndSource(source: DndSource | null) {
  activeDndSource = source;
}

export function clearActiveDndSource(sourceId?: string) {
  if (!sourceId || activeDndSource?.id === sourceId) {
    activeDndSource = null;
  }
}

export function getActiveDndSource() {
  return activeDndSource;
}

function safeJson(input: string): unknown {
  try {
    return JSON.parse(input);
  } catch {
    return null;
  }
}

function normalizeFileKind(kind: unknown): "file" | "dir" {
  return kind === "dir" || kind === "directory" ? "dir" : "file";
}

export function filePayloadToEntity(file: DragFilePayload): DndEntity {
  const fileKind = normalizeFileKind(file.kind);
  return {
    id: `${file.source || "samaris"}:${file.token || file.path || file.name}`,
    kind: file.source === "host" || file.source === "browser" ? "external-file" : fileKind === "dir" ? "directory" : "file",
    name: file.name || basename(file.path || ""),
    source: file.source || "samaris",
    path: file.source === "host" || file.source === "browser" ? file.path : file.path ? normalizeVirtualPath(file.path) : undefined,
    token: file.token,
    fileKind,
    size: Number.isFinite(file.size) ? file.size : 0,
    mime: file.mime,
    lastModified: file.lastModified,
    originalPath: file.originalPath
  };
}

export function entityToFilePayload(entity: DndEntity): DragFilePayload | null {
  if (!["file", "directory", "external-file"].includes(entity.kind)) return null;
  return {
    name: entity.name,
    path: entity.path || (entity.token ? `host://${entity.token}` : entity.name),
    kind: entity.fileKind || (entity.kind === "directory" ? "dir" : "file"),
    size: entity.size || 0,
    source: entity.source,
    token: entity.token,
    mime: entity.mime,
    lastModified: entity.lastModified,
    originalPath: entity.originalPath
  };
}

function isValidEntity(input: unknown): input is DndEntity {
  const value = input as Partial<DndEntity>;
  return Boolean(
    value &&
    typeof value === "object" &&
    typeof value.name === "string" &&
    typeof value.kind === "string" &&
    (value.source === "samaris" || value.source === "host" || value.source === "browser")
  );
}

export function buildDndSource(files: DragFilePayload[], sourceId?: string): DndSource {
  const entities = files.map(filePayloadToEntity);
  return {
    id: sourceId || `dnd-${Date.now()}-${Math.random().toString(16).slice(2)}`,
    entities,
    allowedActions: ["copy", "move", "link", "import", "trash", "restore", "open"],
    nativeExport: true
  };
}

export function writeDndPayload(dataTransfer: DataTransfer, files: DragFilePayload[], sourceId?: string): DndSource {
  const source = buildDndSource(files, sourceId);
  const payload: SamarisDndPayloadV2 = { version: 2, source };
  setActiveDndSource(source);
  dataTransfer.setData(MIME_SAMARIS_DND_V2, JSON.stringify(payload));
  dataTransfer.setData(MIME_PLAIN, files.map((f) => f.path).join(", "));
  dataTransfer.setData(
    MIME_URI_LIST,
    files
      .filter((f) => (f.source || "samaris") === "samaris")
      .map((f) => `file://${normalizeVirtualPath(f.path)}`)
      .join("\n")
  );
  return source;
}

export function readDndSource(dataTransfer: DataTransfer): DndSource | null {
  const rawV2 = dataTransfer.getData(MIME_SAMARIS_DND_V2);
  if (rawV2) {
    const parsed = safeJson(rawV2) as Partial<SamarisDndPayloadV2> | null;
    if (parsed?.version === 2 && parsed.source && Array.isArray(parsed.source.entities)) {
      const entities = parsed.source.entities.filter(isValidEntity).map((entity) => ({
        ...entity,
        path: entity.source === "host" || entity.source === "browser" ? entity.path : entity.path ? normalizeVirtualPath(entity.path) : undefined,
        fileKind: normalizeFileKind(entity.fileKind)
      }));
      return {
        id: typeof parsed.source.id === "string" ? parsed.source.id : `dnd-${Date.now()}`,
        appId: parsed.source.appId,
        windowId: parsed.source.windowId,
        entities,
        allowedActions: Array.isArray(parsed.source.allowedActions) ? parsed.source.allowedActions : ["copy", "move"],
        nativeExport: Boolean(parsed.source.nativeExport)
      };
    }
  }

  if (activeDndSource) return null;

  const legacy = dataTransfer.getData(MIME_SAMARIS_LEGACY_FILES);
  if (legacy) {
    const parsed = safeJson(legacy);
    if (Array.isArray(parsed)) {
      const files = parsed
        .filter((item) => item && typeof item === "object" && typeof item.name === "string" && typeof item.path === "string")
        .map((item) => ({
          name: String(item.name),
          path: normalizeVirtualPath(String(item.path)),
          kind: normalizeFileKind(item.kind),
          size: Number(item.size || 0),
          source: "samaris" as const
        }));
      return buildDndSource(files);
    }
  }

  return null;
}

export function readDndFiles(dataTransfer: DataTransfer): DragFilePayload[] {
  const source = readDndSource(dataTransfer);
  if (!source) return [];
  return source.entities.map(entityToFilePayload).filter(Boolean) as DragFilePayload[];
}

export function hasSamarisFileDrop(dataTransfer: DataTransfer): boolean {
  if (dataTransfer.types.includes(MIME_SAMARIS_DND_V2)) return true;
  if (activeDndSource) return false;
  return dataTransfer.types.includes(MIME_SAMARIS_LEGACY_FILES);
}
