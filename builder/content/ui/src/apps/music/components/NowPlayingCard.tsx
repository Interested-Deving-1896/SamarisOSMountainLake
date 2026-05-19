import React from "react";
import {
  Expand,
  Heart,
  HeartOff,
  Music4,
  Pause,
  Play,
  Repeat,
  Repeat1,
  Shuffle,
  SkipBack,
  SkipForward,
  Volume2
} from "lucide-react";
import type { MusicPlaylist, MusicTrack, RepeatMode } from "../types";

function formatTime(seconds: number) {
  if (!Number.isFinite(seconds) || seconds <= 0) return "--:--";
  const minutes = Math.floor(seconds / 60);
  const rest = Math.floor(seconds % 60);
  return `${minutes}:${String(rest).padStart(2, "0")}`;
}

export function NowPlayingCard(props: {
  track: MusicTrack | null;
  isPlaying: boolean;
  loading: boolean;
  currentTime: number;
  duration: number;
  volume: number;
  shuffle: boolean;
  repeat: RepeatMode;
  liked: boolean;
  playlists: MusicPlaylist[];
  fullscreen: boolean;
  onTogglePlayback: () => void;
  onPrevious: () => void;
  onNext: () => void;
  onSeek: (seconds: number) => void;
  onVolumeChange: (volume: number) => void;
  onToggleShuffle: () => void;
  onCycleRepeat: () => void;
  onToggleLike: () => void;
  onToggleFullscreen: () => void;
  onAddToPlaylist: (playlistId: string) => void;
}) {
  const [playlistTarget, setPlaylistTarget] = React.useState<string>("");
  const playlistTargetRef = React.useRef(playlistTarget);
  playlistTargetRef.current = playlistTarget;

  React.useEffect(() => {
    if (!props.playlists.length) {
      setPlaylistTarget("");
      return;
    }
    const stillExists = props.playlists.some((playlist) => playlist.id === playlistTargetRef.current);
    if (!playlistTargetRef.current || !stillExists) {
      setPlaylistTarget(props.playlists[0].id);
    }
  }, [props.playlists]);

  const progress = props.duration > 0 ? Math.min(100, (props.currentTime / props.duration) * 100) : 0;

  return (
    <aside className={`music__nowPlaying ${props.fullscreen ? "music__nowPlaying--fullscreen" : ""}`}>
      <div className="music__cover">
        {props.track?.coverDataUrl ? (
          <img src={props.track.coverDataUrl} alt="" className="music__coverImage" />
        ) : (
          <div className="music__coverDisc">
            <Music4 size={32} strokeWidth={2.1} />
          </div>
        )}
      </div>

      <div className="music__nowMeta">
        <div className="music__eyebrow">Now Playing</div>
        <div className="music__trackHeadline">{props.track?.title || "Nothing selected"}</div>
        <div className="music__trackSubline">
          {props.track ? `${props.track.artist} • ${props.track.album}` : "Choose a song from the library."}
        </div>
        {props.track?.genre ? <div className="music__trackCaption">{props.track.genre}</div> : null}
      </div>

      <div className="music__playerControls">
        <button type="button" className="music__transportBtn" onClick={props.onPrevious} aria-label="Previous track">
          <SkipBack size={15} strokeWidth={2.2} />
        </button>
        <button type="button" className="music__transportBtn music__transportBtn--primary" onClick={props.onTogglePlayback} aria-label="Play or pause">
          {props.isPlaying ? <Pause size={18} strokeWidth={2.2} /> : <Play size={18} strokeWidth={2.2} />}
        </button>
        <button type="button" className="music__transportBtn" onClick={props.onNext} aria-label="Next track">
          <SkipForward size={15} strokeWidth={2.2} />
        </button>
      </div>

      <div className="music__progress">
        <input
          type="range"
          min={0}
          max={Math.max(props.duration, 1)}
          step={0.1}
          value={Math.min(props.currentTime, props.duration || 0)}
          className="music__progressSlider"
          onChange={(event) => props.onSeek(Number(event.target.value))}
        />
        <div className="music__progressMeta">
          <span>{props.loading ? "Loading audio…" : props.isPlaying ? formatTime(props.currentTime) : "Paused"}</span>
          <span>{formatTime(props.duration || props.track?.durationSeconds || 0)}</span>
        </div>
      </div>

      <div className="music__playerOptions">
        <button
          type="button"
          className={`music__miniAction ${props.shuffle ? "music__miniAction--active" : ""}`}
          onClick={props.onToggleShuffle}
          aria-label="Toggle shuffle"
        >
          <Shuffle size={14} strokeWidth={2.2} />
        </button>
        <button
          type="button"
          className={`music__miniAction ${props.repeat !== "off" ? "music__miniAction--active" : ""}`}
          onClick={props.onCycleRepeat}
          aria-label="Cycle repeat mode"
        >
          {props.repeat === "one" ? <Repeat1 size={14} strokeWidth={2.2} /> : <Repeat size={14} strokeWidth={2.2} />}
        </button>
        <button
          type="button"
          className={`music__miniAction ${props.liked ? "music__miniAction--active" : ""}`}
          onClick={props.onToggleLike}
          aria-label="Like track"
        >
          {props.liked ? <Heart size={14} strokeWidth={2.2} /> : <HeartOff size={14} strokeWidth={2.2} />}
        </button>
        <button type="button" className="music__miniAction" onClick={props.onToggleFullscreen} aria-label="Toggle full player">
          <Expand size={14} strokeWidth={2.2} />
        </button>
      </div>

      <div className="music__volumeRow">
        <Volume2 size={14} strokeWidth={2.1} />
        <input
          type="range"
          min={0}
          max={1}
          step={0.01}
          value={props.volume}
          className="music__volumeSlider"
          onChange={(event) => props.onVolumeChange(Number(event.target.value))}
        />
      </div>

      <div className="music__playlistAdd">
        <select value={playlistTarget} onChange={(event) => setPlaylistTarget(event.target.value)}>
          <option value="" disabled>
            Add to playlist
          </option>
          {props.playlists.map((playlist) => (
            <option key={playlist.id} value={playlist.id}>
              {playlist.name}
            </option>
          ))}
        </select>
        <button type="button" className="music__toolbarIconBtn" disabled={!playlistTarget || !props.track} onClick={() => props.onAddToPlaylist(playlistTarget)}>
          Add
        </button>
      </div>
    </aside>
  );
}
