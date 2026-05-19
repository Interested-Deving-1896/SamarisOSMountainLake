import React from "react";
import { Search } from "lucide-react";
import { searchKernel, type SearchResult } from "../services/kernel/search";
import { appLoader } from "../os/apps/appLoader";
import { openPathInApp } from "../os/filesystem/fileActions";

export function Spotlight() {
  const [open, setOpen] = React.useState(false);
  const [query, setQuery] = React.useState("");
  const [results, setResults] = React.useState<SearchResult[]>([]);

  React.useEffect(() => {
    const onKeyDown = (event: KeyboardEvent) => {
      if ((event.metaKey || event.ctrlKey) && event.code === "Space") {
        event.preventDefault();
        setOpen((current) => !current);
      }
      if (event.key === "Escape") setOpen(false);
    };
    window.addEventListener("keydown", onKeyDown);
    return () => window.removeEventListener("keydown", onKeyDown);
  }, []);

  React.useEffect(() => {
    if (!open) return;
    void searchKernel.query(query).then(setResults).catch(() => setResults([]));
  }, [open, query]);

  if (!open) return null;

  return (
    <div className="spotlight" onPointerDown={(event) => event.target === event.currentTarget && setOpen(false)}>
      <div className="spotlight__panel">
        <div className="spotlight__search">
          <Search size={16} strokeWidth={2.1} />
          <input autoFocus value={query} onChange={(event) => setQuery(event.target.value)} placeholder="Search Samaris OS" />
        </div>
        <div className="spotlight__results">
          {results.map((result) => (
            <button
              key={`${result.kind}:${result.id}`}
              type="button"
              className="spotlight__result"
              onClick={() => {
                setOpen(false);
                if (result.kind === "app") {
                  void appLoader.openApp(result.id);
                } else if (result.kind === "file") {
                  void openPathInApp(result.id, "file");
                } else {
                  void appLoader.openApp("settings");
                }
              }}
            >
              <span>{result.title}</span>
              <small>{result.subtitle}</small>
            </button>
          ))}
          {!results.length ? <div className="spotlight__empty">No results.</div> : null}
        </div>
      </div>
    </div>
  );
}
