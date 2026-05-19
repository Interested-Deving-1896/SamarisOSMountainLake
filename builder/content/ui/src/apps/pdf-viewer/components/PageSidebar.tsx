import React, { useMemo, useState, useCallback, useRef, useEffect } from "react";
import { Search, Loader } from "lucide-react";
import PageThumbnail from "./PageThumbnail";

type PageSidebarProps = {
  getPage: (n: number) => Promise<any>;
  numPages: number;
  activePage: number;
  searchResults: Map<number, number>;
  searching: boolean;
  onJumpToPage: (pageNumber: number) => void;
};

const PageSidebar: React.FC<PageSidebarProps> = ({ getPage, numPages, activePage, searchResults, searching, onJumpToPage }) => {
  const [query, setQuery] = useState("");
  const [debouncedQuery, setDebouncedQuery] = useState("");
  const debounceRef = useRef<ReturnType<typeof setTimeout>>();

  useEffect(() => {
    if (debounceRef.current) clearTimeout(debounceRef.current);
    debounceRef.current = setTimeout(() => setDebouncedQuery(query), 300);
    return () => { if (debounceRef.current) clearTimeout(debounceRef.current); };
  }, [query]);

  const filteredPages = useMemo(() => {
    const q = debouncedQuery.trim().toLowerCase();
    const pages = Array.from({ length: numPages }, (_, i) => i + 1);
    if (!q) return pages.map((n) => ({ pageNumber: n, matchCount: searchResults.get(n) || 0 }));
    return pages
      .map((n) => ({ pageNumber: n, matchCount: searchResults.get(n) || 0 }))
      .filter((p) => String(p.pageNumber).includes(q) || p.matchCount > 0);
  }, [numPages, debouncedQuery, searchResults]);

  const totalHits = useMemo(() => [...searchResults.values()].reduce((s, c) => s + c, 0), [searchResults]);

  return (
    <div style={{ width: 240, overflow: "auto", borderRight: "1px solid rgba(226,232,240,0.6)", background: "rgba(255,255,255,0.65)", padding: 16, backdropFilter: "blur(20px)" }}>
      <div style={{ position: "relative", marginBottom: 16 }}>
        <Search size={16} style={{ position: "absolute", left: 12, top: "50%", transform: "translateY(-50%)", color: "#94a3b8" }} />
        <input
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          placeholder="Search…"
          style={{
            width: "100%",
            borderRadius: 16,
            border: "1px solid rgba(226,232,240,0.7)",
            background: "rgba(255,255,255,0.8)",
            padding: "8px 12px 8px 36px",
            fontSize: 14,
            outline: "none",
            boxSizing: "border-box",
          }}
        />
        {searching && <Loader size={12} style={{ position: "absolute", right: 12, top: "50%", transform: "translateY(-50%)", color: "#94a3b8" }} />}
        {totalHits > 0 && (
          <div style={{ position: "absolute", right: 12, top: "50%", transform: "translateY(-50%)", fontSize: 12, fontWeight: 600, color: "#2563eb" }}>{totalHits}</div>
        )}
      </div>

      <div>
        {filteredPages.map(({ pageNumber, matchCount }) => (
          <PageThumbnail
            key={pageNumber}
            getPage={getPage}
            pageNumber={pageNumber}
            active={pageNumber === activePage}
            onClick={onJumpToPage}
          />
        ))}
      </div>
    </div>
  );
};

export default React.memo(PageSidebar);
