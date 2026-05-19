import { useSyncExternalStore } from "react";
import type { EqualizerPreset, MusicPlaylist, MusicSort, MusicViewMode, RepeatMode } from "../types";

const STORAGE_KEY = "samaris-os/music-state";

type MusicStoreState = {
  playlists: MusicPlaylist[];
  likedTrackPaths: string[];
  viewMode: MusicViewMode;
  sortBy: MusicSort;
  equalizerPreset: EqualizerPreset;
  shuffle: boolean;
  repeat: RepeatMode;
  fullPlayer: boolean;
};

const DEFAULT_STATE: MusicStoreState = {
  playlists: [],
  likedTrackPaths: [],
  viewMode: "list",
  sortBy: "title",
  equalizerPreset: "flat",
  shuffle: false,
  repeat: "off",
  fullPlayer: false
};

type Listener = () => void;

class MusicStore {
  private state: MusicStoreState = DEFAULT_STATE;
  private listeners = new Set<Listener>();

  constructor() {
    this.state = this.load();
  }

  private load(): MusicStoreState {
    try {
      const raw = globalThis.localStorage?.getItem(STORAGE_KEY);
      if (!raw) return DEFAULT_STATE;
      return {
        ...DEFAULT_STATE,
        ...JSON.parse(raw)
      } satisfies MusicStoreState;
    } catch {
      return DEFAULT_STATE;
    }
  }

  private persist() {
    try {
      globalThis.localStorage?.setItem(STORAGE_KEY, JSON.stringify(this.state));
    } catch {}
  }

  subscribe(listener: Listener) {
    this.listeners.add(listener);
    return () => this.listeners.delete(listener);
  }

  snapshot() {
    return this.state;
  }

  private patch(next: Partial<MusicStoreState>) {
    this.state = {
      ...this.state,
      ...next
    };
    this.persist();
    for (const listener of this.listeners) listener();
  }

  createPlaylist(name: string) {
    const trimmed = name.trim();
    if (!trimmed) return null;
    const playlist: MusicPlaylist = {
      id: `playlist-${Date.now()}`,
      name: trimmed,
      trackPaths: [],
      createdAt: Date.now(),
      updatedAt: Date.now()
    };
    this.patch({ playlists: [...this.state.playlists, playlist] });
    return playlist;
  }

  deletePlaylist(id: string) {
    this.patch({ playlists: this.state.playlists.filter((playlist) => playlist.id !== id) });
  }

  renamePlaylist(id: string, name: string) {
    const trimmed = name.trim();
    if (!trimmed) return;
    this.patch({
      playlists: this.state.playlists.map((playlist) =>
        playlist.id === id ? { ...playlist, name: trimmed, updatedAt: Date.now() } : playlist
      )
    });
  }

  addTrackToPlaylist(playlistId: string, trackPath: string) {
    this.patch({
      playlists: this.state.playlists.map((playlist) => {
        if (playlist.id !== playlistId) return playlist;
        if (playlist.trackPaths.includes(trackPath)) return playlist;
        return {
          ...playlist,
          trackPaths: [...playlist.trackPaths, trackPath],
          updatedAt: Date.now()
        };
      })
    });
  }

  removeTrackFromPlaylist(playlistId: string, trackPath: string) {
    this.patch({
      playlists: this.state.playlists.map((playlist) =>
        playlist.id === playlistId
          ? {
              ...playlist,
              trackPaths: playlist.trackPaths.filter((path) => path !== trackPath),
              updatedAt: Date.now()
            }
          : playlist
      )
    });
  }

  toggleLikedTrack(trackPath: string) {
    const likedTrackPaths = this.state.likedTrackPaths.includes(trackPath)
      ? this.state.likedTrackPaths.filter((path) => path !== trackPath)
      : [...this.state.likedTrackPaths, trackPath];
    this.patch({ likedTrackPaths });
  }

  setViewMode(viewMode: MusicViewMode) {
    this.patch({ viewMode });
  }

  setSortBy(sortBy: MusicSort) {
    this.patch({ sortBy });
  }

  setEqualizerPreset(equalizerPreset: EqualizerPreset) {
    this.patch({ equalizerPreset });
  }

  setShuffle(shuffle: boolean) {
    this.patch({ shuffle });
  }

  setRepeat(repeat: RepeatMode) {
    this.patch({ repeat });
  }

  setFullPlayer(fullPlayer: boolean) {
    this.patch({ fullPlayer });
  }
}

const store = new MusicStore();

export function useMusicStore() {
  return useSyncExternalStore(
    (listener) => store.subscribe(listener),
    () => store.snapshot()
  );
}

export const musicStoreActions = {
  createPlaylist: (name: string) => store.createPlaylist(name),
  deletePlaylist: (id: string) => store.deletePlaylist(id),
  renamePlaylist: (id: string, name: string) => store.renamePlaylist(id, name),
  addTrackToPlaylist: (playlistId: string, trackPath: string) => store.addTrackToPlaylist(playlistId, trackPath),
  removeTrackFromPlaylist: (playlistId: string, trackPath: string) =>
    store.removeTrackFromPlaylist(playlistId, trackPath),
  toggleLikedTrack: (trackPath: string) => store.toggleLikedTrack(trackPath),
  setViewMode: (viewMode: MusicViewMode) => store.setViewMode(viewMode),
  setSortBy: (sortBy: MusicSort) => store.setSortBy(sortBy),
  setEqualizerPreset: (preset: EqualizerPreset) => store.setEqualizerPreset(preset),
  setShuffle: (shuffle: boolean) => store.setShuffle(shuffle),
  setRepeat: (repeat: RepeatMode) => store.setRepeat(repeat),
  setFullPlayer: (fullPlayer: boolean) => store.setFullPlayer(fullPlayer)
};
