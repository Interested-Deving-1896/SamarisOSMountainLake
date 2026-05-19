import React, { useEffect, useRef, useState } from "react";

type PageThumbnailProps = {
  getPage: (n: number) => Promise<any>;
  pageNumber: number;
  active: boolean;
  onClick: (pageNumber: number) => void;
};

const PageThumbnail: React.FC<PageThumbnailProps> = ({ getPage, pageNumber, active, onClick }) => {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const wrapperRef = useRef<HTMLButtonElement>(null);
  const [ready, setReady] = useState(pageNumber <= 4);

  useEffect(() => {
    const node = wrapperRef.current;
    if (!node) return;
    const observer = new IntersectionObserver(
      (entries) => { if (entries[0]?.isIntersecting) setReady(true); },
      { rootMargin: "200px 0px" }
    );
    observer.observe(node);
    return () => observer.disconnect();
  }, []);

  useEffect(() => {
    let cancelled = false;
    if (!ready || !canvasRef.current) return;
    (async () => {
      try {
        const page = await getPage(pageNumber);
        if (cancelled || !canvasRef.current) return;
        const viewport = page.getViewport({ scale: 0.22 });
        const canvas = canvasRef.current;
        const context = canvas.getContext("2d");
        if (!context) return;
        const dpr = Math.min(window.devicePixelRatio || 1, 2);
        canvas.width = Math.floor(viewport.width * dpr);
        canvas.height = Math.floor(viewport.height * dpr);
        canvas.style.width = `${Math.floor(viewport.width)}px`;
        canvas.style.height = `${Math.floor(viewport.height)}px`;
        context.setTransform(dpr, 0, 0, dpr, 0, 0);
        await page.render({ canvasContext: context, viewport }).promise;
      } catch { /* ignore */ }
    })();
    return () => { cancelled = true; };
  }, [getPage, pageNumber, ready]);

  const borderActive = active ? "1px solid rgba(15,23,42,0.15)" : "1px solid rgba(226,232,240,0.6)";
  const bgActive = active ? "rgba(15,23,42,0.05)" : "rgba(255,255,255,0.75)";

  return (
    <button
      ref={wrapperRef}
      type="button"
      onClick={() => onClick(pageNumber)}
      style={{
        width: "100%",
        borderRadius: 16,
        border: borderActive,
        padding: 8,
        textAlign: "left",
        cursor: "pointer",
        background: bgActive,
        transition: "background 0.15s",
        marginBottom: 8,
      }}
    >
      <div style={{ fontSize: 11, fontWeight: 600, color: "#374151", marginBottom: 4 }}>Page {pageNumber}</div>
      <div style={{ overflow: "hidden", borderRadius: 12, border: "1px solid rgba(226,232,240,0.6)", background: "#fff" }}>
        <canvas ref={canvasRef} style={{ display: "block", width: "100%", height: "auto" }} />
      </div>
    </button>
  );
};

export default React.memo(PageThumbnail);
