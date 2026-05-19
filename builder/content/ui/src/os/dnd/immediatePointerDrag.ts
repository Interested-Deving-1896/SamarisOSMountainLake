import type { DragFilePayload } from "./types";

export type ImmediateDropTarget =
  | { kind: "trash"; label: "Trash" }
  | { kind: "folder"; path: string; label: string }
  | null;

export function findImmediateDropTarget(clientX: number, clientY: number): ImmediateDropTarget {
  const element = document.elementFromPoint(clientX, clientY) as HTMLElement | null;
  if (!element) return null;

  if (element.closest("#dock-icon-trash")) {
    return { kind: "trash", label: "Trash" };
  }

  const folder = element.closest<HTMLElement>("[data-dnd-drop-path]");
  const path = folder?.dataset.dndDropPath;
  if (path) {
    return {
      kind: "folder",
      path,
      label: folder.dataset.dndDropLabel || path
    };
  }

  return null;
}

export function setImmediateTrashHover(active: boolean) {
  document.getElementById("dock-icon-trash")?.classList.toggle("dock__btn--trash-hover", active);
}

export function createPointerDragGhost(files: DragFilePayload[]) {
  const el = document.createElement("div");
  el.className = "dnd-pointer-ghost";
  el.style.cssText =
    "position:fixed;z-index:100000;left:0;top:0;transform:translate3d(-9999px,-9999px,0);" +
    "display:flex;align-items:center;gap:6px;max-width:180px;padding:5px 9px;border-radius:8px;" +
    "background:rgba(255,255,255,0.96);box-shadow:0 10px 28px rgba(20,32,52,0.22),0 0 0 1px rgba(20,32,52,0.08);" +
    "font:700 11px -apple-system,BlinkMacSystemFont,Segoe UI,sans-serif;color:#172033;pointer-events:none;";

  const badge = document.createElement("span");
  badge.style.cssText =
    "display:grid;place-items:center;min-width:18px;height:18px;border-radius:5px;background:#2563eb;color:white;font-size:10px;";
  badge.textContent = files.length > 1 ? String(files.length) : files[0]?.kind === "dir" ? "DIR" : "FILE";

  const label = document.createElement("span");
  label.style.cssText = "overflow:hidden;text-overflow:ellipsis;white-space:nowrap;";
  label.textContent = files.length > 1 ? `${files.length} items` : files[0]?.name || "Item";

  el.append(badge, label);
  document.body.appendChild(el);
  return el;
}

export function movePointerDragGhost(ghost: HTMLElement | null, clientX: number, clientY: number) {
  if (!ghost) return;
  ghost.style.transform = `translate3d(${clientX + 12}px, ${clientY + 12}px, 0)`;
}

export function removePointerDragGhost(ghost: HTMLElement | null) {
  if (ghost?.parentNode) ghost.remove();
}
