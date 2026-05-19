import React from "react";
import { useFs } from "../../../services/fs/useFs";
import type { PhotoAsset, SortMode, ViewFilter } from "../types";
import { getThumbnail } from "../../../services/fs/fileThumbnails";

const IMAGE_EXTENSIONS = [".png", ".jpg", ".jpeg", ".webp", ".gif", ".svg", ".bmp", ".tiff", ".tif"];
const THUMB_SIZE = 280;

const SCAN_DIRS = [
  "/User/Desktop",
  "/User/Documents",
  "/User/Downloads",
  "/User/Photos",
  "/User/Pictures",
  "/User/Music",
  "/User/Videos",
  "/User/Public",
];

function isImageFile(name: string) {
  const lower = name.toLowerCase();
  return IMAGE_EXTENSIONS.some((ext) => lower.endsWith(ext));
}

export function usePhotosLibrary(preferredPath?: string) {
  const fs = useFs();
  const [allPhotos, setAllPhotos] = React.useState<PhotoAsset[]>([]);
  const [loading, setLoading] = React.useState(true);
  const [error, setError] = React.useState<string | null>(null);
  const [activePhotoId, setActivePhotoId] = React.useState<string | null>(null);
  const [sortMode, setSortMode] = React.useState<SortMode>("name");
  const [searchQuery, setSearchQuery] = React.useState("");
  const [viewFilter, setViewFilter] = React.useState<ViewFilter>("all");
  const abortRef = React.useRef<AbortController | null>(null);

  const sourcePaths = React.useMemo(() => {
    if (!preferredPath) return SCAN_DIRS;
    // If preferredPath looks like a file path (has extension), list its parent dir
    if (preferredPath.match(/\.([a-zA-Z0-9]+)$/)) {
      const lastSlash = preferredPath.lastIndexOf("/");
      return lastSlash > 0 ? [preferredPath.slice(0, lastSlash)] : SCAN_DIRS;
    }
    return [preferredPath];
  }, [preferredPath]);

  const sourcePath = sourcePaths[0];

  const refresh = React.useCallback(async () => {
    abortRef.current?.abort();
    const controller = new AbortController();
    abortRef.current = controller;
    setError(null);
    setLoading(true);

    try {
      // List all base scan directories + one-level deep subdirectories
      const allDirs = new Set(sourcePaths);
      const baseResults = await Promise.allSettled(
        sourcePaths.map((dir) => fs.list(dir))
      );

      for (const r of baseResults) {
        if (controller.signal.aborted) return;
        if (r.status !== "fulfilled") continue;
        for (const node of r.value.nodes) {
          if (node.kind === "dir") {
            allDirs.add(`${r.value.path}/${node.name}`.replace(/\/+/g, "/"));
          }
        }
      }

      // Re-scan with expanded dir list (skip already listed)
      const dirsToScan = [...allDirs].filter((d) => !sourcePaths.includes(d));
      const subResults = await Promise.allSettled(
        dirsToScan.map((dir) => fs.list(dir))
      );

      const allResults = [...baseResults, ...subResults];

      const seen = new Set<string>();
      const imageNodes: { name: string; path: string; size?: number; modifiedAt?: number | string | null }[] = [];

      for (const r of allResults) {
        if (controller.signal.aborted) return;
        if (r.status !== "fulfilled") continue;
        for (const node of r.value.nodes) {
          if (node.kind !== "file" || !isImageFile(node.name)) continue;
          const fullPath = `${r.value.path}/${node.name}`.replace(/\/+/g, "/");
          if (seen.has(fullPath)) continue;
          seen.add(fullPath);
          imageNodes.push({
            name: node.name,
            path: fullPath,
            size: node.size,
            modifiedAt: node.modifiedAt,
          });
        }
      }

      if (controller.signal.aborted) return;

      const assets: PhotoAsset[] = imageNodes.map((node) => ({
        id: node.path,
        name: node.name,
        path: node.path,
        dataUrl: null,
        size: node.size,
        modifiedAt: node.modifiedAt,
      }));

      setAllPhotos(assets);

      if (assets.length > 0 && !activePhotoId) {
        setActivePhotoId(assets[0].id);
      }

      const batchSize = 8;
      const allIds = assets.map((a) => a.id);
      for (let i = 0; i < allIds.length; i += batchSize) {
        if (controller.signal.aborted) return;
        const batch = allIds.slice(i, i + batchSize);
        const thumbResults = await Promise.allSettled(
          batch.map((id) => getThumbnail(id, THUMB_SIZE, fs))
        );
        setAllPhotos((prev) =>
          prev.map((p) => {
            const idx = batch.indexOf(p.id);
            if (idx >= 0 && thumbResults[idx].status === "fulfilled" && thumbResults[idx].value) {
              return { ...p, dataUrl: thumbResults[idx].value };
            }
            return p;
          })
        );
      }
      if (!controller.signal.aborted) setLoading(false);
    } catch (err) {
      if (controller.signal.aborted) return;
      setError(err instanceof Error ? err.message : "Failed to load library");
      setAllPhotos([]);
      setLoading(false);
    }
  }, [fs, sourcePaths, preferredPath]);

  React.useEffect(() => {
    void refresh();
    return () => { abortRef.current?.abort(); };
  }, [refresh]);

  const allPhotosRef = React.useRef(allPhotos);
  allPhotosRef.current = allPhotos;

  const loadDataUrl = React.useCallback(async (photoId: string): Promise<string | null> => {
    const existing = allPhotosRef.current.find((p) => p.id === photoId);
    if (existing?.dataUrl) return existing.dataUrl;
    try {
      const thumb = await getThumbnail(photoId, 1200, fs);
      if (thumb) {
        setAllPhotos((prev) => prev.map((p) => (p.id === photoId ? { ...p, dataUrl: thumb } : p)));
      }
      return thumb;
    } catch {
      return null;
    }
  }, [fs]);

  const sortedPhotos = React.useMemo(() => {
    let filtered = allPhotos;
    if (viewFilter === "recent") {
      const dayAgo = Date.now() - 86400000;
      filtered = allPhotos.filter((p) => p.modifiedAt && new Date(p.modifiedAt).getTime() > dayAgo);
    }
    if (searchQuery) {
      const q = searchQuery.toLowerCase();
      filtered = filtered.filter((p) => p.name.toLowerCase().includes(q));
    }
    const sorted = [...filtered];
    switch (sortMode) {
      case "name":
        sorted.sort((a, b) => a.name.localeCompare(b.name));
        break;
      case "size":
        sorted.sort((a, b) => (b.size ?? 0) - (a.size ?? 0));
        break;
      case "newest":
        sorted.sort((a, b) => (b.modifiedAt ? new Date(b.modifiedAt).getTime() : 0) - (a.modifiedAt ? new Date(a.modifiedAt).getTime() : 0));
        break;
    }
    return sorted;
  }, [allPhotos, searchQuery, sortMode, viewFilter]);

  const activePhoto = React.useMemo(
    () => sortedPhotos.find((photo) => photo.id === activePhotoId) || sortedPhotos[0] || null,
    [activePhotoId, sortedPhotos]
  );

  const activeIndex = React.useMemo(
    () => sortedPhotos.findIndex((p) => p.id === activePhotoId),
    [activePhotoId, sortedPhotos]
  );

  return {
    allPhotos: sortedPhotos,
    loading,
    error,
    refresh,
    activePhoto,
    activePhotoId,
    activeIndex,
    setActivePhotoId,
    loadDataUrl,
    sortMode,
    setSortMode,
    searchQuery,
    setSearchQuery,
    viewFilter,
    setViewFilter,
    sourcePath,
  };
}
