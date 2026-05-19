import React from "react";
import { Cpu, Dot, RefreshCw } from "lucide-react";
import type { OrbitModelManifest } from "../types";

export function ModelStatusCard(props: {
  model: OrbitModelManifest | null;
  busy: boolean;
  onRefresh: () => void;
}) {
  const status = props.model?.runtimeStatus || "loading";

  return (
    <div className="orbit__modelCard orbit__modelCard--compact">
      <div className="orbit__modelHead">
        <div className={`orbit__statusDot orbit__statusDot--${status}`} />
        <Cpu size={14} strokeWidth={2.2} />
        <span>{props.model?.runtimeLabel || "Checking local model…"}</span>
      </div>
      <div className="orbit__modelMeta">
        <span>{props.model?.name || "Qwen model"}</span>
        <Dot size={16} strokeWidth={2.2} />
        <span>{props.model?.sizeLabel || "--"}</span>
      </div>
      <div className="orbit__privacy">
        <span>{props.model?.provider || "Local runtime"}</span>
        <span>•</span>
        <span>No cloud calls</span>
        <button type="button" className="orbit__iconBtn" onClick={props.onRefresh} disabled={props.busy}>
          <RefreshCw size={14} strokeWidth={2.2} className={props.busy ? "orbit__spinner" : ""} />
        </button>
      </div>
    </div>
  );
}
