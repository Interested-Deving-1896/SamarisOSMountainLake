import React from "react";
import { ArrowLeft, ArrowRight, RotateCcw, X, Plus, Globe, ShieldCheck, Bookmark, EllipsisVertical, ExternalLink, EyeOff, ZoomIn, ZoomOut, Clock, Star, Trash2 } from "lucide-react";
import type { PeregrineTab } from "../types";
import type { HistoryEntry } from "../hooks/useHistory";
import "../peregrine.css";

export function PeregrineToolbar(props: {
  tabs: PeregrineTab[]; activeTabId: string | null; value: string; loading: boolean;
  onChange: (v: string) => void; onSubmit: () => void;
  onGoBack: () => void; onGoForward: () => void; onReload: () => void;
  onCloseTab: (id: string) => void; onSelectTab: (id: string) => void; onNewTab: () => void;
  onNewPrivateTab?: () => void;
  history?: HistoryEntry[];
  bookmarks?: HistoryEntry[];
  isBookmarked?: boolean;
  onAddBookmark?: () => void;
  onRemoveBookmark?: () => void;
  onClear?: () => void;
  onOpenHistory?: (url: string) => void;
  zoom?: number;
  onZoomIn?: () => void;
  onZoomOut?: () => void;
  onZoomReset?: () => void;
}) {
  const activeTab = props.tabs.find((t) => t.id === props.activeTabId);
  const [menuOpen, setMenuOpen] = React.useState(false);
  const [urlFocused, setUrlFocused] = React.useState(false);
  const [autocompleteIdx, setAutocompleteIdx] = React.useState(-1);
  const menuRef = React.useRef<HTMLDivElement>(null);
  const urlWrapRef = React.useRef<HTMLDivElement>(null);
  const inputRef = React.useRef<HTMLInputElement>(null);

  React.useEffect(() => {
    if (!menuOpen) return;
    const close = (e: PointerEvent) => { if (!menuRef.current?.contains(e.target as Node)) setMenuOpen(false); };
    window.addEventListener("pointerdown", close);
    return () => window.removeEventListener("pointerdown", close);
  }, [menuOpen]);

  React.useEffect(() => {
    if (!urlFocused) return;
    const close = (e: PointerEvent) => {
      if (urlWrapRef.current && !urlWrapRef.current.contains(e.target as Node)) {
        setUrlFocused(false);
      }
    };
    window.addEventListener("pointerdown", close);
    return () => window.removeEventListener("pointerdown", close);
  }, [urlFocused]);

  const isSecure = activeTab?.url?.startsWith("https://");
  const isBookmarked = props.isBookmarked ?? false;
  const zoom = props.zoom ?? 1;
  const zoomPct = Math.round(zoom * 100);

  const hist = props.history ?? [];
  const bm = props.bookmarks ?? [];
  const query = props.value.trim().toLowerCase();

  const filteredHistory = query ? hist.filter((e) => e.url.toLowerCase().includes(query) || e.title.toLowerCase().includes(query)).slice(0, 5) : [];
  const filteredBookmarks = query ? bm.filter((e) => e.url.toLowerCase().includes(query) || e.title.toLowerCase().includes(query)).slice(0, 3) : [];
  const suggestions = [...filteredBookmarks.map((e) => ({ ...e, kind: "bookmark" as const })), ...filteredHistory.filter((e) => !filteredBookmarks.some((b) => b.url === e.url)).map((e) => ({ ...e, kind: "history" as const }))];

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === "Enter") {
      if (autocompleteIdx >= 0 && autocompleteIdx < suggestions.length) {
        const s = suggestions[autocompleteIdx];
        props.onChange(s.url);
        setUrlFocused(false);
        setAutocompleteIdx(-1);
        setTimeout(() => props.onSubmit(), 0);
        return;
      }
      setUrlFocused(false);
      setAutocompleteIdx(-1);
      props.onSubmit();
      return;
    }
    if (e.key === "Escape") {
      setUrlFocused(false);
      setAutocompleteIdx(-1);
      inputRef.current?.blur();
      return;
    }
    if (e.key === "ArrowDown") {
      e.preventDefault();
      setAutocompleteIdx((i) => Math.min(i + 1, suggestions.length - 1));
      return;
    }
    if (e.key === "ArrowUp") {
      e.preventDefault();
      setAutocompleteIdx((i) => Math.max(i - 1, -1));
      return;
    }
    setAutocompleteIdx(-1);
  };

  const selectSuggestion = (url: string) => {
    props.onChange(url);
    setUrlFocused(false);
    setAutocompleteIdx(-1);
    setTimeout(() => props.onSubmit(), 0);
  };

  const zoomLevelClass = zoom > 1 ? "pr-zoomBadge--up" : zoom < 1 ? "pr-zoomBadge--down" : "";

  const openMenu = (event: React.MouseEvent<HTMLButtonElement>) => {
    event.preventDefault();
    event.stopPropagation();
    setMenuOpen((open) => !open);
  };

  return (
    <div className="pr-toolbar">
      {/* Tab strip */}
      <div className="pr-tabstrip">
        {props.tabs.map((tab) => (
          <div key={tab.id} className={`pr-tab ${tab.id === props.activeTabId ? "pr-tab--active" : ""}`} onClick={() => props.onSelectTab(tab.id)}>
            {tab.favicon ? <img src={tab.favicon} alt="" className="pr-tabFavicon" /> : <Globe size={12} className="pr-tabFavicon" />}
            <span className="pr-tabTitle">{tab.title || "New Tab"}</span>
            {tab.loading && <span className="pr-tabLoader" />}
            <button className="pr-tabClose" onClick={(e) => { e.stopPropagation(); props.onCloseTab(tab.id); }}><X size={10} /></button>
          </div>
        ))}
        <button className="pr-newTab" onClick={props.onNewTab} title="New tab"><Plus size={14} /></button>
        {props.onNewPrivateTab && (
          <button className="pr-newPrivateTab" onClick={props.onNewPrivateTab} title="New private tab"><EyeOff size={13} /></button>
        )}
      </div>

      {/* Navigation + URL */}
      <div className="pr-navRow">
        <button className="pr-navBtn" onClick={props.onGoBack} disabled={!activeTab?.canGoBack} title="Back"><ArrowLeft size={16} /></button>
        <button className="pr-navBtn" onClick={props.onGoForward} disabled={!activeTab?.canGoForward} title="Forward"><ArrowRight size={16} /></button>
        <button className="pr-navBtn" onClick={props.onReload} title="Reload"><RotateCcw size={15} /></button>

        <div ref={urlWrapRef} className="pr-urlWrap">
          {activeTab?.url ? (
            isSecure ? <ShieldCheck size={13} className="pr-urlSecure" /> : <Globe size={13} className="pr-urlInsecure" />
          ) : null}
          <input
            ref={inputRef}
            className="pr-urlInput"
            value={props.value}
            onChange={(e) => { props.onChange(e.target.value); setAutocompleteIdx(-1); }}
            onFocus={() => setUrlFocused(true)}
            onKeyDown={handleKeyDown}
            placeholder="Search or enter URL"
            spellCheck={false}
          />
          <button
            className={`pr-urlBookmark ${isBookmarked ? "pr-urlBookmark--active" : ""}`}
            onClick={() => isBookmarked ? props.onRemoveBookmark?.() : props.onAddBookmark?.()}
            title={isBookmarked ? "Remove bookmark" : "Bookmark this page"}
          >
            <Bookmark size={14} fill={isBookmarked ? "#f59e0b" : "none"} />
          </button>

          {/* Autocomplete dropdown */}
          {urlFocused && suggestions.length > 0 && (
            <div className="pr-autocomplete">
              {suggestions.map((item, i) => (
                <button
                  key={`${item.kind}-${item.id}`}
                  className={`pr-ac-item ${i === autocompleteIdx ? "pr-ac-item--active" : ""}`}
                  onPointerDown={(e) => { e.preventDefault(); selectSuggestion(item.url); }}
                >
                  <span className="pr-ac-icon">
                    {item.kind === "bookmark" ? <Star size={12} fill="#f59e0b" /> : <Clock size={12} />}
                  </span>
                  <div className="pr-ac-text">
                    <span className="pr-ac-title">{item.title || item.url}</span>
                    <span className="pr-ac-url">{item.url}</span>
                  </div>
                </button>
              ))}
            </div>
          )}
        </div>

        {/* Zoom controls */}
        <div className="pr-zoomGroup">
          <button className="pr-zoomBtn" onClick={props.onZoomOut} title="Zoom out" disabled={zoom <= 0.25}><ZoomOut size={14} /></button>
          <button className={`pr-zoomBadge ${zoomLevelClass}`} onClick={props.onZoomReset} title="Reset zoom">{zoomPct}%</button>
          <button className="pr-zoomBtn" onClick={props.onZoomIn} title="Zoom in" disabled={zoom >= 5}><ZoomIn size={14} /></button>
        </div>

        <div ref={menuRef} style={{ position: "relative" }}>
          <button className="pr-menuBtn" onClick={openMenu} title="Menu"><EllipsisVertical size={16} /></button>
          {menuOpen && (
            <div className="pr-menu">
              <button className="pr-menuItem" onClick={() => { setMenuOpen(false); props.onNewTab(); }}>
                <Plus size={14} /> New tab
              </button>
              {props.onNewPrivateTab && (
                <button className="pr-menuItem" onClick={() => { setMenuOpen(false); props.onNewPrivateTab?.(); }}>
                  <EyeOff size={14} /> New private tab
                </button>
              )}
              <button className="pr-menuItem" onClick={() => { setMenuOpen(false); props.onReload(); }}>
                <RotateCcw size={14} /> Reload
              </button>
              <div className="pr-menuSep" />
              {props.onZoomReset && (
                <button className="pr-menuItem" onClick={() => { setMenuOpen(false); props.onZoomReset?.(); }}>
                  <RotateCcw size={14} /> Reset zoom
                </button>
              )}
              {props.onClear && (
                <button className="pr-menuItem" onClick={() => { setMenuOpen(false); props.onClear?.(); }}>
                  <Trash2 size={14} /> Clear browsing data
                </button>
              )}
              <div className="pr-menuSep" />
              <button className="pr-menuItem" onClick={() => { setMenuOpen(false); if (activeTab?.url) window.electronAPI?.shell.openExternal(activeTab.url); }} disabled={!activeTab?.url}>
                <ExternalLink size={14} /> Open in system browser
              </button>
            </div>
          )}
        </div>
      </div>

      {/* Loading progress bar */}
      {props.loading && <div className="pr-progress"><div className="pr-progressBar" /></div>}
    </div>
  );
}
