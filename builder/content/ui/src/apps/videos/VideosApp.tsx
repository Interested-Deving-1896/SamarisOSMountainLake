import React from "react";
import { osStore } from "../../os/core/osStore";
import { useVideoLibrary } from "./hooks/useVideoLibrary";
import { VideoGrid } from "./components/VideoGrid";
import { VideoFullscreen } from "./components/VideoFullscreen";
import { useFileDrop } from "../shared/useFileDrop";
import { useFs } from "../../services/fs/useFs";
import { commitFileDrop } from "../../os/dnd";

export function VideosApp(props: { windowId: string }) {
  const state = React.useSyncExternalStore(
    (listener) => osStore.subscribe(listener),
    () => osStore.getState()
  );
  const preferredPath = state.windows.find((w) => w.id === props.windowId)?.params?.path as string | undefined;
  const lib = useVideoLibrary(preferredPath);
  const fs = useFs();
  const videoDrop = useFileDrop({
    accept: [".mp4", ".webm", ".mov", ".avi", ".mkv", ".wmv", ".flv", ".m4v"],
    target: { id: "videos-library", label: "Videos", path: "/User/Videos", kind: "app" },
    allowedChoices: ["import", "copy"],
    recommendedAction: "import",
    onDrop: async (_files, context) => {
      await commitFileDrop(fs, context.plan, { ...context.decision, choice: context.decision.choice === "import" ? "copy" : context.decision.choice });
      lib.refresh();
    }
  });

  const [fsIndex, setFsIndex] = React.useState<number | null>(null);
  const [activeSource, setActiveSource] = React.useState<string | null>(null);
  const autoPlayed = React.useRef(false);

  const fsVideo = fsIndex !== null && lib.videos[fsIndex] ? lib.videos[fsIndex] : null;

  // Preload source when opening a video
  const openVideo = React.useCallback((idx: number) => {
    setFsIndex(idx);
    const video = lib.videos[idx];
    if (!video) return;
    if (video.src) {
      setActiveSource(video.src);
    } else {
      setActiveSource(null);
      lib.ensureVideoSource(video.id).then((s) => {
        if (s) setActiveSource(s);
      });
    }
  }, [lib]);

  React.useEffect(() => {
    if (autoPlayed.current || !lib.videos.length) return;
    if (preferredPath) {
      const idx = lib.videos.findIndex((v) => v.id === preferredPath);
      if (idx >= 0) { openVideo(idx); autoPlayed.current = true; }
    }
  }, [lib.videos, preferredPath, openVideo]);

  return (
    <div className={`videos${videoDrop.isDragging ? " videos--drop-target" : ""}`}
      {...videoDrop.dragProps}>
      <div className="videos__main">
        <div className="videos__header">
          <div className="videos__headerTitle">Videos</div>
          <span className="videos__headerCount">{lib.videos.length} items</span>
        </div>
        <VideoGrid
          videos={lib.videos}
          loading={lib.loading}
          onOpen={(id) => {
            const idx = lib.videos.findIndex((v) => v.id === id);
            if (idx >= 0) openVideo(idx);
          }}
        />
      </div>

      {fsVideo && (
        <VideoFullscreen
          video={fsVideo}
          index={fsIndex!}
          total={lib.videos.length}
          source={activeSource}
          startAt={lib.getSavedProgress(fsVideo.id)}
          onPrev={() => {
            const prevIdx = Math.max(0, (fsIndex ?? 0) - 1);
            openVideo(prevIdx);
          }}
          onNext={() => {
            const nextIdx = Math.min(lib.videos.length - 1, (fsIndex ?? 0) + 1);
            openVideo(nextIdx);
          }}
          onClose={() => { setFsIndex(null); setActiveSource(null); }}
          onProgress={(sec) => lib.savePlaybackProgress(fsVideo.id, sec)}
        />
      )}
    </div>
  );
}
