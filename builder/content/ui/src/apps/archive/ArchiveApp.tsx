import React, { useSyncExternalStore } from "react";
import { ArchiveSidebar } from "./components/ArchiveSidebar";
import { ArchiveList } from "./components/ArchiveList";
import { ArchiveActions } from "./components/ArchiveActions";
import { useArchive } from "./hooks/useArchive";
import { osStore } from "../../os/core/osStore";
import { useFileDrop } from "../shared/useFileDrop";
import { commitFileDrop } from "../../os/dnd";

export function ArchiveApp(props: { windowId: string }) {
  const state = useSyncExternalStore(
    (listener) => osStore.subscribe(listener),
    () => osStore.getState()
  );
  const archivePath = state.windows.find((w) => w.id === props.windowId)?.params?.path as string | undefined;
  const { entries, loading, extracting, error, successNotice, archiveName, doExtract, destination } = useArchive(archivePath);

  const archiveDrop = useFileDrop({
    accept: [".zip", ".tar", ".gz", ".tgz", ".bz2", ".7z", ".rar"],
    target: { id: "archive-open", label: "Archive", path: "/User/Downloads", kind: "app" },
    allowedChoices: ["open", "import", "copy"],
    recommendedAction: "open",
    onDrop: async (files, context) => {
      if (files.length === 0) return;
      const appLoader = (await import("../../os/apps/appLoader")).appLoader;
      if (files[0].source === "host") {
        const result = await commitFileDrop(context.fs, context.plan, { ...context.decision, choice: "copy" });
        const path = result.completed[0];
        if (path) void appLoader.openApp("archive", { windowParams: { path } });
        return;
      }
      void appLoader.openApp("archive", { windowParams: { path: files[0].path } });
    }
  });

  if (!archivePath) {
    return (
      <div className="archive">
        <div className="archive__empty">No archive selected.</div>
      </div>
    );
  }

  return (
    <div className={`archive${archiveDrop.isDragging ? " archive--drop-target" : ""}`}
      {...archiveDrop.dragProps}>
      <ArchiveSidebar name={archiveName} destination={destination} />
      <div className="archive__main">
        <ArchiveActions
          archivePath={archivePath}
          destination={destination}
          onExtract={doExtract}
          extracting={extracting}
        />
        <ArchiveList entries={entries} loading={loading} error={error} successNotice={successNotice} />
      </div>
    </div>
  );
}
