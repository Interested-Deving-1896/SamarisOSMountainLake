import React from "react";
import { Check, Globe, LoaderCircle, LockKeyhole, RefreshCw, Wifi, WifiOff, Trash2 } from "lucide-react";
import { connectivityStore } from "../../system/connectivity/connectivityStore";
import { systemSounds } from "../../system/sounds/systemSounds";
import { useAirBar } from "../airbar/useAirBar";
import { SYSTEM_PANEL_CLASSES } from "./panel.styles";

function SignalBars({ strength }: { strength: number }) {
  const bars = strength > 75 ? 4 : strength > 50 ? 3 : strength > 25 ? 2 : 1;
  return (
    <div className="wifi-bars" style={{ display: "flex", alignItems: "flex-end", gap: 2, height: 16 }}>
      {[1, 2, 3, 4].map((i) => (
        <div key={i} style={{ width: 4, height: `${4 + i * 3}px`, borderRadius: 2, background: i <= bars ? "var(--accent)" : "var(--stroke)", transition: "background 200ms" }} />
      ))}
    </div>
  );
}

export function WifiPanel() {
  const air = useAirBar();
  const open = air.activePanel === "wifi";
  const c = React.useSyncExternalStore((l) => connectivityStore.subscribe(l), () => connectivityStore.getState());
  const [targetId, setTargetId] = React.useState<string | null>(null);
  const [password, setPassword] = React.useState("");
  const [busyId, setBusyId] = React.useState<string | null>(null);
  const [error, setError] = React.useState<string | null>(null);
  const [refreshing, setRefreshing] = React.useState(false);
  const [savedNetworks, setSavedNetworks] = React.useState<{ ssid: string }[]>([]);
  const [showSaved, setShowSaved] = React.useState(false);
  const style = air.getPanelStyle("wifi", { width: 380, align: "center" });
  const selected = c.networks.find((n) => n.label === targetId) || null;
  const currentNetwork = c.networks.find((n) => n.connected) || null;

  // Auto-refresh every 10s when open
  React.useEffect(() => {
    if (!open) { setTargetId(null); setPassword(""); setBusyId(null); setError(null); return; }
    setRefreshing(true);
    connectivityStore.refresh().finally(() => setRefreshing(false));
    connectivityStore.getSavedNetworks().then(setSavedNetworks);
    const interval = setInterval(() => { connectivityStore.refresh(); }, 10000);
    return () => clearInterval(interval);
  }, [open]);

  async function connectNetwork(label: string, pw?: string) {
    setBusyId(label); setError(null);
    const ok = await connectivityStore.connectNetwork(label, pw);
    if (!ok) setError("Connection failed. Check the password and try again.");
    else { systemSounds.play("notification"); air.closePanels(); }
    setBusyId(null);
  }

  async function forgetNetwork(ssid: string) {
    await connectivityStore.forgetNetwork(ssid);
    const saved = await connectivityStore.getSavedNetworks();
    setSavedNetworks(saved);
  }

  return (
    <section style={style} className={`airbar-panel airbar-system-panel ${open ? "open" : ""}`} role="dialog" aria-label="Wi-Fi">
      <div className={SYSTEM_PANEL_CLASSES.panel}>
        <div className={SYSTEM_PANEL_CLASSES.section}>
          <div className={SYSTEM_PANEL_CLASSES.heading}>
            Wi-Fi
            <button type="button" className={SYSTEM_PANEL_CLASSES.button} style={{ marginLeft: "auto" }} onClick={() => connectivityStore.refresh()} disabled={refreshing}>
              {refreshing ? <LoaderCircle size={14} className="spin" /> : <RefreshCw size={14} />}
            </button>
          </div>
          <button type="button" className={SYSTEM_PANEL_CLASSES.row} onClick={() => connectivityStore.toggleWifi()} disabled={!c.capabilities.wifiToggle}>
            <span className={SYSTEM_PANEL_CLASSES.rowIcon}>{c.wifiEnabled ? <Wifi size={18} /> : <WifiOff size={18} />}</span>
            <span className={SYSTEM_PANEL_CLASSES.rowText}>
              <span className={SYSTEM_PANEL_CLASSES.rowLabel}>{c.wifiEnabled ? "Wi‑Fi On" : "Wi‑Fi Off"}</span>
              <span className={SYSTEM_PANEL_CLASSES.rowMeta}>{c.currentNetworkLabel || "Tap to turn on"}</span>
            </span>
            <span className={`${SYSTEM_PANEL_CLASSES.switch} ${c.wifiEnabled ? SYSTEM_PANEL_CLASSES.switchActive : ""}`} />
          </button>
        </div>

        {currentNetwork ? (
          <div className={SYSTEM_PANEL_CLASSES.section}>
            <div className={SYSTEM_PANEL_CLASSES.heading}>Connected</div>
            <div className={SYSTEM_PANEL_CLASSES.row}>
              <span className={SYSTEM_PANEL_CLASSES.rowIcon}><SignalBars strength={currentNetwork.strength} /></span>
              <span className={SYSTEM_PANEL_CLASSES.rowText}>
                <span className={SYSTEM_PANEL_CLASSES.rowLabel}>{currentNetwork.label}</span>
                <span className={SYSTEM_PANEL_CLASSES.rowMeta}>
                  {currentNetwork.secured ? "Secured" : "Open"}
                  {currentNetwork.band ? ` • ${currentNetwork.band}` : ""}
                  {currentNetwork.channel ? ` • Ch ${currentNetwork.channel}` : ""}
                </span>
              </span>
              <span className={SYSTEM_PANEL_CLASSES.statusPill}><Check size={14} /> Connected</span>
            </div>
            <div className={SYSTEM_PANEL_CLASSES.actions}>
              <button type="button" className={SYSTEM_PANEL_CLASSES.button} disabled={busyId === "disconnect"} onClick={async () => { setBusyId("disconnect"); await connectivityStore.disconnectNetwork(); setBusyId(null); }}>
                {busyId === "disconnect" ? <LoaderCircle size={14} className="spin" /> : null} Disconnect
              </button>
              <button type="button" className={SYSTEM_PANEL_CLASSES.button} style={{ color: "#dc2626", borderColor: "rgba(239,68,68,0.2)" }} onClick={() => forgetNetwork(currentNetwork.label)}>
                <Trash2 size={14} /> Forget
              </button>
            </div>
          </div>
        ) : null}

        {selected ? (
          <div className={SYSTEM_PANEL_CLASSES.section}>
            <div className={SYSTEM_PANEL_CLASSES.heading}>Connect to {selected.label}</div>
            {selected.secured && (
              <input className={SYSTEM_PANEL_CLASSES.input} type="password" value={password} onChange={(e) => setPassword(e.target.value)} placeholder="Wi‑Fi password" autoFocus />
            )}
            {error && <div className={SYSTEM_PANEL_CLASSES.helper}>{error}</div>}
            <div className={SYSTEM_PANEL_CLASSES.actions}>
              <button type="button" className={SYSTEM_PANEL_CLASSES.button} onClick={() => setTargetId(null)}>Back</button>
              <button type="button" className={`${SYSTEM_PANEL_CLASSES.button} ${SYSTEM_PANEL_CLASSES.buttonPrimary}`} disabled={!!busyId || (selected.secured && !password.trim())} onClick={() => connectNetwork(selected.label, password.trim())}>
                {busyId === selected.label ? <LoaderCircle size={14} className="spin" /> : null} Connect
              </button>
            </div>
          </div>
        ) : (
          <div className={SYSTEM_PANEL_CLASSES.section}>
            <div className={SYSTEM_PANEL_CLASSES.heading}>Networks</div>
            {refreshing && c.networks.length === 0 ? <div className={SYSTEM_PANEL_CLASSES.helper}>Scanning…</div> : null}
            {!refreshing && c.networks.length === 0 && c.wifiEnabled ? <div className={SYSTEM_PANEL_CLASSES.helper}>No networks found. Try refreshing.</div> : null}
            {!c.wifiEnabled ? <div className={SYSTEM_PANEL_CLASSES.helper}>Turn Wi‑Fi on to scan.</div> : null}
            {error && !selected ? <div className={SYSTEM_PANEL_CLASSES.helper}>{error}</div> : null}
            <div className={SYSTEM_PANEL_CLASSES.networkList}>
              {c.networks.map((net) => (
                <button key={net.id} type="button" className={SYSTEM_PANEL_CLASSES.networkRow} onClick={() => {
                  if (net.connected) return;
                  if (net.secured) { setTargetId(net.label); setPassword(""); setError(null); return; }
                  connectNetwork(net.label);
                }}>
                  <span className={SYSTEM_PANEL_CLASSES.rowIcon}>{net.secured ? <LockKeyhole size={16} /> : <Globe size={16} />}</span>
                  <span className={SYSTEM_PANEL_CLASSES.rowText}>
                    <span className={SYSTEM_PANEL_CLASSES.rowLabel}>{net.label}</span>
                    <span className={SYSTEM_PANEL_CLASSES.rowMeta}>
                      {net.secured ? "Secured" : "Open"}
                      {net.band ? ` • ${net.band}` : ""}
                    </span>
                  </span>
                  {busyId === net.label ? <LoaderCircle size={16} className="spin" /> : net.connected ? <Check size={16} /> : <SignalBars strength={net.strength} />}
                </button>
              ))}
            </div>

            {savedNetworks.length > 0 && (
              <div style={{ marginTop: 12 }}>
                <button type="button" onClick={() => setShowSaved(!showSaved)} style={{ fontSize: 12, fontWeight: 600, color: "#64748b", background: "none", border: "none", cursor: "pointer", padding: "4px 0", display: "flex", alignItems: "center", gap: 4, width: "100%" }}>
                  Saved Networks ({savedNetworks.length}) <span style={{ transform: showSaved ? "rotate(180deg)" : "none", transition: "transform 200ms" }}>▼</span>
                </button>
                {showSaved && (
                  <div style={{ marginTop: 8, display: "grid", gap: 4 }}>
                    {savedNetworks.map((s) => (
                      <div key={s.ssid} style={{ display: "flex", alignItems: "center", justifyContent: "space-between", padding: "6px 4px", fontSize: 12, color: "#475569" }}>
                        <span>{s.ssid}</span>
                        <button type="button" onClick={() => forgetNetwork(s.ssid)} style={{ background: "none", border: "none", cursor: "pointer", color: "#94a3b8", padding: 2 }}>
                          <Trash2 size={12} />
                        </button>
                      </div>
                    ))}
                  </div>
                )}
              </div>
            )}
          </div>
        )}
      </div>
    </section>
  );
}
