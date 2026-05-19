import React from "react";
import { Bell, BellOff, Wifi, BatteryMedium, Bluetooth, Volume2, Download, Moon, Trash2 } from "lucide-react";
import { useAirBar } from "../useAirBar";
import { connectivityStore } from "../../../system/connectivity/connectivityStore";
import { batteryStore } from "../../../system/battery/batteryStore";
import { audioStore } from "../../../system/audio/audioStore";
import { downloadStore, type DownloadItem } from "../../../system/downloads/downloadStore";

type Notification = { id: string; icon: React.ReactNode; title: string; message: string; time: string; read: boolean };

function relativeTime(d: Date): string {
  const diff = Date.now() - d.getTime();
  if (diff < 60000) return "Just now";
  if (diff < 3600000) return `${Math.floor(diff / 60000)}m ago`;
  if (diff < 86400000) return `${Math.floor(diff / 3600000)}h ago`;
  return d.toLocaleDateString();
}

export const SidebarPanel = React.memo(function SidebarPanel() {
  const air = useAirBar();
  const open = air.activePanel === "sidebar";
  const style = air.getPanelStyle("sidebar", { width: 380, align: "end" });

  const conn = React.useSyncExternalStore((l) => connectivityStore.subscribe(l), () => connectivityStore.getState());
  const bat = React.useSyncExternalStore((l) => batteryStore.subscribe(l), () => batteryStore.getState());
  const audio = React.useSyncExternalStore((l) => audioStore.subscribe(l), () => audioStore.getState());
  const [downloads, setDownloads] = React.useState<DownloadItem[]>([]);
  const [quiet, setQuiet] = React.useState(false);
  const [cleared, setCleared] = React.useState(false);

  React.useEffect(() => {
    connectivityStore.init();
    audioStore.init();
    batteryStore.refresh();
    const unsub = downloadStore.subscribe(setDownloads);
    return () => { if (typeof unsub === "function") unsub(); };
  }, []);

  React.useEffect(() => {
    if (!open) { setCleared(false); }
  }, [open]);

  const now = new Date();
  const notifications: Notification[] = [];

  if (conn.wifiEnabled && conn.currentNetworkLabel)
    notifications.push({ id: "wifi", icon: <Wifi size={16} />, title: "Wi‑Fi Connected", message: conn.currentNetworkLabel, time: relativeTime(now), read: false });
  if (bat.percentage !== undefined && bat.percentage <= 20 && bat.available)
    notifications.push({ id: "battery", icon: <BatteryMedium size={16} />, title: "Low Battery", message: `${bat.percentage}% remaining · ${bat.source || ""}`, time: relativeTime(now), read: false });
  if (conn.bluetoothEnabled && conn.devices.some((d) => d.connected))
    notifications.push({ id: "bt", icon: <Bluetooth size={16} />, title: "Bluetooth Connected", message: conn.devices.filter((d) => d.connected).map((d) => d.label).join(", "), time: relativeTime(now), read: false });
  if (audio.muted)
    notifications.push({ id: "sound", icon: <Volume2 size={16} />, title: "Sound Muted", message: `Volume is at ${audio.volume}%`, time: relativeTime(now), read: false });
  downloads.filter((d) => d.state === "completed").slice(0, 3).forEach((d) =>
    notifications.push({ id: `dl-${d.id}`, icon: <Download size={16} />, title: "Download Complete", message: d.filename, time: relativeTime(new Date(d.startTime)), read: false })
  );

  if (cleared) {
    notifications.length = 0;
  }

  const unread = notifications.filter((n) => !n.read).length;

  const handleClear = () => setCleared(true);

  return (
    <aside style={style} className={`airbar-panel sidebar-panel ${open ? "open" : ""}`} role="dialog" aria-label="Notification Center">
      <div className="nc-toolbar">
        <button className={`nc-quiet ${quiet ? "nc-quiet--on" : ""}`} onClick={() => setQuiet(!quiet)} title="Quiet mode">
          <Moon size={16} />
        </button>
        <div className="nc-title">
          <Bell size={16} />
          <span>Notifications</span>
          {unread > 0 && <span className="nc-badge">{unread}</span>}
        </div>
        {notifications.length > 0 && (
          <button className="nc-clear" onClick={handleClear} title="Clear all"><Trash2 size={14} /></button>
        )}
      </div>

      {quiet ? (
        <div className="nc-empty">
          <Moon size={32} strokeWidth={1.5} />
          <span>Quiet mode is on</span>
          <small>Notifications are paused. Tap the moon icon to resume.</small>
        </div>
      ) : notifications.length === 0 ? (
        <div className="nc-empty">
          <BellOff size={32} strokeWidth={1.5} />
          <span>No notifications</span>
          <small>Connect to Wi‑Fi or download files to see notifications here.</small>
        </div>
      ) : (
        <div className="nc-list">
          {notifications.map((n) => (
            <div key={n.id} className={`nc-item ${n.read ? "nc-item--read" : ""}`}>
              <span className="nc-itemIcon">{n.icon}</span>
              <div className="nc-itemBody">
                <div className="nc-itemTitle">{n.title}</div>
                <div className="nc-itemMsg">{n.message}</div>
              </div>
              <span className="nc-itemTime">{n.time}</span>
            </div>
          ))}
        </div>
      )}
    </aside>
  );
});
