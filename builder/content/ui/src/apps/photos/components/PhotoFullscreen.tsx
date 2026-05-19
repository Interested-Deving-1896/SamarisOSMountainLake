import React, { useEffect, useRef } from "react";
import { ChevronLeft, ChevronRight, X, ZoomIn, ZoomOut } from "lucide-react";
import { useFs } from "../../../services/fs/useFs";
import type { PhotoAsset } from "../types";

export function PhotoFullscreen(props: {
  photo: PhotoAsset | null;
  index: number;
  total: number;
  onPrev: () => void;
  onNext: () => void;
  onClose: () => void;
}) {
  const fs = useFs();
  const [zoom, setZoom] = React.useState(1);
  const [loaded, setLoaded] = React.useState(false);
  const [fullDataUrl, setFullDataUrl] = React.useState<string | null>(null);
  const isDragging = useRef(false);
  const dragStart = useRef({ x: 0, y: 0 });
  const pan = useRef({ x: 0, y: 0 });
  const [panOffset, setPanOffset] = React.useState({ x: 0, y: 0 });

  useEffect(() => {
    setZoom(1);
    pan.current = { x: 0, y: 0 };
    setPanOffset({ x: 0, y: 0 });

    if (!props.photo) return;

    setLoaded(false);
    setFullDataUrl(null);

    let cancelled = false;
    fs.readDataUrl(props.photo.path)
      .then(({ dataUrl }) => {
        if (!cancelled) {
          setFullDataUrl(dataUrl);
          setLoaded(true);
        }
      })
      .catch(() => {
        if (!cancelled) setLoaded(true);
      });

    return () => { cancelled = true; };
  }, [props.photo?.id, props.photo?.path]);

  if (!props.photo) return null;

  const imageStyle: React.CSSProperties = zoom > 1
    ? { transform: `scale(${zoom}) translate(${panOffset.x / zoom}px, ${panOffset.y / zoom}px)`, cursor: "grab", transition: "none" }
    : {};

  const handleWheel = (e: React.WheelEvent) => {
    setZoom((z) => Math.max(0.25, Math.min(z - e.deltaY * 0.003, 8)));
  };

  const handleMouseDown = (e: React.MouseEvent) => {
    if (zoom <= 1) return;
    isDragging.current = true;
    dragStart.current = { x: e.clientX - panOffset.x, y: e.clientY - panOffset.y };

    const handleMove = (ev: MouseEvent) => {
      pan.current = { x: ev.clientX - dragStart.current.x, y: ev.clientY - dragStart.current.y };
      setPanOffset({ x: pan.current.x, y: pan.current.y });
    };
    const handleUp = () => {
      isDragging.current = false;
      window.removeEventListener("mousemove", handleMove);
      window.removeEventListener("mouseup", handleUp);
    };
    window.addEventListener("mousemove", handleMove);
    window.addEventListener("mouseup", handleUp);
  };

  const imageSrc = fullDataUrl || props.photo.dataUrl;

  return (
    <div className="photos__fullscreen" onWheel={handleWheel}>
      <div className="photos__fullscreenTop">
        <button type="button" className="photos__fsClose" onClick={props.onClose} title="Close (Esc)">
          <X size={22} />
        </button>
        <span className="photos__fsCounter">{props.index + 1} / {props.total}</span>
      </div>

      <button type="button" className="photos__fsNav photos__fsPrev" onClick={props.onPrev} disabled={props.index <= 0} title="Previous (←)">
        <ChevronLeft size={22} />
      </button>
      <button type="button" className="photos__fsNav photos__fsNext" onClick={props.onNext} disabled={props.index >= props.total - 1} title="Next (→)">
        <ChevronRight size={22} />
      </button>

      {imageSrc && (
        <img
          src={imageSrc}
          alt={props.photo.name}
          className="photos__fullscreenImage"
          style={imageStyle}
          draggable={false}
          onMouseDown={handleMouseDown}
        />
      )}

      {!loaded && (
        <div className="photos__center" style={{ position: "absolute", inset: 0, display: "flex", justifyContent: "center", alignItems: "center", pointerEvents: "none" }}>
          <span style={{ color: "rgba(255,255,255,0.7)", fontSize: 14 }}>Loading full resolution…</span>
        </div>
      )}

      <div className="photos__fsBottom">
        <div className="photos__fsZoom">
          <button type="button" className="photos__fsBtn" onClick={() => setZoom((z) => Math.max(0.25, z / 1.5))} title="Zoom out">
            <ZoomOut size={16} />
          </button>
          <span>{Math.round(zoom * 100)}%</span>
          <button type="button" className="photos__fsBtn" onClick={() => setZoom((z) => Math.min(8, z * 1.5))} title="Zoom in">
            <ZoomIn size={16} />
          </button>
        </div>
      </div>
    </div>
  );
}
