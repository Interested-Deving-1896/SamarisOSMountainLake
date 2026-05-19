import React, { useEffect, useRef, useState } from "react";
import { X, Play, Pause, ChevronLeft, ChevronRight, Volume2, VolumeX, Loader } from "lucide-react";
import type { VideoAsset } from "../types";

function fmt(s: number) {
  if (!Number.isFinite(s) || s <= 0) return "0:00";
  const m = Math.floor(s / 60);
  const sc = Math.floor(s % 60);
  return `${m}:${String(sc).padStart(2, "0")}`;
}

export function VideoFullscreen(props: {
  video: VideoAsset | null;
  index: number;
  total: number;
  source: string | null;
  startAt: number;
  onPrev: () => void;
  onNext: () => void;
  onClose: () => void;
  onProgress: (seconds: number) => void;
}) {
  const vr = useRef<HTMLVideoElement | null>(null);
  const srcRef = useRef<string | null>(null);
  const [ready, setReady] = useState(false);
  const [paused, setPaused] = useState(true);
  const [ct, setCt] = useState(0);
  const [dur, setDur] = useState(0);
  const [muted, setMuted] = useState(true);
  const [showUI, setShowUI] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const hideTimer = useRef<number | null>(null);

  // Load source only when video ID or source URL changes
  useEffect(() => {
    const v = vr.current;
    if (!v || !props.source) return;
    if (srcRef.current === props.source) return; // same URL, skip reload

    srcRef.current = props.source;
    setReady(false);
    setError(null);
    v.muted = true;

    const onCanPlay = () => {
      setReady(true);
      setPaused(false);
      v.play().catch(() => {});
    };
    const onError = () => {
      setError("Unable to play this file.");
      setReady(true);
    };

    v.addEventListener("canplay", onCanPlay, { once: true });
    v.addEventListener("error", onError, { once: true });
    v.src = props.source;
    v.load();

    return () => {
      v.removeEventListener("canplay", onCanPlay);
      v.removeEventListener("error", onError);
    };
  }, [props.video?.id, props.source]);

  // Apply startAt once when ready
  const hasResumed = useRef(false);
  useEffect(() => {
    if (!ready || !vr.current || hasResumed.current) return;
    if (props.startAt > 0) vr.current.currentTime = props.startAt;
    hasResumed.current = true;
  }, [ready, props.startAt]);

  // Reset resume flag on video change
  useEffect(() => { hasResumed.current = false; }, [props.video?.id]);

  // Keep muted state in sync
  useEffect(() => { if (vr.current) vr.current.muted = muted; }, [muted]);

  // Keyboard shortcuts
  useEffect(() => {
    const key = (e: KeyboardEvent) => {
      if (e.key === "Escape") { props.onClose(); return; }
      if (e.key === "ArrowLeft") { e.preventDefault(); props.onPrev(); return; }
      if (e.key === "ArrowRight") { e.preventDefault(); props.onNext(); return; }
      if (e.key === " ") { e.preventDefault(); const v = vr.current; if (v) { v.paused ? v.play() : v.pause(); } }
    };
    window.addEventListener("keydown", key);
    return () => window.removeEventListener("keydown", key);
  }, [props]);

  const show = () => {
    setShowUI(true);
    if (hideTimer.current) clearTimeout(hideTimer.current);
    hideTimer.current = window.setTimeout(() => setShowUI(false), 2500);
  };

  const togglePlay = () => {
    const v = vr.current;
    if (!v) return;
    v.paused ? v.play().catch(() => {}) : v.pause();
  };

  const seek = (s: number) => {
    const v = vr.current;
    if (v) v.currentTime = Math.max(0, Math.min(s, v.duration || 0));
  };

  if (!props.video) return null;

  return (
    <div className="videos__fs" onMouseMove={show} onMouseDown={show}>
      <video
        ref={vr}
        className="videos__fsVideo"
        playsInline
        onClick={togglePlay}
        onPlay={() => setPaused(false)}
        onPause={() => setPaused(true)}
        onTimeUpdate={(e) => { const t = e.currentTarget.currentTime; setCt(t); props.onProgress(t); }}
        onLoadedMetadata={(e) => setDur(e.currentTarget.duration || 0)}
      />

      {!ready && !error && (
        <div className="videos__fsOverlay"><Loader size={24} className="videos__spin" /></div>
      )}

      {error && <div className="videos__fsError">{error}</div>}

      {showUI && ready && (
        <>
          <div className="videos__fsTop">
            <button className="videos__fsClose" onClick={props.onClose}><X size={22} /></button>
            <span className="videos__fsCounter">{props.index + 1} / {props.total}</span>
            <div />
          </div>

          <button className="videos__fsNav videos__fsPrev" onClick={props.onPrev} disabled={props.index <= 0}>
            <ChevronLeft size={24} />
          </button>
          <button className="videos__fsNav videos__fsNext" onClick={props.onNext} disabled={props.index >= props.total - 1}>
            <ChevronRight size={24} />
          </button>

          <div className="videos__fsBottom">
            <button className="videos__fsPlay" onClick={togglePlay}>
              {paused ? <Play size={18} /> : <Pause size={18} />}
            </button>
            <span className="videos__fsTime">{fmt(ct)}</span>
            <input type="range" className="videos__fsSlider" min={0} max={Math.max(dur, 1)} step={0.1}
              value={Math.min(ct, dur || 0)} onChange={(e) => seek(Number(e.target.value))} />
            <span className="videos__fsTime">{fmt(dur)}</span>
            <button className="videos__fsMute" onClick={() => setMuted((m) => !m)}>
              {muted ? <VolumeX size={16} /> : <Volume2 size={16} />}
            </button>
          </div>
        </>
      )}
    </div>
  );
}
