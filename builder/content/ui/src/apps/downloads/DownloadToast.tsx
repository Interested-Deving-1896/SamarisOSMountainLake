import React from "react";
import { X, Download } from "lucide-react";
import type { DownloadItem } from "../../system/downloads/downloadStore";
import { systemSounds } from "../../system/sounds/systemSounds";

function formatBytes(bytes: number): string {
  if (bytes === 0) return "0 B";
  const k = 1024;
  const sizes = ["B", "KB", "MB", "GB"];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + " " + sizes[i];
}

export function DownloadToast({ items }: { items: DownloadItem[] }) {
  const [visible, setVisible] = React.useState(true);
  const prevActiveRef = React.useRef(0);
  const active = items.filter((i) => i.state === "downloading");
  const justCompleted = items.filter((i) => i.state === "completed").slice(0, 3);

  // Reset visible when new active downloads appear
  React.useEffect(() => {
    if (active.length > prevActiveRef.current) setVisible(true);
    prevActiveRef.current = active.length;
  }, [active.length]);

  // Play notification when new items complete
  const prevCompletedRef = React.useRef(0);
  React.useEffect(() => {
    if (justCompleted.length > prevCompletedRef.current) {
      systemSounds.play("notification");
    }
    prevCompletedRef.current = justCompleted.length;
  }, [justCompleted.length]);

  if (active.length === 0 && justCompleted.length === 0) return null;
  if (!visible) return null;

  return (
    <div className="download-toast">
      <div className="download-toast__header">
        <Download size={14} />
        <span>{active.length > 0 ? `${active.length} downloading` : "Downloads complete"}</span>
        <button className="download-toast__close" onClick={() => setVisible(false)}><X size={12} /></button>
      </div>
      {active.slice(0, 2).map((item) => (
        <div key={item.id} className="download-toast__item">
          <span className="download-toast__name">{item.filename}</span>
          <span className="download-toast__size">{formatBytes(item.received)} / {formatBytes(item.totalBytes)}</span>
          <div className="download-toast__progress"><div className="download-toast__progressBar" style={{ width: `${item.totalBytes > 0 ? (item.received / item.totalBytes) * 100 : 0}%` }} /></div>
        </div>
      ))}
      {justCompleted.map((item) => (
        <div key={item.id} className="download-toast__item download-toast__item--done">
          <span className="download-toast__name">{item.filename}</span>
          <span className="download-toast__size">✓ Complete</span>
        </div>
      ))}
      {active.length > 2 && <div className="download-toast__more">+{active.length - 2} more</div>}
    </div>
  );
}
