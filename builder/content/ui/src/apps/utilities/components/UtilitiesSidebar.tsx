import { Activity, Cpu, Globe, HardDrive, Lock, Download, Printer } from "lucide-react";

const NAV_ITEMS = [
  { id: "monitor", label: "System Monitor", Icon: Activity },
  { id: "processes", label: "Processes", Icon: Cpu },
  { id: "storage", label: "Storage", Icon: HardDrive },
  { id: "network", label: "Network", Icon: Globe },
  { id: "security", label: "Security", Icon: Lock },
  { id: "updates", label: "Software Update", Icon: Download },
  { id: "print", label: "Print", Icon: Printer },
];

export function UtilitiesSidebar(props: { active: string; onSelect: (id: string) => void }) {
  return (
    <nav className="uts-sidebar">
      {NAV_ITEMS.map((item) => (
        <button
          key={item.id}
          type="button"
          className={`uts-sidebar__item ${props.active === item.id ? "uts-sidebar__item--active" : ""}`}
          onClick={() => props.onSelect(item.id)}
        >
          <span className="uts-sidebar__icon"><item.Icon size={15} strokeWidth={2} /></span>
          <span>{item.label}</span>
        </button>
      ))}
    </nav>
  );
}
