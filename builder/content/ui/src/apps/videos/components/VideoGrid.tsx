import React from "react";
import { Film, Loader, Play } from "lucide-react";
import type { VideoAsset } from "../types";
import { startFileDrag } from "../../../os/filesystem/dragDrop";

export function VideoGrid(props: {
  videos: VideoAsset[];
  loading: boolean;
  onOpen: (id: string) => void;
}) {
  if (props.loading) {
    return <div className="videos__center"><Loader size={20} className="videos__spin" /><span>Scanning library…</span></div>;
  }

  if (!props.videos.length) {
    return <div className="videos__center"><Film size={20} /><span>No videos found.</span></div>;
  }

  return (
    <div className="videos__grid">
      {props.videos.map((video) => (
        <button key={video.id} type="button" className="videos__card" draggable onClick={() => props.onOpen(video.id)}
          onDragStart={(e) => {
            startFileDrag(e.dataTransfer, [{
              name: video.fileName,
              path: video.path,
              kind: "file",
              size: video.size
            }]);
          }}
        >
          <div className="videos__cardThumb"><Film size={20} /></div>
          <div className="videos__cardBody">
            <div className="videos__cardTitle">{video.title}</div>
            <div className="videos__cardMeta">
              {video.format.toUpperCase()} · {video.size > 1024 * 1024 ? `${(video.size / 1024 / 1024).toFixed(1)} MB` : `${(video.size / 1024).toFixed(0)} KB`}
            </div>
          </div>
          <div className="videos__cardPlay"><Play size={16} /></div>
        </button>
      ))}
    </div>
  );
}
