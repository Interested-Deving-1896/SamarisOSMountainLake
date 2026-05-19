import React, { useState } from "react";
import { Download, FileText, Image, FileArchive, Film, Music, X, FolderOpen, RotateCw, Trash2 } from "lucide-react";
import { useDownloads } from "../../system/downloads/useDownloads";
import { downloadStore } from "../../system/downloads/downloadStore";
import { startFileDrag } from "../../os/filesystem/dragDrop";
import "./downloads.css";

function formatBytes(bytes: number): string {
  if (bytes === 0) return "0 B";
  const k = 1024;
  const sizes = ["B", "KB", "MB", "GB"];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + " " + sizes[i];
}

function formatTime(ms: number): string {
  const s = Math.floor(ms / 1000);
  if (s < 60) return `${s}s`;
  const m = Math.floor(s / 60);
  return `${m}m ${s % 60}s`;
}

function FileIcon({ filename, size = 18 }: { filename: string; size?: number }) {
  const lower = filename.toLowerCase();
  if (lower.match(/\.(png|jpg|jpeg|webp|gif|svg)$/)) return <Image size={size} />;
  if (lower.match(/\.(mp4|webm|avi|mkv|mov)$/)) return <Film size={size} />;
  if (lower.match(/\.(mp3|wav|m4a|aac|ogg|flac)$/)) return <Music size={size} />;
  if (lower.match(/\.(zip|tar|gz|rar|7z)$/)) return <FileArchive size={size} />;
  if (lower.match(/\.(pdf|doc|docx|txt|md)$/)) return <FileText size={size} />;
  return <Download size={size} />;
}

export function DownloadsApp() {
  const { items, active, completed, failed, cancel, clearHistory, removeItem } = useDownloads();
  const [filter, setFilter] = useState<"all" | "active" | "completed">("all");

  const filtered = filter === "all" ? items : filter === "active" ? [...active, ...failed] : completed;

  return (
    <div className="downloads">
      <div className="downloads__header">
        <h2 className="downloads__title">Downloads</h2>
        <div className="downloads__filters">
          <button className={`downloads__filter ${filter === "all" ? "downloads__filter--active" : ""}`} onClick={() => setFilter("all")}>All ({items.length})</button>
          <button className={`downloads__filter ${filter === "active" ? "downloads__filter--active" : ""}`} onClick={() => setFilter("active")}>Active ({active.length + failed.length})</button>
          <button className={`downloads__filter ${filter === "completed" ? "downloads__filter--active" : ""}`} onClick={() => setFilter("completed")}>Completed ({completed.length})</button>
        </div>
        {items.length > 0 && <button className="downloads__clear" onClick={clearHistory}><Trash2 size={14} /> Clear</button>}
      </div>

      <div className="downloads__list">
        {filtered.length === 0 ? (
          <div className="downloads__empty">
            <Download size={32} />
            <p>No downloads yet</p>
          </div>
        ) : (
          filtered.map((item) => (
            <div key={item.id} className={`downloads__item downloads__item--${item.state}`}
              draggable={item.state === "completed" && !!item.savePath}
              onDragStart={(e) => {
                if (!item.savePath) { e.preventDefault(); return; }
                startFileDrag(e.dataTransfer, [{ name: item.filename, path: item.savePath, kind: "file", size: item.totalBytes }]);
              }}>
              <div className="downloads__itemIcon">
                <FileIcon filename={item.filename} />
              </div>
              <div className="downloads__itemInfo">
                <div className="downloads__itemName">{item.filename}</div>
                <div className="downloads__itemMeta">
                  {item.state === "downloading" && (
                    <span>
                      {formatBytes(item.received)} / {formatBytes(item.totalBytes)}
                      {" — "}
                      {item.totalBytes > 0 ? Math.round((item.received / item.totalBytes) * 100) : 0}%
                    </span>
                  )}
                  {item.state === "completed" && <span>Completed {item.savePath ? "— " + item.savePath.split("/").slice(-2).join("/") : ""}</span>}
                  {item.state === "failed" && <span className="downloads__itemError">Failed: {item.error || "Unknown error"}</span>}
                  {item.state === "cancelled" && <span>Cancelled</span>}
                </div>
                {item.state === "downloading" && (
                  <div className="downloads__progress">
                    <div
                      className="downloads__progressBar"
                      style={{ width: `${item.totalBytes > 0 ? (item.received / item.totalBytes) * 100 : 0}%` }}
                    />
                  </div>
                )}
              </div>
              <div className="downloads__itemActions">
                {item.state === "downloading" && (
                  <button className="downloads__actionBtn" onClick={() => cancel(item.id)} title="Cancel">
                    <X size={14} />
                  </button>
                )}
                {item.state === "completed" && item.savePath && (
                  <button
                    className="downloads__actionBtn"
                    onClick={() => window.electronAPI?.shell.showItemInFolder(item.savePath!)}
                    title="Show in folder"
                  >
                    <FolderOpen size={14} />
                  </button>
                )}
                <button className="downloads__actionBtn" onClick={() => removeItem(item.id)} title="Remove">
                  <Trash2 size={14} />
                </button>
              </div>
            </div>
          ))
        )}
      </div>
    </div>
  );
}
