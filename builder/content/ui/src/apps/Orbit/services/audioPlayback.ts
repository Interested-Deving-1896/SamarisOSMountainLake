export type PlaybackCallbacks = {
  onStart: () => void;
  onSentenceStart: (index: number) => void;
  onSentenceEnd: (index: number) => void;
  onEnd: () => void;
};

export class AudioPlaybackQueue {
  private queue: string[] = [];
  private isPlaying = false;
  private currentEl: HTMLAudioElement | null = null;
  private callbacks: PlaybackCallbacks;
  private sentenceIndex = 0;
  private interrupted = false;

  constructor(callbacks: PlaybackCallbacks) {
    this.callbacks = callbacks;
  }

  async enqueueBase64(base64Audio: string): Promise<void> {
    if (this.interrupted) return;
    this.queue.push(base64Audio);
    this.sentenceIndex++;
    if (!this.isPlaying) {
      this.playNext();
    }
  }

  interrupt(): void {
    this.interrupted = true;
    this.queue = [];
    if (this.currentEl) {
      this.currentEl.pause();
      this.currentEl.remove();
      this.currentEl = null;
    }
    this.isPlaying = false;
  }

  async stop(): Promise<void> {
    this.interrupt();
  }

  private async playNext(): Promise<void> {
    if (this.interrupted || this.queue.length === 0) {
      this.isPlaying = false;
      this.callbacks.onEnd();
      return;
    }

    this.isPlaying = true;
    const b64 = this.queue.shift()!;
    const idx = this.sentenceIndex - this.queue.length - 1;
    this.callbacks.onSentenceStart(idx);

    return new Promise((resolve) => {
      const el = document.createElement("audio");
      el.src = `data:audio/wav;base64,${b64}`;
      this.currentEl = el;

      el.onended = () => {
        document.body.removeChild(el);
        this.currentEl = null;
        this.callbacks.onSentenceEnd(idx);
        if (!this.interrupted) {
          this.playNext();
        }
        resolve();
      };

      el.onerror = () => {
        document.body.removeChild(el);
        this.currentEl = null;
        this.callbacks.onSentenceEnd(idx);
        if (!this.interrupted) {
          this.playNext();
        }
        resolve();
      };

      document.body.appendChild(el);
      el.play().catch(() => {
        document.body.removeChild(el);
        this.currentEl = null;
        this.callbacks.onSentenceEnd(idx);
        if (!this.interrupted) {
          this.playNext();
        }
        resolve();
      });
    });
  }
}
