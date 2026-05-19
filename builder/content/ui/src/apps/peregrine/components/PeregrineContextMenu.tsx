import React, { useEffect, useLayoutEffect, useRef, useState } from "react";
import {
  ArrowLeft, ArrowRight, RotateCcw, XCircle, Download, Printer, Globe,
  Copy, Scissors, ClipboardPaste, FileCode2, Inspect, PanelTop,
  Search, FileDown,
} from "lucide-react";

type MenuItem = { id: string; label: string; icon: React.ReactNode; disabled?: boolean; separator?: boolean; action: () => void };

export function PeregrineContextMenu(props: {
  x: number; y: number; tabId: string | null; url: string;
  onClose: () => void; onNavigate: (url: string) => void;
}) {
  const ref = useRef<HTMLDivElement>(null);
  const isElectron = typeof window !== "undefined" && !!window.electronAPI;
  const [menuSize, setMenuSize] = useState({ width: 220, height: 380 });

  useEffect(() => {
    const h = (e: PointerEvent) => { if (ref.current && !ref.current.contains(e.target as Node)) props.onClose(); };
    const k = (e: KeyboardEvent) => { if (e.key === "Escape") props.onClose(); };
    window.addEventListener("pointerdown", h);
    window.addEventListener("keydown", k);
    return () => { window.removeEventListener("pointerdown", h); window.removeEventListener("keydown", k); };
  }, [props]);

  useLayoutEffect(() => {
    const rect = ref.current?.getBoundingClientRect();
    if (!rect) return;
    setMenuSize({ width: Math.ceil(rect.width), height: Math.ceil(rect.height) });
  }, [props.x, props.y, props.tabId, props.url]);

  const items: MenuItem[] = [
    // Navigation
    { id: "back", label: "Back", icon: <ArrowLeft size={14} />, disabled: !props.tabId, action: () => { if (props.tabId && isElectron) window.electronAPI!.browser.command(props.tabId, "back"); } },
    { id: "forward", label: "Forward", icon: <ArrowRight size={14} />, disabled: !props.tabId, action: () => { if (props.tabId && isElectron) window.electronAPI!.browser.command(props.tabId, "forward"); } },
    { id: "reload", label: "Reload", icon: <RotateCcw size={14} />, disabled: !props.tabId, action: () => { if (props.tabId && isElectron) window.electronAPI!.browser.command(props.tabId, "reload"); } },
    { id: "stop", label: "Stop", icon: <XCircle size={14} />, disabled: !props.tabId, action: () => { if (props.tabId && isElectron) window.electronAPI!.browser.command(props.tabId, "stop"); } },
    // Separator
    { id: "sep1", label: "", icon: null, separator: true, action: () => {} },
    // Save / Print
    { id: "save", label: "Save Page As…", icon: <FileDown size={14} />, disabled: !props.tabId || !isElectron, action: () => { if (props.tabId && isElectron) window.electronAPI!.browser.command(props.tabId, "savePage").catch(() => {}); } },
    { id: "print", label: "Print…", icon: <Printer size={14} />, disabled: !props.tabId || !isElectron, action: () => { if (props.tabId && isElectron) window.electronAPI!.browser.command(props.tabId, "print").catch(() => {}); } },
    // Separator
    { id: "sep2", label: "", icon: null, separator: true, action: () => {} },
    // Clipboard
    { id: "copy", label: "Copy", icon: <Copy size={14} />, disabled: !props.tabId, action: () => { if (props.tabId && isElectron) window.electronAPI!.browser.command(props.tabId, "copy").catch(() => {}); } },
    { id: "cut", label: "Cut", icon: <Scissors size={14} />, disabled: !props.tabId, action: () => { if (props.tabId && isElectron) window.electronAPI!.browser.command(props.tabId, "cut").catch(() => {}); } },
    { id: "paste", label: "Paste", icon: <ClipboardPaste size={14} />, disabled: !props.tabId, action: () => { if (props.tabId && isElectron) window.electronAPI!.browser.command(props.tabId, "paste").catch(() => {}); } },
    { id: "selectall", label: "Select All", icon: <PanelTop size={14} />, disabled: !props.tabId, action: () => { if (props.tabId && isElectron) window.electronAPI!.browser.command(props.tabId, "selectAll").catch(() => {}); } },
    // Separator
    { id: "sep3", label: "", icon: null, separator: true, action: () => {} },
    // Dev
    { id: "source", label: "View Page Source", icon: <FileCode2 size={14} />, disabled: !props.url, action: () => { props.onNavigate(`view-source:${props.url}`); } },
    { id: "inspect", label: "Inspect Element", icon: <Inspect size={14} />, disabled: !props.tabId || !isElectron, action: () => { if (props.tabId && isElectron) window.electronAPI!.browser.command(props.tabId, "openDevTools").catch(() => {}); } },
  ];

  const gutter = 8;
  const maxX = window.innerWidth - menuSize.width - gutter;
  const maxY = window.innerHeight - menuSize.height - gutter;
  const left = Math.min(props.x, maxX);
  const top = Math.min(props.y, maxY);

  return (
    <div ref={ref} className="pr-context-menu" style={{ position: "fixed", left: Math.max(0, left), top: Math.max(0, top), zIndex: 999999 }}>
      {items.map((item) =>
        item.separator ? (
          <div key={item.id} className="pr-cm-sep" />
        ) : (
          <button key={item.id} className={`pr-cm-item ${item.disabled ? "pr-cm-item--disabled" : ""}`} disabled={item.disabled} onClick={() => { if (!item.disabled) item.action(); props.onClose(); }}>
            <span className="pr-cm-icon">{item.icon}</span>
            <span>{item.label}</span>
          </button>
        )
      )}
    </div>
  );
}
