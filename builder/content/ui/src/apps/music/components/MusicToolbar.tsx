import React from "react";
import { Grid2X2, Import, List, Maximize2, Minimize2, Search, SlidersHorizontal } from "lucide-react";
import type { EqualizerPreset, MusicSort, MusicViewMode } from "../types";

export function MusicToolbar(props: {
  query: string;
  onQueryChange: (value: string) => void;
  sortBy: MusicSort;
  onSortChange: (value: MusicSort) => void;
  viewMode: MusicViewMode;
  onViewModeChange: (value: MusicViewMode) => void;
  equalizerPreset: EqualizerPreset;
  onEqualizerChange: (value: EqualizerPreset) => void;
  onImportRequest: (files: FileList | null) => void;
  fullscreenPlayer: boolean;
  onToggleFullscreenPlayer: () => void;
  resultLabel: string;
}) {
  const inputRef = React.useRef<HTMLInputElement | null>(null);

  return (
    <div className="music__toolbar">
      <div className="music__search">
        <Search size={14} strokeWidth={2.2} />
        <input
          value={props.query}
          onChange={(event) => props.onQueryChange(event.target.value)}
          placeholder="Search songs, artists, albums"
          aria-label="Search library"
        />
      </div>

      <div className="music__toolbarControls">
        <label className="music__controlSelect">
          <span>Sort</span>
          <select value={props.sortBy} onChange={(event) => props.onSortChange(event.target.value as MusicSort)}>
            <option value="title">Title</option>
            <option value="artist">Artist</option>
            <option value="album">Album</option>
            <option value="recent">Recent</option>
          </select>
        </label>

        <label className="music__controlSelect">
          <SlidersHorizontal size={13} strokeWidth={2.1} />
          <select
            value={props.equalizerPreset}
            onChange={(event) => props.onEqualizerChange(event.target.value as EqualizerPreset)}
          >
            <option value="flat">EQ: Flat</option>
            <option value="bass-boost">EQ: Bass Boost</option>
            <option value="vocal">EQ: Vocal</option>
            <option value="acoustic">EQ: Acoustic</option>
            <option value="night">EQ: Night</option>
          </select>
        </label>

        <div className="music__viewToggle" role="tablist" aria-label="Library view">
          <button
            type="button"
            className={props.viewMode === "list" ? "music__viewToggleBtn music__viewToggleBtn--active" : "music__viewToggleBtn"}
            onClick={() => props.onViewModeChange("list")}
            aria-label="List view"
          >
            <List size={14} strokeWidth={2.2} />
          </button>
          <button
            type="button"
            className={props.viewMode === "grid" ? "music__viewToggleBtn music__viewToggleBtn--active" : "music__viewToggleBtn"}
            onClick={() => props.onViewModeChange("grid")}
            aria-label="Grid view"
          >
            <Grid2X2 size={14} strokeWidth={2.2} />
          </button>
        </div>

        <button type="button" className="music__toolbarIconBtn" onClick={() => inputRef.current?.click()}>
          <Import size={14} strokeWidth={2.2} />
          <span>Import</span>
        </button>

        <button type="button" className="music__toolbarIconBtn" onClick={props.onToggleFullscreenPlayer}>
          {props.fullscreenPlayer ? <Minimize2 size={14} strokeWidth={2.2} /> : <Maximize2 size={14} strokeWidth={2.2} />}
          <span>{props.fullscreenPlayer ? "Mini player" : "Full player"}</span>
        </button>

        <input
          ref={inputRef}
          hidden
          type="file"
          accept=".mp3,.flac,.wav,.m4a,.aac,.ogg,.weba,audio/*"
          multiple
          onChange={(event) => {
            props.onImportRequest(event.target.files);
            event.currentTarget.value = "";
          }}
        />
      </div>

      <div className="music__toolbarMeta">{props.resultLabel}</div>
    </div>
  );
}
