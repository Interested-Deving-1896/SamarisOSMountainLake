import React from "react";
import { Trash2, Search, ChevronDown, RotateCcw, AlertTriangle } from "lucide-react";

export function TrashHeader(props: {
  itemCount: number;
  totalSize: string;
  selectedCount: number;
  onRestoreSelected: () => void;
  onEmptyTrash: () => void;
  onSecureEmpty: () => void;
  searchQuery: string;
  onSearchChange: (q: string) => void;
  hasSelection: boolean;
  emitting: boolean;
}) {
  const [menuOpen, setMenuOpen] = React.useState(false);
  const menuRef = React.useRef<HTMLDivElement>(null);

  React.useEffect(() => {
    function handleClick(e: MouseEvent) {
      if (menuRef.current && !menuRef.current.contains(e.target as Node)) {
        setMenuOpen(false);
      }
    }
    if (menuOpen) document.addEventListener("pointerdown", handleClick);
    return () => document.removeEventListener("pointerdown", handleClick);
  }, [menuOpen]);

  return (
    <div className="trash__head">
      <div className="trash__brand">
        <div className="trash__brandGlyph">
          <Trash2 size={22} strokeWidth={2} />
        </div>
        <div>
          <div className="trash__brandTitle">Trash</div>
          <div className="trash__brandMeta">
            {props.itemCount} {props.itemCount === 1 ? "item" : "items"}
            {props.totalSize ? ` \u00B7 ${props.totalSize}` : ""}
          </div>
        </div>
      </div>

      <div className="trash__search">
        <Search size={14} strokeWidth={2.2} />
        <input
          className="trash__searchInput"
          placeholder="Search trash\u2026"
          value={props.searchQuery}
          onChange={(e) => props.onSearchChange(e.target.value)}
        />
      </div>

      <div className="trash__actions">
        <button
          type="button"
          className="trash__actionBtn trash__actionBtn--restore"
          disabled={!props.hasSelection || props.emitting}
          onClick={props.onRestoreSelected}
        >
          <RotateCcw size={14} strokeWidth={2.2} />
          {props.selectedCount > 0
            ? `Restore (${props.selectedCount})`
            : "Restore"}
        </button>

        <div className="trash__actionMenuWrap" ref={menuRef}>
          <button
            type="button"
            className="trash__actionBtn trash__actionBtn--empty"
            disabled={props.itemCount === 0 || props.emitting}
            onClick={() => setMenuOpen(!menuOpen)}
          >
            <Trash2 size={14} strokeWidth={2.2} />
            Empty Trash
            <ChevronDown size={12} strokeWidth={2.2} />
          </button>
          {menuOpen ? (
            <div className="trash__actionMenu" role="menu">
              <button
                type="button"
                role="menuitem"
                className="trash__actionMenuItem"
                onClick={() => {
                  setMenuOpen(false);
                  props.onEmptyTrash();
                }}
              >
                <Trash2 size={14} strokeWidth={2.2} />
                Empty All
              </button>
              <button
                type="button"
                role="menuitem"
                className="trash__actionMenuItem trash__actionMenuItem--danger"
                onClick={() => {
                  setMenuOpen(false);
                  props.onSecureEmpty();
                }}
              >
                <AlertTriangle size={14} strokeWidth={2.2} />
                Secure Empty Trash
              </button>
            </div>
          ) : null}
        </div>
      </div>
    </div>
  );
}
