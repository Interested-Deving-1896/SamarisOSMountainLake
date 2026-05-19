import React from "react";
import { ClipboardPaste, Copy, FilePlus2, FolderOpen, FolderPlus, Info, Pencil, Play, Send, Trash2 } from "lucide-react";
import "./finder.css";
import type { ContextMenuItem } from "../../components/ContextMenu";
import { ContextMenu } from "../../components/ContextMenu";
import { ConfirmModal, InfoModal, PromptModal } from "../../components/PromptModal";
import type { FinderSection } from "./model";
import { FinderSidebar } from "./components/FinderSidebar";
import { FinderToolbar } from "./components/FinderToolbar";
import { FinderInspector } from "./components/FinderInspector";
import { FinderStatusBar } from "./components/FinderStatusBar";
import { systemClipboard } from "../../os/filesystem/clipboard";
import { formatBytes, formatDate, fileKind, joinPath } from "./utils";
import { SamarisIcon, iconNameForFile } from "../../modules/icons";
import { getDroppedFiles, hasFileDrop } from "../../os/filesystem/dragDrop";
import { useFileDrop } from "../../apps/shared/useFileDrop";
import {
  buildFileDropPlan,
  commitFileDrop,
  createPointerDragGhost,
  findImmediateDropTarget,
  movePointerDragGhost,
  nativeDndBridge,
  removePointerDragGhost,
  setImmediateTrashHover,
  useDnd
} from "../../os/dnd";
import type { DragFilePayload } from "../../os/dnd";
import { useFinderController } from "./hooks/useFinderController";
import { storageKernel, type StorageDevice } from "../../services/kernel/storage";
import { fileSystemClient } from "../../os/filesystem/fileSystemClient";
import { kernelClient } from "../../os/kernel/kernelClient";

const BASE_SIDEBAR_SECTIONS: FinderSection[] = [
  {
    title: "FAVORITES",
    items: [
      { id: "desktop", label: "Desktop", path: "/User/Desktop", icon: "computer" },
      { id: "documents", label: "Documents", path: "/User/Documents", icon: "folder" },
      { id: "downloads", label: "Downloads", path: "/User/Downloads", icon: "folder" },
      { id: "photos", label: "Photos", path: "/User/Photos", icon: "photos" },
      { id: "music", label: "Music", path: "/User/Music", icon: "music" },
      { id: "videos", label: "Videos", path: "/User/Videos", icon: "videos" },
    ]
  },
  {
    title: "LOCATIONS",
    items: [{ id: "disk-main", label: "Samaris Drive", path: "/User", icon: "tools" }]
  },
];

export function FinderWindow(props: {
  windowId: string;
  defaultPath?: string;
  chrome?: React.ReactNode;
}) {
  const finder = useFinderController(props.windowId, props.defaultPath || "/User/Desktop");
  const dnd = useDnd();
  const suppressClickRef = React.useRef(false);
  const [storageDevices, setStorageDevices] = React.useState<StorageDevice[]>([]);
  const [storageMessage, setStorageMessage] = React.useState("");
  const activeIsExe = Boolean(finder.activeName && finder.activeName.toLowerCase().endsWith(".exe"));

  const finderDrop = useFileDrop({
    target: () => ({ id: `finder:${finder.pathStr}`, label: finder.pathStr, path: finder.pathStr, kind: "folder" }),
    allowedChoices: ["copy", "move", "link", "import"],
    recommendedAction: "move",
    ignoreSourceIds: (sourceId) => sourceId === `finder:${finder.pathStr}`,
    onDrop: async (_files, context) => {
      if (!finder.pathStr) return;
      await commitFileDrop(finder.fs, context.plan, context.decision);
      await finder.refresh();
    }
  });

  const dropFilesIntoPath = React.useCallback(
    async (files: DragFilePayload[], targetPath: string, label: string) => {
      if (files.length === 0) return;
      if (files.some((file) => file.path === targetPath || targetPath.startsWith(`${file.path}/`))) {
        finder.setDropTargetName(null);
        return;
      }
      const plan = await buildFileDropPlan(
        finder.fs,
        files,
        { id: `finder-folder:${targetPath}`, label, path: targetPath, kind: "folder" },
        { allowedChoices: ["copy", "move", "link", "import"], recommendedAction: "move" }
      );
      const decision = await dnd.requestFileDrop(plan);
      if (!decision) return;
      await commitFileDrop(finder.fs, plan, decision);
      finder.setDropTargetName(null);
      await finder.refresh();
    },
    [dnd, finder]
  );

  const handleDropIntoPath = React.useCallback(
    async (event: React.DragEvent, targetPath: string, label: string) => {
      event.preventDefault();
      event.stopPropagation();
      const internalFiles = getDroppedFiles(event.dataTransfer);
      const externalFiles = internalFiles.length > 0
        ? []
        : await nativeDndBridge.resolveExternalFiles(Array.from(event.dataTransfer.files || []));
      const files = [...internalFiles, ...externalFiles];
      await dropFilesIntoPath(files, targetPath, label);
    },
    [dropFilesIntoPath]
  );

  const handleImmediateFilePointerDown = React.useCallback(
    (item: import("../../services/fs/types").FsNode, event: React.PointerEvent<HTMLElement>) => {
      if (event.button !== 0) return;
      const target = event.target as HTMLElement | null;
      if (target?.closest("input, textarea, button[data-no-file-drag='true']")) return;
      const file: DragFilePayload = {
        name: item.name,
        path: joinPath(finder.pathStr, item.name),
        kind: item.kind,
        size: item.size || 0
      };
      const startX = event.clientX;
      const startY = event.clientY;
      let moved = false;
      let ghost: HTMLElement | null = null;

      const cleanup = () => {
        removePointerDragGhost(ghost);
        setImmediateTrashHover(false);
        finder.setDropTargetName(null);
        window.removeEventListener("pointermove", handleMove);
        window.removeEventListener("pointerup", handleUp);
        window.removeEventListener("pointercancel", handleCancel);
      };

      const handleMove = (moveEvent: PointerEvent) => {
        const dx = moveEvent.clientX - startX;
        const dy = moveEvent.clientY - startY;
        if (!moved && dx === 0 && dy === 0) return;
        moveEvent.preventDefault();
        if (!moved) {
          moved = true;
          suppressClickRef.current = true;
          finder.selectOne(item.name);
          ghost = createPointerDragGhost([file]);
        }
        movePointerDragGhost(ghost, moveEvent.clientX, moveEvent.clientY);
        const dropTarget = findImmediateDropTarget(moveEvent.clientX, moveEvent.clientY);
        setImmediateTrashHover(dropTarget?.kind === "trash");
        finder.setDropTargetName(dropTarget?.kind === "folder" ? dropTarget.label : null);
      };

      const handleUp = (upEvent: PointerEvent) => {
        const dropTarget = moved ? findImmediateDropTarget(upEvent.clientX, upEvent.clientY) : null;
        cleanup();
        if (!moved || !dropTarget) return;
        if (dropTarget.kind === "trash") {
          void buildFileDropPlan(
            finder.fs,
            [file],
            { id: "dock-trash", label: "Trash", path: "/User/Trash", kind: "trash" },
            { allowedChoices: ["trash"], recommendedAction: "trash" }
          )
            .then((plan) => commitFileDrop(finder.fs, plan, { choice: "trash", conflictStrategy: "rename" }))
            .then(() => finder.refresh())
            .catch(finder.reportOpError);
          return;
        }
        void dropFilesIntoPath([file], dropTarget.path, dropTarget.label).catch(finder.reportOpError);
      };

      const handleCancel = () => {
        cleanup();
      };

      window.addEventListener("pointermove", handleMove);
      window.addEventListener("pointerup", handleUp, { once: true });
      window.addEventListener("pointercancel", handleCancel, { once: true });
    },
    [dropFilesIntoPath, finder]
  );

  React.useEffect(() => {
    const currentPath = finder.pathStr;
    void fileSystemClient.watch(currentPath).catch(() => {});
    const unsubscribe = kernelClient.on<{ root?: string; path?: string }>("fs.watch.event", (event) => {
      if (event?.root === currentPath || event?.path?.startsWith(`${currentPath}/`)) {
        void finder.refresh(currentPath);
      }
    });
    return () => {
      unsubscribe();
      void fileSystemClient.unwatch(currentPath).catch(() => {});
    };
  }, [finder.pathStr, finder.refresh]);

  React.useEffect(() => {
    let cancelled = false;
    const syncDevices = async () => {
      try { const status = await storageKernel.status(); if (!cancelled) setStorageDevices(status.devices || []); }
      catch { if (!cancelled) setStorageDevices([]); }
    };
    void syncDevices();
    const intervalId = window.setInterval(() => { void syncDevices(); }, 12000);
    return () => { cancelled = true; window.clearInterval(intervalId); };
  }, []);

  const sidebarSections = React.useMemo<FinderSection[]>(() => {
    const mountedDevices = storageDevices.map((device) => ({
      id: `disk-${device.id}`,
      label: device.label,
      path: device.mounted ? `/Volumes/${device.id}` : finder.pathStr,
      icon: "tools" as const,
      hint: device.mounted
        ? `${device.filesystem.toUpperCase()}${device.size ? ` • ${device.size}` : ""}`
        : `Unmounted${device.filesystem ? ` • ${device.filesystem.toUpperCase()}` : ""}`,
      disabled: false,
      devicePath: device.path,
      mounted: device.mounted,
      ejectable: true
    }));
    return [
      ...BASE_SIDEBAR_SECTIONS.slice(0, 1),
      { title: "LOCATIONS", items: [{ id: "disk-main", label: "Samaris Drive", path: "/User", icon: "tools" }, ...mountedDevices] },
    ];
  }, [finder.pathStr, storageDevices]);

  const handleMountDevice = React.useCallback(async (devicePath: string) => {
    const result = await storageKernel.mount(devicePath);
    setStorageMessage(result.message || "");
    if (result.devices) setStorageDevices(result.devices);
  }, []);
  const handleUnmountDevice = React.useCallback(async (devicePath: string) => {
    const result = await storageKernel.unmount(devicePath);
    setStorageMessage(result.message || "");
    if (result.devices) setStorageDevices(result.devices);
  }, []);

  const selectedList = React.useMemo(() => [...finder.selected], [finder.selected]);
  const selCount = selectedList.length;
  const selName = selCount === 1 ? selectedList[0] : null;

  return (
    <div className="finder">
      <FinderToolbar
        canGoBack={finder.historyIndex > 0}
        canGoForward={finder.historyIndex < finder.history.length - 1}
        onBack={() => { if (finder.historyIndex > 0) { const ni = finder.historyIndex - 1; finder.setHistoryIndex(ni); finder.applyPath(finder.history[ni]); } }}
        onForward={() => { if (finder.historyIndex < finder.history.length - 1) { const ni = finder.historyIndex + 1; finder.setHistoryIndex(ni); finder.applyPath(finder.history[ni]); } }}
        onRefresh={() => void finder.refresh()}
        crumbs={finder.crumbs}
        onOpenCrumb={(targetPath) => finder.goTo(targetPath)}
        searchQuery={finder.query}
        onSearchQueryChange={finder.setQuery}
        viewMode={finder.viewMode}
        onChangeViewMode={finder.setViewMode}
        canDeleteSelected={selCount > 0}
        selectedCount={selCount}
        onDeleteSelected={() => { for (const n of selectedList) void finder.moveToTrash(joinPath(finder.pathStr, n)).then(() => finder.refresh()).catch(finder.reportOpError); }}
      />

      <div className={`finder-body${finderDrop.isDragging ? " finder-body--drop-target" : ""}`}
        data-dnd-drop-path={finder.pathStr}
        data-dnd-drop-label={finder.pathStr}
        {...finderDrop.dragProps}
        onPointerDown={() => { finder.clearSelection(); finder.setMenu(null); }}
        onContextMenu={(event) => { event.preventDefault(); finder.openMenuAt(event, null); }}>
        <FinderSidebar
          sections={sidebarSections}
          currentPath={finder.pathStr}
          onSelect={(targetPath) => finder.goTo(targetPath)}
          onMountDevice={handleMountDevice}
          onUnmountDevice={handleUnmountDevice}
        />

        <section className="finder-main" onKeyDown={finder.handleKeyDown} tabIndex={-1}>
          {finder.query.trim() ? (
            <>
              <div className="finder-main__head finder-main__head--search">
                <span className="finder-main__col" style={{ flex: 0.9 }}>Name</span>
                <span className="finder-main__col" style={{ flex: 1.3 }}>Path</span>
                <span className="finder-main__col" style={{ width: 120 }}>Updated</span>
              </div>
              <div className="finder-main__list" role="list">
                {finder.searching ? <div className="finder-main__empty">Searching across Samaris files…</div> : null}
                {!finder.searching && finder.searchError ? <div className="finder-main__empty">Search unavailable: {finder.searchError}</div> : null}
                {!finder.searching && !finder.searchError && finder.globalSearchResults.length === 0 ? <div className="finder-main__empty">No files match that search.</div> : null}
                {finder.globalSearchResults.map((result) => (
                  <button key={result.id} type="button" className="finder-row finder-row--search"
                    onClick={() => void finder.revealSearchResult(result)}
                    onDoubleClick={() => void finder.revealSearchResult(result)}>
                    <div className="finder-row__name">
                      <span className="finder-row__kindBadge">{result.kind === "dir" ? "Folder" : "File"}</span>
                      <span className="finder-row__nameText">{result.title}</span>
                    </div>
                    <span className="finder-row__path" title={result.path}>{result.path}</span>
                    <span className="finder-row__meta">
                      {result.modifiedAt ? new Date(result.modifiedAt).toLocaleDateString() : result.kind === "file" ? formatBytes(result.size || undefined) : "—"}
                    </span>
                  </button>
                ))}
              </div>
            </>
          ) : (
            <>
              {finder.viewMode === "list" ? (
                <div className="finder-main__head">
                  <button className={`finder-main__col finder-main__col--sortable ${finder.sortField === "name" ? "finder-main__col--active" : ""}`} onClick={() => finder.toggleSort("name")} aria-sort={finder.sortField === "name" ? (finder.sortOrder === "asc" ? "ascending" : "descending") : "none"}>
                    Name {finder.sortField === "name" ? (finder.sortOrder === "asc" ? "▲" : "▼") : ""}
                  </button>
                  <button className={`finder-main__col finder-main__col--sortable ${finder.sortField === "kind" ? "finder-main__col--active" : ""}`} onClick={() => finder.toggleSort("kind")} aria-sort={finder.sortField === "kind" ? (finder.sortOrder === "asc" ? "ascending" : "descending") : "none"} style={{ width: 100 }}>
                    Kind {finder.sortField === "kind" ? (finder.sortOrder === "asc" ? "▲" : "▼") : ""}
                  </button>
                  <button className={`finder-main__col finder-main__col--sortable ${finder.sortField === "size" ? "finder-main__col--active" : ""}`} onClick={() => finder.toggleSort("size")} aria-sort={finder.sortField === "size" ? (finder.sortOrder === "asc" ? "ascending" : "descending") : "none"} style={{ width: 80 }}>
                    Size {finder.sortField === "size" ? (finder.sortOrder === "asc" ? "▲" : "▼") : ""}
                  </button>
                  <button className={`finder-main__col finder-main__col--sortable ${finder.sortField === "date" ? "finder-main__col--active" : ""}`} onClick={() => finder.toggleSort("date")} aria-sort={finder.sortField === "date" ? (finder.sortOrder === "asc" ? "ascending" : "descending") : "none"} style={{ width: 90 }}>
                    Date {finder.sortField === "date" ? (finder.sortOrder === "asc" ? "▲" : "▼") : ""}
                  </button>
                </div>
              ) : null}

              {finder.loading ? <div className="finder-main__empty">Loading folders…</div> : null}
              {!finder.loading && finder.error ? <div className="finder-main__empty finder-main__empty--error">Filesystem unavailable: {finder.error}</div> : null}
              {!finder.loading && finder.operationError ? <div className="finder-main__empty finder-main__empty--error">{finder.operationError}</div> : null}
              {!finder.loading && !finder.error && !finder.operationError && finder.filteredItems.length === 0 ? (
                <div className="finder-main__empty">This folder is empty.</div>
              ) : null}

              {!finder.loading && !finder.error && finder.filteredItems.length > 0 && finder.viewMode === "list" ? (
                <div className="finder-main__list finder-main__list--list" role="list">
                  {finder.filteredItems.map((item) => {
                    const sel = finder.selected.has(item.name);
                    return (
                      <button key={item.name} type="button"
                        className={`finder-row${sel ? " finder-row--selected" : ""}`}
                        data-dnd-drop-path={item.kind === "dir" ? joinPath(finder.pathStr, item.name) : undefined}
                        data-dnd-drop-label={item.kind === "dir" ? item.name : undefined}
                        data-drop-target={item.kind === "dir" && finder.dropTargetName === item.name ? "true" : "false"}
                        aria-selected={sel}
                        role="listitem"
                        onPointerDown={(e) => handleImmediateFilePointerDown(item, e)}
                        onDragOver={(e) => {
                          if (item.kind !== "dir" || !hasFileDrop(e.dataTransfer)) return;
                          e.preventDefault();
                          e.stopPropagation();
                          e.dataTransfer.dropEffect = "move";
                          finder.setDropTargetName(item.name);
                        }}
                        onDragEnd={() => { finder.setDropTargetName(null); }}
                        onDragLeave={() => { if (finder.dropTargetName === item.name) finder.setDropTargetName(null); }}
                        onDrop={(e) => { if (item.kind !== "dir") return; void handleDropIntoPath(e, joinPath(finder.pathStr, item.name), item.name); }}
                        onClick={(e) => {
                          e.stopPropagation();
                          if (suppressClickRef.current) { suppressClickRef.current = false; return; }
                          finder.setMenu(null);
                          finder.toggleSelect(item.name, e.metaKey, e.shiftKey);
                        }}
                        onDoubleClick={() => void finder.openNode(item)}
                        onContextMenu={(e) => { e.preventDefault(); e.stopPropagation(); if (!sel) finder.selectOne(item.name); finder.openMenuAt(e, item.name); }}>
                        <div className="finder-row__name">
                          <SamarisIcon className="finder-row__icon" name={iconNameForFile(item.name, item.kind)} size={22} variant="soft" />
                          {finder.inlineRename === item.name ? (
                            <input autoFocus className="finder-row__renameInput"
                              defaultValue={item.name}
                              onBlur={(e) => finder.confirmInlineRename(item.name, e.target.value)}
                              onKeyDown={(e) => { if (e.key === "Enter") finder.confirmInlineRename(item.name, e.currentTarget.value); if (e.key === "Escape") finder.setInlineRename(null); }}
                              onClick={(e) => e.stopPropagation()}
                            />
                          ) : (
                            <span className="finder-row__nameText">{item.name}</span>
                          )}
                        </div>
                        <span className="finder-row__col" style={{ width: 100 }}>{item.kind === "dir" ? "Folder" : fileKind(item.name)}</span>
                        <span className="finder-row__col" style={{ width: 80 }}>{item.kind === "file" ? formatBytes(item.size) : "—"}</span>
                        <span className="finder-row__col" style={{ width: 90 }}>{formatDate(item.modifiedAt)}</span>
                      </button>
                    );
                  })}
                </div>
              ) : null}

              {!finder.loading && !finder.error && finder.filteredItems.length > 0 && finder.viewMode === "grid" ? (
                <div className="finder-main__list finder-main__list--grid" role="list">
                  {finder.filteredItems.map((item) => {
                    const sel = finder.selected.has(item.name);
                    return (
                      <button key={item.name} type="button"
                        className={`finder-row finder-row--grid${sel ? " finder-row--selected" : ""}`}
                        data-dnd-drop-path={item.kind === "dir" ? joinPath(finder.pathStr, item.name) : undefined}
                        data-dnd-drop-label={item.kind === "dir" ? item.name : undefined}
                        aria-selected={sel} role="listitem"
                        onPointerDown={(e) => handleImmediateFilePointerDown(item, e)}
                        onDragOver={(e) => {
                          if (item.kind !== "dir" || !hasFileDrop(e.dataTransfer)) return;
                          e.preventDefault();
                          e.stopPropagation();
                          e.dataTransfer.dropEffect = "move";
                          finder.setDropTargetName(item.name);
                        }}
                        onDragEnd={() => { finder.setDropTargetName(null); }}
                        onDragLeave={() => { if (finder.dropTargetName === item.name) finder.setDropTargetName(null); }}
                        onDrop={(e) => { if (item.kind !== "dir") return; void handleDropIntoPath(e, joinPath(finder.pathStr, item.name), item.name); }}
                        onClick={(e) => {
                          e.stopPropagation();
                          if (suppressClickRef.current) { suppressClickRef.current = false; return; }
                          finder.setMenu(null);
                          finder.toggleSelect(item.name, e.metaKey, e.shiftKey);
                        }}
                        onDoubleClick={() => void finder.openNode(item)}
                        onContextMenu={(e) => { e.preventDefault(); e.stopPropagation(); if (!sel) finder.selectOne(item.name); finder.openMenuAt(e, item.name); }}>
                        <SamarisIcon className="finder-row__icon" name={iconNameForFile(item.name, item.kind)} size={48} variant="soft" />
                        {finder.inlineRename === item.name ? (
                          <input autoFocus className="finder-row__renameInput"
                            defaultValue={item.name}
                            onBlur={(e) => finder.confirmInlineRename(item.name, e.target.value)}
                            onKeyDown={(e) => { if (e.key === "Enter") finder.confirmInlineRename(item.name, e.currentTarget.value); if (e.key === "Escape") finder.setInlineRename(null); }}
                            onClick={(e) => e.stopPropagation()}
                          />
                        ) : (
                          <span className="finder-row__nameText">{item.name}</span>
                        )}
                        <span className="finder-row__gridMeta">{item.kind === "file" ? formatBytes(item.size) : "Folder"} · {formatDate(item.modifiedAt)}</span>
                      </button>
                    );
                  })}
                </div>
              ) : null}

              {!finder.loading && !finder.error && finder.filteredItems.length > 0 && finder.viewMode === "columns" ? (
                <ColumnView pathStr={finder.pathStr} items={finder.filteredItems} selected={finder.selected}
                  onNavigate={(targetPath) => finder.goTo(targetPath)}
                  onSelect={(name, meta, shift) => finder.toggleSelect(name, meta, shift)}
	                  onOpen={(node) => void finder.openNode(node)}
                  onDropIntoFolder={handleDropIntoPath}
                  onPointerFileDragStart={handleImmediateFilePointerDown}
                  consumeSuppressedClick={() => {
                    if (!suppressClickRef.current) return false;
                    suppressClickRef.current = false;
                    return true;
                  }}
	                />
              ) : null}
            </>
          )}

          <FinderStatusBar itemCount={finder.filteredItems.length} selectedCount={selCount} />
        </section>

        {finder.inspectorOpen ? (
          <FinderInspector
            selectedNode={finder.selectedNode}
            preview={finder.preview}
            previewLoading={finder.previewLoading}
            pathStr={finder.pathStr}
            selectionCount={selCount}
            onRename={(name) => finder.setInlineRename(name)}
            onDelete={(name) => {
              void finder.moveToTrash(joinPath(finder.pathStr, name)).then(() => { finder.clearSelection(); void finder.refresh(); }).catch(finder.reportOpError);
            }}
            onSendToDocuments={(path) => { void finder.sendTo(path, "/User/Documents").then(() => finder.refresh()).catch(finder.reportOpError); }}
          />
        ) : null}
      </div>

      {finder.menu ? (
        <ContextMenu x={finder.menu.x} y={finder.menu.y} ariaLabel="Finder menu" onClose={() => finder.setMenu(null)}
          items={
            [
              { id: "new-folder", label: "New Folder", icon: FolderPlus, onSelect: () => { const name = "New Folder"; void finder.fs.mkdir(joinPath(finder.pathStr, name)).then(() => finder.refresh()).catch(finder.reportOpError); } },
              { id: "new-text", label: "New Text File", icon: FilePlus2, onSelect: () => { const name = "untitled.txt"; void finder.fs.write(joinPath(finder.pathStr, name), "").then(() => finder.refresh()).catch(finder.reportOpError); } },
              { id: "open", label: "Open", icon: FolderOpen, disabled: !finder.activeName, onSelect: async () => { if (finder.activeName && finder.activeNode) await finder.openNode(finder.activeNode); } },
              ...(activeIsExe ? [{ id: "open-wine", label: "Open with Wine", icon: Play, onSelect: async () => { if (finder.activeName) await finder.openNode({ name: finder.activeName, kind: "file" }); } }] as ContextMenuItem[] : []),
              { id: "rename", label: "Rename", icon: Pencil, disabled: !finder.activeName, onSelect: () => { if (finder.activeName) finder.setInlineRename(finder.activeName); } },
              { id: "copy", label: "Copy", icon: Copy, disabled: !finder.activeName, onSelect: () => { if (finder.activeName) systemClipboard.writeFiles({ path: joinPath(finder.pathStr, finder.activeName), action: "copy" }); } },
              { id: "paste", label: "Paste", icon: ClipboardPaste, disabled: !finder.clipboard, onSelect: () => { void finder.pasteFromClipboard(finder.pathStr).then(() => finder.refresh()).catch(finder.reportOpError); } },
              { id: "send-documents", label: "Move to Documents", icon: Send, disabled: !finder.activeName, onSelect: () => { if (finder.activeName) void finder.sendTo(joinPath(finder.pathStr, finder.activeName), "/User/Documents").then(() => finder.refresh()).catch(finder.reportOpError); } },
              { id: "properties", label: "Get Info", icon: Info, disabled: !finder.activeName, onSelect: () => { finder.setInspectorOpen(true); } },
              { id: "delete", label: finder.pathStr === "/User/Trash" ? "Restore from Trash" : "Move to Trash", icon: Trash2, danger: true, disabled: !finder.activeName,
                onSelect: () => {
                  if (!finder.activeName) return;
                  if (finder.pathStr === "/User/Trash") { void finder.restoreFromTrash(joinPath(finder.pathStr, finder.activeName)).then(() => finder.refresh()).catch(finder.reportOpError); return; }
                  void finder.moveToTrash(joinPath(finder.pathStr, finder.activeName)).then(() => { finder.clearSelection(); void finder.refresh(); }).catch(finder.reportOpError);
                }
              },
            ] as ContextMenuItem[]
          }
        />
      ) : null}

      {storageMessage ? (
        <div className="finder-toast">{storageMessage}</div>
      ) : null}
    </div>
  );
}

function ColumnView(props: {
  pathStr: string;
  items: import("../../services/fs/types").FsNode[];
  selected: Set<string>;
  onNavigate: (path: string) => void;
  onSelect: (name: string, meta: boolean, shift: boolean) => void;
  onOpen: (node: import("../../services/fs/types").FsNode) => void;
  onDropIntoFolder: (event: React.DragEvent, targetPath: string, label: string) => void;
  onPointerFileDragStart: (node: import("../../services/fs/types").FsNode, event: React.PointerEvent<HTMLElement>) => void;
  consumeSuppressedClick: () => boolean;
}) {
  const folders = props.items.filter((n) => n.kind === "dir");
  const files = props.items.filter((n) => n.kind === "file");

  return (
    <div className="finder-columns">
      <div className="finder-columns__col">
        <div className="finder-columns__path">{props.pathStr}</div>
        {folders.map((item) => {
          const sel = props.selected.has(item.name);
          return (
            <button key={item.name} type="button"
              className={`finder-columns__item${sel ? " finder-columns__item--selected" : ""}`}
              data-dnd-drop-path={joinPath(props.pathStr, item.name)}
              data-dnd-drop-label={item.name}
              onPointerDown={(e) => props.onPointerFileDragStart(item, e)}
              onDragOver={(e) => {
                if (!hasFileDrop(e.dataTransfer)) return;
                e.preventDefault();
                e.stopPropagation();
                e.dataTransfer.dropEffect = "move";
              }}
              onDrop={(e) => props.onDropIntoFolder(e, joinPath(props.pathStr, item.name), item.name)}
              onClick={() => {
                if (props.consumeSuppressedClick()) return;
                props.onNavigate(joinPath(props.pathStr, item.name));
                props.onSelect(item.name, false, false);
              }}
              onDoubleClick={() => props.onNavigate(joinPath(props.pathStr, item.name))}
              onContextMenu={(e) => { e.preventDefault(); props.onSelect(item.name, false, false); }}>
              <SamarisIcon name={iconNameForFile(item.name, "dir")} size={18} variant="soft" surface="bare" />
              <span>{item.name}</span>
            </button>
          );
        })}
        {files.map((item) => {
          const sel = props.selected.has(item.name);
          return (
            <button key={item.name} type="button"
              className={`finder-columns__item${sel ? " finder-columns__item--selected" : ""}`}
              onPointerDown={(e) => props.onPointerFileDragStart(item, e)}
              onClick={() => {
                if (props.consumeSuppressedClick()) return;
                props.onSelect(item.name, false, false);
              }}
              onDoubleClick={() => props.onOpen(item)}
              onContextMenu={(e) => { e.preventDefault(); props.onSelect(item.name, false, false); }}>
              <SamarisIcon name={iconNameForFile(item.name, "file")} size={18} variant="soft" surface="bare" />
              <span>{item.name}</span>
            </button>
          );
        })}
      </div>
    </div>
  );
}

export default FinderWindow;
