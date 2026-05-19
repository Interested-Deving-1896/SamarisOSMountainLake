import React from "react";
import { Palette, Monitor, Lock, Users, Wifi, Shield, Bell, Accessibility, HardDrive, Download, Languages, Code2, Info } from "lucide-react";

type SettingsNavItem = {
  id: string;
  label: string;
  Icon: React.ComponentType<any>;
};

const NAV_ITEMS: SettingsNavItem[] = [
  { id: "appearance", label: "Appearance", Icon: Palette },
  { id: "desktop", label: "Desktop", Icon: Monitor },
  { id: "session", label: "Session & Lock", Icon: Lock },
  { id: "accounts", label: "Accounts", Icon: Users },
  { id: "network", label: "Network", Icon: Wifi },
  { id: "security", label: "Security", Icon: Shield },
  { id: "notifications", label: "Notifications", Icon: Bell },
  { id: "accessibility", label: "Accessibility", Icon: Accessibility },
  { id: "storage", label: "Storage", Icon: HardDrive },
  { id: "updates", label: "Software Update", Icon: Download },
  { id: "language", label: "Language", Icon: Languages },
  { id: "developer", label: "Developer", Icon: Code2 },
  { id: "about", label: "About", Icon: Info },
];

export function SettingsSidebar(props: {
  activeSection: string;
  onSelect: (id: string) => void;
  hiddenSections: Set<string>;
}) {
  return (
    <nav className="sts-sidebar">
      {NAV_ITEMS.map((item) => {
        const hidden = props.hiddenSections.has(item.id);
        return (
          <button
            key={item.id}
            type="button"
            className={`sts-sidebar__item ${props.activeSection === item.id ? "sts-sidebar__item--active" : ""} ${hidden ? "sts-sidebar__item--hidden" : ""}`}
            onClick={() => props.onSelect(item.id)}
          >
            <span className="sts-sidebar__icon">
              <item.Icon size={16} strokeWidth={2} />
            </span>
            <span className="sts-sidebar__label">{item.label}</span>
          </button>
        );
      })}
    </nav>
  );
}
