import React from "react";
import { ChevronDown } from "lucide-react";
import { TrashItem } from "./TrashItem";
import type { TrashEntry } from "../trashIndex";

export function TrashGroup(props: {
  label: string;
  entries: TrashEntry[];
  selected: Set<string>;
  onToggle: (name: string) => void;
  onRestore: (name: string) => void;
  onDelete: (name: string) => void;
  disabled: boolean;
}) {
  const [collapsed, setCollapsed] = React.useState(false);

  return (
    <div className="trash__group">
      <button
        type="button"
        className="trash__groupHead"
        onClick={() => setCollapsed(!collapsed)}
        aria-expanded={!collapsed}
      >
        <ChevronDown
          size={14}
          strokeWidth={2.5}
          className={`trash__groupChevron${collapsed ? " trash__groupChevron--collapsed" : ""}`}
        />
        <span className="trash__groupLabel">{props.label}</span>
        <span className="trash__groupCount">{props.entries.length}</span>
      </button>
      {!collapsed ? (
        <div className="trash__groupBody" role="list">
          {props.entries.map((entry) => (
            <TrashItem
              key={entry.id}
              entry={entry}
              selected={props.selected.has(entry.name)}
              onToggle={props.onToggle}
              onRestore={props.onRestore}
              onDelete={props.onDelete}
              disabled={props.disabled}
            />
          ))}
        </div>
      ) : null}
    </div>
  );
}
