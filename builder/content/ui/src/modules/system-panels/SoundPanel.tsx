import React from "react";
import { Speaker, Volume1, Volume2, VolumeX, Check } from "lucide-react";
import { audioStore } from "../../system/audio/audioStore";
import { useAirBar } from "../airbar/useAirBar";
import { SYSTEM_PANEL_CLASSES } from "./panel.styles";

export function SoundPanel() {
  const air = useAirBar();
  const open = air.activePanel === "sound";
  const audio = React.useSyncExternalStore((l) => audioStore.subscribe(l), () => audioStore.getState());
  const style = air.getPanelStyle("sound", { width: 340, align: "center" });
  const lastAudible = React.useRef(Math.max(audio.volume, 38));
  const [localVolume, setLocalVolume] = React.useState(audio.volume);
  const dragging = React.useRef(false);

  React.useEffect(() => { audioStore.init(); }, []);

  React.useEffect(() => {
    if (audio.volume > 0) lastAudible.current = audio.volume;
  }, [audio.volume]);

  React.useEffect(() => {
    if (!dragging.current) setLocalVolume(audio.volume);
  }, [audio.volume]);

  const commitVolume = (v: number) => {
    const next = Math.max(0, Math.min(100, Math.round(v)));
    setLocalVolume(next);
    audioStore.setVolume(next);
  };

  const toggleMute = () => {
    if (audio.muted) {
      commitVolume(lastAudible.current || 42);
    } else {
      lastAudible.current = audio.volume;
      commitVolume(0);
    }
  };

  const pct = audio.muted ? 0 : localVolume;
  const sliderColor = "#2f6df6";

  return (
    <section style={style} className={`airbar-panel airbar-system-panel ${open ? "open" : ""}`} role="dialog" aria-label="Sound">
      <div className={SYSTEM_PANEL_CLASSES.panel}>
        <div className={SYSTEM_PANEL_CLASSES.section}>
          <div className={SYSTEM_PANEL_CLASSES.heading}>Volume</div>

          <div style={{ display: "flex", alignItems: "center", gap: 12, padding: "4px 0 8px" }}>
            <button
              type="button"
              onClick={toggleMute}
              style={{ background: "none", border: "none", cursor: "pointer", padding: 4, color: "var(--air-text-soft)", display: "flex" }}
            >
              {audio.muted ? <VolumeX size={18} strokeWidth={2.2} /> : pct < 34 ? <Volume1 size={18} strokeWidth={2.2} /> : <Volume2 size={18} strokeWidth={2.2} />}
            </button>

            <input
              type="range"
              min={0}
              max={100}
              value={pct}
              onPointerDown={() => { dragging.current = true; }}
              onPointerUp={() => { dragging.current = false; commitVolume(localVolume); }}
              onPointerLeave={() => { dragging.current = false; }}
              onChange={(e) => {
                const v = Math.max(0, Math.min(100, Number(e.target.value)));
                setLocalVolume(v);
                if (!dragging.current) commitVolume(v);
              }}
              className="sound-slider"
              style={{
                "--pct": `${pct}%`,
                "--slider-color": sliderColor,
              } as React.CSSProperties}
            />

            <span style={{ fontSize: 13, fontWeight: 700, color: "var(--air-text)", minWidth: 40, textAlign: "right", fontVariantNumeric: "tabular-nums" }}>
              {pct}%
            </span>
          </div>
        </div>

        <div className={SYSTEM_PANEL_CLASSES.section}>
          <div className={SYSTEM_PANEL_CLASSES.heading}>Output</div>
          <div className={SYSTEM_PANEL_CLASSES.actions}>
            {audio.outputs.map((out) => {
              const isActive = out.id === audio.activeOutputId;
              return (
                <button
                  key={out.id}
                  type="button"
                  className={SYSTEM_PANEL_CLASSES.button}
                  style={isActive ? { background: "rgba(var(--air-accent-r),var(--air-accent-g),var(--air-accent-b),0.1)", borderColor: "rgba(var(--air-accent-r),var(--air-accent-g),var(--air-accent-b),0.2)" } : undefined}
                  onClick={() => audioStore.setOutput(out.id)}
                >
                  <span className={SYSTEM_PANEL_CLASSES.rowIcon}>
                    <Speaker size={16} strokeWidth={2} />
                  </span>
                  <span className={SYSTEM_PANEL_CLASSES.rowText}>
                    <span className={SYSTEM_PANEL_CLASSES.rowLabel}>{out.label}</span>
                  </span>
                  {isActive && <Check size={15} strokeWidth={2.5} style={{ color: "var(--air-accent)" }} />}
                </button>
              );
            })}
          </div>
        </div>
      </div>
    </section>
  );
}
