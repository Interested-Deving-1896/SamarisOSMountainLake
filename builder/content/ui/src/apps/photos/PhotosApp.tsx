import React from "react";
import { PhotoGrid } from "./components/PhotoGrid";
import { PhotosSidebar } from "./components/PhotosSidebar";
import { PhotoFullscreen } from "./components/PhotoFullscreen";
import { PhotoToolbar } from "./components/PhotoToolbar";
import { usePhotosLibrary } from "./hooks/usePhotosLibrary";
import { useFileDrop } from "../shared/useFileDrop";
import { useFs } from "../../services/fs/useFs";
import { osStore } from "../../os/core/osStore";
import { commitFileDrop } from "../../os/dnd";

export function PhotosApp(props: { windowId: string }) {
  const fs = useFs();
  const selectedPath = React.useSyncExternalStore(
    (listener) => osStore.subscribe(listener),
    () => osStore.getState().windows.find((w) => w.id === props.windowId)?.params?.path as string | undefined
  );
  const lib = usePhotosLibrary(selectedPath);
  const photoDrop = useFileDrop({
    accept: [".jpg", ".jpeg", ".png", ".gif", ".webp", ".heic", ".heif", ".bmp", ".svg", ".tiff", ".tif"],
    target: { id: "photos-library", label: "Photos", path: "/User/Photos", kind: "app" },
    allowedChoices: ["import", "copy"],
    recommendedAction: "import",
    onDrop: async (_files, context) => {
      await commitFileDrop(fs, context.plan, { ...context.decision, choice: context.decision.choice === "import" ? "copy" : context.decision.choice });
      lib.refresh();
    }
  });

  const [slideshowActive, setSlideshowActive] = React.useState(false);
  const [fsIndex, setFsIndex] = React.useState<number | null>(null);
  const intervalRef = React.useRef<number | null>(null);
  const photosRef = React.useRef(lib.allPhotos);
  photosRef.current = lib.allPhotos;

  React.useEffect(() => {
    if (!slideshowActive) {
      if (intervalRef.current) { clearInterval(intervalRef.current); intervalRef.current = null; }
      return;
    }
    intervalRef.current = window.setInterval(() => {
      setFsIndex((i) => {
        const len = photosRef.current.length;
        if (len === 0) return 0;
        return i !== null ? (i + 1) % len : 0;
      });
    }, 3000);
    return () => { if (intervalRef.current) clearInterval(intervalRef.current); };
  }, [slideshowActive]);

  React.useEffect(() => {
    const handleKey = (e: KeyboardEvent) => {
      if (fsIndex === null) return;
      const photos = photosRef.current;
      if (e.key === "Escape") { setFsIndex(null); setSlideshowActive(false); return; }
      if (e.key === "ArrowLeft") { e.preventDefault(); setFsIndex((i) => Math.max(0, (i ?? 0) - 1)); }
      if (e.key === "ArrowRight") { e.preventDefault(); setFsIndex((i) => Math.min(photos.length - 1, (i ?? 0) + 1)); }
    };
    window.addEventListener("keydown", handleKey);
    return () => window.removeEventListener("keydown", handleKey);
  }, [fsIndex]);

  const fsPhoto = fsIndex !== null && lib.allPhotos[fsIndex] ? lib.allPhotos[fsIndex] : null;

  return (
    <div className={`photos${photoDrop.isDragging ? " photos--drop-target" : ""}`}
      {...photoDrop.dragProps}>
      <PhotosSidebar
        photoCount={lib.allPhotos.length}
        sourcePath={lib.sourcePath}
        viewFilter={lib.viewFilter}
        onViewChange={lib.setViewFilter}
      />
      <div className="photos__main">
        <PhotoToolbar
          photoCount={lib.allPhotos.length}
          searchQuery={lib.searchQuery}
          sortMode={lib.sortMode}
          onSearchChange={lib.setSearchQuery}
          onSortChange={lib.setSortMode}
          onSlideshow={() => {
            setSlideshowActive((a) => !a);
            if (fsIndex === null && lib.allPhotos.length > 0) setFsIndex(0);
          }}
          sourcePath={lib.sourcePath}
        />
        <PhotoGrid
          photos={lib.allPhotos}
          loading={lib.loading}
          error={lib.error}
          onRetry={lib.refresh}
          onOpen={(id) => {
            const idx = lib.allPhotos.findIndex((p) => p.id === id);
            if (idx >= 0) setFsIndex(idx);
          }}
          loadDataUrl={lib.loadDataUrl}
        />
      </div>

      {fsPhoto && (
        <PhotoFullscreen
          photo={fsPhoto}
          index={fsIndex!}
          total={lib.allPhotos.length}
          onPrev={() => setFsIndex((i) => Math.max(0, (i ?? 0) - 1))}
          onNext={() => setFsIndex((i) => Math.min(lib.allPhotos.length - 1, (i ?? 0) + 1))}
          onClose={() => { setFsIndex(null); setSlideshowActive(false); }}
        />
      )}
    </div>
  );
}
