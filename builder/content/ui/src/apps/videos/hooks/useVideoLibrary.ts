import React from "react";
import { useFs } from "../../../services/fs/useFs";
import type { SavedVideoProgress, VideoAsset, SortMode } from "../types";

const STORAGE_KEY = "samaris-os/videos-progress";
const VIDEO_EXTS = [".mp4", ".webm", ".avi", ".mkv", ".mov", ".m4v", ".ogv", ".3gp", ".wmv", ".flv"];
const KERNEL_HTTP = "http://127.0.0.1:9999";

function isVideo(name: string) {
  const lower = name.toLowerCase();
  return VIDEO_EXTS.some((e) => lower.endsWith(e));
}

function loadProgress(): SavedVideoProgress {
  try { return JSON.parse(globalThis.localStorage?.getItem(STORAGE_KEY) || "{}") as SavedVideoProgress; }
  catch { return {}; }
}

function saveProgress(progress: SavedVideoProgress) {
  try { globalThis.localStorage?.setItem(STORAGE_KEY, JSON.stringify(progress)); } catch {}
}

export function useVideoLibrary(preferredPath?: string) {
  const fs = useFs();
  const [videos, setVideos] = React.useState<VideoAsset[]>([]);
  const [loading, setLoading] = React.useState(true);
  const [searchQuery, setSearchQuery] = React.useState("");
  const [sortMode, setSortMode] = React.useState<SortMode>("name");
  const [refreshToken, setRefreshToken] = React.useState(0);
  const progressRef = React.useRef<SavedVideoProgress>(loadProgress());

  const sourcePath = React.useMemo(() => {
    if (!preferredPath) return "/User/Videos";
    const lastSlash = preferredPath.lastIndexOf("/");
    return lastSlash > 0 ? preferredPath.slice(0, lastSlash) : "/User/Videos";
  }, [preferredPath]);

  const [activeVideoId, setActiveVideoId] = React.useState<string | null>(preferredPath || null);

  React.useEffect(() => {
    let cancelled = false;

    async function load() {
      setLoading(true);
      try {
        const result = await fs.list(sourcePath);
        const fileNodes = result.nodes
          .filter((n) => n.kind === "file" && isVideo(n.name))
          .sort((a, b) => a.name.localeCompare(b.name));

        const assets: VideoAsset[] = fileNodes.map((n) => {
          const path = `${sourcePath}/${n.name}`.replace(/\/+/g, "/");
          return {
            id: path,
            path,
            fileName: n.name,
            title: n.name.replace(/\.[^.]+$/, "").replace(/[_-]+/g, " ").trim(),
            format: (n.name.split(".").pop() || "").toUpperCase(),
            size: n.size || 0,
          };
        });

        if (cancelled) return;
        setVideos(assets);
        setActiveVideoId((id) => id || preferredPath || assets[0]?.id || null);
      } catch {
        if (!cancelled) setVideos([]);
      } finally {
        if (!cancelled) setLoading(false);
      }
    }

    void load();
    return () => { cancelled = true; };
  }, [fs, sourcePath, preferredPath, refreshToken]);

  const refresh = React.useCallback(() => {
    setRefreshToken((t) => t + 1);
  }, []);

  React.useEffect(() => {
    if (!preferredPath) return;
    setActiveVideoId(preferredPath);
  }, [preferredPath]);

  const ensureVideoSource = React.useCallback(async (videoId: string) => {
    const existing = videos.find((v) => v.id === videoId);
    if (existing?.src) return existing.src;
    const path = existing?.path || videoId;
    const streamUrl = `${KERNEL_HTTP}/api/fs/read-file?path=${encodeURIComponent(path)}`;
    setVideos((curr) => curr.map((v) => (v.id === videoId ? { ...v, src: streamUrl } : v)));
    return streamUrl;
  }, [videos]);

  const savePlaybackProgress = React.useCallback((videoId: string, seconds: number) => {
    progressRef.current = { ...progressRef.current, [videoId]: seconds };
    saveProgress(progressRef.current);
  }, []);

  const filteredVideos = React.useMemo(() => {
    let result = [...videos];
    if (searchQuery) result = result.filter((v) => v.title.toLowerCase().includes(searchQuery.toLowerCase()));
    switch (sortMode) {
      case "name": result.sort((a, b) => a.title.localeCompare(b.title)); break;
      case "size": result.sort((a, b) => b.size - a.size); break;
      case "format": result.sort((a, b) => a.format.localeCompare(b.format)); break;
    }
    return result;
  }, [videos, searchQuery, sortMode]);

  const activeVideo = filteredVideos.find((v) => v.id === activeVideoId) || filteredVideos[0] || null;

  return {
    videos: filteredVideos,
    loading,
    activeVideoId,
    activeVideo,
    setActiveVideoId,
    ensureVideoSource,
    savePlaybackProgress,
    getSavedProgress: (id: string) => progressRef.current[id] || 0,
    searchQuery,
    setSearchQuery,
    sortMode,
    setSortMode,
    refresh,
  };
}
