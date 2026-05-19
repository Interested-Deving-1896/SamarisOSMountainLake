import React from "react";
import { Maximize2, Pause, Play, Volume2, VolumeX } from "lucide-react";
import type { VideoAsset } from "../types";

function formatTime(seconds: number) {
  if (!Number.isFinite(seconds) || seconds <= 0) return "0:00";
  const minutes = Math.floor(seconds / 60);
  const rest = Math.floor(seconds % 60);
  return `${minutes}:${String(rest).padStart(2, "0")}`;
}

export function VideoPlayer(props: {
  video: VideoAsset | null;
  source: string | null;
  startAt: number;
  autoPlay: boolean;
  onProgress: (seconds: number) => void;
}) {
  const videoRef = React.useRef<HTMLVideoElement | null>(null);
  const [isPlaying, setIsPlaying] = React.useState(false);
  const [muted, setMuted] = React.useState(false);
  const [volume, setVolume] = React.useState(0.86);
  const [currentTime, setCurrentTime] = React.useState(0);
  const [duration, setDuration] = React.useState(0);

  React.useEffect(() => {
    const video = videoRef.current;
    if (!video || !props.source) return;
    video.src = props.source;
    video.load();
    const handleLoaded = () => {
      if (props.startAt > 0) {
        video.currentTime = props.startAt;
      }
      if (props.autoPlay) {
        void video.play().catch(() => {});
      }
    };
    video.addEventListener("loadedmetadata", handleLoaded, { once: true });
    return () => video.removeEventListener("loadedmetadata", handleLoaded);
  }, [props.autoPlay, props.source, props.startAt]);

  React.useEffect(() => {
    const video = videoRef.current;
    if (!video) return;
    video.volume = volume;
    video.muted = muted;
  }, [muted, volume]);

  React.useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => {
      if (!props.source) return;
      if (event.key === " ") {
        event.preventDefault();
        void togglePlayback();
      } else if (event.key.toLowerCase() === "m") {
        setMuted((current) => !current);
      } else if (event.key === "ArrowRight") {
        seekTo(currentTime + 10);
      } else if (event.key === "ArrowLeft") {
        seekTo(currentTime - 10);
      }
    };
    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [currentTime, props.source]);

  const togglePlayback = React.useCallback(async () => {
    const video = videoRef.current;
    if (!video) return;
    if (video.paused) {
      await video.play().catch(() => {});
      setIsPlaying(true);
    } else {
      video.pause();
      setIsPlaying(false);
    }
  }, []);

  const seekTo = React.useCallback(
    (seconds: number) => {
      const video = videoRef.current;
      if (!video) return;
      const next = Math.min(Math.max(0, seconds), Number.isFinite(video.duration) ? video.duration : seconds);
      video.currentTime = next;
      setCurrentTime(next);
      props.onProgress(next);
    },
    [props]
  );

  if (!props.video) {
    return <div className="videos__empty videos__playerEmpty">Select a video from your library.</div>;
  }

  return (
    <section className="videos__playerShell">
      <div className="videos__playerFrame">
        <video
          ref={videoRef}
          className="videos__player"
          playsInline
          onPlay={() => setIsPlaying(true)}
          onPause={() => setIsPlaying(false)}
          onTimeUpdate={(event) => {
            const next = event.currentTarget.currentTime || 0;
            setCurrentTime(next);
            props.onProgress(next);
          }}
          onLoadedMetadata={(event) => setDuration(event.currentTarget.duration || 0)}
        />
      </div>
      <div className="videos__playerMeta">
        <div>
          <div className="videos__playerTitle">{props.video.title}</div>
          <div className="videos__playerSubline">{props.video.fileName}</div>
        </div>
        <button
          type="button"
          className="videos__playerAction"
          onClick={() => {
            const shell = videoRef.current?.parentElement;
            if (!shell) return;
            void shell.requestFullscreen?.();
          }}
          aria-label="Full screen"
        >
          <Maximize2 size={14} strokeWidth={2.2} />
        </button>
      </div>
      <div className="videos__controls">
        <button type="button" className="videos__transport" onClick={() => void togglePlayback()}>
          {isPlaying ? <Pause size={16} strokeWidth={2.1} /> : <Play size={16} strokeWidth={2.1} />}
        </button>
        <span className="videos__time">{formatTime(currentTime)}</span>
        <input
          type="range"
          min={0}
          max={Math.max(duration, 1)}
          step={0.1}
          value={Math.min(currentTime, duration || 0)}
          className="videos__slider"
          onChange={(event) => seekTo(Number(event.target.value))}
        />
        <span className="videos__time">{formatTime(duration)}</span>
        <button type="button" className="videos__transport" onClick={() => setMuted((current) => !current)}>
          {muted ? <VolumeX size={15} strokeWidth={2.1} /> : <Volume2 size={15} strokeWidth={2.1} />}
        </button>
        <input
          type="range"
          min={0}
          max={1}
          step={0.01}
          value={volume}
          className="videos__volume"
          onChange={(event) => setVolume(Number(event.target.value))}
        />
      </div>
    </section>
  );
}
