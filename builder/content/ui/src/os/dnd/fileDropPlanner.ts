import type { FsService } from "../../services/fs/types";
import { moveToTrash, restoreFromTrash, uniquePath } from "../filesystem/fileActions";
import { basename, dirname, joinPath, matchesAccept } from "./path";
import { nativeDndBridge } from "./nativeDndBridge";
import type {
  ConflictStrategy,
  DndEntity,
  DndSource,
  DragFilePayload,
  DropChoice,
  DropDecision,
  DropPlan,
  DropTarget
} from "./types";
import { buildDndSource, entityToFilePayload, filePayloadToEntity } from "./payload";

function actionLabel(choice: DropChoice): string {
  return choice;
}

function allowedForTarget(target: DropTarget, requested?: DropChoice[]): DropChoice[] {
  if (requested?.length) return requested;
  if (target.kind === "trash") return ["trash"];
  if (target.kind === "app") return ["open", "import", "copy"];
  return ["copy", "move", "link"];
}

function recommendedForTarget(target: DropTarget, source: DndSource, requested?: DropChoice): DropChoice {
  if (requested && requested !== "cancel") return requested;
  if (target.kind === "trash") return "trash";
  if (target.kind === "app") return "import";
  if (source.entities.some((entity) => entity.source === "host" || entity.source === "browser")) return "import";
  if (target.path?.startsWith("/User/Trash")) return "restore";
  return "move";
}

async function exists(fs: FsService, targetPath: string) {
  try {
    const list = await fs.list(dirname(targetPath));
    const name = basename(targetPath);
    return list.nodes.find((node) => node.name === name) || null;
  } catch {
    return null;
  }
}

export function filterAcceptedFiles(files: DragFilePayload[], accept?: string[]) {
  return files.filter((file) => matchesAccept(file.name, accept));
}

export async function buildFileDropPlan(
  fs: FsService,
  files: DragFilePayload[],
  target: DropTarget,
  options?: {
    allowedChoices?: DropChoice[];
    recommendedAction?: DropChoice;
    sourceId?: string;
  }
): Promise<DropPlan> {
  const source = buildDndSource(files, options?.sourceId);
  const conflicts = [];
  if (target.path && target.kind !== "trash") {
    for (const file of files) {
      const destinationPath = joinPath(target.path, file.name);
      const existing = await exists(fs, destinationPath);
      if (existing) {
        conflicts.push({ itemName: file.name, destinationPath, existingKind: existing.kind });
      }
    }
  }
  const fromTrash = files.some((file) => file.path.startsWith("/User/Trash/"));
  const baseAllowed = allowedForTarget(target, options?.allowedChoices).filter((choice) => choice !== "cancel") as DropChoice[];
  const allowedChoices: DropChoice[] = fromTrash && target.kind === "folder" && !baseAllowed.includes("restore")
    ? ["restore", ...baseAllowed]
    : baseAllowed;
  const recommendedAction: DropChoice = fromTrash && target.kind === "folder"
    ? "restore"
    : recommendedForTarget(target, source, options?.recommendedAction);
  return {
    id: `plan-${Date.now()}-${Math.random().toString(16).slice(2)}`,
    source,
    target,
    allowedChoices,
    recommendedAction: allowedChoices.includes(recommendedAction) ? recommendedAction : allowedChoices[0] || "copy",
    conflicts,
    warnings: files.length === 0 ? ["No compatible items were dropped."] : [],
    estimatedCount: files.length,
    estimatedBytes: files.reduce((sum, file) => sum + (file.size || 0), 0)
  };
}

async function destinationFor(
  fs: FsService,
  destinationFolder: string,
  file: DragFilePayload,
  strategy: ConflictStrategy
) {
  const destination = joinPath(destinationFolder, file.name);
  const existing = await exists(fs, destination);
  if (!existing) return destination;
  if (strategy === "skip") return null;
  if (strategy === "replace") {
    await fs.remove(destination, { recursive: existing.kind === "dir" });
    return destination;
  }
  return uniquePath(fs, destination);
}

export async function commitFileDrop(
  fs: FsService,
  plan: DropPlan,
  decision: DropDecision
): Promise<{ ok: boolean; completed: string[]; skipped: string[]; failed: Array<{ name: string; error: string }> }> {
  const files = plan.source.entities.map(entityToFilePayload).filter(Boolean) as DragFilePayload[];
  const completed: string[] = [];
  const skipped: string[] = [];
  const failed: Array<{ name: string; error: string }> = [];

  if (decision.choice === "trash") {
    for (const file of files) {
      try {
        if (file.path.startsWith("/User/Trash/")) { skipped.push(file.name); continue; }
        completed.push(await moveToTrash(fs, file.path));
      } catch (error) {
        failed.push({ name: file.name, error: error instanceof Error ? error.message : "Move to Trash failed" });
      }
    }
    return { ok: failed.length === 0, completed, skipped, failed };
  }

  if (!plan.target.path) {
    return { ok: true, completed, skipped, failed };
  }

  const hostFiles = files.filter((file) => file.source === "host");
  const internalFiles = files.filter((file) => (file.source || "samaris") === "samaris");

  if (hostFiles.length > 0) {
    try {
      const result = await nativeDndBridge.importHostFiles(hostFiles, plan.target.path, decision.conflictStrategy);
      completed.push(...((result as { imported?: string[] }).imported || []));
    } catch (error) {
      for (const file of hostFiles) {
        failed.push({ name: file.name, error: error instanceof Error ? error.message : "Host import failed" });
      }
    }
  }

  for (const file of internalFiles) {
    try {
      if (decision.choice === "restore") {
        completed.push(await restoreFromTrash(fs, file.path, plan.target.path));
        continue;
      }
      const destination = await destinationFor(fs, plan.target.path, file, decision.conflictStrategy);
      if (!destination) {
        skipped.push(file.name);
        continue;
      }
      if (decision.choice === "move") {
        await fs.rename(file.path, destination);
      } else if (decision.choice === "link") {
        await fs.write(`${destination}.samaris-link`, JSON.stringify({ target: file.path, name: file.name }, null, 2));
      } else {
        await fs.copy(file.path, destination);
      }
      completed.push(destination);
    } catch (error) {
      failed.push({ name: file.name, error: error instanceof Error ? error.message : `${actionLabel(decision.choice)} failed` });
    }
  }

  return { ok: failed.length === 0, completed, skipped, failed };
}

export function filesToEntities(files: DragFilePayload[]): DndEntity[] {
  return files.map(filePayloadToEntity);
}
