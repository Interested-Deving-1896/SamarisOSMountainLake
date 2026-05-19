import React, { useMemo, useRef, useState, useCallback, useEffect } from "react";
import { osStore } from "../../os/core/osStore";
import { useFs } from "../../services/fs/useFs";
import { usePdfDocument } from "./hooks/usePdfDocument";
import ViewerHeader from "./components/ViewerHeader";
import PageSidebar from "./components/PageSidebar";
import ContinuousViewer from "./components/ContinuousViewer";
import "./pdf-viewer.css";

export default function PdfViewer(props: { windowId: string }) {
  const fs = useFs();
  const state = React.useSyncExternalStore((listener) => osStore.subscribe(listener), () => osStore.getState());
  const pdfPath = state.windows.find((w) => w.id === props.windowId)?.params?.path as string | undefined;

  const [dataUrl, setDataUrl] = useState("");
  const [scale, setScale] = useState(0.75);
  const [activePage, setActivePage] = useState(1);
  const [searchResults, setSearchResults] = useState<Map<number, number>>(new Map());
  const [searching, setSearching] = useState(false);
  const scrollRef = useRef<HTMLDivElement>(null);

  const { doc, numPages, loading, error, progress } = usePdfDocument(dataUrl);

  useEffect(() => {
    if (!pdfPath) return;
    let cancelled = false;
    (async () => {
      try {
        const file = await fs.readDataUrl(pdfPath);
        if (!cancelled && file.dataUrl) setDataUrl(file.dataUrl);
      } catch {
        if (!cancelled) setDataUrl("");
      }
    })();
    return () => { cancelled = true; };
  }, [fs, pdfPath]);

  const getPage = useCallback((n: number) => doc!.getPage(n), [doc]);

  const jumpToPage = useCallback((pageNumber: number) => {
    setActivePage(pageNumber);
    const node = scrollRef.current?.querySelector<HTMLElement>(`[data-page-number="${pageNumber}"]`);
    node?.scrollIntoView({ behavior: "smooth", block: "start" });
  }, []);

  const subtitle = useMemo(() => {
    if (error) return "Error";
    if (loading || !doc) return progress > 0 ? `Loading ${progress}%…` : "Loading…";
    return `Page ${activePage}/${numPages} · Continuous scroll · ${Math.round(scale * 100)}%`;
  }, [activePage, doc, error, loading, numPages, progress, scale]);

  useEffect(() => {
    if (!doc || !dataUrl) { setSearchResults(new Map()); return; }
    let cancelled = false;
    setSearching(true);
    (async () => {
      const results = new Map<number, number>();
      for (let i = 1; i <= doc.numPages; i++) {
        if (cancelled) return;
        try {
          const page = await doc.getPage(i);
          const tc = await page.getTextContent();
          const text = tc.items.map((item: any) => item.str || "").join(" ");
          results.set(i, text.length);
          if (i % 5 === 0) setSearchResults(new Map(results));
        } catch { results.set(i, 0); }
      }
      if (!cancelled) { setSearchResults(results); setSearching(false); }
    })();
    return () => { cancelled = true; };
  }, [doc, dataUrl]);

  if (!pdfPath) {
    return (
      <div style={{ display: "flex", height: "100%", alignItems: "center", justifyContent: "center", background: "linear-gradient(135deg,rgba(255,255,255,0.7),rgba(248,250,252,0.4))" }}>
        <div style={{ fontSize: 14, color: "#94a3b8" }}>Open a PDF from Finder to preview it here.</div>
      </div>
    );
  }

  if (error) {
    return (
      <div style={{ display: "flex", height: "100%", justifyContent: "center", background: "linear-gradient(135deg,rgba(255,255,255,0.7),rgba(248,250,252,0.4))", padding: 24 }}>
        <div style={{ width: "100%", maxWidth: 512, borderRadius: 24, border: "1px solid #fecaca", background: "#fff1f2", padding: 20, fontSize: 14, color: "#be123c" }}>{error}</div>
      </div>
    );
  }

  if (loading || !doc) {
    return (
      <div style={{ display: "flex", height: "100%", justifyContent: "center", background: "linear-gradient(135deg,rgba(255,255,255,0.7),rgba(248,250,252,0.4))", padding: 24 }}>
        <div style={{ width: "100%", maxWidth: 512, borderRadius: 24, border: "1px solid rgba(226,232,240,0.6)", background: "rgba(255,255,255,0.75)", padding: 24, fontSize: 14, color: "#475569" }}>
          {progress > 0 ? `Loading PDF (${progress}%)…` : "Loading PDF…"}
        </div>
      </div>
    );
  }

  return (
    <div style={{ display: "flex", height: "100%", flexDirection: "column", background: "linear-gradient(135deg,rgba(255,255,255,0.7),rgba(248,250,252,0.4))" }}>
      <ViewerHeader
        title={pdfPath.split("/").pop() || "Document"}
        subtitle={subtitle}
        onZoomOut={() => setScale((s) => Math.max(0.45, Number((s - 0.1).toFixed(2))))}
        onZoomIn={() => setScale((s) => Math.min(1.6, Number((s + 0.1).toFixed(2))))}
      />
      <div style={{ display: "flex", minHeight: 0, flex: 1 }}>
        <PageSidebar
          getPage={getPage}
          numPages={numPages}
          activePage={activePage}
          searchResults={searchResults}
          searching={searching}
          onJumpToPage={jumpToPage}
        />
        <ContinuousViewer
          getPage={getPage}
          numPages={numPages}
          scale={scale}
          activePage={activePage}
          scrollRef={scrollRef}
          onVisible={setActivePage}
        />
      </div>
    </div>
  );
}
