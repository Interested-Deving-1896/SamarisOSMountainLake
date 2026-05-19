import { describe, it, expect, beforeEach } from "vitest";
import { renderHook } from "@testing-library/react";

beforeEach(() => {
  localStorage.clear();
  vi.resetModules();
});

describe("MusicStore", () => {
  it("starts with default state", async () => {
    const { useMusicStore } = await import("./useMusicStore");
    const { result } = renderHook(() => useMusicStore());
    expect(result.current.playlists).toEqual([]);
    expect(result.current.likedTrackPaths).toEqual([]);
    expect(result.current.viewMode).toBe("list");
    expect(result.current.sortBy).toBe("title");
    expect(result.current.equalizerPreset).toBe("flat");
    expect(result.current.shuffle).toBe(false);
    expect(result.current.repeat).toBe("off");
    expect(result.current.fullPlayer).toBe(false);
  });

  it("createPlaylist adds a playlist", async () => {
    const { useMusicStore, musicStoreActions } = await import("./useMusicStore");
    musicStoreActions.createPlaylist("Test");
    const { result } = renderHook(() => useMusicStore());
    expect(result.current.playlists).toHaveLength(1);
    expect(result.current.playlists[0].name).toBe("Test");
  });

  it("createPlaylist returns null for empty name", async () => {
    const { useMusicStore, musicStoreActions } = await import("./useMusicStore");
    expect(musicStoreActions.createPlaylist("  ")).toBeNull();
    const { result } = renderHook(() => useMusicStore());
    expect(result.current.playlists).toHaveLength(0);
  });

  it("deletePlaylist removes a playlist", async () => {
    const { useMusicStore, musicStoreActions } = await import("./useMusicStore");
    const playlist = musicStoreActions.createPlaylist("To Delete")!;
    expect(renderHook(() => useMusicStore()).result.current.playlists).toHaveLength(1);
    musicStoreActions.deletePlaylist(playlist.id);
    expect(renderHook(() => useMusicStore()).result.current.playlists).toHaveLength(0);
  });

  it("renamePlaylist updates the name", async () => {
    const { useMusicStore, musicStoreActions } = await import("./useMusicStore");
    const playlist = musicStoreActions.createPlaylist("Old")!;
    musicStoreActions.renamePlaylist(playlist.id, "New");
    expect(renderHook(() => useMusicStore()).result.current.playlists[0].name).toBe("New");
  });

  it("renamePlaylist ignores empty name", async () => {
    const { useMusicStore, musicStoreActions } = await import("./useMusicStore");
    const playlist = musicStoreActions.createPlaylist("Old")!;
    musicStoreActions.renamePlaylist(playlist.id, "  ");
    expect(renderHook(() => useMusicStore()).result.current.playlists[0].name).toBe("Old");
  });

  it("addTrackToPlaylist and removeTrackFromPlaylist", async () => {
    const { useMusicStore, musicStoreActions } = await import("./useMusicStore");
    const playlist = musicStoreActions.createPlaylist("Test")!;
    musicStoreActions.addTrackToPlaylist(playlist.id, "/path/to/song.mp3");
    expect(renderHook(() => useMusicStore()).result.current.playlists[0].trackPaths).toEqual(["/path/to/song.mp3"]);

    musicStoreActions.addTrackToPlaylist(playlist.id, "/path/to/song.mp3");
    expect(renderHook(() => useMusicStore()).result.current.playlists[0].trackPaths).toHaveLength(1);

    musicStoreActions.removeTrackFromPlaylist(playlist.id, "/path/to/song.mp3");
    expect(renderHook(() => useMusicStore()).result.current.playlists[0].trackPaths).toEqual([]);
  });

  it("toggleLikedTrack adds and removes paths", async () => {
    const { useMusicStore, musicStoreActions } = await import("./useMusicStore");
    musicStoreActions.toggleLikedTrack("/path/1.mp3");
    expect(renderHook(() => useMusicStore()).result.current.likedTrackPaths).toContain("/path/1.mp3");

    musicStoreActions.toggleLikedTrack("/path/2.mp3");
    expect(renderHook(() => useMusicStore()).result.current.likedTrackPaths).toHaveLength(2);

    musicStoreActions.toggleLikedTrack("/path/1.mp3");
    expect(renderHook(() => useMusicStore()).result.current.likedTrackPaths).toEqual(["/path/2.mp3"]);
  });

  it("setViewMode updates view mode", async () => {
    const { useMusicStore, musicStoreActions } = await import("./useMusicStore");
    musicStoreActions.setViewMode("grid");
    expect(renderHook(() => useMusicStore()).result.current.viewMode).toBe("grid");
  });

  it("setSortBy updates sort", async () => {
    const { useMusicStore, musicStoreActions } = await import("./useMusicStore");
    musicStoreActions.setSortBy("artist");
    expect(renderHook(() => useMusicStore()).result.current.sortBy).toBe("artist");
  });

  it("setEqualizerPreset updates preset", async () => {
    const { useMusicStore, musicStoreActions } = await import("./useMusicStore");
    musicStoreActions.setEqualizerPreset("bass-boost");
    expect(renderHook(() => useMusicStore()).result.current.equalizerPreset).toBe("bass-boost");
  });

  it("setShuffle toggles shuffle", async () => {
    const { useMusicStore, musicStoreActions } = await import("./useMusicStore");
    musicStoreActions.setShuffle(true);
    expect(renderHook(() => useMusicStore()).result.current.shuffle).toBe(true);
  });

  it("setRepeat cycles repeat", async () => {
    const { useMusicStore, musicStoreActions } = await import("./useMusicStore");
    musicStoreActions.setRepeat("one");
    expect(renderHook(() => useMusicStore()).result.current.repeat).toBe("one");
  });

  it("setFullPlayer toggles full player", async () => {
    const { useMusicStore, musicStoreActions } = await import("./useMusicStore");
    musicStoreActions.setFullPlayer(true);
    expect(renderHook(() => useMusicStore()).result.current.fullPlayer).toBe(true);
  });

  it("persists state to localStorage", async () => {
    const { musicStoreActions } = await import("./useMusicStore");
    musicStoreActions.createPlaylist("Persist Me");
    musicStoreActions.setShuffle(true);

    const raw = localStorage.getItem("samaris-os/music-state");
    expect(raw).not.toBeNull();
    const parsed = JSON.parse(raw!);
    expect(parsed.playlists).toHaveLength(1);
    expect(parsed.playlists[0].name).toBe("Persist Me");
    expect(parsed.shuffle).toBe(true);
  });

  it("recovers from corrupted localStorage", async () => {
    localStorage.setItem("samaris-os/music-state", "not-json");
    const { useMusicStore } = await import("./useMusicStore");
    const { result } = renderHook(() => useMusicStore());
    expect(result.current.playlists).toEqual([]);
  });
});
