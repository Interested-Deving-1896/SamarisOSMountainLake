import React from "react";
import { PeregrineToolbar } from "./components/PeregrineToolbar";
import { PeregrineViewport } from "./components/PeregrineViewport";
import { PeregrineContextMenu } from "./components/PeregrineContextMenu";
import { DownloadBar } from "./components/DownloadBar";
import { usePeregrine } from "./hooks/usePeregrine";
import { useHistory } from "./hooks/useHistory";
import { downloadStore, type DownloadItem } from "../../system/downloads/downloadStore";
import "./peregrine.css";

const isElectron = typeof window !== "undefined" && !!window.electronAPI;

export function PeregrineApp(_props: { windowId: string }) {
  const p = usePeregrine();
  const hist = useHistory();
  const pRef = React.useRef(p);
  const initialized = React.useRef(false);
  const lastHistoryKey = React.useRef("");
  const [ctxMenu, setCtxMenu] = React.useState<{ x: number; y: number } | null>(null);
  const [downloads, setDownloads] = React.useState<DownloadItem[]>([]);
  const [visibleDownloadIds, setVisibleDownloadIds] = React.useState<Set<string>>(() => new Set());
  const [zoom, setZoom] = React.useState(1);
  const downloadStates = React.useRef(new Map<string, DownloadItem["state"]>());
  const downloadsInitialized = React.useRef(false);
  const downloadHideTimers = React.useRef(new Map<string, ReturnType<typeof setTimeout>>());

  const activeTab = p.tabs.find((tab) => tab.id === p.activeTabId) || null;
  const visibleDownloads = React.useMemo(
    () => downloads.filter((item) => visibleDownloadIds.has(item.id)).slice(0, 5),
    [downloads, visibleDownloadIds],
  );

  React.useEffect(() => {
    pRef.current = p;
  }, [p]);

  React.useEffect(() => {
    if (!isElectron || initialized.current) return;
    initialized.current = true;
    pRef.current.refreshStatus().then(async () => {
      const snapshot = await window.electronAPI!.browser.getSnapshot();
      if (snapshot.tabs.length === 0) await pRef.current.launch("about:blank");
    }).catch(() => {});
  }, []);

  React.useEffect(() => {
    const clearHideTimer = (id: string) => {
      const timer = downloadHideTimers.current.get(id);
      if (timer) clearTimeout(timer);
      downloadHideTimers.current.delete(id);
    };

    const scheduleHide = (id: string) => {
      clearHideTimer(id);
      downloadHideTimers.current.set(id, setTimeout(() => {
        setVisibleDownloadIds((prev) => {
          const next = new Set(prev);
          next.delete(id);
          return next;
        });
        downloadHideTimers.current.delete(id);
      }, 3000));
    };

    const unsub = downloadStore.subscribe((items) => {
      setDownloads(items);

      const previous = downloadStates.current;
      const initial = !downloadsInitialized.current;
      const itemIds = new Set(items.map((item) => item.id));

      setVisibleDownloadIds((prev) => {
        const next = new Set(prev);
        for (const id of next) {
          if (!itemIds.has(id)) {
            next.delete(id);
            clearHideTimer(id);
          }
        }

        for (const item of items) {
          const prevState = previous.get(item.id);
          if (item.state === "downloading") {
            if (!initial) next.add(item.id);
            clearHideTimer(item.id);
          } else if (!initial && prevState === "downloading") {
            next.add(item.id);
            scheduleHide(item.id);
          }
        }
        return next;
      });

      downloadStates.current = new Map(items.map((item) => [item.id, item.state]));
      downloadsInitialized.current = true;
    });

    return () => {
      unsub();
      for (const timer of downloadHideTimers.current.values()) clearTimeout(timer);
      downloadHideTimers.current.clear();
    };
  }, []);

  React.useEffect(() => {
    if (!activeTab || activeTab.private || !/^https?:\/\//i.test(activeTab.url)) return;
    const key = `${activeTab.id}:${activeTab.url}`;
    if (lastHistoryKey.current === key) return;
    lastHistoryKey.current = key;
    hist.addToHistory(activeTab.url, activeTab.title || activeTab.url, activeTab.favicon || undefined);
  }, [activeTab?.id, activeTab?.url, activeTab?.title, activeTab?.favicon, activeTab?.private, hist]);

  React.useEffect(() => {
    if (activeTab?.zoom) setZoom(activeTab.zoom);
  }, [activeTab?.id, activeTab?.zoom]);

  React.useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      if (!(e.metaKey || e.ctrlKey)) return;
      const api = pRef.current;
      if (e.key === "t") { e.preventDefault(); void api.launch("about:blank"); return; }
      if (e.key === "w") { e.preventDefault(); if (api.activeTabId) void api.closeTab(api.activeTabId); return; }
      if (e.key === "l") { e.preventDefault(); (document.querySelector(".pr-urlInput") as HTMLInputElement | null)?.select(); return; }
      if (e.key === "r") { e.preventDefault(); api.reload(); return; }
      if (e.key === "=" || e.key === "+") {
        e.preventDefault();
        setZoom((z) => { const n = Math.min(5, +(z + 0.25).toFixed(2)); if (api.activeTabId) void api.setTabZoom(api.activeTabId, n); return n; });
      }
      if (e.key === "-") {
        e.preventDefault();
        setZoom((z) => { const n = Math.max(0.25, +(z - 0.25).toFixed(2)); if (api.activeTabId) void api.setTabZoom(api.activeTabId, n); return n; });
      }
      if (e.key === "0") {
        e.preventDefault();
        setZoom(1);
        if (api.activeTabId) void api.setTabZoom(api.activeTabId, 1);
      }
    };
    window.addEventListener("keydown", handler);
    return () => window.removeEventListener("keydown", handler);
  }, []);

  const viewportUrl = activeTab?.url || p.value || "";

  const handleContextMenu = React.useCallback((e: React.MouseEvent | any) => {
    e.preventDefault?.();
    const x = e.x ?? e.clientX;
    const y = e.y ?? e.clientY;
    if (x != null && y != null) setCtxMenu({ x, y });
  }, []);

  const handleSubmit = React.useCallback(() => {
    if (p.value) p.navigate(p.value);
  }, [p]);

  return (
    <div className="peregrine">
      <PeregrineToolbar
        tabs={p.tabs}
        activeTabId={p.activeTabId}
        value={p.value}
        loading={Boolean(activeTab?.loading)}
        onChange={p.setValue}
        onSubmit={handleSubmit}
        onGoBack={p.goBack}
        onGoForward={p.goForward}
        onReload={p.reload}
        onCloseTab={p.closeTab}
        onSelectTab={p.selectTab}
        onNewTab={() => void p.launch("about:blank")}
        onNewPrivateTab={() => void p.createPrivateTab("about:blank")}
        history={hist.history}
        bookmarks={hist.bookmarks}
        isBookmarked={hist.isBookmarked(viewportUrl)}
        onAddBookmark={() => hist.addBookmark(viewportUrl, activeTab?.title || viewportUrl)}
        onRemoveBookmark={() => hist.removeBookmark(viewportUrl)}
        onClear={() => { void p.clearData("all"); hist.clearHistory(); }}
        onOpenHistory={(url) => p.navigate(url)}
        zoom={zoom}
        onZoomIn={() => { setZoom((z) => { const n = Math.min(5, +(z + 0.25).toFixed(2)); if (p.activeTabId) void p.setTabZoom(p.activeTabId, n); return n; }); }}
        onZoomOut={() => { setZoom((z) => { const n = Math.max(0.25, +(z - 0.25).toFixed(2)); if (p.activeTabId) void p.setTabZoom(p.activeTabId, n); return n; }); }}
        onZoomReset={() => { setZoom(1); if (p.activeTabId) void p.setTabZoom(p.activeTabId, 1); }}
      />

      <PeregrineViewport
        tabs={p.tabs}
        activeTabId={p.activeTabId}
        quickLinks={p.quickLinks}
        onOpenQuickLink={(url) => p.activeTabId ? p.navigate(url) : void p.launch(url)}
        onBoundsChange={p.setBounds}
        onContextMenu={handleContextMenu}
      />

      <DownloadBar items={visibleDownloads} onCancel={(id) => { if (isElectron) void window.electronAPI!.downloads.cancel(id); }} />

      {ctxMenu && (
        <PeregrineContextMenu
          x={ctxMenu.x}
          y={ctxMenu.y}
          tabId={p.activeTabId}
          url={viewportUrl}
          onClose={() => setCtxMenu(null)}
          onNavigate={(url) => p.navigate(url)}
        />
      )}
    </div>
  );
}
