import { Cable, Globe2, Wifi } from "lucide-react";
import type { NetworkInterfaceModel } from "../types";

export function NetworkSidebar(props: {
  interfaces: NetworkInterfaceModel[];
  selectedId: string | null;
  onSelect: (id: string) => void;
}) {
  return (
    <aside className="network__sidebar">
      <div className="network__brand">
        <div className="network__brandMark">Network</div>
        <div className="network__brandMeta">Configure interfaces, DNS, and IP addressing.</div>
      </div>
      <div className="network__interfaceList">
        {props.interfaces.map((entry) => {
          const Icon = entry.type === "wifi" ? Wifi : entry.type === "ethernet" ? Cable : Globe2;
          return (
            <button
              key={entry.interfaceId}
              type="button"
              className={`network__interfaceItem ${props.selectedId === entry.interfaceId ? "network__interfaceItem--active" : ""}`}
              onClick={() => props.onSelect(entry.interfaceId)}
            >
              <span className="network__interfaceGlyph">
                <Icon size={15} strokeWidth={2.1} />
              </span>
              <span className="network__interfaceMeta">
                <strong>{entry.label}</strong>
                <small>{entry.connected ? entry.address || "Connected" : "Not connected"}</small>
              </span>
            </button>
          );
        })}
      </div>
    </aside>
  );
}
