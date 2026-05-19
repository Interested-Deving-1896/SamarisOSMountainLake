import React from "react";
import type { EqualizerPreset, MusicTrack, RepeatMode } from "../types";

const PRESETS: Record<EqualizerPreset, { low: number; mid: number; high: number }> = {
  flat: { low: 0, mid: 0, high: 0 },
  "bass-boost": { low: 6, mid: 0, high: 2 },
  vocal: { low: -2, mid: 4, high: 1 },
  acoustic: { low: 2, mid: 1, high: 2 },
  night: { low: -1, mid: -1, high: 1 }
};

type UsePlaybackArgs = {
  queue: MusicTrack[];
  resolveTrackSource: (trackId: string) => Promise<string>;
  shuffle: boolean;
  repeat: RepeatMode;
  equalizerPreset: EqualizerPreset;
  initialTrackId?: string;
};

function clamp(value: number, min: number, max: number) {
  return Math.min(max, Math.max(min, value));
}

export function usePlayback(args: UsePlaybackArgs) {
  const audioRef = React.useRef<HTMLAudioElement | null>(null);
  const audioContextRef = React.useRef<AudioContext | null>(null);
  const mediaSourceRef = React.useRef<MediaElementAudioSourceNode | null>(null);
  const lowRef = React.useRef<BiquadFilterNode | null>(null);
  const midRef = React.useRef<BiquadFilterNode | null>(null);
  const highRef = React.useRef<BiquadFilterNode | null>(null);
  const [currentTrackId, setCurrentTrackId] = React.useState<string | null>(args.initialTrackId || args.queue[0]?.id || null);
  const [isPlaying, setIsPlaying] = React.useState(false);
  const [loading, setLoading] = React.useState(false);
  const [currentTime, setCurrentTime] = React.useState(0);
  const [duration, setDuration] = React.useState(0);
  const [volume, setVolumeState] = React.useState(0.82);
  const repeatRef = React.useRef(args.repeat);
  const nextTrackRef = React.useRef<() => Promise<void>>(async () => {});

  React.useEffect(() => {
    repeatRef.current = args.repeat;
  }, [args.repeat]);

  const currentTrack = React.useMemo(
    () => args.queue.find((track) => track.id === currentTrackId) || args.queue[0] || null,
    [args.queue, currentTrackId]
  );

  React.useEffect(() => {
    if (currentTrackId && args.queue.some((track) => track.id === currentTrackId)) {
      return;
    }
    setCurrentTrackId(args.initialTrackId || args.queue[0]?.id || null);
  }, [args.initialTrackId, currentTrackId, args.queue]);

  React.useEffect(() => {
    const audio = new Audio();
    audio.preload = "metadata";
    audio.volume = volume;
    audioRef.current = audio;

    const syncTime = () => {
      setCurrentTime(audio.currentTime || 0);
      setDuration(Number.isFinite(audio.duration) ? audio.duration : 0);
    };
    const syncPlay = () => setIsPlaying(true);
    const syncPause = () => setIsPlaying(false);
    const syncEnded = () => {
      if (repeatRef.current === "one") {
        audio.currentTime = 0;
        void audio.play().catch(() => {});
        return;
      }
      void nextTrackRef.current();
    };

    audio.addEventListener("timeupdate", syncTime);
    audio.addEventListener("loadedmetadata", syncTime);
    audio.addEventListener("play", syncPlay);
    audio.addEventListener("pause", syncPause);
    audio.addEventListener("ended", syncEnded);

    return () => {
      audio.pause();
      audio.removeEventListener("timeupdate", syncTime);
      audio.removeEventListener("loadedmetadata", syncTime);
      audio.removeEventListener("play", syncPlay);
      audio.removeEventListener("pause", syncPause);
      audio.removeEventListener("ended", syncEnded);
      audioRef.current = null;
    };
  }, []);

  React.useEffect(() => {
    const audio = audioRef.current;
    if (!audio) return;
    audio.volume = volume;
  }, [volume]);

  React.useEffect(() => {
    if (!audioRef.current || mediaSourceRef.current) return;
    const AudioContextCtor = window.AudioContext || (window as typeof window & { webkitAudioContext?: typeof AudioContext }).webkitAudioContext;
    if (!AudioContextCtor) return;

    try {
      const context = new AudioContextCtor();
      const source = context.createMediaElementSource(audioRef.current);
      const low = context.createBiquadFilter();
      low.type = "lowshelf";
      low.frequency.value = 120;
      const mid = context.createBiquadFilter();
      mid.type = "peaking";
      mid.frequency.value = 1000;
      mid.Q.value = 0.9;
      const high = context.createBiquadFilter();
      high.type = "highshelf";
      high.frequency.value = 5200;

      source.connect(low);
      low.connect(mid);
      mid.connect(high);
      high.connect(context.destination);

      audioContextRef.current = context;
      mediaSourceRef.current = source;
      lowRef.current = low;
      midRef.current = mid;
      highRef.current = high;
    } catch {}

    return () => {
      try {
        highRef.current?.disconnect();
        midRef.current?.disconnect();
        lowRef.current?.disconnect();
        mediaSourceRef.current?.disconnect();
        audioContextRef.current?.close();
      } catch {}
      audioContextRef.current = null;
      mediaSourceRef.current = null;
      lowRef.current = null;
      midRef.current = null;
      highRef.current = null;
    };
  }, []);

  React.useEffect(() => {
    const preset = PRESETS[args.equalizerPreset];
    if (lowRef.current) lowRef.current.gain.value = preset.low;
    if (midRef.current) midRef.current.gain.value = preset.mid;
    if (highRef.current) highRef.current.gain.value = preset.high;
  }, [args.equalizerPreset]);

  const playTrack = React.useCallback(
    async (trackId: string) => {
      const audio = audioRef.current;
      const track = args.queue.find((entry) => entry.id === trackId) || null;
      if (!audio || !track) return;
      setLoading(true);
      try {
        const source = track.src || (await args.resolveTrackSource(track.id));
        if (audio.src !== source) {
          audio.src = source;
          audio.load();
        }
        setCurrentTrackId(track.id);
        await audio.play();
      } catch {
        setIsPlaying(false);
      } finally {
        setLoading(false);
      }
    },
    [args]
  );

  const pickRelative = React.useCallback(
    (direction: -1 | 1) => {
      if (!args.queue.length) return null;
      const activeIndex = args.queue.findIndex((track) => track.id === (currentTrackId || args.queue[0]?.id));
      if (args.shuffle) {
        const candidates = args.queue.filter((track) => track.id !== currentTrackId);
        return candidates[Math.floor(Math.random() * candidates.length)] || args.queue[0];
      }
      const nextIndex = activeIndex >= 0 ? activeIndex + direction : 0;
      if (nextIndex < 0) {
        return args.repeat === "off" ? null : args.queue[args.queue.length - 1];
      }
      if (nextIndex >= args.queue.length) {
        return args.repeat === "off" ? null : args.queue[0];
      }
      return args.queue[nextIndex] || null;
    },
    [args.queue, args.repeat, args.shuffle, currentTrackId]
  );

  const nextTrack = React.useCallback(async () => {
    const next = pickRelative(1);
    if (!next) {
      audioRef.current?.pause();
      return;
    }
    await playTrack(next.id);
  }, [pickRelative, playTrack]);

  const previousTrack = React.useCallback(async () => {
    const audio = audioRef.current;
    if (audio && audio.currentTime > 4) {
      audio.currentTime = 0;
      setCurrentTime(0);
      return;
    }
    const previous = pickRelative(-1);
    if (!previous) return;
    await playTrack(previous.id);
  }, [pickRelative, playTrack]);

  const togglePlayback = React.useCallback(async () => {
    const audio = audioRef.current;
    if (!audio) return;
    if (!currentTrackId && args.queue[0]) {
      await playTrack(args.queue[0].id);
      return;
    }
    if (audio.paused) {
      if (!audio.src && currentTrackId) {
        await playTrack(currentTrackId);
      } else {
        await audio.play().catch(() => {});
      }
      return;
    }
    audio.pause();
  }, [args.queue, currentTrackId, playTrack]);

  const seekTo = React.useCallback((value: number) => {
    const audio = audioRef.current;
    if (!audio) return;
    const nextTime = clamp(value, 0, Number.isFinite(audio.duration) ? audio.duration : value);
    audio.currentTime = nextTime;
    setCurrentTime(nextTime);
  }, []);

  const setVolume = React.useCallback((value: number) => {
    setVolumeState(clamp(value, 0, 1));
  }, []);

  React.useEffect(() => {
    nextTrackRef.current = nextTrack;
  }, [nextTrack]);

  return {
    currentTrackId,
    currentTrack,
    isPlaying,
    loading,
    currentTime,
    duration,
    volume,
    setCurrentTrackId,
    playTrack,
    togglePlayback,
    nextTrack,
    previousTrack,
    seekTo,
    setVolume
  };
}
