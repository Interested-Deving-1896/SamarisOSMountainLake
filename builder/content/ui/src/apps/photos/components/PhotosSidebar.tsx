import { ImageIcon, Clock } from "lucide-react";
import type { ViewFilter } from "../types";

export function PhotosSidebar(props: {
  photoCount: number;
  sourcePath: string;
  viewFilter: ViewFilter;
  onViewChange: (view: ViewFilter) => void;
}) {
  return (
    <aside className="photos__sidebar">
      <div className="photos__brand">
        <div className="photos__brandGlyph">
          <ImageIcon size={18} strokeWidth={2.2} />
        </div>
        <div>
          <div className="photos__brandTitle">Photos</div>
          <div className="photos__brandMeta">{props.photoCount} items</div>
        </div>
      </div>
      <div className="photos__section">
        <div className="photos__sectionTitle">Library</div>
        <button
          type="button"
          className={`photos__navItem ${props.viewFilter === "all" ? "photos__navItem--active" : ""}`}
          onClick={() => props.onViewChange("all")}
        >
          <ImageIcon size={14} strokeWidth={2.2} />
          All Photos
        </button>
        <button
          type="button"
          className={`photos__navItem ${props.viewFilter === "recent" ? "photos__navItem--active" : ""}`}
          onClick={() => props.onViewChange("recent")}
        >
          <Clock size={14} strokeWidth={2.2} />
          Recent
        </button>
      </div>
    </aside>
  );
}
