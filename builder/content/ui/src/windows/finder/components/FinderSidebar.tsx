import React from "react";
import { ArrowDownToLine, ArrowUpFromLine } from "lucide-react";
import { SamarisIcon } from "../../../modules/icons";
import type { FinderSection } from "../model";

export const FinderSidebar = React.memo(function FinderSidebar(props: {
  sections: FinderSection[];
  currentPath: string;
  onSelect: (path: string) => void;
  onMountDevice?: (devicePath: string) => void;
  onUnmountDevice?: (devicePath: string) => void;
}) {
  return (
    <aside className="finder-sidebar">
      {props.sections.map((section) => (
        <div key={section.title} className="finder-sidebar__section">
          <div className="finder-sidebar__heading">{section.title}</div>
          <div className="finder-sidebar__items">
            {section.items.map((item) => {
              const active = item.path === props.currentPath;
              return (
                <button
                  key={item.id}
                  type="button"
                  className={`finder-sidebar__item${active ? " finder-sidebar__item--active" : ""}`}
                  disabled={item.disabled}
                  onClick={() => { if (!item.disabled) props.onSelect(item.path); }}
                >
                  <span className="finder-sidebar__icon">
                    <SamarisIcon name={item.icon} size={20} variant="mono" surface="bare" />
                  </span>
                  <span className="finder-sidebar__label">{item.label}</span>
                  {item.hint ? <span className="finder-sidebar__hint">{item.hint}</span> : null}
                  {item.ejectable ? (
                    item.mounted ? (
                      <button type="button" className="finder-sidebar__action"
                        onClick={(e) => { e.stopPropagation(); props.onUnmountDevice?.(item.devicePath!); }}
                        aria-label={`Eject ${item.label}`}
                        title={`Eject ${item.label}`}
                      >
                        <ArrowUpFromLine size={12} />
                      </button>
                    ) : (
                      <button type="button" className="finder-sidebar__action"
                        onClick={(e) => { e.stopPropagation(); props.onMountDevice?.(item.devicePath!); }}
                        aria-label={`Mount ${item.label}`}
                        title={`Mount ${item.label}`}
                      >
                        <ArrowDownToLine size={12} />
                      </button>
                    )
                  ) : null}
                </button>
              );
            })}
          </div>
        </div>
      ))}
    </aside>
  );
});
