import React, { useCallback, useEffect, useRef, useState } from "react";
import { appStoreKernel, type InstalledApp } from "../../../services/kernel/appStore";
import { wineService } from "../../wine/WineService";
import { openInstalledWebApp } from "../../../os/apps/installedWebApp";
import { appLoader } from "../../../os/apps/appLoader";
import { SamarisIcon, resolveAppIconName } from "../../../modules/icons";
import type { SamarisIconName } from "../../../modules/icons/types";
import { useAirBar } from "../useAirBar";

const SORT_KEY = "samaris-airbar/app-sort";

function loadSort(): string[] {
  try { return JSON.parse(localStorage.getItem(SORT_KEY) || "[]"); } catch { return []; }
}
function saveSort(order: string[]) {
  localStorage.setItem(SORT_KEY, JSON.stringify(order));
}

const SYSTEM_APPS = appLoader.listApps().filter((app) => !app.hiddenFromDock && app.id !== "about");
const GAME_APPS = appLoader.listApps().filter((app) => ["doom"].includes(app.id));

const APP_COLORS = [
  "#4d84ff", "#f472b6", "#34d399", "#fbbf24", "#a78bfa",
  "#fb923c", "#2dd4bf", "#e879f9", "#22d3ee", "#f87171"
];

function colorForId(id: string) {
  let hash = 0;
  for (let i = 0; i < id.length; i++) hash = id.charCodeAt(i) + ((hash << 5) - hash);
  return APP_COLORS[Math.abs(hash) % APP_COLORS.length];
}

function InstalledAppIcon({ name, appId, size = 20 }: { name: string; appId: string; size?: number }) {
  const bg = colorForId(appId);
  const letter = (name || appId).charAt(0).toUpperCase();
  const fontSize = Math.round(size * 0.46);
  return (
    <span
      style={{
        width: size, height: size, borderRadius: Math.round(size * 0.28),
        background: `linear-gradient(135deg, ${bg}, ${bg}cc)`,
        color: "#fff", fontWeight: 800, fontSize,
        display: "inline-flex", alignItems: "center", justifyContent: "center",
        flexShrink: 0, lineHeight: 1
      }}
    >{letter}</span>
  );
}

function useSortableApps(apps: typeof SYSTEM_APPS) {
  const [sorted, setSorted] = useState<typeof SYSTEM_APPS>(() => {
    const saved = loadSort();
    if (saved.length === 0) return apps;
    const map = new Map(apps.map((a) => [a.id, a]));
    const ordered: typeof SYSTEM_APPS = [];
    const seen = new Set<string>();
    for (const id of saved) {
      const app = map.get(id);
      if (app && !seen.has(id)) { ordered.push(app); seen.add(id); }
    }
    for (const app of apps) {
      if (!seen.has(app.id)) { ordered.push(app); seen.add(app.id); }
    }
    return ordered;
  });

  const persist = useCallback((next: typeof SYSTEM_APPS) => {
    setSorted(next);
    saveSort(next.map((a) => a.id));
  }, []);

  const dragItem = useRef<number | null>(null);
  const dragOverItem = useRef<number | null>(null);

  const onDragStart = (index: number) => () => { dragItem.current = index; };
  const onDragOver = (index: number) => (e: React.DragEvent) => { e.preventDefault(); dragOverItem.current = index; };
  const onDrop = () => {
    if (dragItem.current == null || dragOverItem.current == null) return;
    const copy = [...sorted];
    const [removed] = copy.splice(dragItem.current, 1);
    copy.splice(dragOverItem.current, 0, removed);
    dragItem.current = null;
    dragOverItem.current = null;
    persist(copy);
  };

  return { sorted, onDragStart, onDragOver, onDrop };
}

export const AppsPanel = React.memo(function AppsPanel() {
  const air = useAirBar();
  const open = air.activePanel === "apps";
  const [storeApps, setStoreApps] = useState<InstalledApp[]>([]);
  const [wineApps, setWineApps] = useState<string[]>([]);
  const { sorted, onDragStart, onDragOver, onDrop } = useSortableApps(SYSTEM_APPS);

  useEffect(() => {
    if (!open) return;
    void Promise.allSettled([appStoreKernel.listInstalled(), wineService.getStatus()]).then(([a, w]) => {
      setStoreApps(a.status === "fulfilled" ? a.value : []);
      setWineApps(w.status === "fulfilled" ? w.value.recentExecutables : []);
    });
  }, [open]);

  const style = air.getPanelStyle("apps", { width: 884, align: "start", maxWidth: 936 });

  return (
    <section style={style} className={`airbar-panel apps-panel apps-panel--full ${open ? "open" : ""}`} role="dialog" aria-label="Apps">
      <div className="app-section"><div className="app-section__title">Native Apps</div>
        <div className="app-grid app-grid--system">
          {sorted.map((app, i) => (
            <button
              key={app.id}
              type="button"
              className="app-tile app-tile--draggable"
              draggable
              onDragStart={onDragStart(i)}
              onDragOver={onDragOver(i)}
              onDrop={onDrop}
              onClick={() => { air.closePanels(); void appLoader.openApp(app.id); }}
            >
              <div className="app-tile__icon">
                <SamarisIcon name={resolveAppIconName(app.id)} size={24} variant="soft" />
              </div>
              <span className="app-tile__name">{app.name}</span>
              {app.subtitle && <small className="app-tile__meta">{app.subtitle}</small>}
            </button>
          ))}
        </div>
      </div>
      {GAME_APPS.length > 0 ? <div className="app-section__sep" /> : null}
      {GAME_APPS.length > 0 ? (
        <div className="app-section"><div className="app-section__title">Games</div>
          <div className="app-grid app-grid--system">
            {GAME_APPS.map((app) => (
              <button key={app.id} type="button" className="app-tile" onClick={() => { air.closePanels(); void appLoader.openApp(app.id); }}>
                <div className="app-tile__icon"><SamarisIcon name={resolveAppIconName(app.id)} size={24} variant="soft" /></div>
                <span className="app-tile__name">{app.name}</span>
                {app.subtitle && <small className="app-tile__meta">{app.subtitle}</small>}
              </button>
            ))}
          </div>
        </div>
      ) : null}
      {(storeApps.length > 0 || wineApps.length > 0) ? <div className="app-section__sep" /> : null}
      {storeApps.length > 0 ? (
        <div className="app-section"><div className="app-section__title">AppStore Apps</div>
          <div className="app-grid">
            {storeApps.map((e) => {
              const name = e.manifest?.displayName || e.repoName || e.appId;
              return (
                <button key={`st:${e.appId}`} type="button" className="app-tile" disabled={!e.launchable} onClick={() => { air.closePanels(); void openInstalledWebApp(e); }}>
                  <div className="app-tile__icon"><InstalledAppIcon name={name} appId={e.appId} size={24} /></div>
                  <span className="app-tile__name">{name}</span>
                  <small className="app-tile__meta">{e.launchable ? "App" : "Error"}</small>
                </button>
              );
            })}
          </div>
        </div>
      ) : null}
      {wineApps.length > 0 ? (
        <div className="app-section"><div className="app-section__title">Wine Apps</div>
          <div className="app-grid">
            {wineApps.map((entry) => {
              const name = entry.split("/").pop()?.replace(/\.exe$/i, "") || "App";
              return (
                <button key={`wi:${entry}`} type="button" className="app-tile" onClick={() => { air.closePanels(); void wineService.launchExe(entry, {}); }}>
                  <div className="app-tile__icon"><InstalledAppIcon name={name} appId={`wine:${name}`} size={24} /></div>
                  <span className="app-tile__name">{name}</span>
                  <small className="app-tile__meta">Wine</small>
                </button>
              );
            })}
          </div>
        </div>
      ) : null}
      {sorted.length === 0 && storeApps.length === 0 && wineApps.length === 0 && GAME_APPS.length === 0 ? <div className="app-empty">No apps available.</div> : null}
    </section>
  );
});
