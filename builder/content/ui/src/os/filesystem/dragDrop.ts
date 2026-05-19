import type { DragFilePayload } from "../dnd";
import { clearActiveDndSource, hasSamarisFileDrop, nativeDndBridge, readDndFiles, writeDndPayload } from "../dnd";

export type { DragFilePayload } from "../dnd";

export function startFileDrag(
  dataTransfer: DataTransfer,
  files: DragFilePayload[],
  opts?: { ghost?: "visible" | "hidden"; nativeExport?: boolean; sourceId?: string }
): void {
  dataTransfer.effectAllowed = "copyMove";
  const source = writeDndPayload(dataTransfer, files, opts?.sourceId);

  const cleanupSource = () => clearActiveDndSource(source.id);
  document.addEventListener("dragend", cleanupSource, { once: true });
  window.setTimeout(cleanupSource, 5000);

  if (opts?.ghost === "hidden") {
    const blank = new Image();
    blank.src = "data:image/gif;base64,R0lGODlhAQABAIAAAAAAAP///yH5BAEAAAAALAAAAAABAAEAAAIBRAA7";
    dataTransfer.setDragImage(blank, -9999, -9999);
  } else {
    const img = buildDragImage(files);
    if (img) {
      dataTransfer.setDragImage(img, 2, 2);
      const rm = () => { if (img.parentNode) img.remove(); };
      const onEnd = () => { rm(); document.removeEventListener("dragend", onEnd); };
      document.addEventListener("dragend", onEnd);
      setTimeout(() => { document.removeEventListener("dragend", onEnd); rm(); }, 5000);
    }
  }
  if (opts?.nativeExport) {
    void nativeDndBridge.startNativeDrag(files).catch(() => {});
  }
}

export function getDroppedFiles(
  dt: DataTransfer
): DragFilePayload[] {
  return readDndFiles(dt);
}

export function hasFileDrop(dt: DataTransfer): boolean {
  if (hasSamarisFileDrop(dt)) return true;
  return dt.types.includes("Files");
}

function buildDragImage(files: DragFilePayload[]): HTMLElement | null {
  if (files.length === 0) return null;

  const el = document.createElement("div");
  el.style.cssText =
    "display:flex;align-items:center;gap:4px;padding:3px 7px 3px 4px;" +
    "border-radius:7px;" +
    "background:rgba(255,255,255,0.97);" +
    "font-family:-apple-system,BlinkMacSystemFont,sans-serif;" +
    "font-size:10.5px;font-weight:600;color:#1a1423;" +
    "box-shadow:0 2px 8px rgba(0,0,0,0.1), 0 0 0 0.5px rgba(0,0,0,0.07);" +
    "pointer-events:none;white-space:nowrap;line-height:1;";

  if (files.length === 1) {
    const f = files[0];
    const dot = document.createElement("span");
    dot.style.cssText =
      `width:7px;height:7px;border-radius:50%;flex-shrink:0;background:${f.kind === "dir" ? "#3b82f6" : "#9ca3af"};`;
    el.appendChild(dot);

    const label = document.createElement("span");
    label.style.cssText = "overflow:hidden;text-overflow:ellipsis;max-width:110px;";
    label.textContent = f.name;
    el.appendChild(label);
  } else {
    const count = document.createElement("span");
    count.style.cssText =
      "display:grid;place-items:center;min-width:16px;height:16px;border-radius:4px;" +
      "padding:0 3px;background:#3b82f6;color:#fff;flex-shrink:0;" +
      "font-size:9px;font-weight:800;line-height:1;";
    count.textContent = `${files.length}`;
    el.appendChild(count);

    const label = document.createElement("span");
    label.style.cssText = "color:#6b7280;font-weight:500;";
    label.textContent = `items`;
    el.appendChild(label);
  }

  document.body.appendChild(el);
  return el;
}
