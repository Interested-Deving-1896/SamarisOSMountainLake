import React, { useEffect, useRef, useState } from "react";

type PdfPageProps = {
  getPage: (n: number) => Promise<any>;
  pageNumber: number;
  scale: number;
  active: boolean;
  onVisible: (pageNumber: number) => void;
};

const s: React.CSSProperties = {
  borderRadius: 28,
  padding: 20,
  transition: "all 0.2s",
  marginBottom: 24,
  position: "relative",
};

const PdfPage: React.FC<PdfPageProps> = ({ getPage, pageNumber, scale, active, onVisible }) => {
  const wrapperRef = useRef<HTMLDivElement>(null);
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [nearViewport, setNearViewport] = useState(pageNumber <= 3);
  const [minHeight, setMinHeight] = useState<number | null>(null);

  useEffect(() => {
    const node = wrapperRef.current;
    if (!node) return;
    const observer = new IntersectionObserver(
      (entries) => {
        const entry = entries[0];
        if (!entry) return;
        if (entry.isIntersecting) {
          onVisible(pageNumber);
          setNearViewport(true);
        }
      },
      { rootMargin: "700px 0px", threshold: 0.2 }
    );
    observer.observe(node);
    return () => observer.disconnect();
  }, [onVisible, pageNumber]);

  useEffect(() => {
    let cancelled = false;
    if (!nearViewport || !canvasRef.current) return;
    (async () => {
      try {
        const page = await getPage(pageNumber);
        if (cancelled || !canvasRef.current) return;
        const viewport = page.getViewport({ scale });
        const canvas = canvasRef.current;
        const context = canvas.getContext("2d");
        if (!context) return;
        const dpr = Math.min(window.devicePixelRatio || 1, 2);
        canvas.width = Math.floor(viewport.width * dpr);
        canvas.height = Math.floor(viewport.height * dpr);
        canvas.style.width = `${Math.floor(viewport.width)}px`;
        canvas.style.height = `${Math.floor(viewport.height)}px`;
        context.setTransform(dpr, 0, 0, dpr, 0, 0);
        setMinHeight(Math.floor(viewport.height) + 56);
        await page.render({ canvasContext: context, viewport }).promise;
      } catch { /* best effort */ }
    })();
    return () => { cancelled = true; };
  }, [getPage, nearViewport, pageNumber, scale]);

  const borderColor = active ? "rgba(15,23,42,0.15)" : "rgba(226,232,240,0.6)";
  const bg = active ? "#fff" : "rgba(255,255,255,0.8)";
  const boxShadow = active ? "0 4px 12px rgba(0,0,0,0.08)" : "0 1px 3px rgba(0,0,0,0.04)";

  return (
    <div
      ref={wrapperRef}
      data-page-number={pageNumber}
      style={{ ...s, border: `1px solid ${borderColor}`, background: bg, boxShadow, minHeight: minHeight ? `${minHeight}px` : undefined }}
    >
      <div style={{ display: "flex", alignItems: "center", justifyContent: "space-between", marginBottom: 16 }}>
        <div style={{ fontSize: 12, fontWeight: 600, textTransform: "uppercase", letterSpacing: "0.2em", color: "#94a3b8" }}>Page {pageNumber}</div>
        <div style={{ fontSize: 12, color: "#94a3b8" }}>{Math.round(scale * 100)}%</div>
      </div>
      <div style={{ display: "flex", justifyContent: "center" }}>
        <canvas ref={canvasRef} style={{ display: "block", maxWidth: "100%", borderRadius: 16 }} />
      </div>
    </div>
  );
};

export default React.memo(PdfPage);
