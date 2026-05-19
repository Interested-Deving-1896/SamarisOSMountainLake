import React, { useSyncExternalStore } from "react";
import { SamarisIcon, resolveAppIconName } from "../modules/icons";
import { ContextMenu } from "./ContextMenu";
import { DockSeparator } from "./dock/DockSeparator";
import { processManager } from "../os/core/processManager";
import { osStore } from "../os/core/osStore";
import { appRegistry } from "../os/apps/appRegistry";
import type { DockStore } from "../system/dock/dockStore";
import type { DockItemModel } from "./dockModel";
import { hasFileDrop, getDroppedFiles } from "../os/filesystem/dragDrop";
import { buildFileDropPlan, commitFileDrop, nativeDndBridge } from "../os/dnd";
import { useFs } from "../services/fs/useFs";
import "../shell/dock/dock.css";

export function Dock(props: {
  store: DockStore;
  onLaunch: (appId: string) => void;
  onLaunchChild?: (appId: string) => void;
}) {
  const [contextMenu, setContextMenu] = React.useState<{ x: number; y: number; item: DockItemModel } | null>(null);
  const [draggingId, setDraggingId] = React.useState<string | null>(null);
  const [openFolderId, setOpenFolderId] = React.useState<string | null>(null);
  const [trashHover, setTrashHover] = React.useState(false);
  const dockRef = React.useRef<HTMLDivElement | null>(null);
  const dockFs = useFs();
  const [dockPrefs, setDockPrefs] = React.useState(() => {
    try { return JSON.parse(localStorage.getItem("samaris-dock/settings") || "{}"); } catch { return {}; }
  });

  React.useEffect(() => {
    const handler = () => {
      try { setDockPrefs(JSON.parse(localStorage.getItem("samaris-dock/settings") || "{}")); } catch {}
    };
    window.addEventListener("storage", handler);
    return () => window.removeEventListener("storage", handler);
  }, []);

  const dockSize = dockPrefs.size || 44;
  const dockMagnify = dockPrefs.magnify !== false;
  const dockMagnifyScale = dockPrefs.magnifyScale || 1.5;
  const dockAutoHide = dockPrefs.autoHide === true;

  const pinned = useSyncExternalStore(
    (cb) => props.store.subscribe(cb),
    () => props.store.getState().pinned
  );

  const processCounts = React.useMemo(() => {
    const counts: Record<string, number> = {};
    for (const p of osStore.getState().processes) {
      if (p.state === "running") counts[p.appId] = (counts[p.appId] || 0) + 1;
    }
    return counts;
  }, [osStore.getState().processes]);

  const runningAppIds = React.useMemo(
    () => Object.keys(processCounts),
    [processCounts]
  );

  const { pinnedItems, runningItems } = React.useMemo(() => {
    const pinnedList: DockItemModel[] = [];
    for (const id of pinned) {
      if (id === "games") {
        pinnedList.push({
          id: "games", kind: "folder", label: "Games",
          running: runningAppIds.includes("doom"),
          children: [{ id: "doom", label: "DOOM", running: runningAppIds.includes("doom") }],
        });
        continue;
      }
      const app = appRegistry[id];
      if (!app || app.hiddenFromDock) continue;
      pinnedList.push({ id: app.id, kind: "app", label: app.name, running: runningAppIds.includes(app.id) });
    }

    const runningList: DockItemModel[] = [];
    for (const appId of runningAppIds) {
      if (pinned.includes(appId)) continue;
      const app = appRegistry[appId];
      if (!app || app.hiddenFromDock) continue;
      runningList.push({ id: app.id, kind: "app", label: app.name, running: true });
    }
    return { pinnedItems: pinnedList, runningItems: runningList };
  }, [pinned, runningAppIds]);

  React.useEffect(() => {
    if (!openFolderId) return;
    const close = (e: PointerEvent) => {
      if (dockRef.current && !dockRef.current.contains(e.target as Node)) {
        setOpenFolderId(null);
      }
    };
    window.addEventListener("pointerdown", close);
    return () => window.removeEventListener("pointerdown", close);
  }, [openFolderId]);

  const handleKeyDown = React.useCallback((event: React.KeyboardEvent, item: DockItemModel, index: number, list: DockItemModel[]) => {
    switch (event.key) {
      case "ArrowRight": {
        event.preventDefault();
        const next = list[(index + 1) % list.length];
        document.getElementById(`dock-icon-${next.id}`)?.focus();
        break;
      }
      case "ArrowLeft": {
        event.preventDefault();
        const prev = list[(index - 1 + list.length) % list.length];
        document.getElementById(`dock-icon-${prev.id}`)?.focus();
        break;
      }
      case "Escape":
        if (openFolderId) { event.preventDefault(); setOpenFolderId(null); }
        break;
      case "Enter":
      case " ":
        if (item.kind === "folder") {
          event.preventDefault();
          setOpenFolderId((c) => c === item.id ? null : item.id);
        }
        break;
    }
  }, [openFolderId]);

  const renderItem = (item: DockItemModel, index: number, list: DockItemModel[]) => {
    const iconName = resolveAppIconName(item.id);
    const count = processCounts[item.id] || (item.kind === "folder" && item.running ? 1 : 0);

    return (
      <div key={item.id}
        className={`dock__item ${draggingId === item.id ? "dock__item--dragging" : ""}`}
        onContextMenu={(event) => {
          event.preventDefault();
          setContextMenu({ x: event.clientX, y: event.clientY, item });
        }}
        onDragOver={(event) => {
          if (item.kind !== "app") return;
          if (!draggingId || draggingId === item.id) return;
          const allIds = pinned;
          const from = allIds.indexOf(draggingId);
          const to = allIds.indexOf(item.id);
          if (from < 0 || to < 0 || from === to) return;
          event.preventDefault();
          props.store.reorder(from, to);
        }}
      >
        <button
          className={`dock__btn${item.id === "trash" && trashHover ? " dock__btn--trash-hover" : ""}`}
          type="button"
          id={`dock-icon-${item.id}`}
          tabIndex={0}
          draggable={item.kind === "app" && item.id !== "trash"}
          onDragStart={() => setDraggingId(item.id)}
          onDragEnd={() => setDraggingId(null)}
          onDragEnter={(e) => {
            if (item.id !== "trash" || !hasFileDrop(e.dataTransfer)) return;
            e.preventDefault();
            e.stopPropagation();
            setTrashHover(true);
          }}
          onDragOver={(e) => {
            if (item.id !== "trash" || !hasFileDrop(e.dataTransfer)) return;
            e.preventDefault();
            e.stopPropagation();
            e.dataTransfer.dropEffect = "move";
            setTrashHover(true);
          }}
          onDragLeave={(e) => {
            if (item.id !== "trash") return;
            e.stopPropagation();
            setTrashHover(false);
          }}
          onDrop={(e) => {
            if (item.id !== "trash") return;
            e.preventDefault();
            e.stopPropagation();
            setTrashHover(false);
            void (async () => {
              const internalFiles = getDroppedFiles(e.dataTransfer);
              const externalFiles = internalFiles.length > 0 ? [] : await nativeDndBridge.resolveExternalFiles(Array.from(e.dataTransfer.files || []));
              const files = [...internalFiles, ...externalFiles];
              if (files.length === 0) return;
              const plan = await buildFileDropPlan(
                dockFs,
                files,
                { id: "dock-trash", label: "Trash", path: "/User/Trash", kind: "trash" },
                { allowedChoices: ["trash"], recommendedAction: "trash" }
              );
              await commitFileDrop(dockFs, plan, { choice: "trash", conflictStrategy: "rename" });
            })();
          }}
          onClick={() => {
            if (item.kind === "folder") { setOpenFolderId((c) => c === item.id ? null : item.id); return; }
            setOpenFolderId(null);
            props.onLaunch(item.id);
          }}
          onKeyDown={(e) => handleKeyDown(e, item, index, list)}
          title={item.label}
          aria-label={item.label}
        >
          <SamarisIcon
            className="dock__icon"
            name={iconName}
            size={dockSize}
            variant="soft"
            surface="bare"
          />
          <div className="dock__tooltip">{item.label}</div>
        </button>
        {count > 0 ? (
          <div className="dock__dots">
            {count <= 3 ? (
              Array.from({ length: Math.min(count, 3) }).map((_, i) => (
                <span key={i} className="dock__dot" />
              ))
            ) : (
              <>
                <span className="dock__dot" />
                <span className="dock__dot" />
                <span className="dock__dot" />
              </>
            )}
          </div>
        ) : <div className="dock__dots" />}

        {item.kind === "folder" && openFolderId === item.id ? (
          <div className="dock__fan" role="menu" aria-label={`${item.label} folder`}
            onKeyDown={(e) => { if (e.key === "Escape") { e.stopPropagation(); setOpenFolderId(null); } }}>
            {item.children.map((child) => (
              <button key={child.id} type="button" className="dock__fanItem"
                tabIndex={0}
                onClick={() => { setOpenFolderId(null); props.onLaunchChild?.(child.id); }}
                onKeyDown={(e) => { if (e.key === "Escape") { e.stopPropagation(); setOpenFolderId(null); } }}>
                <SamarisIcon className="dock__fanIcon" name={resolveAppIconName(child.id)} size={36} variant="soft" surface="bare" />
                <span className="dock__fanLabel">{child.label}</span>
              </button>
            ))}
          </div>
        ) : null}
      </div>
    );
  };

  return (
    <>
      <div ref={dockRef}
        className={`dock${dockAutoHide ? " dock--autohide" : ""}`}
        role="toolbar" aria-label="Dock"
        style={{
          "--dock-icon-size": `${dockSize}px`,
          "--dock-magnify-scale": dockMagnify ? dockMagnifyScale : 1,
          ...(dockPrefs.position === "left" ? { left: "16px", top: "50%", bottom: "auto", transform: "translateY(-50%)", flexDirection: "column" as const } : {}),
          ...(dockPrefs.position === "right" ? { left: "auto", right: "16px", top: "50%", bottom: "auto", transform: "translateY(-50%)", flexDirection: "column" as const } : {}),
        } as React.CSSProperties}>
        {pinnedItems.map((item, i) => renderItem(item, i, pinnedItems))}
        {runningItems.length > 0 ? <DockSeparator /> : null}
        {runningItems.map((item, i) => renderItem(item, i, runningItems))}
      </div>

      {contextMenu ? (
        <ContextMenu x={contextMenu.x} y={contextMenu.y} ariaLabel={`${contextMenu.item.label} menu`} onClose={() => setContextMenu(null)}
          items={[
            { id: "open", label: "Open", onSelect: () => { setContextMenu(null); void props.onLaunch(contextMenu.item.id); } },
            { id: "toggle-pin", label: props.store.isPinned(contextMenu.item.id) ? "Unpin from Dock" : "Pin to Dock",
              onSelect: () => { setContextMenu(null); props.store.togglePin(contextMenu.item.id); } },
            ...(contextMenu.item.running ? [{
              id: "quit", label: "Quit", danger: true as const,
              onSelect: () => {
                setContextMenu(null);
                const toKill = osStore.getState().processes.filter((p) => p.appId === contextMenu.item.id && p.state === "running");
                for (const p of toKill) processManager.killProcess(p.pid);
              },
            }] : []),
          ]}
        />
      ) : null}
    </>
  );
}
