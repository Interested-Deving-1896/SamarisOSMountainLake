import { Clapperboard, Film } from "lucide-react";

export function VideoSidebar(props: {
  total: number;
  activeLabel?: string;
}) {
  return (
    <aside className="videos__sidebar">
      <div className="videos__brand">
        <div className="videos__brandGlyph">
          <Clapperboard size={18} strokeWidth={2.2} />
        </div>
        <div>
          <div className="videos__brandTitle">Videos</div>
          <div className="videos__brandMeta">{props.total} items</div>
        </div>
      </div>
      <div className="videos__section">
        <div className="videos__sectionTitle">Library</div>
        <button type="button" className="videos__navItem videos__navItem--active">
          <Film size={14} strokeWidth={2.2} />
          All Videos
        </button>
      </div>
      <div className="videos__section">
        <div className="videos__sectionTitle">Sources</div>
        <button type="button" className="videos__navItem">
          <Film size={14} strokeWidth={2.2} />
          ~/Videos
        </button>
      </div>
      {props.activeLabel && (
        <div className="videos__nowPlaying">
          <div className="videos__sectionTitle">Now Playing</div>
          <div className="videos__nowLabel">{props.activeLabel}</div>
        </div>
      )}
    </aside>
  );
}
