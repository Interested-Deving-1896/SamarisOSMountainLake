import type { FsService } from "../../services/fs/types";

export type DndEntityKind =
  | "file"
  | "directory"
  | "external-file"
  | "app"
  | "window"
  | "desktop-icon"
  | "dock-item";

export type DropChoice =
  | "copy"
  | "move"
  | "link"
  | "import"
  | "export"
  | "restore"
  | "trash"
  | "reorder"
  | "snap"
  | "open"
  | "cancel";

export type ConflictStrategy = "rename" | "replace" | "skip";

export type DndFileKind = "file" | "dir";

export type DragFilePayload = {
  name: string;
  path: string;
  kind: DndFileKind;
  size: number;
  source?: "samaris" | "host" | "browser";
  token?: string;
  mime?: string;
  lastModified?: number;
  originalPath?: string;
};

export type DndEntity = {
  id: string;
  kind: DndEntityKind;
  name: string;
  source: "samaris" | "host" | "browser";
  path?: string;
  token?: string;
  fileKind?: DndFileKind;
  size?: number;
  mime?: string;
  lastModified?: number;
  originalPath?: string;
  metadata?: Record<string, unknown>;
};

export type DndSource = {
  id: string;
  appId?: string;
  windowId?: string;
  entities: DndEntity[];
  allowedActions: DropChoice[];
  nativeExport?: boolean;
};

export type DropTarget = {
  id: string;
  label: string;
  path?: string;
  kind?: "folder" | "trash" | "app" | "dock" | "window" | "layout";
  accepts?: DndEntityKind[];
  priority?: number;
};

export type DropConflict = {
  itemName: string;
  destinationPath: string;
  existingKind?: DndFileKind;
};

export type DropPlan = {
  id: string;
  source: DndSource;
  target: DropTarget;
  recommendedAction: DropChoice;
  allowedChoices: DropChoice[];
  conflicts: DropConflict[];
  warnings: string[];
  estimatedCount: number;
  estimatedBytes: number;
};

export type DropDecision = {
  choice: Exclude<DropChoice, "cancel">;
  conflictStrategy: ConflictStrategy;
};

export type FileDropContext = {
  plan: DropPlan;
  decision: DropDecision;
  fs: FsService;
};

export type FileDropOptions = {
  accept?: string[];
  target?: DropTarget | (() => DropTarget);
  allowedChoices?: DropChoice[];
  recommendedAction?: DropChoice;
  requireConfirmation?: boolean;
  ignoreSourceIds?: string[] | ((sourceId: string) => boolean);
};

export type FileDropHandler = (
  files: DragFilePayload[],
  context: FileDropContext
) => void | Promise<void>;
