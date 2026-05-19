import React from "react";
import type { WindowSidebarItem } from "../types";
import { classNames } from "../utils/classNames";

export function WindowSidebar(props: {
  label?: string;
  items?: WindowSidebarItem[];
  children?: React.ReactNode;
  className?: string;
  onSelect?: (item: WindowSidebarItem) => void;
}) {
  return (
    <aside className={classNames("samaris-window-sidebar", props.className)}>
      {props.label ? <div className="samaris-sidebar-label">{props.label}</div> : null}
      {props.items?.map((item) => (
        <button
          key={item.id}
          type="button"
          className={classNames("samaris-nav-item", item.active ? "active" : "")}
          onClick={() => props.onSelect?.(item)}
        >
          <span
            className="samaris-nav-dot"
            aria-hidden="true"
            style={item.accent !== undefined ? ({ ["--accent" as string]: item.accent } as React.CSSProperties) : undefined}
          />
          <span>{item.label}</span>
        </button>
      ))}
      {props.children}
    </aside>
  );
}
