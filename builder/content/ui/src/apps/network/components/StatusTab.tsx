import React from "react";
import { connectivityStore } from "../../../system/connectivity/connectivityStore";
import { networkKernel } from "../../../services/kernel/network";

export function StatusTab() {
  const c = React.useSyncExternalStore((l) => connectivityStore.subscribe(l), () => connectivityStore.getState());
  const [interfaces, setInterfaces] = React.useState<{ name: string; address: string; type: string }[]>([]);

  React.useEffect(() => {
    networkKernel.list().then((list) => {
      setInterfaces(list.map((iface) => ({ name: iface.name, address: iface.address, type: iface.type })));
    }).catch(() => {});
  }, []);

  const currentNetwork = c.networks.find((n) => n.connected) || null;

  const rows: { label: string; value: string }[] = [];

  if (c.wifiEnabled) {
    rows.push({ label: "Wi‑Fi", value: c.wifiEnabled ? "Enabled" : "Disabled" });
  }
  if (currentNetwork) {
    rows.push({ label: "Connected to", value: currentNetwork.label });
    rows.push({ label: "Signal", value: `${currentNetwork.strength}%` });
    rows.push({ label: "Band", value: currentNetwork.band || "—" });
    rows.push({ label: "Channel", value: currentNetwork.channel || "—" });
    rows.push({ label: "Security", value: currentNetwork.secured ? "Secured" : "Open" });
  }
  if (c.currentAddress) {
    rows.push({ label: "IP Address", value: c.currentAddress });
  }
  for (const iface of interfaces) {
    if (iface.address) {
      rows.push({ label: `Interface (${iface.name})`, value: iface.address });
    }
  }

  return (
    <div style={{ padding: 24, display: "grid", gap: 16, alignContent: "start" }}>
      <div style={{ fontSize: 18, fontWeight: 700, color: "#0f172a", marginBottom: 4 }}>Network Status</div>

      <div style={{ padding: 18, borderRadius: 16, background: "rgba(255,255,255,0.8)", border: "1px solid rgba(15,23,42,0.06)" }}>
        {rows.length === 0 ? (
          <div style={{ fontSize: 13, color: "#94a3b8" }}>No network information available.</div>
        ) : (
          <div style={{ display: "grid", gap: 10 }}>
            {rows.map((r, i) => (
              <div key={i} style={{ display: "flex", justifyContent: "space-between", alignItems: "center", padding: "8px 0", borderBottom: i < rows.length - 1 ? "1px solid rgba(15,23,42,0.04)" : "none" }}>
                <div style={{ fontSize: 13, color: "#64748b" }}>{r.label}</div>
                <div style={{ fontSize: 13, fontWeight: 600, color: "#0f172a" }}>{r.value}</div>
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}
