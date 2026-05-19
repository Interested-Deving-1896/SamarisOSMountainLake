import React from "react";
import { ZoomIn, ZoomOut, FileText } from "lucide-react";

type ViewerHeaderProps = {
  title: string;
  subtitle: string;
  onZoomOut: () => void;
  onZoomIn: () => void;
};

const ViewerHeader: React.FC<ViewerHeaderProps> = ({ title, subtitle, onZoomOut, onZoomIn }) => {
  return (
    <div style={{ display: "flex", alignItems: "center", justifyContent: "space-between", gap: 16, borderBottom: "1px solid rgba(226,232,240,0.6)", background: "rgba(255,255,255,0.7)", padding: "16px 20px", backdropFilter: "blur(20px)" }}>
      <div style={{ display: "flex", alignItems: "center", gap: 12, minWidth: 0 }}>
        <div style={{ display: "flex", width: 44, height: 44, alignItems: "center", justifyContent: "center", borderRadius: 16, background: "linear-gradient(135deg,#0f172a,#334155)", boxShadow: "0 1px 3px rgba(0,0,0,0.12)" }}>
          <FileText size={20} color="#fff" />
        </div>
        <div style={{ minWidth: 0 }}>
          <div style={{ overflow: "hidden", textOverflow: "ellipsis", whiteSpace: "nowrap", fontSize: 16, fontWeight: 600, color: "#111827" }}>{title}</div>
          <div style={{ overflow: "hidden", textOverflow: "ellipsis", whiteSpace: "nowrap", fontSize: 12, color: "#6b7280" }}>{subtitle}</div>
        </div>
      </div>

      <div style={{ display: "flex", alignItems: "center", gap: 8 }}>
        <button onClick={onZoomOut} style={{ borderRadius: 12, border: "1px solid rgba(226,232,240,0.7)", background: "rgba(255,255,255,0.8)", padding: 8, cursor: "pointer", display: "flex", alignItems: "center", justifyContent: "center" }} title="Zoom out">
          <ZoomOut size={16} color="#374151" />
        </button>
        <button onClick={onZoomIn} style={{ borderRadius: 12, border: "1px solid rgba(226,232,240,0.7)", background: "rgba(255,255,255,0.8)", padding: 8, cursor: "pointer", display: "flex", alignItems: "center", justifyContent: "center" }} title="Zoom in">
          <ZoomIn size={16} color="#374151" />
        </button>
      </div>
    </div>
  );
};

export default React.memo(ViewerHeader);
