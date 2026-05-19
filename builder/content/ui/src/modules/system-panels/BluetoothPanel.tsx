import React from "react";
import { Bluetooth, BluetoothConnected, BluetoothOff, Check, Headphones, Keyboard, LoaderCircle, Monitor, Mouse, RadioTower, Search, Smartphone, Speaker, X, Trash2 } from "lucide-react";
import { connectivityStore } from "../../system/connectivity/connectivityStore";
import { useAirBar } from "../airbar/useAirBar";
import { SYSTEM_PANEL_CLASSES } from "./panel.styles";

function deviceIcon(name: string) {
  const l = name.toLowerCase();
  if (l.includes("headphone") || l.includes("earphone") || l.includes("airpod") || l.includes("buds")) return <Headphones size={16} />;
  if (l.includes("keyboard")) return <Keyboard size={16} />;
  if (l.includes("mouse") || l.includes("trackpad") || l.includes("magic")) return <Mouse size={16} />;
  if (l.includes("speaker") || l.includes("soundbar") || l.includes("homepod")) return <Speaker size={16} />;
  if (l.includes("phone") || l.includes("iphone") || l.includes("samsung") || l.includes("pixel")) return <Smartphone size={16} />;
  if (l.includes("tv") || l.includes("monitor") || l.includes("display")) return <Monitor size={16} />;
  return <RadioTower size={16} />;
}

export function BluetoothPanel() {
  const air = useAirBar();
  const open = air.activePanel === "bluetooth";
  const c = React.useSyncExternalStore((l) => connectivityStore.subscribe(l), () => connectivityStore.getState());
  const [busyId, setBusyId] = React.useState<string | null>(null);
  const [scanning, setScanning] = React.useState(false);
  const [scanResults, setScanResults] = React.useState<any[]>([]);
  const style = air.getPanelStyle("bluetooth", { width: 364, align: "center" });

  React.useEffect(() => { if (!open) { setBusyId(null); setScanResults([]); } else connectivityStore.refresh(); }, [open]);

  return (
    <section style={style} className={`airbar-panel airbar-system-panel ${open ? "open" : ""}`} role="dialog" aria-label="Bluetooth">
      <div className={SYSTEM_PANEL_CLASSES.panel}>
        <div className={SYSTEM_PANEL_CLASSES.section}>
          <div className={SYSTEM_PANEL_CLASSES.heading}>
            Bluetooth
            <button type="button" className={SYSTEM_PANEL_CLASSES.button} style={{ marginLeft: "auto" }} disabled={scanning} onClick={async () => { setScanning(true); const r = await connectivityStore.scanBluetooth(); setScanResults(Array.isArray(r) ? r : []); setScanning(false); }}>
              {scanning ? <LoaderCircle size={14} className="spin" /> : <Search size={14} />} Scan
            </button>
          </div>
          <button type="button" className={SYSTEM_PANEL_CLASSES.row} onClick={() => connectivityStore.toggleBluetooth()} disabled={!c.capabilities.bluetoothToggle}>
            <span className={SYSTEM_PANEL_CLASSES.rowIcon}>{c.bluetoothEnabled ? <Bluetooth size={18} /> : <BluetoothOff size={18} />}</span>
            <span className={SYSTEM_PANEL_CLASSES.rowText}>
              <span className={SYSTEM_PANEL_CLASSES.rowLabel}>{c.bluetoothEnabled ? "Bluetooth On" : "Bluetooth Off"}</span>
              <span className={SYSTEM_PANEL_CLASSES.rowMeta}>{c.devices.filter((d) => d.connected).length} device(s) connected</span>
            </span>
            <span className={`${SYSTEM_PANEL_CLASSES.switch} ${c.bluetoothEnabled ? SYSTEM_PANEL_CLASSES.switchActive : ""}`} />
          </button>
        </div>

        <div className={SYSTEM_PANEL_CLASSES.section}>
          <div className={SYSTEM_PANEL_CLASSES.heading}>Paired devices</div>
          {c.devices.length === 0 && !c.bluetoothEnabled ? <div className={SYSTEM_PANEL_CLASSES.helper}>Turn Bluetooth on.</div> : null}
          {c.devices.length === 0 && c.bluetoothEnabled ? <div className={SYSTEM_PANEL_CLASSES.helper}>No paired devices.</div> : null}
          <div className={SYSTEM_PANEL_CLASSES.networkList}>
            {c.devices.map((dev) => (
              <div key={dev.id} className={SYSTEM_PANEL_CLASSES.networkRow} style={{ cursor: "default" }}>
                <span className={SYSTEM_PANEL_CLASSES.rowIcon}>{dev.connected ? <BluetoothConnected size={16} /> : deviceIcon(dev.label)}</span>
                <span className={SYSTEM_PANEL_CLASSES.rowText}>
                  <span className={SYSTEM_PANEL_CLASSES.rowLabel}>{dev.label}</span>
                  <span className={SYSTEM_PANEL_CLASSES.rowMeta}>{dev.connected ? "Connected" : "Available"}</span>
                </span>
                <div style={{ display: "flex", gap: 4 }}>
                  {dev.connected ? (
                    <button type="button" className={SYSTEM_PANEL_CLASSES.button} disabled={busyId === dev.id} onClick={async () => { setBusyId(dev.id); await connectivityStore.disconnectBluetooth(dev.id); setBusyId(null); }}>
                      {busyId === dev.id ? <LoaderCircle size={14} className="spin" /> : <X size={14} />}
                    </button>
                  ) : (
                    <button type="button" className={`${SYSTEM_PANEL_CLASSES.button} ${SYSTEM_PANEL_CLASSES.buttonPrimary}`} disabled={busyId === dev.id || !c.capabilities.bluetoothConnect} onClick={async () => { setBusyId(dev.id); await connectivityStore.connectBluetooth(dev.id); setBusyId(null); }}>
                      {busyId === dev.id ? <LoaderCircle size={14} className="spin" /> : <BluetoothConnected size={14} />} Connect
                    </button>
                  )}
                  <button type="button" className={SYSTEM_PANEL_CLASSES.button} style={{ color: "#dc2626" }} disabled={busyId === dev.id} title="Unpair" onClick={async () => { setBusyId(dev.id); await connectivityStore.unpairBluetooth(dev.id); setBusyId(null); }}>
                    {busyId === dev.id ? <LoaderCircle size={14} className="spin" /> : <Trash2 size={14} />}
                  </button>
                </div>
              </div>
            ))}
          </div>
        </div>

        {scanning ? <div className={SYSTEM_PANEL_CLASSES.helper} style={{ padding: "6px 0" }}><LoaderCircle size={14} className="spin" /> Scanning for devices… (10s)</div> : null}
        {scanResults.length > 0 ? (
          <div className={SYSTEM_PANEL_CLASSES.section}>
            <div className={SYSTEM_PANEL_CLASSES.heading}>Nearby devices</div>
            <div className={SYSTEM_PANEL_CLASSES.networkList}>
              {scanResults.map((d, i) => (
                <div key={d.address || i} className={SYSTEM_PANEL_CLASSES.networkRow}>
                  <span className={SYSTEM_PANEL_CLASSES.rowIcon}>{deviceIcon(d.name || "Device")}</span>
                  <span className={SYSTEM_PANEL_CLASSES.rowText}>
                    <span className={SYSTEM_PANEL_CLASSES.rowLabel}>{d.name || "Unknown device"}</span>
                    <span className={SYSTEM_PANEL_CLASSES.rowMeta}>{d.address || ""}</span>
                  </span>
                </div>
              ))}
            </div>
          </div>
        ) : null}
      </div>
    </section>
  );
}
