import { Film } from "lucide-react";
import type { VideoAsset } from "../types";

export function VideoList(props: {
  videos: VideoAsset[];
  loading: boolean;
  activeVideoId: string | null;
  onSelect: (videoId: string) => void;
  onPlay: (videoId: string) => void;
}) {
  return (
    <section className="videos__list">
      <div className="videos__listHead">
        <span>Library</span>
      </div>
      <div className="videos__listBody">
        {props.loading ? <div className="videos__empty">Scanning ~/Videos…</div> : null}
        {!props.loading && props.videos.length === 0 ? <div className="videos__empty">No videos found in ~/Videos.</div> : null}
        {props.videos.map((video) => (
          <button
            key={video.id}
            type="button"
            className={`videos__listRow ${props.activeVideoId === video.id ? "videos__listRow--active" : ""}`}
            onClick={() => props.onSelect(video.id)}
            onDoubleClick={() => props.onPlay(video.id)}
          >
            <span className="videos__listGlyph">
              <Film size={14} strokeWidth={2.1} />
            </span>
            <span className="videos__listText">
              <strong>{video.title}</strong>
              <small>{video.format.toUpperCase()}</small>
            </span>
          </button>
        ))}
      </div>
    </section>
  );
}
