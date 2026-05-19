import React, { useEffect, useRef } from "react";
import { PeregrineStartPage } from "./PeregrineStartPage";
import type { PeregrineTab, PeregrineQuickLink } from "../types";

const WEB_CONTENTS_CORNER_GUARD = 18;

function ActiveViewSlot(props: {
  tab: PeregrineTab;
  onBoundsChange: (tabId: string, bounds: { x: number; y: number; width: number; height: number }) => void;
}) {
  const ref = useRef<HTMLDivElement>(null);
  const lastBounds = useRef({ x: -1, y: -1, w: -1, h: -1 });

  useEffect(() => {
    const node = ref.current;
    if (!node) return;

    let frame = 0;

    const measure = () => {
      const rect = node.getBoundingClientRect();
      const prev = lastBounds.current;
      if (rect.left !== prev.x || rect.top !== prev.y || rect.width !== prev.w || rect.height !== prev.h) {
        prev.x = rect.left; prev.y = rect.top; prev.w = rect.width; prev.h = rect.height;
        props.onBoundsChange(props.tab.id, {
          x: rect.left,
          y: rect.top,
          width: rect.width,
          height: Math.max(1, rect.height - WEB_CONTENTS_CORNER_GUARD),
        });
      }
      frame = requestAnimationFrame(measure);
    };

    frame = requestAnimationFrame(measure);

    return () => { cancelAnimationFrame(frame); };
  }, [props.tab.id, props.onBoundsChange]);

  return (
    <div ref={ref} className="pr-webcontentsHost">
      {props.tab.crashed ? (
        <div className="pr-crash">
          <strong>Page crashed</strong>
          <span>Reload this tab to start a fresh renderer.</span>
        </div>
      ) : null}
    </div>
  );
}

export function PeregrineViewport(props: {
  tabs: PeregrineTab[];
  activeTabId: string | null;
  quickLinks: PeregrineQuickLink[];
  onOpenQuickLink: (url: string) => void;
  onBoundsChange: (tabId: string, bounds: { x: number; y: number; width: number; height: number }) => void;
  onContextMenu: (e: any) => void;
}) {
  const activeTab = props.tabs.find((tab) => tab.id === props.activeTabId) || null;
  const showHome = !activeTab || activeTab.url === "about:blank";

  return (
    <div className="pr-viewportShell" onContextMenu={props.onContextMenu}>
      <div className="pr-viewport">
        {showHome ? (
          <PeregrineStartPage quickLinks={props.quickLinks} onOpen={props.onOpenQuickLink} />
        ) : (
          <ActiveViewSlot tab={activeTab} onBoundsChange={props.onBoundsChange} />
        )}
      </div>
    </div>
  );
}
