import React from "react";
import { Check, Globe, LoaderCircle, LockKeyhole, RefreshCw, SignalHigh, Trash2, Wifi, WifiOff, Signal, SignalMedium, SignalZero } from "lucide-react";
import { connectivityStore, type Network } from "../../../system/connectivity/connectivityStore";

export function WifiTab() {
  const c = React.useSyncExternalStore((l) => connectivityStore.subscribe(l), () => connectivityStore.getState());
  const [targetId, setTargetId] = React.useState<string | null>(null);
  const [password, setPassword] = React.useState("");
  const [busyId, setBusyId] = React.useState<string | null>(null);
  const [error, setError] = React.useState<string | null>(null);
  const [refreshing, setRefreshing] = React.useState(false);
  const [savedNetworks, setSavedNetworks] = React.useState<{ ssid: string }[]>([]);
  const [showSaved, setShowSaved] = React.useState(false);

  const selected = c.networks.find((n) => n.label === targetId) || null;
  const currentNetwork = c.networks.find((n) => n.connected) || null;

  React.useEffect(() => {
    connectivityStore.init();
    connectivityStore.refresh().finally(() => setRefreshing(false));
    connectivityStore.getSavedNetworks().then(setSavedNetworks);
  }, []);

  async function connectNetwork(label: string, pw?: string) {
    setBusyId(label); setError(null);
    const ok = await connectivityStore.connectNetwork(label, pw);
    if (!ok) setError("Connection failed. Check the password and try again.");
    setBusyId(null);
  }

  async function forgetNetwork(ssid: string) {
    await connectivityStore.forgetNetwork(ssid);
    const saved = await connectivityStore.getSavedNetworks();
    setSavedNetworks(saved);
  }

  function SignalBars({ strength }: { strength: number }) {
    const bars = strength > 75 ? 4 : strength > 50 ? 3 : strength > 25 ? 2 : 1;
    return (
      <div style={{ display: "flex", alignItems: "flex-end", gap: 2, height: 16 }}>
        {[1, 2, 3, 4].map((i) => (
          <div key={i} style={{ width: 4, height: `${4 + i * 3}px`, borderRadius: 2, background: i <= bars ? "#2f6df6" : "rgba(15,23,42,0.1)", transition: "background 200ms" }} />
        ))}
      </div>
    );
  }

  return (
    <div style={{ padding: 24, display: "grid", gap: 20, alignContent: "start" }}>
      {/* WiFi Toggle */}
      <div style={{ display: "flex", alignItems: "center", justifyContent: "space-between", padding: "14px 18px", borderRadius: 16, background: "rgba(255,255,255,0.8)", border: "1px solid rgba(15,23,42,0.06)" }}>
        <div style={{ display: "flex", alignItems: "center", gap: 12 }}>
          {c.wifiEnabled ? <Wifi size={20} color="#2f6df6" /> : <WifiOff size={20} color="#94a3b8" />}
          <div>
            <div style={{ fontSize: 14, fontWeight: 600, color: "#0f172a" }}>{c.wifiEnabled ? "Wi‑Fi On" : "Wi‑Fi Off"}</div>
            <div style={{ fontSize: 12, color: "#64748b" }}>{c.currentNetworkLabel || (c.wifiEnabled ? "Scanning…" : "Turn on to scan")}</div>
          </div>
        </div>
        <button type="button" onClick={() => connectivityStore.toggleWifi()} disabled={!c.capabilities.wifiToggle} style={{
          width: 48, height: 26, borderRadius: 13, border: 0, padding: 0, cursor: "pointer", position: "relative", background: c.wifiEnabled ? "#2f6df6" : "rgba(15,23,42,0.1)", transition: "background 200ms",
        }}>
          <span style={{ position: "absolute", top: 2, left: c.wifiEnabled ? 24 : 2, width: 22, height: 22, borderRadius: "50%", background: "#fff", boxShadow: "0 1px 3px rgba(0,0,0,0.15)", transition: "left 200ms" }} />
        </button>
      </div>

      {/* Connected Network */}
      {currentNetwork && (
        <div style={{ padding: 18, borderRadius: 16, background: "rgba(37,99,235,0.06)", border: "1px solid rgba(37,99,235,0.1)" }}>
          <div style={{ display: "flex", alignItems: "center", justifyContent: "space-between" }}>
            <div style={{ display: "flex", alignItems: "center", gap: 12 }}>
              <SignalBars strength={currentNetwork.strength} />
              <div>
                <div style={{ fontSize: 14, fontWeight: 600, color: "#0f172a" }}>{currentNetwork.label}</div>
                <div style={{ fontSize: 12, color: "#64748b" }}>
                  {currentNetwork.secured ? "Secured" : "Open"}
                  {currentNetwork.band ? ` • ${currentNetwork.band}` : ""}
                  {c.currentAddress ? ` • ${c.currentAddress}` : ""}
                </div>
              </div>
            </div>
            <span style={{ display: "inline-flex", alignItems: "center", gap: 4, padding: "4px 10px", borderRadius: 20, background: "rgba(34,197,94,0.1)", color: "#16a34a", fontSize: 12, fontWeight: 600 }}>
              <Check size={12} /> Connected
            </span>
          </div>
          <div style={{ display: "flex", gap: 8, marginTop: 12 }}>
            <button type="button" disabled={busyId === "disconnect"} onClick={async () => { setBusyId("disconnect"); await connectivityStore.disconnectNetwork(); setBusyId(null); }} style={{
              height: 36, padding: "0 16px", borderRadius: 10, border: "1px solid rgba(15,23,42,0.1)", background: "rgba(255,255,255,0.8)", color: "#475569", fontSize: 13, fontWeight: 600, cursor: "pointer",
            }}>
              {busyId === "disconnect" ? "Disconnecting…" : "Disconnect"}
            </button>
            <button type="button" onClick={() => forgetNetwork(currentNetwork.label)} style={{
              height: 36, padding: "0 16px", borderRadius: 10, border: "1px solid rgba(239,68,68,0.15)", background: "rgba(239,68,68,0.06)", color: "#dc2626", fontSize: 13, fontWeight: 600, cursor: "pointer",
            }}>
              <Trash2 size={14} style={{ marginRight: 6, verticalAlign: "middle" }} /> Forget
            </button>
          </div>
        </div>
      )}

      {/* Network List / Connect Modal */}
      {selected ? (
        <div style={{ padding: 24, borderRadius: 16, background: "rgba(255,255,255,0.8)", border: "1px solid rgba(15,23,42,0.06)" }}>
          <div style={{ fontSize: 18, fontWeight: 700, color: "#0f172a", marginBottom: 6 }}>Connect to {selected.label}</div>
          <div style={{ fontSize: 13, color: "#64748b", marginBottom: 18 }}>
            {selected.secured ? "This network is password-protected." : "Open network — connect directly."}
          </div>
          {selected.secured && (
            <input type="password" value={password} onChange={(e) => setPassword(e.target.value)} placeholder="Wi‑Fi password" autoFocus
              style={{ width: "100%", height: 44, borderRadius: 12, border: "1px solid rgba(15,23,42,0.1)", background: "rgba(255,255,255,0.8)", padding: "0 14px", fontSize: 15, outline: "none", marginBottom: 14, boxSizing: "border-box" }}
              onKeyDown={(e) => { if (e.key === "Enter" && !busyId && (!selected.secured || password.trim())) connectNetwork(selected.label, password.trim()); }} />
          )}
          {error && <div style={{ fontSize: 13, color: "#dc2626", marginBottom: 12 }}>{error}</div>}
          <div style={{ display: "flex", gap: 10 }}>
            <button type="button" onClick={() => { setTargetId(null); setPassword(""); setError(null); }} style={{
              flex: 1, height: 42, borderRadius: 12, border: "1px solid rgba(15,23,42,0.1)", background: "rgba(255,255,255,0.8)", color: "#475569", fontSize: 14, fontWeight: 600, cursor: "pointer",
            }}>Back</button>
            <button type="button" disabled={!!busyId || (selected.secured && !password.trim())} onClick={() => connectNetwork(selected.label, password.trim())} style={{
              flex: 1, height: 42, borderRadius: 12, border: 0, background: "#2f6df6", color: "#fff", fontSize: 14, fontWeight: 600, cursor: "pointer", opacity: busyId || (selected.secured && !password.trim()) ? 0.5 : 1,
            }}>
              {busyId === selected.label ? "Connecting…" : "Connect"}
            </button>
          </div>
        </div>
      ) : (
        <div style={{ padding: 18, borderRadius: 16, background: "rgba(255,255,255,0.8)", border: "1px solid rgba(15,23,42,0.06)" }}>
          <div style={{ display: "flex", alignItems: "center", justifyContent: "space-between", marginBottom: 12 }}>
            <div style={{ fontSize: 14, fontWeight: 700, color: "#0f172a" }}>Networks</div>
            <button type="button" onClick={async () => { setRefreshing(true); await connectivityStore.refresh(); setRefreshing(false); }} disabled={refreshing} style={{ background: "none", border: "none", cursor: "pointer", color: "#64748b", padding: 4 }}>
              <RefreshCw size={16} style={{ animation: refreshing ? "spin 1s linear infinite" : "none" }} />
            </button>
          </div>

          {!c.wifiEnabled && <div style={{ fontSize: 13, color: "#94a3b8", padding: "12px 0" }}>Turn Wi‑Fi on to scan for networks.</div>}
          {c.wifiEnabled && refreshing && c.networks.length === 0 && <div style={{ fontSize: 13, color: "#94a3b8", padding: "12px 0" }}>Scanning…</div>}
          {c.wifiEnabled && !refreshing && c.networks.length === 0 && <div style={{ fontSize: 13, color: "#94a3b8", padding: "12px 0" }}>No networks found.</div>}

          {c.networks.slice().sort((a, b) => b.strength - a.strength).map((net) => (
            <button key={net.id} type="button" onClick={() => {
              if (net.connected) return;
              if (net.secured) { setTargetId(net.label); setPassword(""); setError(null); return; }
              connectNetwork(net.label);
            }} style={{
              display: "flex", alignItems: "center", gap: 12, width: "100%", padding: "10px 8px", borderRadius: 12, border: "none", background: "transparent", cursor: "pointer", textAlign: "left", color: "#0f172a", transition: "background 150ms",
            }}
              onMouseEnter={(e) => e.currentTarget.style.background = "rgba(15,23,42,0.04)"}
              onMouseLeave={(e) => e.currentTarget.style.background = "transparent"}
            >
              <div style={{ flexShrink: 0, width: 28, height: 28, display: "flex", alignItems: "center", justifyContent: "center", borderRadius: 8, background: "rgba(47,109,246,0.1)", color: "#2f6df6" }}>
                {net.secured ? <LockKeyhole size={14} /> : <Globe size={14} />}
              </div>
              <div style={{ flex: 1, minWidth: 0 }}>
                <div style={{ fontSize: 13, fontWeight: 600, color: "#0f172a", overflow: "hidden", textOverflow: "ellipsis", whiteSpace: "nowrap" }}>{net.label}</div>
                <div style={{ fontSize: 11, color: "#94a3b8" }}>
                  {net.secured ? "Secured" : "Open"}
                  {net.band ? ` • ${net.band}` : ""}
                </div>
              </div>
              {busyId === net.label ? <LoaderCircle size={16} style={{ animation: "spin 1s linear infinite" }} /> : net.connected ? <Check size={16} color="#16a34a" /> : <SignalBars strength={net.strength} />}
            </button>
          ))}
        </div>
      )}

      {/* Saved Networks */}
      {savedNetworks.length > 0 && (
        <div style={{ padding: 18, borderRadius: 16, background: "rgba(255,255,255,0.8)", border: "1px solid rgba(15,23,42,0.06)" }}>
          <button type="button" onClick={() => setShowSaved(!showSaved)} style={{ display: "flex", alignItems: "center", justifyContent: "space-between", width: "100%", background: "none", border: "none", cursor: "pointer", padding: 0 }}>
            <div style={{ fontSize: 14, fontWeight: 700, color: "#0f172a" }}>Saved Networks ({savedNetworks.length})</div>
            <div style={{ fontSize: 12, color: "#64748b", transform: showSaved ? "rotate(180deg)" : "none", transition: "transform 200ms" }}>▼</div>
          </button>
          {showSaved && (
            <div style={{ marginTop: 12, display: "grid", gap: 6 }}>
              {savedNetworks.map((s) => (
                <div key={s.ssid} style={{ display: "flex", alignItems: "center", justifyContent: "space-between", padding: "8px 4px" }}>
                  <div style={{ fontSize: 13, color: "#475569" }}>{s.ssid}</div>
                  <button type="button" onClick={() => forgetNetwork(s.ssid)} style={{ background: "none", border: "none", cursor: "pointer", color: "#94a3b8", padding: 4 }}>
                    <Trash2 size={14} />
                  </button>
                </div>
              ))}
            </div>
          )}
        </div>
      )}
    </div>
  );
}
