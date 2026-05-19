import React from "react";
import { RotateCcw, X, File, Folder } from "lucide-react";
import type { TrashEntry } from "../trashIndex";
import { startFileDrag } from "../../../os/filesystem/dragDrop";

function formatBytes(bytes: number): string {
  if (bytes === 0) return "0 B";
  const units = ["B", "KB", "MB", "GB", "TB"];
  const i = Math.min(Math.floor(Math.log(bytes) / Math.log(1024)), units.length - 1);
  return `${(bytes / Math.pow(1024, i)).toFixed(i === 0 ? 0 : 1)} ${units[i]}`;
}

function formatTime(iso: string): string {
  const date = new Date(iso);
  const now = new Date();
  const today = new Date(now.getFullYear(), now.getMonth(), now.getDate());
  const yesterday = new Date(today);
  yesterday.setDate(yesterday.getDate() - 1);

  if (date >= today) {
    return date.toLocaleTimeString(undefined, { hour: "numeric", minute: "2-digit" });
  }
  if (date >= yesterday) return "Yesterday";
  return date.toLocaleDateString(undefined, { month: "short", day: "numeric" });
}

export function TrashItem(props: {
  entry: TrashEntry;
  selected: boolean;
  onToggle: (name: string) => void;
  onRestore: (name: string) => void;
  onDelete: (name: string) => void;
  disabled: boolean;
}) {
  const { entry } = props;

  return (
    <div
      className={`trash__item${props.selected ? " trash__item--selected" : ""}`}
      role="listitem"
      draggable
      aria-selected={props.selected}
      onDragStart={(e) => {
        startFileDrag(e.dataTransfer, [{ name: entry.name, path: `/User/Trash/${entry.name}`.replace(/\/+/g, "/"), kind: entry.kind, size: entry.size || 0, originalPath: entry.originalPath }]);
      }}
    >
      <label className="trash__itemCheck">
        <input
          type="checkbox"
          className="trash__itemCheckbox"
          checked={props.selected}
          onChange={() => props.onToggle(entry.name)}
        />
      </label>

      <div className="trash__itemIcon">
        {entry.kind === "dir" ? (
          <Folder size={22} strokeWidth={1.6} />
        ) : (
          <File size={22} strokeWidth={1.6} />
        )}
      </div>

      <div className="trash__itemBody">
        <div className="trash__itemName">{entry.name}</div>
        <div className="trash__itemMeta">
          <span className="trash__itemOrigin" title={entry.originalPath}>
            {entry.originalPath}
          </span>
          <span className="trash__itemSep">&middot;</span>
          <span className="trash__itemSize">{formatBytes(entry.size)}</span>
          <span className="trash__itemSep">&middot;</span>
          <span className="trash__itemTime">{formatTime(entry.deletedAt)}</span>
        </div>
      </div>

      <div className="trash__itemActions">
        <button
          type="button"
          className="trash__itemAction trash__itemAction--restore"
          disabled={props.disabled}
          onClick={() => props.onRestore(entry.name)}
          title="Restore to original location"
        >
          <RotateCcw size={14} strokeWidth={2.2} />
        </button>
        <button
          type="button"
          className="trash__itemAction trash__itemAction--delete"
          disabled={props.disabled}
          onClick={() => props.onDelete(entry.name)}
          title="Delete permanently"
        >
          <X size={14} strokeWidth={2.2} />
        </button>
      </div>
    </div>
  );
}
