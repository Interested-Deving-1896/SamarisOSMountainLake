import React from "react";

export function ConnectModal(props: {
  ssid: string;
  secured: boolean;
  busy: boolean;
  error: string | null;
  onPassword: (password: string) => void;
  onBack: () => void;
}) {
  const [pw, setPw] = React.useState("");

  return (
    <div style={{ padding: 24 }}>
      <div style={{ fontSize: 20, fontWeight: 700, color: "#0f172a", marginBottom: 8 }}>Connect to {props.ssid}</div>
      <div style={{ fontSize: 13, color: "#64748b", marginBottom: 20 }}>
        {props.secured ? "This network requires a password." : "Open network — no password needed."}
      </div>

      {props.secured && (
        <input
          type="password"
          value={pw}
          onChange={(e) => setPw(e.target.value)}
          placeholder="Wi‑Fi password"
          autoFocus
          style={{
            width: "100%",
            height: 44,
            borderRadius: 12,
            border: "1px solid rgba(15,23,42,0.1)",
            background: "rgba(255,255,255,0.8)",
            padding: "0 14px",
            fontSize: 15,
            outline: "none",
            marginBottom: 16,
            boxSizing: "border-box",
          }}
          onKeyDown={(e) => { if (e.key === "Enter" && !props.busy && (!props.secured || pw.trim())) props.onPassword(pw.trim()); }}
        />
      )}

      {props.error && (
        <div style={{ fontSize: 13, color: "#dc2626", marginBottom: 12 }}>{props.error}</div>
      )}

      <div style={{ display: "flex", gap: 10 }}>
        <button type="button" onClick={props.onBack} style={{
          flex: 1, height: 42, borderRadius: 12, border: "1px solid rgba(15,23,42,0.1)", background: "rgba(255,255,255,0.8)", color: "#475569", fontSize: 14, fontWeight: 600, cursor: "pointer",
        }}>Back</button>
        <button type="button" disabled={props.busy || (props.secured && !pw.trim())} onClick={() => props.onPassword(pw.trim())} style={{
          flex: 1, height: 42, borderRadius: 12, border: 0, background: "#2f6df6", color: "#fff", fontSize: 14, fontWeight: 600, cursor: "pointer", opacity: props.busy || (props.secured && !pw.trim()) ? 0.5 : 1,
        }}>
          {props.busy ? "Connecting…" : "Connect"}
        </button>
      </div>
    </div>
  );
}
