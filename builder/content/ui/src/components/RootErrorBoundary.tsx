import React from "react";
import { systemSounds } from "../system/sounds/systemSounds";

type Props = { children: React.ReactNode };
type State = { hasError: boolean; message?: string; stack?: string };

export class RootErrorBoundary extends React.Component<Props, State> {
  state: State = { hasError: false };

  static getDerivedStateFromError(error: unknown) {
    return {
      hasError: true,
      message: error instanceof Error ? error.message : String(error),
      stack: error instanceof Error ? error.stack : undefined,
    };
  }

  componentDidCatch(error: unknown) {
    console.error("[SAMARIS] Root UI error", error);
    systemSounds.play("error");
  }

  handleDismiss = () => {
    this.setState({ hasError: false, message: undefined, stack: undefined });
  };

  render() {
    if (this.state.hasError) {
      const report = [this.state.message, this.state.stack].filter(Boolean).join("\n\n");
      return (
        <div style={{ width: "100vw", height: "100vh", display: "grid", placeItems: "center", background: "#edf2f8", color: "#0f172a", fontFamily: "ui-sans-serif, system-ui, sans-serif" }}>
          <div style={{ width: 560, maxWidth: "min(92vw, 560px)", borderRadius: 24, background: "rgba(255,255,255,0.9)", border: "1px solid rgba(44,72,112,0.12)", boxShadow: "0 28px 64px rgba(40, 65, 106, 0.18)", padding: 24, textAlign: "left" }}>
            <div style={{ fontSize: 24, fontWeight: 800, marginBottom: 10 }}>Something went wrong</div>
            <div style={{ fontSize: 13, color: "#64748b", whiteSpace: "pre-wrap", lineHeight: 1.6 }}>
              {this.state.message || "Unknown error"}
            </div>
            {this.state.stack ? <pre style={{ marginTop: 8, fontSize: 11, color: "#94a3b8", whiteSpace: "pre-wrap", maxHeight: 160, overflow: "auto" }}>{this.state.stack}</pre> : null}
            <div style={{ display: "flex", gap: 10, marginTop: 18 }}>
              <button type="button" style={{ height: 40, padding: "0 16px", borderRadius: 14, border: "1px solid rgba(47,109,246,0.14)", background: "#2f6df6", color: "#fff", fontWeight: 700, cursor: "pointer" }} onClick={() => window.location.reload()}>Reload</button>
              <button type="button" style={{ height: 40, padding: "0 16px", borderRadius: 14, border: "1px solid rgba(44,72,112,0.12)", background: "rgba(255,255,255,0.88)", color: "#10233d", fontWeight: 700, cursor: "pointer" }} onClick={() => void navigator.clipboard?.writeText(report || "Unknown error")}>Copy error report</button>
              <button type="button" style={{ height: 40, padding: "0 16px", borderRadius: 14, border: "1px solid rgba(44,72,112,0.12)", background: "transparent", color: "#64748b", fontWeight: 600, cursor: "pointer" }} onClick={this.handleDismiss}>Dismiss & continue</button>
            </div>
          </div>
        </div>
      );
    }
    return this.props.children;
  }
}
