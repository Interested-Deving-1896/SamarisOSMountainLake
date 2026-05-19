import React from "react";
import {
  Brain,
  Zap
} from "lucide-react";
import { ORBIT_MODES } from "../constants/modes";
import type { OrbitModeId } from "../types";

const modeIcons = {
  fast: Zap,
  smart: Brain
} as const;

export function ModeSelector(props: {
  activeModeId: OrbitModeId;
  onSelect: (modeId: OrbitModeId) => void;
}) {
  const [open, setOpen] = React.useState(false);
  const rootRef = React.useRef<HTMLDivElement | null>(null);
  const activeMode = ORBIT_MODES.find((mode) => mode.id === props.activeModeId) || ORBIT_MODES[0];
  const ActiveIcon = modeIcons[activeMode.id];

  React.useEffect(() => {
    function handlePointerDown(event: PointerEvent) {
      if (!rootRef.current?.contains(event.target as Node)) {
        setOpen(false);
      }
    }

    window.addEventListener("pointerdown", handlePointerDown);
    return () => window.removeEventListener("pointerdown", handlePointerDown);
  }, []);

  return (
    <div ref={rootRef} className="orbit__modeSelector">
      <button
        type="button"
        className="orbit__modeTrigger"
        onClick={() => setOpen((current) => !current)}
      >
        <ActiveIcon size={16} strokeWidth={2.2} />
        <span>{activeMode.label}</span>
      </button>

      {open ? (
        <div className="orbit__modeMenu">
          {ORBIT_MODES.map((mode) => {
            const Icon = modeIcons[mode.id];
            return (
              <button
                key={mode.id}
                type="button"
                className={`orbit__modeOption ${props.activeModeId === mode.id ? "orbit__modeOption--active" : ""}`}
                onClick={() => {
                  props.onSelect(mode.id);
                  setOpen(false);
                }}
              >
                <div className="orbit__modeOptionIcon">
                  <Icon size={16} strokeWidth={2.2} />
                </div>
                <div className="orbit__modeLabel">{mode.label}</div>
              </button>
            );
          })}
        </div>
      ) : null}
    </div>
  );
}
