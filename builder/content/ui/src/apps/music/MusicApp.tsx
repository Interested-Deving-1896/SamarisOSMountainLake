import React from "react";
import { MusicSidebar } from "./components/MusicSidebar";
import { MusicToolbar } from "./components/MusicToolbar";
import { MusicLibrary } from "./components/MusicLibrary";
import { NowPlayingCard } from "./components/NowPlayingCard";
import { useMusicLibrary } from "./hooks/useMusicLibrary";
import { usePlayback } from "./hooks/usePlayback";
import { musicStoreActions, useMusicStore } from "./store/useMusicStore";
import type { MusicAlbum, MusicSection, MusicTrack } from "./types";
import { ConfirmModal, PromptModal } from "../../components/PromptModal";
import { osStore } from "../../os/core/osStore";
import { useFileDrop } from "../shared/useFileDrop";
import { getDroppedFiles } from "../../os/filesystem/dragDrop";
import { useFs } from "../../services/fs/useFs";
import { commitFileDrop } from "../../os/dnd";
import { usePlayerShortcuts } from "./hooks/usePlayerShortcuts";

function sortTracks(tracks: MusicTrack[], sortBy: "title" | "artist" | "album" | "recent") {
  const next = tracks.slice();
  next.sort((left, right) => {
    if (sortBy === "recent") {
      return right.path.localeCompare(left.path);
    }
    return String(left[sortBy]).localeCompare(String(right[sortBy]), undefined, { sensitivity: "base" });
  });
  return next;
}

function buildAlbums(tracks: MusicTrack[]): MusicAlbum[] {
  const albums = new Map<string, MusicAlbum>();
  for (const track of tracks) {
    const key = `${track.album || "Unknown Album"}::${track.artist || "Unknown Artist"}`;
    const current =
      albums.get(key) ||
      {
        id: key,
        title: track.album || "Unknown Album",
        artist: track.artist || "Unknown Artist",
        year: track.year,
        coverDataUrl: track.coverDataUrl,
        tracks: []
      };
    current.tracks.push(track);
    current.coverDataUrl = current.coverDataUrl || track.coverDataUrl || null;
    current.year = current.year || track.year || null;
    albums.set(key, current);
  }
  return Array.from(albums.values()).sort((left, right) => left.title.localeCompare(right.title));
}

export function MusicApp(props: { windowId: string }) {
  const prefs = useMusicStore();
  const [section, setSection] = React.useState<MusicSection>("library");
  const [query, setQuery] = React.useState("");
  const [selectedAlbumId, setSelectedAlbumId] = React.useState<string | null>(null);
  const [selectedPlaylistId, setSelectedPlaylistId] = React.useState<string | null>(prefs.playlists[0]?.id || null);
  const [playlistPromptOpen, setPlaylistPromptOpen] = React.useState(false);
  const [playlistDeleteId, setPlaylistDeleteId] = React.useState<string | null>(null);
  const [playlistRenameId, setPlaylistRenameId] = React.useState<string | null>(null);
  const [isDropping, setIsDropping] = React.useState(false);
  const [importNotice, setImportNotice] = React.useState<string | null>(null);
  const [errorToast, setErrorToast] = React.useState<string | null>(null);
  const errorToastTimerRef = React.useRef<ReturnType<typeof window.setTimeout> | null>(null);

  function showError(message: string) {
    if (errorToastTimerRef.current) window.clearTimeout(errorToastTimerRef.current);
    setErrorToast(message);
    errorToastTimerRef.current = window.setTimeout(() => setErrorToast(null), 5000);
  }

  const consumedSelectedPathRef = React.useRef<string | null>(null);

  const fs = useFs();
  const state = React.useSyncExternalStore(
    (listener) => osStore.subscribe(listener),
    () => osStore.getState()
  );
  const selectedPath = state.windows.find((window) => window.id === props.windowId)?.params?.path as string | undefined;

  const { tracks, loading, importing, refresh, importFiles, ensureTrackSource } = useMusicLibrary(selectedPath);

  const musicDrop = useFileDrop({
    accept: [".mp3", ".wav", ".flac", ".ogg", ".m4a", ".aac", ".wma", ".opus", ".weba"],
    target: { id: "music-library", label: "Music", path: "/User/Music", kind: "app" },
    allowedChoices: ["import", "copy"],
    recommendedAction: "import",
    onDrop: async (_files, context) => {
      const result = await commitFileDrop(fs, context.plan, { ...context.decision, choice: context.decision.choice === "import" ? "copy" : context.decision.choice });
      const count = result.completed.length;
      if (count > 0) {
        setImportNotice(`${count} file${count > 1 ? "s" : ""} imported to your library.`);
        window.setTimeout(() => setImportNotice(null), 4000);
        void refresh();
      }
    }
  });

  const isAnyDragging = isDropping || musicDrop.isDragging;

  React.useEffect(() => {
    if (!prefs.playlists.some((playlist) => playlist.id === selectedPlaylistId)) {
      setSelectedPlaylistId(prefs.playlists[0]?.id || null);
    }
  }, [prefs.playlists, selectedPlaylistId]);

  const filteredTracks = React.useMemo(() => {
    const normalized = query.trim().toLowerCase();
    const sorted = sortTracks(tracks, prefs.sortBy);
    if (!normalized) return sorted;
    return sorted.filter((track) =>
      [track.title, track.artist, track.album, track.genre, track.fileName]
        .filter(Boolean)
        .some((value) => value.toLowerCase().includes(normalized))
    );
  }, [prefs.sortBy, query, tracks]);

  const albums = React.useMemo(() => buildAlbums(filteredTracks), [filteredTracks]);
  const selectedAlbum = albums.find((album) => album.id === selectedAlbumId) || null;
  const selectedPlaylist = prefs.playlists.find((playlist) => playlist.id === selectedPlaylistId) || null;
  const playlistTracks = React.useMemo(() => {
    if (!selectedPlaylist) return [];
    const lookup = new Map(tracks.map((track) => [track.path, track]));
    return selectedPlaylist.trackPaths.map((path) => lookup.get(path)).filter(Boolean) as MusicTrack[];
  }, [selectedPlaylist, tracks]);

  const queue = React.useMemo(() => {
    if (section === "albums" && selectedAlbum) return selectedAlbum.tracks;
    if (section === "playlists") return playlistTracks;
    return filteredTracks;
  }, [filteredTracks, playlistTracks, section, selectedAlbum]);

  const playback = usePlayback({
    queue,
    resolveTrackSource: ensureTrackSource,
    shuffle: prefs.shuffle,
    repeat: prefs.repeat,
    equalizerPreset: prefs.equalizerPreset,
    initialTrackId: selectedPath
  });

  const activeTrack = tracks.find((track) => track.id === playback.currentTrackId) || playback.currentTrack || null;
  const activeTrackLiked = activeTrack ? prefs.likedTrackPaths.includes(activeTrack.path) : false;

  usePlayerShortcuts(React.useMemo(() => ({
    togglePlayback: playback.togglePlayback,
    nextTrack: playback.nextTrack,
    previousTrack: playback.previousTrack,
    seekTo: playback.seekTo,
    setVolume: playback.setVolume,
    volume: playback.volume,
    isPlaying: playback.isPlaying,
    currentTrack: activeTrack,
    currentTime: playback.currentTime,
    duration: playback.duration || activeTrack?.durationSeconds || 0
  }), [playback.togglePlayback, playback.nextTrack, playback.previousTrack, playback.seekTo, playback.setVolume, playback.volume, playback.isPlaying, activeTrack, playback.currentTime, playback.duration]));

  React.useEffect(() => {
    if (!selectedPath) return;
    if (consumedSelectedPathRef.current === selectedPath) return;
    consumedSelectedPathRef.current = selectedPath;
    void playback.playTrack(selectedPath);
  }, [playback.playTrack, selectedPath, tracks.length]);

  async function handleImport(fileList: FileList | File[] | null) {
    if (!fileList || fileList.length === 0) return;
    try {
      const count = await importFiles(Array.from(fileList));
      if (count > 0) {
        setImportNotice(`${count} file${count > 1 ? "s" : ""} imported to your library.`);
        window.setTimeout(() => setImportNotice(null), 4000);
        void refresh();
      } else {
        showError("No supported audio files found in that selection.");
      }
    } catch {
      showError("Failed to import files. Check the file format and try again.");
    }
  }

  return (
    <div
      className={`music ${prefs.fullPlayer ? "music--fullscreenPlayer" : ""} ${isAnyDragging ? "music--dropping" : ""}`}
      {...musicDrop.dragProps}
      onDragOver={(event) => {
        event.preventDefault();
        if (event.dataTransfer?.types.includes("Files")) {
          setIsDropping(true);
        }
      }}
      onDragLeave={(event) => {
        if (event.target === event.currentTarget) {
          setIsDropping(false);
        }
      }}
      onDrop={async (event) => {
        setIsDropping(false);
        const samarisFiles = getDroppedFiles(event.dataTransfer);
        if (samarisFiles.length > 0 || event.dataTransfer?.types.includes("Files")) {
          musicDrop.dragProps.onDrop(event);
        } else {
          event.preventDefault();
          void handleImport(event.dataTransfer?.files || null);
        }
      }}
    >
      <MusicSidebar
        activeSection={section}
        onSelectSection={(nextSection) => {
          setSection(nextSection);
          if (nextSection !== "albums") {
            setSelectedAlbumId(null);
          }
        }}
        trackCount={tracks.length}
        playlists={prefs.playlists}
        selectedPlaylistId={selectedPlaylistId}
        onSelectPlaylist={(playlistId) => {
          setSection("playlists");
          setSelectedPlaylistId(playlistId);
        }}
        onCreatePlaylist={() => setPlaylistPromptOpen(true)}
        onDeletePlaylist={(playlistId) => setPlaylistDeleteId(playlistId)}
        onRenamePlaylist={(playlistId) => setPlaylistRenameId(playlistId)}
        onOpenPlayer={() => musicStoreActions.setFullPlayer(true)}
        canDeletePlaylist={Boolean(selectedPlaylist)}
        selectedPlaylistName={selectedPlaylist?.name}
      />

      <div className="music__main">
        <MusicToolbar
          query={query}
          onQueryChange={setQuery}
          sortBy={prefs.sortBy}
          onSortChange={musicStoreActions.setSortBy}
          viewMode={prefs.viewMode}
          onViewModeChange={musicStoreActions.setViewMode}
          equalizerPreset={prefs.equalizerPreset}
          onEqualizerChange={musicStoreActions.setEqualizerPreset}
          onImportRequest={(files) => void handleImport(files)}
          fullscreenPlayer={prefs.fullPlayer}
          onToggleFullscreenPlayer={() => musicStoreActions.setFullPlayer(!prefs.fullPlayer)}
          resultLabel={
            section === "albums"
              ? `${albums.length} album${albums.length > 1 ? "s" : ""}`
              : `${queue.length} track${queue.length > 1 ? "s" : ""}`
          }
        />

        <MusicLibrary
          loading={loading}
          importing={importing}
          viewMode={prefs.viewMode}
          section={section}
          tracks={queue}
          albums={albums}
          activeTrackId={playback.currentTrackId}
          selectedAlbumId={selectedAlbumId}
          onSelectTrack={(trackId) => playback.setCurrentTrackId(trackId)}
          onPlayTrack={(trackId) => void playback.playTrack(trackId)}
          onSelectAlbum={(albumId) => {
            setSelectedAlbumId(albumId);
            const album = albums.find((entry) => entry.id === albumId);
            if (album?.tracks[0]) {
              void playback.playTrack(album.tracks[0].id);
            }
          }}
          onBackFromAlbum={() => setSelectedAlbumId(null)}
          onRemoveFromPlaylist={(trackPath) => {
            if (!selectedPlaylistId) return;
            musicStoreActions.removeTrackFromPlaylist(selectedPlaylistId, trackPath);
          }}
          emptyLabel={
            section === "playlists"
              ? "Create a playlist or add the current song to one from the player."
              : "Drop audio files into Music or import them from the toolbar."
          }
          importNotice={importNotice}
        />
      </div>

      <NowPlayingCard
        track={activeTrack}
        isPlaying={playback.isPlaying}
        loading={playback.loading}
        currentTime={playback.currentTime}
        duration={playback.duration || activeTrack?.durationSeconds || 0}
        volume={playback.volume}
        shuffle={prefs.shuffle}
        repeat={prefs.repeat}
        liked={activeTrackLiked}
        playlists={prefs.playlists}
        onTogglePlayback={playback.togglePlayback}
        onPrevious={playback.previousTrack}
        onNext={playback.nextTrack}
        onSeek={playback.seekTo}
        onVolumeChange={playback.setVolume}
        onToggleShuffle={() => musicStoreActions.setShuffle(!prefs.shuffle)}
        onCycleRepeat={() =>
          musicStoreActions.setRepeat(
            prefs.repeat === "off" ? "all" : prefs.repeat === "all" ? "one" : "off"
          )
        }
        onToggleLike={() => {
          if (activeTrack) musicStoreActions.toggleLikedTrack(activeTrack.path);
        }}
        onToggleFullscreen={() => musicStoreActions.setFullPlayer(!prefs.fullPlayer)}
        fullscreen={prefs.fullPlayer}
        onAddToPlaylist={(playlistId) => {
          if (!activeTrack) return;
          musicStoreActions.addTrackToPlaylist(playlistId, activeTrack.path);
          setSelectedPlaylistId(playlistId);
        }}
      />

      {isDropping ? <div className="music__dropOverlay">Drop audio files to import them into Samaris Music</div> : null}

      {errorToast ? <div className="music__errorToast">{errorToast}</div> : null}

      {playlistPromptOpen ? (
        <PromptModal
          title="New playlist"
          subtitle="Create a playlist for your library."
          placeholder="Late night drive"
          confirmLabel="Create"
          onCancel={() => setPlaylistPromptOpen(false)}
          onConfirm={(value) => {
            const playlist = musicStoreActions.createPlaylist(value);
            setPlaylistPromptOpen(false);
            if (playlist) {
              setSection("playlists");
              setSelectedPlaylistId(playlist.id);
            }
          }}
        />
      ) : null}

      {playlistDeleteId ? (
        <ConfirmModal
          title="Delete playlist"
          subtitle="The songs stay in your library. Only the playlist is removed."
          confirmLabel="Delete"
          danger
          onCancel={() => setPlaylistDeleteId(null)}
          onConfirm={() => {
            musicStoreActions.deletePlaylist(playlistDeleteId);
            setPlaylistDeleteId(null);
          }}
        />
      ) : null}

      {playlistRenameId ? (
        <PromptModal
          title="Rename playlist"
          subtitle="Give your playlist a new name."
          placeholder={prefs.playlists.find((p) => p.id === playlistRenameId)?.name || ""}
          confirmLabel="Rename"
          onCancel={() => setPlaylistRenameId(null)}
          onConfirm={(value) => {
            musicStoreActions.renamePlaylist(playlistRenameId, value);
            setPlaylistRenameId(null);
          }}
        />
      ) : null}
    </div>
  );
}
