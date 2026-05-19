import React, { useEffect, useRef } from "react";
import { Grid } from "react-window";
import { ImageOff, Loader, RefreshCw } from "lucide-react";
import type { PhotoAsset } from "../types";
import { startFileDrag } from "../../../os/filesystem/dragDrop";

const COL_WIDTH = 200;
const ROW_HEIGHT = 200;
const GAP = 16;

type GridCellProps = {
  photos: PhotoAsset[];
  onOpen: (id: string) => void;
  loadDataUrl: (id: string) => Promise<string | null>;
  columnCount: number;
};

function Cell(props: { columnIndex: number; rowIndex: number; style: React.CSSProperties } & GridCellProps) {
  const { columnIndex, rowIndex, style, photos, onOpen, loadDataUrl, columnCount } = props;
  const index = rowIndex * columnCount + columnIndex;
  const photo = photos[index];

  // Hooks BEFORE any guard — Rules of Hooks compliance
  const [loaded, setLoaded] = React.useState(photo ? !!photo.dataUrl : false);

  // Reset loaded when the photo changes (fixes stale loaded after sort/filter)
  useEffect(() => {
    setLoaded(false);
  }, [photo?.id]);

  useEffect(() => {
    if (photo && !photo.dataUrl && !loaded) {
      loadDataUrl(photo.id).then((url) => { if (url) setLoaded(true); });
    }
  }, [photo?.id, photo?.dataUrl, loadDataUrl]);

  if (!photo || index >= photos.length) return <div style={style} />;

  return (
    <div style={{ ...style, padding: GAP / 2 }}>
      <button type="button" className="photos__card" draggable
        style={{ width: "100%", height: "100%" }}
        onClick={() => onOpen(photo.id)}
        onDoubleClick={() => onOpen(photo.id)}
        onDragStart={(e) => {
          startFileDrag(e.dataTransfer, [{
            name: photo.name,
            path: photo.path,
            kind: "file",
            size: photo.size || 0
          }]);
        }}
      >
        {photo.dataUrl ? (
          <img src={photo.dataUrl} alt={photo.name} className="photos__thumb" />
        ) : (
          <div className="photos__placeholder"><Loader size={18} className="photos__spin" /></div>
        )}
        <div className="photos__cardMeta"><div className="photos__cardTitle">{photo.name}</div></div>
      </button>
    </div>
  );
}

export function PhotoGrid(props: {
  photos: PhotoAsset[];
  loading: boolean;
  error?: string | null;
  onRetry?: () => void;
  onOpen: (photoId: string) => void;
  loadDataUrl: (id: string) => Promise<string | null>;
}) {
  const containerRef = useRef<HTMLDivElement>(null);
  const [dimensions, setDimensions] = React.useState({ width: 800, height: 600 });
  const rafRef = useRef<number>(0);

  useEffect(() => {
    const el = containerRef.current;
    if (!el) return;
    const ro = new ResizeObserver(() => {
      if (el) {
        cancelAnimationFrame(rafRef.current);
        rafRef.current = requestAnimationFrame(() => {
          const rect = el.getBoundingClientRect();
          setDimensions({ width: Math.max(rect.width, 200), height: Math.max(rect.height, 200) });
        });
      }
    });
    ro.observe(el);
    return () => { ro.disconnect(); cancelAnimationFrame(rafRef.current); };
  }, []);

  if (props.loading) {
    return <div className="photos__center"><Loader size={18} className="photos__spin" /><span>Loading photo library…</span></div>;
  }

  if (props.error) {
    return (
      <div className="photos__center">
        <ImageOff size={18} strokeWidth={2.2} />
        <span>Failed to load: {props.error}</span>
        {props.onRetry ? (
          <button type="button" className="photos__retryBtn" onClick={props.onRetry}>
            <RefreshCw size={14} /> Retry
          </button>
        ) : null}
      </div>
    );
  }

  if (!props.photos.length) {
    return <div className="photos__center"><ImageOff size={18} strokeWidth={2.2} /><span>No images found.</span></div>;
  }

  const columnCount = Math.max(1, Math.floor((dimensions.width + GAP) / (COL_WIDTH + GAP)));
  const rowCount = Math.ceil(props.photos.length / columnCount);

  return (
    <div ref={containerRef} className="photos__grid" style={{ width: "100%", height: "100%" }}>
      <Grid<GridCellProps>
        cellComponent={Cell}
        cellProps={{ photos: props.photos, onOpen: props.onOpen, loadDataUrl: props.loadDataUrl, columnCount }}
        columnCount={columnCount}
        columnWidth={COL_WIDTH + GAP}
        rowCount={rowCount}
        rowHeight={ROW_HEIGHT + GAP}
        style={{ width: "100%", height: "100%" }}
      />
    </div>
  );
}
