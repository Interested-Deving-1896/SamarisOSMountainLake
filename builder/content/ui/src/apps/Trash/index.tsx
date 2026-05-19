import React from "react";
import { ConfirmModal } from "../../components/PromptModal";
import { useTrashController } from "./hooks/useTrashController";
import { TrashHeader } from "./components/TrashHeader";
import { TrashGroup } from "./components/TrashGroup";
import { TrashEmptyState } from "./components/TrashEmptyState";
import { TrashProgressOverlay } from "./components/TrashProgressOverlay";
import { useFileDrop } from "../shared/useFileDrop";
import { useFs } from "../../services/fs/useFs";
import { commitFileDrop } from "../../os/dnd";
import "./trash.css";

function formatTotalSize(bytes: number): string {
  if (bytes === 0) return "";
  const units = ["B", "KB", "MB", "GB", "TB"];
  const i = Math.min(Math.floor(Math.log(bytes) / Math.log(1024)), units.length - 1);
  return `${(bytes / Math.pow(1024, i)).toFixed(i === 0 ? 0 : 1)} ${units[i]}`;
}

export default function Trash(props: { windowId: string }) {
  const ctrl = useTrashController();
  const fs = useFs();
  const [confirmAction, setConfirmAction] = React.useState<"empty" | "secure" | null>(null);
  const trashDrop = useFileDrop({
    target: { id: "trash", label: "Trash", path: "/User/Trash", kind: "trash" },
    allowedChoices: ["trash"],
    recommendedAction: "trash",
    onDrop: async (_files, context) => {
      await commitFileDrop(fs, context.plan, context.decision);
      ctrl.refresh();
    }
  });

  return (
    <div className={`trash${trashDrop.isDragging ? " trash--drop-target" : ""}`}
      {...trashDrop.dragProps}>
      {ctrl.loading ? (
        <div className="trash__loading" />
      ) : ctrl.entries.length === 0 ? (
        <TrashEmptyState />
      ) : (
        <>
          <TrashHeader
            itemCount={ctrl.entries.length}
            totalSize={formatTotalSize(ctrl.totalSize)}
            selectedCount={ctrl.selected.size}
            onRestoreSelected={() => void ctrl.restoreSelected()}
            onEmptyTrash={() => setConfirmAction("empty")}
            onSecureEmpty={() => setConfirmAction("secure")}
            searchQuery={ctrl.searchQuery}
            onSearchChange={ctrl.setSearchQuery}
            hasSelection={ctrl.selected.size > 0}
            emitting={ctrl.emitting}
          />

          <div className="trash__body">
            {ctrl.groupedEntries.map((group) => (
              <TrashGroup
                key={group.group}
                label={group.label}
                entries={group.entries}
                selected={ctrl.selected}
                onToggle={ctrl.toggleSelect}
                onRestore={ctrl.restoreSingle}
                onDelete={ctrl.deleteSingle}
                disabled={ctrl.emitting}
              />
            ))}
          </div>

          <div className="trash__status">
            <span>
              {ctrl.entries.length}{" "}
              {ctrl.entries.length === 1 ? "item" : "items"}
              {ctrl.totalSize > 0 ? ` \u00B7 ${formatTotalSize(ctrl.totalSize)}` : ""}
            </span>
            {ctrl.selected.size > 0 ? (
              <span>
                {ctrl.selected.size} selected
              </span>
            ) : null}
          </div>
        </>
      )}

      {confirmAction === "empty" ? (
        <ConfirmModal
          title="Empty Trash?"
          subtitle={
            ctrl.entries.length > 0
              ? `This permanently removes all ${ctrl.entries.length} item${ctrl.entries.length === 1 ? "" : "s"} (${formatTotalSize(ctrl.totalSize)}) from Trash.`
              : undefined
          }
          confirmLabel="Empty"
          danger
          onCancel={() => setConfirmAction(null)}
          onConfirm={() => {
            setConfirmAction(null);
            void ctrl.emptyTrash(false);
          }}
        />
      ) : null}

      {confirmAction === "secure" ? (
        <ConfirmModal
          title="Secure Empty Trash?"
          subtitle={
            ctrl.entries.length > 0
              ? `This securely overwrites and permanently removes all ${ctrl.entries.length} item${ctrl.entries.length === 1 ? "" : "s"} (${formatTotalSize(ctrl.totalSize)}). This cannot be undone.`
              : undefined
          }
          confirmLabel="Secure Empty"
          danger
          onCancel={() => setConfirmAction(null)}
          onConfirm={() => {
            setConfirmAction(null);
            void ctrl.emptyTrash(true);
          }}
        />
      ) : null}

      {ctrl.progress ? (
        <TrashProgressOverlay total={ctrl.progress.total} current={ctrl.progress.current} />
      ) : null}
    </div>
  );
}
