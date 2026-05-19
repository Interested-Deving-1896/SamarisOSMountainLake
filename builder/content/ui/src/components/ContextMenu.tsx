import React, { useEffect, useLayoutEffect, useMemo, useRef } from "react";
import { createPortal } from "react-dom";
import type { LucideIcon } from "lucide-react";
import "./contextMenu.css";

export type ContextMenuItem = {
  id: string;
  label: string;
  icon?: LucideIcon;
  disabled?: boolean;
  danger?: boolean;
  onSelect: () => void;
};

export function ContextMenu(props: {
  x: number;
  y: number;
  items: ContextMenuItem[];
  onClose: () => void;
  ariaLabel?: string;
}) {
  const menuRef = useRef<HTMLDivElement>(null);

  const filtered = useMemo(() => props.items.filter(Boolean), [props.items]);

  useEffect(() => {
    const onPointerDown = (event: PointerEvent) => {
      const target = event.target as HTMLElement | null;
      if (!target) return;
      if (menuRef.current && menuRef.current.contains(target)) return;
      props.onClose();
    };
    const onKeyDown = (event: KeyboardEvent) => {
      if (event.key === "Escape") props.onClose();
    };
    window.addEventListener("pointerdown", onPointerDown);
    window.addEventListener("keydown", onKeyDown);
    return () => {
      window.removeEventListener("pointerdown", onPointerDown);
      window.removeEventListener("keydown", onKeyDown);
    };
  }, [props]);

  useLayoutEffect(() => {
    if (!menuRef.current) return;
    const rect = menuRef.current.getBoundingClientRect();
    const margin = 6;
    const nextLeft = Math.min(props.x, window.innerWidth - rect.width - margin);
    const nextTop = Math.min(props.y, window.innerHeight - rect.height - margin);
    menuRef.current.style.left = `${Math.max(margin, nextLeft)}px`;
    menuRef.current.style.top = `${Math.max(margin, nextTop)}px`;
  }, [props.x, props.y, filtered.length]);

  const menu = (
    <div
      ref={menuRef}
      className="cm"
      style={{ left: props.x, top: props.y }}
      role="menu"
      aria-label={props.ariaLabel || "Context menu"}
      onContextMenu={(e) => e.preventDefault()}
    >
      {filtered.map((item) => {
        const Icon = item.icon;
        return (
          <button
            key={item.id}
            type="button"
            role="menuitem"
            className={`cm__item ${item.danger ? "cm__item--danger" : ""}`}
            disabled={item.disabled}
            onClick={() => {
              if (item.disabled) return;
              props.onClose();
              item.onSelect();
            }}
          >
            <span className="cm__icon" aria-hidden="true">
              {Icon ? <Icon size={16} strokeWidth={2.2} /> : null}
            </span>
            <span className="cm__label">{item.label}</span>
          </button>
        );
      })}
    </div>
  );

  return createPortal(menu, document.body);
}
