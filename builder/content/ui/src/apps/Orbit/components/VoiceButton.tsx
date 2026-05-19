import React from "react";
import { Mic, MicOff, LoaderCircle, Waves } from "lucide-react";
import type { VoiceState } from "../types";

const STATE_LABELS: Record<VoiceState, string> = {
  idle: "Voice mode",
  listening: "Listening…",
  processing: "Thinking…",
  speaking: "Speaking…",
};

export function VoiceButton(props: {
  voiceState: VoiceState;
  audioLevel: number;
  onToggle: () => void;
}) {
  const isActive = props.voiceState !== "idle";

  return (
    <div className="voice-container">
      {isActive ? (
        <div className="voice-hud">
          <Waves size={14} strokeWidth={2.2} className="voice-hud__icon" />
          <span className="voice-hud__label">{STATE_LABELS[props.voiceState]}</span>
          <span className="voice-hud__duration" />
        </div>
      ) : null}

      <button
        type="button"
        className={`voice-btn ${isActive ? "voice-btn--active" : ""} ${
          props.voiceState === "listening" ? "voice-btn--listening" : ""
        } ${props.voiceState === "speaking" ? "voice-btn--speaking" : ""}`}
        onClick={props.onToggle}
        title={isActive ? "Stop voice mode" : "Start voice mode"}
      >
        {props.voiceState === "processing" ? (
          <LoaderCircle size={18} className="voice-btn__spinner" strokeWidth={2.4} />
        ) : isActive ? (
          <MicOff size={18} strokeWidth={2.2} />
        ) : (
          <Mic size={18} strokeWidth={2.2} />
        )}
        {isActive && props.voiceState === "listening" ? (
          <span
            className="voice-btn__ripple"
            style={{ transform: `scale(${1 + props.audioLevel * 0.3})` }}
          />
        ) : null}
      </button>
    </div>
  );
}
