import React from "react";
import { ChevronLeft, ChevronRight, Columns, LayoutGrid, List, RotateCw, Search, Trash2 } from "lucide-react";
import type { FinderViewMode } from "../model";

const VIEW_ICONS: Record<FinderViewMode, typeof List> = { list: List, grid: LayoutGrid, columns: Columns };

export const FinderToolbar = React.memo(function FinderToolbar(props: {
  canGoBack: boolean;
  canGoForward: boolean;
  onBack: () => void;
  onForward: () => void;
  onRefresh: () => void;
  crumbs: Array<{ label: string; path: string }>;
  onOpenCrumb: (path: string) => void;
  searchQuery: string;
  onSearchQueryChange: (value: string) => void;
  viewMode: FinderViewMode;
  onChangeViewMode: (mode: FinderViewMode) => void;
  canDeleteSelected?: boolean;
  onDeleteSelected?: () => void;
  selectedCount?: number;
}) {
  return (
    <div className="finder-toolbar">
      <div className="finder-toolbar__nav">
        <button type="button" className="finder-toolbar__btn" onClick={props.onBack} disabled={!props.canGoBack} aria-label="Go back">
          <ChevronLeft size={16} />
        </button>
        <button type="button" className="finder-toolbar__btn" onClick={props.onForward} disabled={!props.canGoForward} aria-label="Go forward">
          <ChevronRight size={16} />
        </button>
        <button type="button" className="finder-toolbar__btn" onClick={props.onRefresh} aria-label="Reload">
          <RotateCw size={14} />
        </button>
      </div>

      <div className="finder-toolbar__breadcrumbs" aria-label="Path">
        {props.crumbs.map((crumb, index) => (
          <React.Fragment key={crumb.path}>
            {index > 0 ? <span className="finder-toolbar__sep">›</span> : null}
            <button type="button" className="finder-toolbar__crumb" onClick={() => props.onOpenCrumb(crumb.path)} title={crumb.path}>
              {crumb.label}
            </button>
          </React.Fragment>
        ))}
      </div>

      <div className="finder-toolbar__tools">
        <div className="finder-toolbar__views" role="tablist" aria-label="View mode">
          {(Object.keys(VIEW_ICONS) as FinderViewMode[]).map((mode) => {
            const Icon = VIEW_ICONS[mode];
            return (
              <button key={mode} type="button"
                className={`finder-toolbar__viewBtn ${props.viewMode === mode ? "finder-toolbar__viewBtn--active" : ""}`}
                aria-label={`${mode} view`} role="tab" aria-selected={props.viewMode === mode}
                onClick={() => props.onChangeViewMode(mode)}
              >
                <Icon size={15} />
              </button>
            );
          })}
        </div>
        <button type="button" className="finder-toolbar__btn finder-toolbar__btn--danger"
          onClick={props.onDeleteSelected} disabled={!props.canDeleteSelected} aria-label="Move to Trash">
          <Trash2 size={14} />
        </button>
        <div className="finder-toolbar__search">
          <Search size={14} />
          <input
            value={props.searchQuery}
            onChange={(event) => props.onSearchQueryChange(event.target.value)}
            placeholder="Search"
            aria-label="Search files"
          />
        </div>
        {(props.selectedCount ?? 0) > 0 ? (
          <span className="finder-toolbar__count">{props.selectedCount} selected</span>
        ) : null}
      </div>
    </div>
  );
});
