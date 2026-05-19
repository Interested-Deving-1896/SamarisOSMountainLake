import React from "react";
import { AudioCapture } from "../services/audioCapture";
import { kernelClient } from "../../../os/kernel/kernelClient";
import type { VoiceState } from "../types";

export function useVoiceMode(onUserMessage: (text: string) => void, onAssistantMessage: (text: string) => void) {
  const [voiceState, setVoiceState] = React.useState<VoiceState>("idle");
  const [audioLevel, setAudioLevel] = React.useState(0);
  const capturerRef = React.useRef<AudioCapture | null>(null);
  const audioElRef = React.useRef<HTMLAudioElement | null>(null);
  const activeRef = React.useRef(false);
  const wakeLockRef = React.useRef<WakeLockSentinel | null>(null);
  const onUserMsgRef = React.useRef(onUserMessage);
  const onAssistantMsgRef = React.useRef(onAssistantMessage);
  const stopRef = React.useRef<() => Promise<void>>();

  onUserMsgRef.current = onUserMessage;
  onAssistantMsgRef.current = onAssistantMessage;

  const acquireWakeLock = React.useCallback(async () => {
    try {
      if ("wakeLock" in navigator) {
        wakeLockRef.current = await navigator.wakeLock.request("screen");
      }
    } catch {
      // wake lock not available
    }
  }, []);

  const releaseWakeLock = React.useCallback(async () => {
    if (wakeLockRef.current) {
      await wakeLockRef.current.release();
      wakeLockRef.current = null;
    }
  }, []);

  const playAudioBase64 = React.useCallback((b64: string): Promise<void> => {
    return new Promise((resolve) => {
      const prev = audioElRef.current;
      if (prev) { prev.pause(); prev.remove(); }

      const el = document.createElement("audio");
      el.src = `data:audio/wav;base64,${b64}`;
      el.onended = () => { document.body.removeChild(el); audioElRef.current = null; resolve(); };
      el.onerror = () => { document.body.removeChild(el); audioElRef.current = null; resolve(); };
      document.body.appendChild(el);
      audioElRef.current = el;
      el.play().catch(() => { document.body.removeChild(el); audioElRef.current = null; resolve(); });
    });
  }, []);

  const stopVoiceMode = React.useCallback(async () => {
    activeRef.current = false;
    capturerRef.current?.stop();
    capturerRef.current = null;
    if (audioElRef.current) { audioElRef.current.pause(); audioElRef.current.remove(); audioElRef.current = null; }
    await releaseWakeLock();
    setVoiceState("idle");
    setAudioLevel(0);
  }, [releaseWakeLock]);

  stopRef.current = stopVoiceMode;

  const processVoiceInput = React.useCallback(
    async (audioBlob: Blob) => {
      if (!activeRef.current) return;

      // Interrupt any previous playback
      if (audioElRef.current) { audioElRef.current.pause(); audioElRef.current.remove(); audioElRef.current = null; }

      setVoiceState("processing");

      try {
        const reader = new FileReader();
        const audioBase64 = await new Promise<string>((resolve, reject) => {
          reader.onload = () => resolve(String(reader.result || "").split(",")[1] || "");
          reader.onerror = () => reject(reader.error);
          reader.readAsDataURL(audioBlob);
        });

        const sttResponse = await kernelClient.request<{ text: string }>(
          { type: "stt.transcribe", data: { audioBase64, mimeType: audioBlob.type || "audio/webm", language: "auto" } },
          { timeoutMs: 60000 }
        );

        const transcribedText = String(sttResponse.data?.text || "").trim();
        if (!transcribedText || !activeRef.current) return;

        if (["stop", "arrête", "arrête-toi", "stop orbit"].includes(transcribedText.toLowerCase())) {
          stopRef.current?.();
          return;
        }

        onUserMsgRef.current(transcribedText);

        const response = await kernelClient.request<{ finalAnswer: string }>(
          {
            type: "orbit.generate",
            data: { prompt: transcribedText, modeId: "fast", strategy: "self-consistency" },
            requestId: `voice-${Date.now()}-${Math.random().toString(36).slice(2, 8)}`,
          },
          { timeoutMs: 120000 }
        );

        const fullContent = String(response.data?.finalAnswer || "");
        if (!fullContent || !activeRef.current) return;

        onAssistantMsgRef.current(fullContent);

        if (!activeRef.current) return;
        setVoiceState("speaking");

        // Single TTS call for full text — no per-sentence overhead
        const ttsResponse = await kernelClient.request<{ audioBase64: string }>(
          { type: "tts.speak", data: { text: fullContent, voice: "en_US-lessac-high" } },
          { timeoutMs: 60000 }
        );

        const audioB64 = ttsResponse.data?.audioBase64;
        if (audioB64 && activeRef.current) {
          await playAudioBase64(audioB64);
        }

        if (activeRef.current) {
          setVoiceState("listening");
        }
      } catch {
        if (activeRef.current) {
          setVoiceState("listening");
        }
      }
    },
    [playAudioBase64]
  );

  const startVoiceMode = React.useCallback(async () => {
    if (activeRef.current) return;
    activeRef.current = true;

    await acquireWakeLock();
    setVoiceState("listening");

    const capturer = new AudioCapture(
      {
        onSpeechStart: () => {},
        onSpeechEnd: (blob) => {
          void processVoiceInput(blob);
        },
        onLevel: (level) => {
          setAudioLevel(level);
        },
        onError: () => {
          if (activeRef.current) setVoiceState("listening");
        },
      },
      {
        vadThreshold: 0.015,
        silenceTimeoutMs: 600,
        minSpeechDurationMs: 300,
      }
    );

    capturerRef.current = capturer;
    await capturer.start();
  }, [acquireWakeLock, processVoiceInput]);

  React.useEffect(() => {
    return () => {
      activeRef.current = false;
      capturerRef.current?.stop();
      if (audioElRef.current) { audioElRef.current.pause(); audioElRef.current.remove(); }
      releaseWakeLock();
    };
  }, [releaseWakeLock]);

  return {
    voiceState,
    audioLevel,
    isVoiceActive: voiceState !== "idle",
    startVoiceMode,
    stopVoiceMode,
  };
}
