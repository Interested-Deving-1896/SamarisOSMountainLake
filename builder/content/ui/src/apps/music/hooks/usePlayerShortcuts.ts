import React from "react";
import type { MusicTrack } from "../types";

type ShortcutActions = {
  togglePlayback: () => void;
  nextTrack: () => void;
  previousTrack: () => void;
  seekTo: (seconds: number) => void;
  setVolume: (volume: number) => void;
  volume: number;
  isPlaying: boolean;
  currentTrack: MusicTrack | null;
  currentTime: number;
  duration: number;
};

const SEEK_STEP = 5;
const VOLUME_STEP = 0.05;

export function usePlayerShortcuts(actions: ShortcutActions) {
  React.useEffect(() => {
    function handleKeyDown(event: KeyboardEvent) {
      const target = event.target as HTMLElement;
      if (target.tagName === "INPUT" || target.tagName === "TEXTAREA" || target.tagName === "SELECT") return;
      if (target.isContentEditable) return;

      const ctrl = event.ctrlKey || event.metaKey;

      switch (event.code) {
        case "Space":
          event.preventDefault();
          actions.togglePlayback();
          break;
        case "ArrowLeft":
          if (ctrl) {
            actions.previousTrack();
          } else {
            event.preventDefault();
            actions.seekTo(Math.max(0, actions.currentTime - SEEK_STEP));
          }
          break;
        case "ArrowRight":
          if (ctrl) {
            actions.nextTrack();
          } else {
            event.preventDefault();
            actions.seekTo(Math.min(actions.duration, actions.currentTime + SEEK_STEP));
          }
          break;
        case "ArrowUp":
          event.preventDefault();
          actions.setVolume(Math.min(1, actions.volume + VOLUME_STEP));
          break;
        case "ArrowDown":
          event.preventDefault();
          actions.setVolume(Math.max(0, actions.volume - VOLUME_STEP));
          break;
      }
    }

    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [actions]);

  React.useEffect(() => {
    if (!("mediaSession" in navigator)) return;

    const handlers: [MediaSessionAction, MediaSessionActionHandler][] = [
      ["play", () => actions.togglePlayback()],
      ["pause", () => actions.togglePlayback()],
      ["nexttrack", () => actions.nextTrack()],
      ["previoustrack", () => actions.previousTrack()],
      ["seekforward", (details) => actions.seekTo(Math.min(actions.duration, (details.seekTime ?? actions.currentTime) + 10))],
      ["seekbackward", (details) => actions.seekTo(Math.max(0, (details.seekTime ?? actions.currentTime) - 10))]
    ];

    for (const [action, handler] of handlers) {
      try {
        navigator.mediaSession.setActionHandler(action, handler);
      } catch {}
    }

    return () => {
      for (const [action] of handlers) {
        try {
          navigator.mediaSession.setActionHandler(action, null);
        } catch {}
      }
    };
  }, [actions]);

  React.useEffect(() => {
    if (!("mediaSession" in navigator)) return;
    if (!actions.currentTrack) return;

    navigator.mediaSession.metadata = new MediaMetadata({
      title: actions.currentTrack.title,
      artist: actions.currentTrack.artist,
      album: actions.currentTrack.album,
      artwork: actions.currentTrack.coverDataUrl
        ? [{ src: actions.currentTrack.coverDataUrl, type: "image/jpeg" }]
        : []
    });

    navigator.mediaSession.playbackState = actions.isPlaying ? "playing" : "paused";
  }, [actions.currentTrack, actions.isPlaying]);
}
