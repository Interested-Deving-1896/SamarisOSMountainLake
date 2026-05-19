import { describe, it, expect, vi, beforeEach } from "vitest";
import { renderHook, act } from "@testing-library/react";
import { usePlayback } from "./usePlayback";
import type { MusicTrack } from "../types";

function createMockTrack(overrides: Partial<MusicTrack> = {}): MusicTrack {
  return {
    id: "track-1",
    path: "/path/to/track.mp3",
    fileName: "track.mp3",
    title: "Test Track",
    artist: "Test Artist",
    album: "Test Album",
    genre: "Test",
    year: 2024,
    durationSeconds: 200,
    durationLabel: "3:20",
    size: 5000,
    coverDataUrl: null,
    src: "/mock/src.mp3",
    ...overrides
  };
}

const tracks: MusicTrack[] = [
  createMockTrack({ id: "t1", title: "Track 1" }),
  createMockTrack({ id: "t2", title: "Track 2", artist: "Artist 2", path: "/path/t2.mp3" }),
  createMockTrack({ id: "t3", title: "Track 3", artist: "Artist 3", path: "/path/t3.mp3" })
];

let mockAudioInstance: any;
let eventListeners: Record<string, Array<() => void>>;

beforeEach(() => {
  vi.restoreAllMocks();
  eventListeners = {};

  const audio = {
    src: "",
    currentTime: 0,
    duration: 200,
    volume: 0.82,
    paused: true,
    preload: "",
    load: vi.fn(),
    play: vi.fn(() => {
      audio.paused = false;
      fireEvent("play");
      return Promise.resolve();
    }),
    pause: vi.fn(() => {
      audio.paused = true;
      fireEvent("pause");
    }),
    addEventListener: vi.fn((event: string, handler: () => void) => {
      if (!eventListeners[event]) eventListeners[event] = [];
      eventListeners[event].push(handler);
    }),
    removeEventListener: vi.fn((event: string) => {
      delete eventListeners[event];
    }),
    dispatchEvent: vi.fn()
  };
  mockAudioInstance = audio;

  function fireEvent(event: string) {
    const handlers = eventListeners[event];
    if (handlers) handlers.forEach((h) => h());
  }

  const MockAudio = function () { return audio; } as unknown as typeof Audio;
  vi.stubGlobal("Audio", MockAudio);
});

describe("usePlayback", () => {
  const resolveTrackSource = vi.fn((id: string) => Promise.resolve(`/src/${id}.mp3`));

  it("initializes with first track by default", () => {
    const { result } = renderHook(() =>
      usePlayback({
        queue: tracks,
        resolveTrackSource,
        shuffle: false,
        repeat: "off",
        equalizerPreset: "flat"
      })
    );
    expect(result.current.currentTrackId).toBe("t1");
    expect(result.current.currentTrack?.title).toBe("Track 1");
  });

  it("initializes with initialTrackId when provided", () => {
    const { result } = renderHook(() =>
      usePlayback({
        queue: tracks,
        resolveTrackSource,
        shuffle: false,
        repeat: "off",
        equalizerPreset: "flat",
        initialTrackId: "t2"
      })
    );
    expect(result.current.currentTrackId).toBe("t2");
  });

  it("initializes with null when queue is empty", () => {
    const { result } = renderHook(() =>
      usePlayback({
        queue: [],
        resolveTrackSource,
        shuffle: false,
        repeat: "off",
        equalizerPreset: "flat"
      })
    );
    expect(result.current.currentTrackId).toBeNull();
    expect(result.current.currentTrack).toBeNull();
  });

  it("togglePlayback starts playback when no track is playing", async () => {
    const { result } = renderHook(() =>
      usePlayback({ queue: tracks, resolveTrackSource, shuffle: false, repeat: "off", equalizerPreset: "flat" })
    );

    await act(async () => {
      await result.current.togglePlayback();
    });

    expect(result.current.isPlaying).toBe(true);
  });

  it("nextTrack advances to next track", async () => {
    const { result } = renderHook(() =>
      usePlayback({ queue: tracks, resolveTrackSource, shuffle: false, repeat: "off", equalizerPreset: "flat" })
    );

    await act(async () => {
      await result.current.nextTrack();
    });

    expect(result.current.currentTrackId).toBe("t2");
  });

  it("nextTrack wraps to first when repeat is all", async () => {
    const { result } = renderHook(() =>
      usePlayback({ queue: tracks, resolveTrackSource, shuffle: false, repeat: "all", equalizerPreset: "flat" })
    );

    await act(async () => {
      await result.current.nextTrack();
    });
    expect(result.current.currentTrackId).toBe("t2");

    await act(async () => {
      await result.current.nextTrack();
    });
    expect(result.current.currentTrackId).toBe("t3");

    await act(async () => {
      await result.current.nextTrack();
    });
    expect(result.current.currentTrackId).toBe("t1");
  });

  it("nextTrack pauses at end when repeat is off", async () => {
    const { result } = renderHook(() =>
      usePlayback({
        queue: [tracks[0]],
        resolveTrackSource,
        shuffle: false,
        repeat: "off",
        equalizerPreset: "flat"
      })
    );

    await act(async () => {
      await result.current.nextTrack();
    });
    expect(result.current.currentTrackId).toBe("t1");
    expect(result.current.isPlaying).toBe(false);
  });

  it("previousTrack resets currentTime within 4 seconds", async () => {
    const { result } = renderHook(() =>
      usePlayback({ queue: tracks, resolveTrackSource, shuffle: false, repeat: "off", equalizerPreset: "flat" })
    );

    mockAudioInstance.currentTime = 2;

    await act(async () => {
      await result.current.previousTrack();
    });

    expect(result.current.currentTrackId).toBe("t1");
    expect(result.current.currentTime).toBe(0);
  });

  it("previousTrack goes to previous track after 4 seconds", async () => {
    const { result } = renderHook(() =>
      usePlayback({ queue: tracks, resolveTrackSource, shuffle: false, repeat: "off", equalizerPreset: "flat" })
    );

    mockAudioInstance.currentTime = 10;

    await act(async () => {
      await result.current.previousTrack();
    });

    expect(result.current.currentTrackId).toBe("t1");
  });

  it("setCurrentTrackId updates track selection", () => {
    const { result } = renderHook(() =>
      usePlayback({ queue: tracks, resolveTrackSource, shuffle: false, repeat: "off", equalizerPreset: "flat" })
    );

    act(() => {
      result.current.setCurrentTrackId("t3");
    });

    expect(result.current.currentTrackId).toBe("t3");
  });

  it("seekTo updates currentTime", () => {
    const { result } = renderHook(() =>
      usePlayback({ queue: tracks, resolveTrackSource, shuffle: false, repeat: "off", equalizerPreset: "flat" })
    );

    act(() => {
      result.current.seekTo(50);
    });

    expect(result.current.currentTime).toBe(50);
  });

  it("setVolume clamps between 0 and 1", () => {
    const { result } = renderHook(() =>
      usePlayback({ queue: tracks, resolveTrackSource, shuffle: false, repeat: "off", equalizerPreset: "flat" })
    );

    act(() => {
      result.current.setVolume(1.5);
    });
    expect(result.current.volume).toBe(1);

    act(() => {
      result.current.setVolume(-0.5);
    });
    expect(result.current.volume).toBe(0);
  });
});
