import React from "react";
import { Wifi, Cable, Activity } from "lucide-react";
import { useNetworkInterfaces } from "./hooks/useNetworkInterfaces";
import { NetworkSidebar } from "./components/NetworkSidebar";
import { NetworkForm } from "./components/NetworkForm";
import { WifiTab } from "./components/WifiTab";
import { StatusTab } from "./components/StatusTab";
import type { TabId } from "./types";

const TABS: { id: TabId; label: string; icon: React.ReactNode }[] = [
  { id: "wifi", label: "Wi‑Fi", icon: <Wifi size={15} /> },
  { id: "interfaces", label: "Interfaces", icon: <Cable size={15} /> },
  { id: "status", label: "Status", icon: <Activity size={15} /> },
];

export function NetworkApp(_props: { windowId: string }) {
  const [tab, setTab] = React.useState<TabId>("wifi");
  const iface = useNetworkInterfaces();
  const selected = iface.interfaces.find((entry) => entry.interfaceId === iface.selectedId) || iface.interfaces[0] || null;

  return (
    <div className="network">
      {/* Tab bar */}
      <div className="network__tabs">
        <div className="network__tabsInner">
          {TABS.map((t) => (
            <button
              key={t.id}
              type="button"
              className={`network__tab ${tab === t.id ? "network__tab--active" : ""}`}
              onClick={() => setTab(t.id)}
            >
              {t.icon}
              <span>{t.label}</span>
            </button>
          ))}
        </div>
      </div>

      {/* Content */}
      <div className="network__content">
        {tab === "wifi" && <WifiTab />}
        {tab === "interfaces" && (
          <div className="network__interfaceLayout">
            <NetworkSidebar interfaces={iface.interfaces} selectedId={iface.selectedId} onSelect={iface.setSelectedId} />
            {iface.loading ? (
              <div className="network__empty">Detecting interfaces…</div>
            ) : (
              <NetworkForm network={selected} saving={iface.saving} note={iface.note} onApply={(draft) => void iface.applyConfig(draft)} />
            )}
          </div>
        )}
        {tab === "status" && <StatusTab />}
      </div>
    </div>
  );
}
