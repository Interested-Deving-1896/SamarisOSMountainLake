import React, { useEffect, useMemo, useRef, useState } from "react";
import {
  Copy,
  Folder,
  Info,
  Pencil,
  Send,
  Trash2
} from "lucide-react";
import type { FsNode } from "../services/fs/types";
import { useFs } from "../services/fs/useFs";
import { ContextMenu, type ContextMenuItem } from "./ContextMenu";
import { ConfirmModal, InfoModal, PromptModal } from "./PromptModal";
import { defaultDesktopPosition, loadDesktopLayout, normalizeDesktopLayout, resetDesktopLayout, saveDesktopLayout } from "./desktopLayout";
import { systemClipboard } from "../os/filesystem/clipboard";
import { fileSystemClient } from "../os/filesystem/fileSystemClient";
import { kernelClient } from "../os/kernel/kernelClient";
import { moveToTrash, openPathInApp, pasteFromClipboard, sendTo } from "../os/filesystem/fileActions";
import { SamarisIcon, iconNameForFile } from "../modules/icons";
import { useFileDrop } from "../apps/shared/useFileDrop";
import {
  buildFileDropPlan,
  commitFileDrop,
  createPointerDragGhost,
  findImmediateDropTarget,
  movePointerDragGhost,
  removePointerDragGhost,
  setImmediateTrashHover,
  useDnd
} from "../os/dnd";

import "./desktopIcons.css";

function basename(path: string) {
  return path.replace(/\/+/g, "/").split("/").filter(Boolean).pop() || path;
}

function fullPathFor(name: string) {
  return `/User/Desktop/${name}`.replace(/\/+/g, "/");
}

export function DesktopIcons(props: { refreshToken?: number; rearrangeToken?: number }) {
  const fs = useFs();
  const dnd = useDnd();
  const [items, setItems] = useState<FsNode[]>([]);
  const [selected, setSelected] = useState<string | null>(null);
  const [marqueeSelected, setMarqueeSelected] = useState<string[]>([]);
  const [positions, setPositions] = useState<Record<string, { x: number; y: number }>>(() => loadDesktopLayout());
  const [arranging, setArranging] = useState(false);
  const [menu, setMenu] = useState<{ x: number; y: number; targetName: string | null } | null>(null);
  const [marquee, setMarquee] = useState<{ x: number; y: number; w: number; h: number } | null>(null);
  const [dialog, setDialog] = useState<
    | { kind: "rename"; currentName: string }
    | { kind: "delete"; currentName: string }
    | { kind: "properties"; currentName: string }
    | null
  >(null);
  const [operationError, setOperationError] = useState<string | null>(null);
  useEffect(() => { if (operationError) { const t = setTimeout(() => setOperationError(null), 5000); return () => clearTimeout(t); } }, [operationError]);
  const fileDrop = useFileDrop({
    target: { id: "desktop-icons", label: "Desktop", path: "/User/Desktop", kind: "folder" },
    allowedChoices: ["copy", "move", "link", "import"],
    recommendedAction: "copy",
    ignoreSourceIds: ["desktop-icons"],
    onDrop: async (_files, context) => {
      if (!fs) return;
      await commitFileDrop(fs, context.plan, context.decision);
      refresh();
    }
  });
  const dragRef = useRef<{
    name: string;
    startX: number;
    startY: number;
    originX: number;
    originY: number;
    moved: boolean;
    ghost: HTMLElement | null;
  } | null>(null);
  const marqueeRef = useRef<{ startX: number; startY: number } | null>(null);
  const suppressClickRef = useRef(false);
  const positionsRef = useRef(positions);
  const rootRef = useRef<HTMLDivElement | null>(null);
  const itemsRef = useRef(items);
  useEffect(() => { itemsRef.current = items; }, [items]);
  const selectedRef = useRef(selected);
  useEffect(() => { selectedRef.current = selected; }, [selected]);
  const marqueeSelectedRef = useRef(marqueeSelected);
  useEffect(() => { marqueeSelectedRef.current = marqueeSelected; }, [marqueeSelected]);

  useEffect(() => {
    positionsRef.current = positions;
  }, [positions]);

  useEffect(() => {
    function handleGlobalKey(event: KeyboardEvent) {
      const target = event.target as HTMLElement | null;
      if (target?.closest(".samaris-window, input, textarea, .cm, .pm")) return;

      const sel = selectedRef.current;
      const marquee = marqueeSelectedRef.current;
      let singleName: string | null = null;
      if (sel) singleName = sel;
      else if (marquee.length === 1) singleName = marquee[0];

      if (event.key === "Delete" || event.key === "Backspace") {
        if (!singleName) return;
        event.preventDefault();
        setDialog({ kind: "delete", currentName: singleName });
        return;
      }

      if ((event.metaKey || event.ctrlKey) && event.key === "c") {
        event.preventDefault();
        if (singleName) systemClipboard.writeFiles({ path: fullPathFor(singleName), action: "copy" });
        return;
      }

      if ((event.metaKey || event.ctrlKey) && event.key === "x") {
        event.preventDefault();
        if (singleName) systemClipboard.writeFiles({ path: fullPathFor(singleName), action: "move" });
        return;
      }

      if ((event.metaKey || event.ctrlKey) && event.key === "v") {
        event.preventDefault();
        void pasteFromClipboard(fs, "/User/Desktop").then(() => refresh()).catch(() => setOperationError("Paste failed"));
        return;
      }

      if ((event.metaKey || event.ctrlKey) && event.key === "a") {
        event.preventDefault();
        setMarqueeSelected(itemsRef.current.map((n) => n.name));
        return;
      }

      if (event.key === "F2") {
        event.preventDefault();
        if (singleName) setDialog({ kind: "rename", currentName: singleName });
        return;
      }
    }

    window.addEventListener("keydown", handleGlobalKey);
    return () => window.removeEventListener("keydown", handleGlobalKey);
  }, [fs]);

  const sorted = useMemo(() => {
    const next = items.slice();
    next.sort((a, b) => {
      if (a.kind !== b.kind) return a.kind === "dir" ? -1 : 1;
      return a.name.localeCompare(b.name);
    });
    return next;
  }, [items]);

  const selectedNode = useMemo(() => {
    const targetName = selected || (marqueeSelected.length === 1 ? marqueeSelected[0] : null);
    return sorted.find((node) => node.name === targetName) || null;
  }, [marqueeSelected, selected, sorted]);

  async function refresh() {
    try {
      const result = await fs.list("/User/Desktop");
      setItems(result.nodes);
      setPositions(normalizeDesktopLayout(result.nodes.map((node) => node.name)));
    } catch {
      setItems([]);
    }
  }

  useEffect(() => {
    void refresh();
    void fileSystemClient.watch("/User/Desktop").catch(() => {});
    const unsubscribe = kernelClient.on<{ root?: string; path?: string }>("fs.watch.event", (event) => {
      if (event?.root === "/User/Desktop" || event?.path?.startsWith("/User/Desktop")) void refresh();
    });
    return () => {
      unsubscribe();
      void fileSystemClient.unwatch("/User/Desktop").catch(() => {});
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  useEffect(() => {
    void refresh();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [props.refreshToken]);

  useEffect(() => {
    if (!props.rearrangeToken) return;
    const next = resetDesktopLayout(sorted.map((node) => node.name));
    setArranging(true);
    commitPositions(next);
    const timeoutId = window.setTimeout(() => setArranging(false), 430);
    return () => window.clearTimeout(timeoutId);
  }, [props.rearrangeToken, sorted]);

  function commitPositions(next: Record<string, { x: number; y: number }>) {
    setPositions(next);
    saveDesktopLayout(next);
  }

  function menuItems(): ContextMenuItem[] {
    const clipboard = systemClipboard.readFiles();
    return [
      {
        id: "open",
        label: "Open",
        icon: Folder,
        disabled: !selectedNode,
        onSelect: () => {
          if (!selectedNode) return;
          void openPathInApp(fullPathFor(selectedNode.name), selectedNode.kind);
        }
      },
      {
        id: "rename",
        label: "Rename",
        icon: Pencil,
        disabled: !selectedNode,
        onSelect: () => {
          if (!selectedNode) return;
          setDialog({ kind: "rename", currentName: selectedNode.name });
        }
      },
      {
        id: "copy",
        label: "Copy",
        icon: Copy,
        disabled: !selectedNode,
        onSelect: () => {
          if (!selectedNode) return;
          systemClipboard.writeFiles({ path: fullPathFor(selectedNode.name), action: "copy" });
        }
      },
      {
        id: "paste",
        label: "Paste",
        icon: Copy,
        disabled: !clipboard,
        onSelect: () => {
          void pasteFromClipboard(fs, "/User/Desktop").then(() => refresh()).catch(() => setOperationError("Paste failed"));
        }
      },
      {
        id: "send-documents",
        label: "Send to Documents",
        icon: Send,
        disabled: !selectedNode,
        onSelect: () => {
          if (!selectedNode) return;
          void sendTo(fs, fullPathFor(selectedNode.name), "/User/Documents").then(() => refresh());
        }
      },
      {
        id: "send-downloads",
        label: "Send to Downloads",
        icon: Send,
        disabled: !selectedNode,
        onSelect: () => {
          if (!selectedNode) return;
          void sendTo(fs, fullPathFor(selectedNode.name), "/User/Downloads").then(() => refresh());
        }
      },
      {
        id: "delete",
        label: "Move to Trash",
        icon: Trash2,
        danger: true,
        disabled: !selectedNode,
        onSelect: () => {
          if (!selectedNode) return;
          setDialog({ kind: "delete", currentName: selectedNode.name });
        }
      },
      {
        id: "properties",
        label: "Properties",
        icon: Info,
        disabled: !selectedNode,
        onSelect: () => {
          if (!selectedNode) return;
          setDialog({ kind: "properties", currentName: selectedNode.name });
        }
      }
    ];
  }

  return (
    <div
      ref={rootRef}
      className={`di ${arranging ? "di--arranging" : ""}${fileDrop.isDragging ? " di--drop-target" : ""}`}
      role="presentation"
      {...fileDrop.dragProps}
      onPointerDown={(e) => {
        if (e.button !== 0) return;
        const target = e.target as HTMLElement | null;
        if (target?.closest(".di__item, .cm, .pm")) return;
        setSelected(null);
        setMarqueeSelected([]);
        setMenu(null);
        const bounds = rootRef.current?.getBoundingClientRect();
        const startX = e.clientX - (bounds?.left || 0);
        const startY = e.clientY - (bounds?.top || 0);
        marqueeRef.current = { startX, startY };

        const handleMove = (moveEvent: PointerEvent) => {
          const origin = marqueeRef.current;
          if (!origin) return;
          const moveX = moveEvent.clientX - (bounds?.left || 0);
          const moveY = moveEvent.clientY - (bounds?.top || 0);
          const left = Math.min(origin.startX, moveX);
          const top = Math.min(origin.startY, moveY);
          const width = Math.abs(moveX - origin.startX);
          const height = Math.abs(moveY - origin.startY);
          setMarquee({ x: left, y: top, w: width, h: height });

          const picked = sorted
            .filter((node, index) => {
              const pos = positionsRef.current[node.name] || defaultDesktopPosition(index);
              const iconRect = {
                left: pos.x,
                top: pos.y,
                right: pos.x + 92,
                bottom: pos.y + 88
              };
              return !(
                iconRect.right < left ||
                iconRect.left > left + width ||
                iconRect.bottom < top ||
                iconRect.top > top + height
              );
            })
            .map((node) => node.name);

          setMarqueeSelected(picked);
          setSelected(picked.length === 1 ? picked[0] : null);
        };

        const handleUp = () => {
          marqueeRef.current = null;
          setMarquee(null);
          window.removeEventListener("pointermove", handleMove);
          window.removeEventListener("pointerup", handleUp);
        };

        window.addEventListener("pointermove", handleMove);
        window.addEventListener("pointerup", handleUp, { once: true });
      }}
    >
      <div className="di__grid" role="list" aria-label="Desktop icons">
        {sorted.map((node, index) => {
          const fullPath = fullPathFor(node.name);
          const isSelected = selected === node.name || marqueeSelected.includes(node.name);
          const pos = positions[node.name] || defaultDesktopPosition(index);

          return (
            <button
              key={node.name}
              type="button"
              role="listitem"
              className={`di__item ${isSelected ? "di__item--selected" : ""}`}
              style={{ left: pos.x, top: pos.y }}
              onClick={() => {
                if (suppressClickRef.current) {
                  suppressClickRef.current = false;
                  return;
                }
                setSelected(node.name);
                setMarqueeSelected([node.name]);
              }}
              onDoubleClick={() => {
                void openPathInApp(fullPath, node.kind);
              }}
              onContextMenu={(event) => {
                event.preventDefault();
                event.stopPropagation();
                setSelected(node.name);
                setMarqueeSelected([node.name]);
                setMenu({ x: event.clientX + 1, y: event.clientY + 1, targetName: node.name });
              }}
              onPointerDown={(event) => {
                if (event.button !== 0) return;
                dragRef.current = {
                  name: node.name,
                  startX: event.clientX,
                  startY: event.clientY,
                  originX: pos.x,
                  originY: pos.y,
                  moved: false,
                  ghost: null
                };

                const handleMove = (moveEvent: PointerEvent) => {
                  const drag = dragRef.current;
                  if (!drag || drag.name !== node.name) return;
                  const dx = moveEvent.clientX - drag.startX;
                  const dy = moveEvent.clientY - drag.startY;
                  if (!drag.moved && dx === 0 && dy === 0) return;
                  moveEvent.preventDefault();
                  if (!drag.moved) {
                    drag.moved = true;
                    suppressClickRef.current = true;
                    setSelected(node.name);
                    setMarqueeSelected([node.name]);
                  }
                  const files = [{ name: node.name, path: fullPath, kind: node.kind, size: node.size || 0 }];
                  const immediateTarget = findImmediateDropTarget(moveEvent.clientX, moveEvent.clientY);
                  setImmediateTrashHover(immediateTarget?.kind === "trash");
                  if (immediateTarget && !drag.ghost) {
                    drag.ghost = createPointerDragGhost(files);
                  }
                  if (drag.ghost) movePointerDragGhost(drag.ghost, moveEvent.clientX, moveEvent.clientY);
                  const el = document.elementFromPoint(moveEvent.clientX, moveEvent.clientY);
                  if (el?.closest(".samaris-window, .win, .dock, .samaris-airbar-shell")) return;
                  setArranging(false);
                  const nextX = Math.max(12, drag.originX + dx);
                  const nextY = Math.max(12, drag.originY + dy);
                  setPositions((current) => {
                    const next = {
                      ...current,
                      [node.name]: { x: nextX, y: nextY }
                    };
                    positionsRef.current = next;
                    return next;
                  });
                };

                const handleUp = (upEvent: PointerEvent) => {
                  const drag = dragRef.current;
                  if (drag?.moved) {
                    const dropSurface = document.elementFromPoint(upEvent.clientX, upEvent.clientY);
                    const immediateTarget = findImmediateDropTarget(upEvent.clientX, upEvent.clientY);
                    const escapedToDropSurface = Boolean(dropSurface?.closest(".samaris-window, .win, .dock, .samaris-airbar-shell"));
                    suppressClickRef.current = true;
                    if (immediateTarget?.kind === "trash" || immediateTarget?.kind === "folder") {
                      setPositions((current) => {
                        const next = { ...current, [node.name]: { x: drag.originX, y: drag.originY } };
                        positionsRef.current = next;
                        return next;
                      });
                      if (immediateTarget.kind === "trash") {
                        void moveToTrash(fs, fullPath).then(() => refresh()).catch(() => setOperationError("Move to Trash failed"));
                      } else {
                        const files = [{ name: node.name, path: fullPath, kind: node.kind, size: node.size || 0 }];
                        void buildFileDropPlan(
                          fs,
                          files,
                          { id: `desktop-to:${immediateTarget.path}`, label: immediateTarget.label, path: immediateTarget.path, kind: "folder" },
                          { allowedChoices: ["copy", "move", "link"], recommendedAction: "move" }
                        )
                          .then((plan) => dnd.requestFileDrop(plan).then((decision) => ({ plan, decision })))
                          .then(({ plan, decision }) => decision ? commitFileDrop(fs, plan, decision) : null)
                          .then(() => refresh())
                          .catch(() => setOperationError("Drop failed"));
                      }
                    } else if (escapedToDropSurface) {
                      setPositions((current) => {
                        const next = { ...current, [node.name]: { x: drag.originX, y: drag.originY } };
                        positionsRef.current = next;
                        return next;
                      });
                    } else {
                      saveDesktopLayout(positionsRef.current);
                    }
                  }
                  removePointerDragGhost(drag?.ghost || null);
                  setImmediateTrashHover(false);
                  dragRef.current = null;
                  window.removeEventListener("pointermove", handleMove);
                  window.removeEventListener("pointerup", handleUp);
                  window.removeEventListener("pointercancel", handleCancel);
                };

                const handleCancel = () => {
                  const drag = dragRef.current;
                  if (drag) {
                    setPositions((current) => {
                      const next = { ...current, [node.name]: { x: drag.originX, y: drag.originY } };
                      positionsRef.current = next;
                      return next;
                    });
                    removePointerDragGhost(drag.ghost);
                  }
                  setImmediateTrashHover(false);
                  dragRef.current = null;
                  window.removeEventListener("pointermove", handleMove);
                  window.removeEventListener("pointerup", handleUp);
                  window.removeEventListener("pointercancel", handleCancel);
                };

                window.addEventListener("pointermove", handleMove);
                window.addEventListener("pointerup", handleUp, { once: true });
                window.addEventListener("pointercancel", handleCancel, { once: true });
              }}
              title={node.name}
            >
              <SamarisIcon className="di__icon" name={iconNameForFile(node.name, node.kind)} size={58} variant="soft" surface="bare" />
              <div className="di__label">{node.name}</div>
            </button>
          );
        })}
      </div>

      {marquee ? (
        <div
          className="di__marquee"
          style={{
            left: marquee.x,
            top: marquee.y,
            width: marquee.w,
            height: marquee.h
          }}
          aria-hidden="true"
        />
      ) : null}

      {menu ? (
        <ContextMenu
          x={menu.x}
          y={menu.y}
          ariaLabel="Desktop item menu"
          onClose={() => setMenu(null)}
          items={menuItems()}
        />
      ) : null}

      {dialog?.kind === "rename" ? (
        <PromptModal
          title="Rename Item"
          subtitle={fullPathFor(dialog.currentName)}
          defaultValue={dialog.currentName}
          confirmLabel="Rename"
          onCancel={() => setDialog(null)}
          onConfirm={(value) => {
            setDialog(null);
            if (!value || value === dialog.currentName) return;
            void fs
              .rename(fullPathFor(dialog.currentName), fullPathFor(value))
              .then(() => {
                const current = loadDesktopLayout();
                if (current[dialog.currentName]) {
                  current[value] = current[dialog.currentName];
                  delete current[dialog.currentName];
                  saveDesktopLayout(current);
                }
              })
              .finally(() => refresh());
          }}
        />
      ) : null}

      {dialog?.kind === "delete" ? (
        <ConfirmModal
          title="Move to Trash"
          subtitle={dialog.currentName}
          confirmLabel="Move"
          danger
          onCancel={() => setDialog(null)}
          onConfirm={() => {
            const targetName = dialog.currentName;
            setDialog(null);
            void moveToTrash(fs, fullPathFor(targetName))
              .then(() => refresh())
              .catch((err) => { refresh(); setOperationError(err instanceof Error ? err.message : "Delete failed"); });
          }}
        />
      ) : null}

      {dialog?.kind === "properties" ? (
        <InfoModal
          title={dialog.currentName}
          subtitle="Desktop item"
          onClose={() => setDialog(null)}
        >
          <div className="di__properties">
            <div><strong>Path:</strong> {fullPathFor(dialog.currentName)}</div>
            <div><strong>Kind:</strong> {selectedNode?.kind === "dir" ? "Folder" : "File"}</div>
            <div><strong>Size:</strong> {selectedNode?.size ? `${selectedNode.size} bytes` : "—"}</div>
          </div>
        </InfoModal>
      ) : null}

      {operationError ? (
        <div className="di__error" role="alert">
          {operationError}
        </div>
      ) : null}
    </div>
  );
}
