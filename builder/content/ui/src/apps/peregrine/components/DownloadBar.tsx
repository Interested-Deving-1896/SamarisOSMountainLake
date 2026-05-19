import React from "react";
import { Download, X, AlertCircle } from "lucide-react";
import type { DownloadItem } from "../../../system/downloads/downloadStore";

export function DownloadBar(props: { items: DownloadItem[]; onCancel: (id: string) => void }) {
  if (props.items.length === 0) return null;

  return (
    <div className="pr-downloads">
      {props.items.map((item) => {
        const pct = item.totalBytes > 0 ? Math.round((item.received / item.totalBytes) * 100) : 0;
        const done = item.state === "completed" || item.state === "failed" || item.state === "cancelled";
        return (
          <div key={item.id} className="pr-dl-item">
            <Download size={14} className="pr-dl-icon" />
            <div className="pr-dl-info">
              <span className="pr-dl-name">{item.filename}</span>
              {!done ? (
                <div className="pr-dl-bar"><div className="pr-dl-fill" style={{ width: `${pct}%` }} /></div>
              ) : item.state === "completed" ? (
                <span className="pr-dl-done">Done</span>
              ) : (
                <span className="pr-dl-fail"><AlertCircle size={12} /> {item.error || item.state}</span>
              )}
            </div>
            {!done && (
              <button className="pr-dl-cancel" onClick={() => props.onCancel(item.id)}><X size={12} /></button>
            )}
          </div>
        );
      })}
    </div>
  );
}
