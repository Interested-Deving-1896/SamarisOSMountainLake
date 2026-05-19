import React from "react";
import { useSyncExternalStore } from "react";
import { useFs } from "../../../services/fs/useFs";
import type { FsNode } from "../../../services/fs/types";
import { searchKernel } from "../../../services/kernel/search";
import { osStore } from "../../../os/core/osStore";
import type { FinderMenuState, FinderPreview, FinderSearchResult, FinderViewMode, FinderSortField, FinderSortOrder } from "../model";
import { systemClipboard } from "../../../os/filesystem/clipboard";
import { moveToTrash, openPathInApp, pasteFromClipboard, restoreFromTrash, sendTo } from "../../../os/filesystem/fileActions";
import { joinPath, looksTextLike, splitPath, formatBytes, formatDate, fileKind } from "../utils";
import { getAppForFile } from "../../../os/filesystem/fileAssociations";
import { appLoader } from "../../../os/apps/appLoader";

const VALID_PATH_RE = /^\/(User|Volumes)(\/[a-zA-Z0-9._\-\s]+)*$/;
function validatePath(targetPath: string): boolean {
  return VALID_PATH_RE.test(targetPath) || targetPath === "/User/Trash" || targetPath === "/Volumes";
}

export function useFinderController(windowId: string, defaultPath = "/User/Desktop") {
  const fs = useFs();
  const state = useSyncExternalStore(
    (listener) => osStore.subscribe(listener),
    () => osStore.getState()
  );
  const requestedPath =
    (state.windows.find((window) => window.id === windowId)?.params?.path as string | undefined) || defaultPath;

  const [path, setPath] = React.useState<string[]>(splitPath(defaultPath));
  const [history, setHistory] = React.useState<string[]>([defaultPath]);
  const [historyIndex, setHistoryIndex] = React.useState(0);
  const [items, setItems] = React.useState<FsNode[]>([]);
  const [selected, setSelected] = React.useState<Set<string>>(new Set());
  const [inlineRename, setInlineRename] = React.useState<string | null>(null);
  const [menu, setMenu] = React.useState<FinderMenuState | null>(null);
  const [viewMode, setViewMode] = React.useState<FinderViewMode>("list");
  const [sortField, setSortField] = React.useState<FinderSortField>("name");
  const [sortOrder, setSortOrder] = React.useState<FinderSortOrder>("asc");
  const [inspectorOpen, setInspectorOpen] = React.useState(true);

  const toggleSort = React.useCallback((field: FinderSortField) => {
    setSortField((prev) => {
      if (prev === field) { setSortOrder((o) => o === "asc" ? "desc" : "asc"); return prev; }
      setSortOrder("asc"); return field;
    });
  }, []);
  const [query, setQuery] = React.useState("");
  const [loading, setLoading] = React.useState(true);
  const [error, setError] = React.useState<string | null>(null);
  const [preview, setPreview] = React.useState<FinderPreview>(null);
  const [previewLoading, setPreviewLoading] = React.useState(false);
  const [lastRequestedPath, setLastRequestedPath] = React.useState<string | null>(null);
  const [dropTargetName, setDropTargetName] = React.useState<string | null>(null);
  const [globalSearchResults, setGlobalSearchResults] = React.useState<FinderSearchResult[]>([]);
  const [searching, setSearching] = React.useState(false);
  const [searchError, setSearchError] = React.useState<string | null>(null);
  const [operationError, setOperationError] = React.useState<string | null>(null);

  const pathStr = React.useMemo(() => `/${path.join("/")}`.replace(/\/+$/, "") || "/", [path]);
  const crumbs = React.useMemo(
    () => path.map((segment, index) => ({
      label: segment,
      path: `/${path.slice(0, index + 1).join("/")}`.replace(/\/+$/, "") || "/"
    })),
    [path]
  );
  const selectedNode = React.useMemo(() => {
    if (selected.size === 1) return items.find((item) => item.name === [...selected][0]) || null;
    return null;
  }, [items, selected]);
  const filteredItems = React.useMemo(() => {
    const normalized = query.trim().toLowerCase();
    let result = normalized ? items.filter((item) => item.name.toLowerCase().includes(normalized)) : [...items];
    result.sort((a, b) => {
      const order = sortOrder === "asc" ? 1 : -1;
      if (a.kind !== b.kind) return a.kind === "dir" ? -1 : 1;
      switch (sortField) {
        case "name": return a.name.localeCompare(b.name) * order;
        case "kind": return (fileKind(a.name).localeCompare(fileKind(b.name)) || a.name.localeCompare(b.name)) * order;
        case "size": return ((a.size || 0) - (b.size || 0)) * order;
        case "date": return ((a.modifiedAt ? new Date(a.modifiedAt).getTime() : 0) - (b.modifiedAt ? new Date(b.modifiedAt).getTime() : 0)) * order;
        default: return 0;
      }
    });
    return result;
  }, [items, query, sortField, sortOrder]);

  const selectOne = React.useCallback((name: string) => { setSelected(new Set([name])); }, []);
  const toggleSelect = React.useCallback((name: string, metaKey: boolean, shiftKey: boolean) => {
    setSelected((prev) => {
      if (metaKey) {
        const next = new Set(prev);
        next.has(name) ? next.delete(name) : next.add(name);
        return next;
      }
      if (shiftKey && prev.size === 1) {
        const prevName = [...prev][0];
        const idx1 = filteredItems.findIndex((n) => n.name === prevName);
        const idx2 = filteredItems.findIndex((n) => n.name === name);
        if (idx1 === -1 || idx2 === -1) return new Set([name]);
        const lo = Math.min(idx1, idx2);
        const hi = Math.max(idx1, idx2);
        return new Set(filteredItems.slice(lo, hi + 1).map((n) => n.name));
      }
      return new Set([name]);
    });
  }, [filteredItems]);

  const selectAll = React.useCallback(() => {
    setSelected(new Set(filteredItems.map((n) => n.name)));
  }, [filteredItems]);

  const clearSelection = React.useCallback(() => { setSelected(new Set()); }, []);

  const refresh = React.useCallback(
    async (targetPath = pathStr) => {
      setLoading(true); setError(null);
      try { const result = await fs.list(targetPath); setItems(result.nodes); }
      catch (err) { setItems([]); setError(err instanceof Error ? err.message : "Filesystem unavailable"); }
      finally { setLoading(false); }
    },
    [fs, pathStr]
  );

  const applyPath = React.useCallback((targetPath: string) => {
    setPath(splitPath(targetPath)); clearSelection(); setMenu(null); setPreview(null); setQuery(""); setInlineRename(null);
  }, [clearSelection]);

  const syncPath = React.useCallback(
    (targetPath: string, mode: "push" | "replace" = "push") => {
      applyPath(targetPath);
      if (mode === "replace") {
        setHistory((current) => current.map((entry, index) => (index === historyIndex ? targetPath : entry)));
        return;
      }
      setHistory((current) => {
        const base = current.slice(0, historyIndex + 1);
        if (base[base.length - 1] === targetPath) return base;
        return [...base, targetPath];
      });
      setHistoryIndex((prev) => {
        const baseLen = history.slice(0, historyIndex + 1).length;
        return baseLen;
      });
    },
    [applyPath, history, historyIndex]
  );

  const goTo = React.useCallback(
    (targetPath: string, mode: "push" | "replace" = "push") => {
      if (!validatePath(targetPath)) return;
      if (targetPath === pathStr && mode === "push") return;
      syncPath(targetPath, mode);
    },
    [pathStr, syncPath]
  );

  const openMenuAt = React.useCallback((event: React.MouseEvent | React.PointerEvent, targetName: string | null) => {
    setMenu({ x: event.clientX + 1, y: event.clientY + 1, targetName });
  }, []);

  const previewFile = React.useCallback(
    async (fileName: string) => {
      const fullPath = joinPath(pathStr, fileName);
      if (!looksTextLike(fileName)) { setPreview({ path: fullPath, content: "Preview unavailable for this file type." }); return; }
      setPreviewLoading(true);
      try { const result = await fs.read(fullPath); setPreview({ path: result.path, content: result.content }); }
      catch (err) { setPreview({ path: fullPath, content: `Unable to open file: ${err instanceof Error ? err.message : "unknown_error"}` }); }
      finally { setPreviewLoading(false); }
    },
    [fs, pathStr]
  );

  const openNode = React.useCallback(
    async (node: FsNode) => {
      if (node.kind === "dir") { goTo(joinPath(pathStr, node.name)); return; }
      const fullPath = joinPath(pathStr, node.name);
      if (node.name.toLowerCase().match(/\.(md|txt|mp3|wav|m4a|aac|ogg|flac)$/)) { await openPathInApp(fullPath, "file"); return; }
      const appId = getAppForFile(node.name);
      if (appId) { await appLoader.openApp(appId, { windowParams: { path: fullPath } }); return; }
      await previewFile(node.name);
    },
    [goTo, pathStr, previewFile]
  );

  const confirmInlineRename = React.useCallback(
    async (from: string, to: string) => {
      setInlineRename(null);
      if (!to || from === to) return;
      try {
        await fs.rename(joinPath(pathStr, from), joinPath(pathStr, to));
        setSelected(new Set([to]));
        setOperationError(null);
        await refresh();
      } catch (err) {
        setOperationError(err instanceof Error ? err.message : "Rename failed");
      }
    },
    [fs, pathStr, refresh]
  );

  const moveNodeIntoFolder = React.useCallback(
    async (sourceName: string, folderName: string) => {
      if (!sourceName || !folderName || sourceName === folderName) return;
      const sourceNode = items.find((item) => item.name === sourceName);
      const targetNode = items.find((item) => item.name === folderName);
      if (!sourceNode || !targetNode || targetNode.kind !== "dir") return;
      const sourcePath = joinPath(pathStr, sourceName);
      const targetPath = joinPath(joinPath(pathStr, folderName), sourceName);
      setDropTargetName(folderName);
      try { await fs.rename(sourcePath, targetPath); clearSelection(); await refresh(); }
      finally { setDropTargetName(null); }
    },
    [fs, items, pathStr, refresh, clearSelection]
  );

  React.useEffect(() => {
    let cancelled = false;
    setLoading(true); setError(null);
    fs.list(pathStr)
      .then((result) => { if (cancelled) return; setItems(result.nodes); setLoading(false); })
      .catch((err) => { if (cancelled) return; setItems([]); setError(err instanceof Error ? err.message : "Filesystem unavailable"); setLoading(false); });
    return () => { cancelled = true; };
  }, [fs, pathStr]);

  React.useEffect(() => { clearSelection(); setMenu(null); setPreview(null); }, [pathStr, windowId]);

  React.useEffect(() => {
    if (!requestedPath) return;
    const normalized = requestedPath.replace(/\/+$/, "") || "/";
    if (normalized === lastRequestedPath) return;
    setLastRequestedPath(normalized);
    setPath(splitPath(normalized));
    setHistory([normalized]); setHistoryIndex(0);
    clearSelection(); setMenu(null); setPreview(null);
  }, [lastRequestedPath, requestedPath, clearSelection]);

  const reportOpError = React.useCallback((err: unknown) => {
    setOperationError(err instanceof Error ? err.message : "Operation failed");
    void refresh();
  }, [refresh]);

  const activeName = menu?.targetName || selectedNode?.name || null;
  const activeNode = activeName ? items.find((item) => item.name === activeName) || null : null;
  const clipboard = systemClipboard.readFiles();

  React.useEffect(() => {
    const normalized = query.trim();
    if (!normalized) { setGlobalSearchResults([]); setSearching(false); setSearchError(null); return; }
    setSearching(true); setSearchError(null);
    const timer = window.setTimeout(() => {
      void searchKernel.queryFiles(normalized)
        .then((results) => setGlobalSearchResults(results.map((entry) => ({ id: entry.id, title: entry.title, path: entry.path || entry.id, kind: entry.fileKind || "file", modifiedAt: entry.modifiedAt || null, size: entry.size || null }))))
        .catch((err) => { setGlobalSearchResults([]); setSearchError(err instanceof Error ? err.message : "Search unavailable"); })
        .finally(() => setSearching(false));
    }, 220);
    return () => { window.clearTimeout(timer); };
  }, [query]);

  const selectedRef = React.useRef(selected); selectedRef.current = selected;
  const pathStrRef = React.useRef(pathStr); pathStrRef.current = pathStr;
  const filteredItemsRef = React.useRef(filteredItems); filteredItemsRef.current = filteredItems;

  const handleKeyDown = React.useCallback((event: React.KeyboardEvent) => {
    const list = filteredItemsRef.current;
    const sel = selectedRef.current;
    const path = pathStrRef.current;
    const firstSel = sel.size > 0 ? [...sel][0] : null;
    const currentIndex = firstSel ? list.findIndex((item) => item.name === firstSel) : -1;
    const node = currentIndex >= 0 ? list[currentIndex] : null;
    switch (event.key) {
      case "ArrowDown":
        event.preventDefault();
        if (currentIndex < list.length - 1) {
          const next = list[currentIndex + 1];
          selectOne(next.name);
          if (next.kind === "file") previewFile(next.name).catch(() => {});
        }
        break;
      case "ArrowUp":
        event.preventDefault();
        if (currentIndex > 0) {
          const prev = list[currentIndex - 1];
          selectOne(prev.name);
          if (prev.kind === "file") previewFile(prev.name).catch(() => {});
        }
        break;
      case "Enter":
        if (event.metaKey) break;
        event.preventDefault();
        if (node) openNode(node).catch(() => {});
        break;
      case "Backspace":
        if (event.metaKey) break;
        event.preventDefault();
        goTo(splitPath(path).slice(0, -1).join("/") || "/");
        break;
      case "Delete":
        event.preventDefault();
        for (const name of sel) { moveToTrash(fs, joinPath(path, name)).catch(() => {}); }
        void refresh();
        break;
      case "a":
        if (event.metaKey || event.ctrlKey) { event.preventDefault(); selectAll(); }
        break;
      case "c":
        if (!event.metaKey && !event.ctrlKey) break;
        event.preventDefault();
        if (node) systemClipboard.writeFiles({ path: joinPath(path, node.name), action: "copy" });
        break;
      case "x":
        if (!event.metaKey && !event.ctrlKey) break;
        event.preventDefault();
        if (node) systemClipboard.writeFiles({ path: joinPath(path, node.name), action: "move" });
        break;
      case "v":
        if (!event.metaKey && !event.ctrlKey) break;
        event.preventDefault();
        pasteFromClipboard(fs, path).then(() => refresh()).catch(() => {});
        break;
      case "i":
        if (event.metaKey || event.ctrlKey) { event.preventDefault(); setInspectorOpen((o) => !o); }
        break;
      case "n":
        if (event.metaKey || event.ctrlKey) { event.preventDefault(); /* handled in parent */ }
        break;
    }
  }, [fs, selectOne, previewFile, openNode, goTo, moveToTrash, refresh, selectAll]);

  const revealSearchResult = React.useCallback(
    async (result: FinderSearchResult) => {
      if (result.kind === "dir") { goTo(result.path); return; }
      await openPathInApp(result.path, "file");
    },
    [goTo]
  );

  return {
    fs, pathStr, crumbs, history, historyIndex, items, filteredItems, query,
    selected, selectedNode, activeName, activeNode, clipboard,
    menu, viewMode, dropTargetName, inlineRename,
    loading, error, preview, previewLoading, operationError,
    sortField, sortOrder, toggleSort,
    globalSearchResults, searching, searchError,
    inspectorOpen,
    selectOne, toggleSelect, selectAll, clearSelection, setSelected,
    setInlineRename, confirmInlineRename,
    setMenu, setPreview, setViewMode, setQuery, setDropTargetName,
    setInspectorOpen, setOperationError,
    refresh, applyPath, goTo, openMenuAt, previewFile, openNode,
    handleKeyDown, revealSearchResult,
    setHistoryIndex, reportOpError,
    moveToTrash: (targetPath: string) => moveToTrash(fs, targetPath),
    restoreFromTrash: (targetPath: string) => restoreFromTrash(fs, targetPath),
    pasteFromClipboard: (destinationFolder: string) => pasteFromClipboard(fs, destinationFolder),
    sendTo: (sourcePath: string, destinationFolder: string) => sendTo(fs, sourcePath, destinationFolder),
    moveNodeIntoFolder,
  };
}
