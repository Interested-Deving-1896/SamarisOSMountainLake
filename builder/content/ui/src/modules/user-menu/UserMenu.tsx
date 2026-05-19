import React from "react";
import { Cpu, HardDrive, Monitor, Database, Laptop, Settings, Lock, Power, RotateCcw, Moon } from "lucide-react";
import { appLoader } from "../../os/apps/appLoader";
import { osStore } from "../../os/core/osStore";
import { kernelClient } from "../../os/kernel/kernelClient";
import { connectivityStore } from "../../system/connectivity/connectivityStore";
import { securityStore } from "../../system/session/securityStore";
import { useAirBar } from "../airbar/useAirBar";
import "./userMenu.css";

function fmt(b: number) {
  if (!b || b <= 0) return "0 B";
  const u = ["B", "KB", "MB", "GB", "TB"]; let i = 0, v = b;
  while (v >= 1024 && i < u.length - 1) { v /= 1024; i++; }
  return v.toFixed(i > 0 ? 1 : 0) + " " + u[i];
}

function gl() {
  const c = document.createElement("canvas");
  const g = c.getContext("webgl") || (c.getContext("experimental-webgl") as WebGLRenderingContext | null);
  if (!g) return null;
  const di = g.getExtension("WEBGL_debug_renderer_info");
  return di ? (g.getParameter(di.UNMASKED_RENDERER_WEBGL) as string).replace(/ .*$/, "") : null;
}

export function UserMenu() {
  const air = useAirBar();
  const open = air.activePanel === "samaris";
  const style = air.getPanelStyle("samaris", { width: 400, align: "start" });
  const sec = React.useSyncExternalStore((l) => securityStore.subscribe(l), () => securityStore.getState());
  const conn = React.useSyncExternalStore((l) => connectivityStore.subscribe(l), () => connectivityStore.getState());
  const rootRef = React.useRef<HTMLElement | null>(null);
  const [gpu, setGpu] = React.useState<string | null>(null);
  const [st, setSt] = React.useState({ pct: 0, used: 0, total: 1 });

  React.useEffect(() => { setGpu(gl()); }, []);

  React.useEffect(() => {
    if (!open || !rootRef.current) return;
    const firstBtn = rootRef.current.querySelector<HTMLButtonElement>(".air-user-action");
    firstBtn?.focus();
  }, [open]);

  React.useEffect(() => {
    if (!open) return;
    let dead = false;
    const ref = () => navigator.storage?.estimate?.().then((e) => { if (!dead) { const u = e?.usage || 0, t = Math.max(e?.quota || 1, 1); setSt({ pct: Math.round((u / t) * 100), used: u, total: t }); } }).catch(() => {});
    ref();
    return () => { dead = true; };
  }, [open]);

  const online = !!(conn.currentNetworkId || conn.currentNetworkLabel);
  const nav = navigator as Navigator & { deviceMemory?: number; userAgentData?: { platform: string } };
  const dm = nav.deviceMemory;
  const connected = conn.devices.filter((d) => d.connected);

  return (
    <section ref={rootRef} style={style} className={`airbar-panel samaris-user-menu${open ? " open" : ""}`} role="dialog" aria-label="User menu">
      <div style={{ padding: 20, display: "grid", gap: 14, color: "var(--air-text, inherit)", fontSize: 14 }}>

        {/* Profile */}
        <div style={{ display: "flex", alignItems: "center", gap: 14 }}>
          <div style={{ width: 52, height: 52, borderRadius: "50%", display: "grid", placeItems: "center", fontSize: 22, fontWeight: 700, color: "#fff", background: "linear-gradient(135deg, #14b8a6, #2563eb)", flexShrink: 0, boxShadow: "0 4px 16px rgba(37,99,235,0.25)" }}>
            {sec.displayName?.charAt(0).toUpperCase() || "U"}
          </div>
          <div style={{ flex: 1, minWidth: 0 }}>
            <div style={{ fontSize: 17, fontWeight: 750, color: "var(--air-text, inherit)" }}>{sec.displayName || "User"}</div>
            <div style={{ fontSize: 13, color: "var(--air-text-soft, #666)", marginTop: 2 }}>@{sec.username || "user"}</div>
          </div>
          <div style={{ display: "flex", alignItems: "center", gap: 5, fontSize: 11, fontWeight: 700, color: "var(--air-text-soft, #888)", flexShrink: 0 }}>
            <span style={{ width: 7, height: 7, borderRadius: "50%", background: online ? "#22c55e" : "var(--air-text-faint, #aaa)", boxShadow: online ? "0 0 8px rgba(34,197,94,0.4)" : "none" }} />
            {online ? "Online" : "Offline"}
          </div>
        </div>

        {/* Storage */}
        <div style={{ padding: 14, borderRadius: 14, background: "var(--panel, rgba(255,255,255,0.32))" }}>
          <div style={{ display: "flex", alignItems: "center", gap: 6, fontSize: 11, fontWeight: 700, color: "var(--air-text-faint, #888)", marginBottom: 10, textTransform: "uppercase", letterSpacing: "0.04em" }}>
            <HardDrive size={14} /> Storage <span style={{ marginLeft: "auto", fontWeight: 600 }}>{st.pct}% used</span>
          </div>
          <div style={{ height: 6, borderRadius: 3, background: "var(--air-glass-border-soft, rgba(0,0,0,0.06))", overflow: "hidden" }}>
            <div style={{ height: "100%", borderRadius: 3, background: "linear-gradient(90deg, #14b8a6, #2563eb)", width: `${Math.min(st.pct, 100)}%`, transition: "width 400ms ease" }} />
          </div>
          <div style={{ display: "flex", justifyContent: "space-between", fontSize: 11, color: "var(--air-text-faint, #999)", marginTop: 6 }}>
            <span>{fmt(st.used)} used</span>
            <span>{fmt(st.total - st.used)} free</span>
          </div>
        </div>

        {/* System */}
        <div style={{ padding: 14, borderRadius: 14, background: "var(--panel, rgba(255,255,255,0.32))" }}>
          <div style={{ fontSize: 11, fontWeight: 700, color: "var(--air-text-faint, #888)", marginBottom: 10, textTransform: "uppercase", letterSpacing: "0.04em", display: "flex", alignItems: "center", gap: 6 }}>
            <Cpu size={14} /> System
          </div>
          <div style={{ display: "grid", gridTemplateColumns: "1fr 1fr", gap: 6 }}>
            {[
              { icon: Cpu, label: "CPU", value: `${navigator.hardwareConcurrency || 1} cores` },
              { icon: HardDrive, label: "RAM", value: dm ? `${dm} GB` : "—" },
              { icon: Monitor, label: "GPU", value: gpu || "—" },
              { icon: Monitor, label: "Display", value: `${window.screen?.width || 0}×${window.screen?.height || 0}` },
            ].map(({ icon: Icon, label, value }) => (
              <div key={label} style={{ display: "flex", alignItems: "center", gap: 8, padding: "7px 9px", borderRadius: 9, background: "rgba(255,255,255,0.15)" }}>
                <Icon size={14} style={{ color: "var(--accent, #2563eb)", flexShrink: 0 }} />
                <div style={{ minWidth: 0 }}>
                  <div style={{ fontSize: 9, fontWeight: 700, color: "var(--air-text-faint, #888)", textTransform: "uppercase", letterSpacing: "0.04em" }}>{label}</div>
                  <div style={{ fontSize: 11, fontWeight: 700, color: "var(--air-text, inherit)", marginTop: 1 }}>{value}</div>
                </div>
              </div>
            ))}
          </div>
        </div>

        {/* Devices */}
        <div style={{ padding: "10px 14px 8px", borderRadius: 14, background: "var(--panel, rgba(255,255,255,0.32))" }}>
          <div style={{ fontSize: 11, fontWeight: 700, color: "var(--air-text-faint, #888)", marginBottom: 8, textTransform: "uppercase", letterSpacing: "0.04em", display: "flex", alignItems: "center", gap: 6 }}>
            <Database size={14} /> Devices
          </div>
          {[
            { icon: Laptop, label: nav.userAgentData?.platform || navigator.platform || "Device", value: conn.currentNetworkLabel || (conn.wifiEnabled ? "Online" : "—") },
            ...connected.map((d) => ({ icon: Database, label: d.label, value: "Connected" })),
            ...(connected.length === 0 ? [{ icon: Database, label: "No external devices", value: `${osStore.getState().devices.length || 1}` }] : []),
          ].slice(0, 3).map(({ icon: Icon, label, value }, i) => (
            <div key={i} style={{ display: "flex", alignItems: "center", gap: 8, padding: "4px 0", fontSize: 12, color: "var(--air-text-soft, #666)" }}>
              <Icon size={13} style={{ color: "var(--accent, #2563eb)", flexShrink: 0 }} />
              <span style={{ flex: 1, overflow: "hidden", textOverflow: "ellipsis", whiteSpace: "nowrap" }}>{label}</span>
              <span style={{ fontSize: 10, fontWeight: 700, color: "var(--air-text-faint, #999)", flexShrink: 0 }}>{value}</span>
            </div>
          ))}
        </div>

        {/* Actions */}
        <div style={{ display: "grid", gridTemplateColumns: "repeat(5, 1fr)", gap: 6, marginTop: 4 }}>
          {[
            { icon: Settings, label: "Settings", action: () => { air.closePanels(); void appLoader.openApp("settings"); } },
            { icon: Lock, label: "Lock", action: () => { air.closePanels(); void securityStore.lock(); } },
            { icon: Moon, label: "Sleep", action: () => { air.closePanels(); void kernelClient.request({ type: "power.sleep", data: {} }); } },
            { icon: RotateCcw, label: "Restart", action: () => { air.closePanels(); void kernelClient.request({ type: "power.restart", data: {} }); } },
            { icon: Power, label: "Shut Down", danger: true, action: () => { air.closePanels(); void kernelClient.request({ type: "power.shutdown", data: {} }); } },
          ].map(({ icon: Icon, label, action, danger }) => (
            <button key={label} onClick={action} aria-label={label}
              className={`air-user-action${danger ? " air-user-action--danger" : ""}`}
            >
              <Icon size={17} />
              <span>{label}</span>
            </button>
          ))}
        </div>

        {/* Version */}
        <div style={{ textAlign: "center", fontSize: 10, fontWeight: 600, color: "var(--air-text-faint, #aaa)", paddingTop: 10, borderTop: "1px solid var(--air-glass-border-soft, rgba(0,0,0,0.05))", letterSpacing: "0.02em" }}>
          Samaris OS 1.0 Mountain Lake
        </div>
      </div>
    </section>
  );
}
