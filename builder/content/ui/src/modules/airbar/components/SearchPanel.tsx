import React from "react";
import { Search, X } from "lucide-react";
import { SamarisIcon, iconNameForFile } from "../../../modules/icons";
import { openPathInApp } from "../../../os/filesystem/fileActions";
import { searchKernel, type SearchResult } from "../../../services/kernel/search";
import { useAirBar } from "../useAirBar";

export const SearchPanel = React.memo(function SearchPanel() {
  const air = useAirBar();
  const open = air.activePanel === "search";
  const inputRef = React.useRef<HTMLInputElement | null>(null);
  const [query, setQuery] = React.useState("");
  const [results, setResults] = React.useState<SearchResult[]>([]);
  const [loading, setLoading] = React.useState(false);
  const [error, setError] = React.useState<string | null>(null);
  const [activeIndex, setActiveIndex] = React.useState(0);
  const style = air.getPanelStyle("search", { width: 620, maxWidth: 720, align: "center" });

  React.useEffect(() => {
    if (!open) return;
    const frame = window.setTimeout(() => inputRef.current?.focus(), 0);
    return () => window.clearTimeout(frame);
  }, [open]);

  React.useEffect(() => {
    if (!open) {
      setQuery("");
      setResults([]);
      setError(null);
      setLoading(false);
      setActiveIndex(0);
      return;
    }
  }, [open]);

  React.useEffect(() => {
    const trimmed = query.trim();
    if (!open || !trimmed) {
      setResults([]);
      setError(null);
      setLoading(false);
      setActiveIndex(0);
      return;
    }

    setLoading(true);
    setError(null);
    const timer = window.setTimeout(() => {
      void searchKernel
        .queryFiles(trimmed)
        .then((items) => {
          setResults(items);
          setActiveIndex(0);
        })
        .catch((err) => {
          setResults([]);
          setError(err instanceof Error ? err.message : "Search unavailable");
        })
        .finally(() => setLoading(false));
    }, 180);

    return () => window.clearTimeout(timer);
  }, [open, query]);

  async function openResult(result: SearchResult | undefined) {
    if (!result?.path) return;
    await openPathInApp(result.path, result.fileKind === "dir" ? "dir" : "file");
    air.closePanels();
  }

  return (
    <section
      style={style}
      className={`airbar-panel search-panel ${open ? "open" : ""}`}
      role="dialog"
      aria-label="Search"
      onKeyDown={(event) => {
        if (event.key === "ArrowDown") {
          event.preventDefault();
          setActiveIndex((current) => Math.min(current + 1, Math.max(results.length - 1, 0)));
        } else if (event.key === "ArrowUp") {
          event.preventDefault();
          setActiveIndex((current) => Math.max(current - 1, 0));
        } else if (event.key === "Enter") {
          event.preventDefault();
          void openResult(results[activeIndex]);
        }
      }}
    >
      <div className="search-shell">
        <div className="search-head">
          <Search size={16} strokeWidth={2.2} />
          <input
            ref={inputRef}
            className="search-input"
            placeholder="Search all files"
            value={query}
            onChange={(event) => setQuery(event.target.value)}
          />
          {query ? (
            <button type="button" className="search-clear" onClick={() => setQuery("")} aria-label="Clear search">
              <X size={14} strokeWidth={2.2} />
            </button>
          ) : null}
        </div>
        <div className="search-results">
          {!query.trim() ? <div className="search-empty">Search across Desktop, Documents, Downloads, Music, Photos, Videos, and your folders.</div> : null}
          {loading ? <div className="search-empty">Searching…</div> : null}
          {!loading && error ? <div className="search-empty">Search unavailable: {error}</div> : null}
          {!loading && !error && query.trim() && results.length === 0 ? <div className="search-empty">No files found.</div> : null}
          {!loading &&
            !error &&
            results.map((result, index) => (
              <button
                key={result.id}
                type="button"
                className={`search-result ${index === activeIndex ? "active" : ""}`}
                onMouseEnter={() => setActiveIndex(index)}
                onClick={() => void openResult(result)}
              >
                <span className="search-result__icon">
                  <SamarisIcon name={iconNameForFile(result.title, result.fileKind === "dir" ? "dir" : "file")} size={14} variant="mono" surface="bare" />
                </span>
                <span className="search-result__text">
                  <span className="search-result__title">{result.title}</span>
                  <span className="search-result__meta">{result.path || result.subtitle}</span>
                </span>
              </button>
            ))}
        </div>
      </div>
    </section>
  );
});
