import { Search, Play } from "lucide-react";
import type { SortMode } from "../types";

export function PhotoToolbar(props: {
  photoCount: number;
  searchQuery: string;
  sortMode: SortMode;
  onSearchChange: (query: string) => void;
  onSortChange: (mode: SortMode) => void;
  onSlideshow: () => void;
  sourcePath: string;
}) {
  return (
    <div className="photos__toolbar">
      <div className="photos__toolbarLeft">
        <div className="photos__toolbarTitle">{props.sourcePath.split("/").pop() || "Photos"}</div>
        <span className="photos__toolbarCount">{props.photoCount} items</span>
      </div>
      <div className="photos__toolbarRight">
        <div className="photos__search">
          <Search size={14} strokeWidth={2.2} />
          <input
            type="text"
            className="photos__searchInput"
            placeholder="Search photos..."
            value={props.searchQuery}
            onChange={(e) => props.onSearchChange(e.target.value)}
          />
        </div>
        <select
          className="photos__sort"
          value={props.sortMode}
          onChange={(e) => props.onSortChange(e.target.value as SortMode)}
        >
          <option value="name">Name</option>
          <option value="size">Size</option>
          <option value="newest">Newest</option>
        </select>
        <button type="button" className="photos__slideshowBtn" onClick={props.onSlideshow} title="Slideshow">
          <Play size={14} strokeWidth={2.2} />
        </button>
      </div>
    </div>
  );
}
