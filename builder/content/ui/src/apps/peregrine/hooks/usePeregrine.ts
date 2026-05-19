import React from "react";
import type { PeregrineTab, PeregrineLaunchRecord, PeregrineQuickLink } from "../types";

const RECENTS_KEY = "samaris.peregrine.recent";
const QUICK_LINKS: PeregrineQuickLink[] = [
  { id: "google", label: "Google", url: "google.com" },
  { id: "youtube", label: "YouTube", url: "youtube.com" },
  { id: "github", label: "GitHub", url: "github.com" },
  { id: "maps", label: "OpenStreetMap", url: "openstreetmap.org" },
];

const isElectron = typeof window !== "undefined" && !!window.electronAPI;

function isHistoryUrl(url: string): boolean {
  return /^https?:\/\//i.test(url);
}

function isHomeUrl(url?: string): boolean {
  return !url || url === "about:blank";
}

export function usePeregrine() {
  const [value, setValue] = React.useState("");
  const [tabs, setTabs] = React.useState<PeregrineTab[]>([]);
  const [activeTabId, setActiveTabId] = React.useState<string | null>(null);
  const [recent, setRecent] = React.useState<PeregrineLaunchRecord[]>(() => {
    try { return JSON.parse(localStorage.getItem(RECENTS_KEY) || "[]"); } catch { return []; }
  });
  const activeTabIdRef = React.useRef<string | null>(null);
  const tabsRef = React.useRef<PeregrineTab[]>([]);

  React.useEffect(() => { activeTabIdRef.current = activeTabId; }, [activeTabId]);
  React.useEffect(() => { tabsRef.current = tabs; }, [tabs]);

  const applySnapshot = React.useCallback((snapshot: { activeTabId: string | null; tabs: PeregrineTab[] }) => {
    const nextTabs = snapshot.tabs || [];
    setTabs(nextTabs);
    setActiveTabId(snapshot.activeTabId);
    const active = nextTabs.find((tab) => tab.id === snapshot.activeTabId) || nextTabs[0] || null;
    if (active) setValue(isHomeUrl(active.url) ? "" : active.url || "");
  }, []);

  const refreshStatus = React.useCallback(async () => {
    if (!isElectron) return;
    try {
      applySnapshot(await window.electronAPI!.browser.getSnapshot());
    } catch {}
  }, [applySnapshot]);

  React.useEffect(() => {
    if (!isElectron) return;
    const cleanupSnapshot = window.electronAPI!.browser.onSnapshot(applySnapshot);
    void refreshStatus();
    return cleanupSnapshot;
  }, [applySnapshot, refreshStatus]);

  React.useEffect(() => {
    return () => {
      if (!isElectron) return;
      window.electronAPI!.browser.destroyAll().catch(() => {});
    };
  }, []);

  const rememberLaunch = React.useCallback((tab: PeregrineTab) => {
    if (tab.private || !isHistoryUrl(tab.url)) return;
    setRecent((prev) => {
      const next = [
        { id: `launch-${Date.now()}`, title: tab.title || tab.url, url: tab.url, openedAt: new Date().toISOString(), pid: null },
        ...prev.filter((item) => item.url !== tab.url),
      ].slice(0, 8);
      try { localStorage.setItem(RECENTS_KEY, JSON.stringify(next)); } catch {}
      return next;
    });
  }, []);

  const launch = React.useCallback(async (raw: string, privateMode = false) => {
    if (!isElectron) return;
    try {
      const tab = await window.electronAPI!.browser.createTab({ url: raw || "about:blank", private: privateMode, activate: true });
      rememberLaunch(tab as PeregrineTab);
    } catch {}
  }, [rememberLaunch]);

  const navigate = React.useCallback((url: string) => {
    setValue(url);
    const tabId = activeTabIdRef.current;
    if (tabId && isElectron) {
      window.electronAPI!.browser.navigate(tabId, url).catch(() => {});
      return;
    }
    void launch(url);
  }, [launch]);

  const command = React.useCallback((name: Parameters<NonNullable<typeof window.electronAPI>["browser"]["command"]>[1], payload?: unknown) => {
    const tabId = activeTabIdRef.current;
    if (!tabId || !isElectron) return;
    window.electronAPI!.browser.command(tabId, name, payload).catch(() => {});
  }, []);

  const closeTab = React.useCallback(async (tabId: string) => {
    if (!isElectron) return;
    await window.electronAPI!.browser.closeTab(tabId).catch(() => {});
  }, []);

  const selectTab = React.useCallback((tabId: string) => {
    const tab = tabsRef.current.find((item) => item.id === tabId);
    if (tab) setValue(isHomeUrl(tab.url) ? "" : tab.url);
    if (isElectron) window.electronAPI!.browser.activateTab(tabId).catch(() => {});
  }, []);

  const createPrivateTab = React.useCallback(async (url: string) => {
    return launch(url || "about:blank", true);
  }, [launch]);

  const setTabZoom = React.useCallback(async (tabId: string, factor: number) => {
    if (!isElectron) return;
    await window.electronAPI!.browser.setZoom(tabId, factor).catch(() => {});
  }, []);

  const clearData = React.useCallback(async (scope: "active" | "all" = "all") => {
    if (!isElectron) return;
    await window.electronAPI!.browser.clearData({
      scope,
      data: ["cache", "cookies", "storage", "serviceWorkers", "history"],
    }).catch(() => {});
    if (scope === "all") {
      try { localStorage.removeItem(RECENTS_KEY); } catch {}
      setRecent([]);
    }
  }, []);

  const setBounds = React.useCallback((tabId: string, bounds: { x: number; y: number; width: number; height: number }) => {
    if (!isElectron) return;
    window.electronAPI!.browser.setBounds(tabId, bounds).catch(() => {});
  }, []);

  return {
    value,
    setValue,
    loading: Boolean(tabs.find((tab) => tab.id === activeTabId)?.loading),
    recent,
    tabs,
    activeTabId,
    quickLinks: QUICK_LINKS,
    refreshStatus,
    launch,
    navigate,
    goBack: () => command("back"),
    goForward: () => command("forward"),
    reload: () => command("reload"),
    stop: () => command("stop"),
    closeTab,
    selectTab,
    clearData,
    createPrivateTab,
    setTabZoom,
    setBounds,
  };
}
