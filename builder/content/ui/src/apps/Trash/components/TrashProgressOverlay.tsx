import React from "react";
import { Trash2 } from "lucide-react";

export function TrashProgressOverlay(props: {
  total: number;
  current: number;
}) {
  const pct = props.total > 0 ? Math.round((props.current / props.total) * 100) : 0;
  const label =
    props.current < props.total
      ? `Emptying ${props.total - props.current} item${props.total - props.current === 1 ? "" : "s"}\u2026`
      : "Done";

  return (
    <div className="trash__overlay">
      <div className="trash__overlayCard">
        <div className="trash__overlayGlyph">
          <Trash2 size={40} strokeWidth={1.4} />
        </div>
        <div className="trash__overlayLabel">{label}</div>
        <div className="trash__overlayBar">
          <div
            className="trash__overlayBarFill"
            style={{ width: `${pct}%` }}
          />
        </div>
        <div className="trash__overlayPct">{pct}%</div>
      </div>
    </div>
  );
}
