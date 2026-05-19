import React from "react";
import { ArrowUp, LoaderCircle, Mic, Square } from "lucide-react";
import { kernelClient } from "../../../os/kernel/kernelClient";

export function ChatInput(props: {
  busy: boolean;
  onSubmit: (value: string) => void;
}) {
  const [value, setValue] = React.useState("");
  const [voiceState, setVoiceState] = React.useState<"idle" | "recording" | "transcribing">("idle");
  const textareaRef = React.useRef<HTMLTextAreaElement | null>(null);
  const recorderRef = React.useRef<MediaRecorder | null>(null);
  const chunksRef = React.useRef<BlobPart[]>([]);

  function resizeTextarea() {
    const textarea = textareaRef.current;
    if (!textarea) return;
    textarea.style.height = "0px";
    textarea.style.height = `${Math.min(textarea.scrollHeight, 220)}px`;
  }

  React.useEffect(() => {
    resizeTextarea();
  }, [value]);

  async function blobToBase64(blob: Blob) {
    return new Promise<string>((resolve, reject) => {
      const reader = new FileReader();
      reader.onload = () => resolve(String(reader.result || "").split(",")[1] || "");
      reader.onerror = () => reject(reader.error || new Error("audio_read_failed"));
      reader.readAsDataURL(blob);
    });
  }

  async function startVoice() {
    if (props.busy || voiceState !== "idle") return;
    if (!navigator.mediaDevices?.getUserMedia || typeof MediaRecorder === "undefined") return;
    try {
      const stream = await navigator.mediaDevices.getUserMedia({ audio: true });
      chunksRef.current = [];
      const recorder = new MediaRecorder(stream);
      recorderRef.current = recorder;
      recorder.ondataavailable = (event) => {
        if (event.data.size > 0) chunksRef.current.push(event.data);
      };
      recorder.onstop = () => {
        stream.getTracks().forEach((track) => track.stop());
        void transcribeVoice(new Blob(chunksRef.current, { type: recorder.mimeType || "audio/webm" })).catch(() => {});
      };
      recorder.start();
      setVoiceState("recording");
    } catch {
      setVoiceState("idle");
    }
  }

  function stopVoice() {
    if (voiceState !== "recording") return;
    setVoiceState("transcribing");
    recorderRef.current?.stop();
  }

  async function transcribeVoice(blob: Blob) {
    try {
      const audioBase64 = await blobToBase64(blob);
      const response = await kernelClient.request<{ text: string }>(
        { type: "stt.transcribe", data: { audioBase64, mimeType: blob.type || "audio/webm", language: "auto" } },
        { timeoutMs: 180000 }
      );
      const text = String(response.data?.text || "").trim();
      if (text) {
        setValue(text);
        props.onSubmit(text);
      }
    } finally {
      recorderRef.current = null;
      chunksRef.current = [];
      setVoiceState("idle");
    }
  }

  return (
    <div className="orbit__composer">
      <div className="orbit__composerInner">
        <textarea
          ref={textareaRef}
          className="orbit__input"
          value={value}
          rows={1}
          placeholder="Message Orbit…"
          onChange={(event) => setValue(event.target.value)}
          onKeyDown={(event) => {
            if (event.key === "Enter" && !event.shiftKey) {
              event.preventDefault();
              const next = value.trim();
              if (!next || props.busy) return;
              props.onSubmit(next);
              setValue("");
            }
          }}
        />
        <div className="orbit__composerFooter">
          <div className="orbit__composerMeta">
            {voiceState === "recording" ? "Recording voice" : voiceState === "transcribing" ? "Transcribing locally" : "Local model • private by default"}
          </div>
          <button
            type="button"
            className={`orbit__voiceBtn ${voiceState === "recording" ? "orbit__voiceBtn--active" : ""}`}
            disabled={props.busy || voiceState === "transcribing"}
            title={voiceState === "recording" ? "Release to transcribe" : "Hold to speak"}
            onPointerDown={(event) => {
              event.preventDefault();
              void startVoice();
            }}
            onPointerUp={(event) => {
              event.preventDefault();
              stopVoice();
            }}
            onPointerCancel={stopVoice}
            onPointerLeave={stopVoice}
          >
            {voiceState === "transcribing" ? <LoaderCircle size={16} className="orbit__spinner" strokeWidth={2.4} /> : voiceState === "recording" ? <Square size={15} strokeWidth={2.4} /> : <Mic size={16} strokeWidth={2.4} />}
          </button>
          <button
            type="button"
            className="orbit__sendBtn"
            disabled={props.busy || !value.trim()}
            onClick={() => {
              const next = value.trim();
              if (!next || props.busy) return;
              props.onSubmit(next);
              setValue("");
            }}
          >
            {props.busy ? <LoaderCircle size={16} className="orbit__spinner" strokeWidth={2.4} /> : <ArrowUp size={16} strokeWidth={2.4} />}
          </button>
        </div>
      </div>
    </div>
  );
}
