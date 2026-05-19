import React from "react";
import { mediaKernel } from "../../../services/kernel/media";
import { useFs } from "../../../services/fs/useFs";
import type { MusicTrack } from "../types";

const AUDIO_EXTENSIONS = [".mp3", ".flac", ".wav", ".m4a", ".aac", ".ogg", ".weba"];

function isSupportedAudioFile(name: string) {
  const lower = name.toLowerCase();
  return AUDIO_EXTENSIONS.some((ext) => lower.endsWith(ext));
}

function fileToDataUrl(file: File) {
  return new Promise<string>((resolve, reject) => {
    const reader = new FileReader();
    reader.onload = () => resolve(String(reader.result || ""));
    reader.onerror = () => reject(reader.error || new Error("file_read_failed"));
    reader.readAsDataURL(file);
  });
}

async function uniqueMusicPath(fs: ReturnType<typeof useFs>, fileName: string) {
  const listing = await fs.list("/User/Music");
  const existing = new Set(listing.nodes.map((node) => node.name.toLowerCase()));
  if (!existing.has(fileName.toLowerCase())) {
    return `/User/Music/${fileName}`.replace(/\/+/g, "/");
  }

  const dotIndex = fileName.lastIndexOf(".");
  const stem = dotIndex > 0 ? fileName.slice(0, dotIndex) : fileName;
  const ext = dotIndex > 0 ? fileName.slice(dotIndex) : "";
  let counter = 2;
  while (existing.has(`${stem} ${counter}${ext}`.toLowerCase())) {
    counter += 1;
  }
  return `/User/Music/${stem} ${counter}${ext}`.replace(/\/+/g, "/");
}

export function useMusicLibrary(_preferredPath?: string) {
  const fs = useFs();
  const [tracks, setTracks] = React.useState<MusicTrack[]>([]);
  const [loading, setLoading] = React.useState(true);
  const [importing, setImporting] = React.useState(false);

  const refresh = React.useCallback(async () => {
    setLoading(true);
    try {
      const nextTracks = await mediaKernel.musicLibrary();
      setTracks((current) => {
        const sourceCache = new Map(current.filter((entry) => entry.src).map((entry) => [entry.id, entry.src]));
        return nextTracks.map((track) => {
          const src = sourceCache.get(track.id);
          return src ? { ...track, src } : track;
        });
      });
    } catch {
      setTracks([]);
    } finally {
      setLoading(false);
    }
  }, []);

  React.useEffect(() => {
    void refresh();
  }, [refresh]);

  const ensureTrackSource = React.useCallback(
    async (trackId: string) => {
      const existing = tracks.find((track) => track.id === trackId);
      if (existing?.src) {
        return existing.src;
      }
      const path = existing?.path || trackId;
      const result = await fs.readDataUrl(path);
      setTracks((current) =>
        current.map((track) => (track.id === trackId ? { ...track, src: result.dataUrl } : track))
      );
      return result.dataUrl;
    },
    [fs, tracks]
  );

  const importFiles = React.useCallback(
    async (files: File[]) => {
      const candidates = files.filter((file) => isSupportedAudioFile(file.name));
      if (!candidates.length) return 0;
      setImporting(true);
      let imported = 0;
      try {
        for (const file of candidates) {
          const dataUrl = await fileToDataUrl(file);
          const targetPath = await uniqueMusicPath(fs, file.name);
          await fs.writeBase64(targetPath, dataUrl);
          imported += 1;
        }
      } finally {
        setImporting(false);
      }
      await refresh();
      return imported;
    },
    [fs, refresh]
  );

  return {
    tracks,
    loading,
    importing,
    refresh,
    ensureTrackSource,
    importFiles
  };
}
