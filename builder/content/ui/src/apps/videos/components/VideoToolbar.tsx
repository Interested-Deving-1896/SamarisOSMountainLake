import { Search } from "lucide-react";
import type { SortMode } from "../types";

export function VideoToolbar(props: {
  count: number;
  searchQuery: string;
  sortMode: SortMode;
  onSearchChange: (q: string) => void;
  onSortChange: (m: SortMode) => void;
}) {
  return (
    <div className="videos__toolbar">
      <div className="videos__toolbarLeft">
        <div className="videos__toolbarTitle">Videos</div>
        <span className="videos__toolbarCount">{props.count} items</span>
      </div>
      <div className="videos__toolbarRight">
        <div className="videos__search">
          <Search size={14} strokeWidth={2.2} />
          <input
            type="text"
            className="videos__searchInput"
            placeholder="Search videos..."
            value={props.searchQuery}
            onChange={(e) => props.onSearchChange(e.target.value)}
          />
        </div>
        <select
          className="videos__sort"
          value={props.sortMode}
          onChange={(e) => props.onSortChange(e.target.value as SortMode)}
        >
          <option value="name">Name</option>
          <option value="size">Size</option>
          <option value="format">Format</option>
        </select>
      </div>
    </div>
  );
}
