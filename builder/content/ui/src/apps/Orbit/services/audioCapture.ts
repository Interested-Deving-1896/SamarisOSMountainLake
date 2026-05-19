export type AudioCaptureCallbacks = {
  onSpeechStart: () => void;
  onSpeechEnd: (blob: Blob) => void;
  onLevel: (level: number) => void;
  onError: (error: Error) => void;
};

export type AudioCaptureConfig = {
  vadThreshold?: number;
  silenceTimeoutMs?: number;
  minSpeechDurationMs?: number;
};

const DEFAULT_CONFIG: Required<AudioCaptureConfig> = {
  vadThreshold: 0.015,
  silenceTimeoutMs: 800,
  minSpeechDurationMs: 300,
};

export class AudioCapture {
  private stream: MediaStream | null = null;
  private source: MediaStreamAudioSourceNode | null = null;
  private analyser: AnalyserNode | null = null;
  private audioContext: AudioContext | null = null;
  private recorder: MediaRecorder | null = null;
  private chunks: BlobPart[] = [];
  private isListening = false;
  private silenceTimer: ReturnType<typeof setTimeout> | null = null;
  private speechStartedAt = 0;
  private animFrameId = 0;
  private callbacks: AudioCaptureCallbacks;
  private config: Required<AudioCaptureConfig>;
  private abortController: AbortController | null = null;

  constructor(callbacks: AudioCaptureCallbacks, config?: AudioCaptureConfig) {
    this.callbacks = callbacks;
    this.config = { ...DEFAULT_CONFIG, ...config };
  }

  async start(): Promise<void> {
    if (this.isListening) return;
    this.abortController = new AbortController();

    try {
      this.stream = await navigator.mediaDevices.getUserMedia({
        audio: {
          sampleRate: 16000,
          channelCount: 1,
          echoCancellation: true,
          noiseSuppression: true,
        },
      });

      this.audioContext = new AudioContext({ sampleRate: 16000 });
      this.source = this.audioContext.createMediaStreamSource(this.stream);
      this.analyser = this.audioContext.createAnalyser();
      this.analyser.fftSize = 256;
      this.source.connect(this.analyser);

      this.recorder = new MediaRecorder(this.stream, {
        mimeType: MediaRecorder.isTypeSupported("audio/webm;codecs=opus")
          ? "audio/webm;codecs=opus"
          : "audio/webm",
      });

      this.recorder.ondataavailable = (event) => {
        if (event.data.size > 0) this.chunks.push(event.data);
      };

      this.isListening = true;
      this.startLevelDetection();
      this.startSpeechDetection();
    } catch (error) {
      this.callbacks.onError(error instanceof Error ? error : new Error(String(error)));
    }
  }

  stop(): void {
    this.isListening = false;
    this.abortController?.abort();

    if (this.animFrameId) {
      cancelAnimationFrame(this.animFrameId);
      this.animFrameId = 0;
    }

    if (this.silenceTimer) {
      clearTimeout(this.silenceTimer);
      this.silenceTimer = null;
    }

    this.recorder?.stop();
    this.source?.disconnect();
    this.audioContext?.close();
    this.stream?.getTracks().forEach((t) => t.stop());

    this.recorder = null;
    this.source = null;
    this.audioContext = null;
    this.stream = null;
    this.analyser = null;
  }

  private startLevelDetection(): void {
    if (!this.analyser) return;
    const data = new Uint8Array(this.analyser.frequencyBinCount);

    const tick = () => {
      if (!this.isListening || !this.analyser) return;
      this.analyser.getByteTimeDomainData(data);

      let sum = 0;
      for (let i = 0; i < data.length; i++) {
        const value = (data[i] - 128) / 128;
        sum += value * value;
      }
      const rms = Math.sqrt(sum / data.length);

      this.callbacks.onLevel(Math.min(rms * 5, 1));

      if (!this.abortController?.signal.aborted) {
        this.animFrameId = requestAnimationFrame(tick);
      }
    };

    this.animFrameId = requestAnimationFrame(tick);
  }

  private startSpeechDetection(): void {
    if (!this.analyser) return;
    const data = new Uint8Array(this.analyser.frequencyBinCount);
    const threshold = this.config.vadThreshold;
    const minSpeech = this.config.minSpeechDurationMs;
    const silenceTimeout = this.config.silenceTimeoutMs;

    let speechActive = false;

    const tick = () => {
      if (!this.isListening || !this.analyser) {
        return;
      }

      this.analyser.getByteTimeDomainData(data);
      let sum = 0;
      for (let i = 0; i < data.length; i++) {
        const value = (data[i] - 128) / 128;
        sum += value * value;
      }
      const rms = Math.sqrt(sum / data.length);
      const speaking = rms > threshold;

      if (speaking && !speechActive) {
        speechActive = true;
        this.speechStartedAt = Date.now();
        this.chunks = [];
        this.recorder?.start();

        if (this.silenceTimer) {
          clearTimeout(this.silenceTimer);
          this.silenceTimer = null;
        }

        this.callbacks.onSpeechStart();
      }

      if (!speaking && speechActive) {
        const duration = Date.now() - this.speechStartedAt;
        if (duration < minSpeech) {
          speechActive = false;
          this.recorder?.stop();
          this.chunks = [];
          return;
        }

        speechActive = false;

        if (this.silenceTimer) {
          clearTimeout(this.silenceTimer);
        }

        this.silenceTimer = setTimeout(() => {
          if (!this.isListening || !this.recorder || this.recorder.state === "inactive") return;
          this.recorder.stop();
          this.silenceTimer = null;

          setTimeout(() => {
            const blob = new Blob(this.chunks, { type: this.recorder?.mimeType || "audio/webm" });
            this.chunks = [];
            this.callbacks.onSpeechEnd(blob);
          }, 50);
        }, silenceTimeout);
      }

      if (!this.abortController?.signal.aborted) {
        requestAnimationFrame(tick);
      }
    };

    requestAnimationFrame(tick);
  }
}
