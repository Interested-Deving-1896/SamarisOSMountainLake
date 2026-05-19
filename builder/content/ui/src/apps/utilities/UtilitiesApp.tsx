import React, { useSyncExternalStore, useState, useCallback, useEffect, useRef } from "react";
import {
  Pause, Play, X, RefreshCw, Wifi, CheckCircle2,
  ArrowRight, Shield, Loader2, Search,
} from "lucide-react";
import { osStore } from "../../os/core/osStore";
import { processManager } from "../../os/core/processManager";
import { windowManager } from "../../os/core/windowManager";
import { kernelClient } from "../../os/kernel/kernelClient";
import { appLoader } from "../../os/apps/appLoader";
import { UtilitiesSidebar } from "./components/UtilitiesSidebar";
import { MiniChart } from "./components/MiniChart";
import { useDiskUsage } from "./hooks/useDiskUsage";
import { useMonitorSeries } from "./hooks/useMonitorSeries";

let _kernelOnline = false;
const _onlineListeners = new Set<() => void>();
kernelClient.on("connected", () => { _kernelOnline = true; for (const l of _onlineListeners) l(); });
kernelClient.on("disconnected", () => { _kernelOnline = false; for (const l of _onlineListeners) l(); });
function subscribeToOnline(cb: () => void) { _onlineListeners.add(cb); return () => _onlineListeners.delete(cb); }
function getOnlineSnapshot() { return _kernelOnline; }
function useKernelConnected() { return useSyncExternalStore(subscribeToOnline, getOnlineSnapshot, getOnlineSnapshot); }

function formatBytes(value: number) {
  if (value < 1024) return `${value} B`;
  if (value < 1024 * 1024) return `${(value / 1024).toFixed(1)} KB`;
  return `${(value / (1024 * 1024)).toFixed(1)} MB`;
}

const PANEL_IDS = ["monitor", "processes", "storage", "network", "security", "updates", "print"] as const;

export function UtilitiesApp(_props: { windowId: string }) {
  const osState = useSyncExternalStore((l) => osStore.subscribe(l), () => osStore.getState());
  const [activePanel, setActivePanel] = useState<string>("monitor");
  const monitor = useMonitorSeries();
  const disk = useDiskUsage();

  // ── Storage real disk info ──
  const [diskInfo, setDiskInfo] = useState<{ filesystem: string; size: string; used: string; avail: string; usePercent: string; mounted: string }[]>([]);
  // ── Network interfaces ──
  const [netInterfaces, setNetInterfaces] = useState<any[]>([]);
  // ── Printers ──
  const [printers, setPrinters] = useState<{ id: string; name: string; status: string }[]>([]);
  const [printerQueue, setPrinterQueue] = useState<any[]>([]);
  // ── External storage devices ──
  const [storageDevices, setStorageDevices] = useState<any[]>([]);
  // ── Process search/sort ──
  const [processSearch, setProcessSearch] = useState("");
  const [processSort, setProcessSort] = useState<{ by: string; dir: "asc" | "desc" }>({ by: "appId", dir: "asc" });

  useEffect(() => {
    kernelClient.request({ type: "disk.status" }).then(r => { if (Array.isArray(r.data)) setDiskInfo(r.data); }).catch(() => {});
    kernelClient.request({ type: "network.list" }).then(r => { if (Array.isArray(r.data)) setNetInterfaces(r.data); }).catch(() => {});
    kernelClient.request({ type: "print.list" }).then(r => {
      const data = r.data as { printers?: { id: string; name: string; status: string }[]; queue?: any[] } | undefined;
      if (data) {
        setPrinters(data.printers || []);
        setPrinterQueue(data.queue || []);
      }
    }).catch(() => {});
    kernelClient.request({ type: "storage.devices" }).then(r => { if (Array.isArray(r.data)) setStorageDevices(r.data); }).catch(() => {});
  }, []);

  const scrollTo = useCallback((id: string) => {
    setActivePanel(id);
    document.getElementById(`uts-panel-${id}`)?.scrollIntoView({ behavior: "smooth", block: "start" });
  }, []);

  const contentRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const content = contentRef.current;
    if (!content) return;
    const observer = new IntersectionObserver(
      (entries) => {
        for (const entry of entries) {
          if (entry.isIntersecting) {
            const id = entry.target.id.replace("uts-panel-", "");
            if (PANEL_IDS.includes(id as typeof PANEL_IDS[number])) setActivePanel(id);
            break;
          }
        }
      },
      { root: content, rootMargin: "-10% 0px -75% 0px", threshold: 0 }
    );
    for (const id of PANEL_IDS) {
      const el = document.getElementById(`uts-panel-${id}`);
      if (el) observer.observe(el);
    }
    return () => observer.disconnect();
  }, []);

  // ── Network state ──
  const [netPrefs, setNetPrefs] = useState(() => {
    try { return JSON.parse(localStorage.getItem("samaris-os/settings-prefs") || "{}"); } catch { return {}; }
  });
  const updateNet = (p: Record<string, unknown>) => {
    const next = { ...netPrefs, ...p };
    setNetPrefs(next);
    localStorage.setItem("samaris-os/settings-prefs", JSON.stringify(next));
  };

  // ── Update state ──
  const [updateChecking, setUpdateChecking] = useState(false);
  const [updateResult, setUpdateResult] = useState<{ checked: boolean; error?: string; upToDate?: boolean }>({ checked: false });

  const handleCheckUpdates = async () => {
    setUpdateChecking(true);
    try {
      const result = await kernelClient.request<{ latestVersion?: string; upToDate?: boolean }>(
        { type: "system.checkUpdate", data: {} }, { timeoutMs: 10000 }
      );
      const upToDate = result.data?.upToDate ?? true;
      setUpdateResult({ checked: true, upToDate, error: undefined });
    } catch {
      setUpdateResult({ checked: true, error: "No update server available" });
    } finally { setUpdateChecking(false); }
  };

  // ── Cache clearing ──
  const [cacheState, setCacheState] = useState<Record<string, string>>({});

  const clearCache = async (name: string) => {
    setCacheState((p) => ({ ...p, [name]: "clearing" }));
    try {
      if (name === "browser" && window.electronAPI?.browser.clearData) {
        await window.electronAPI.browser.clearData({ scope: "all", data: ["cache", "cookies", "storage", "serviceWorkers"] });
      } else if (name === "appstore") {
        await kernelClient.request({ type: "fs.delete", data: { path: "/User/.volt/appstore", recursive: true } });
      } else if (name === "temp") {
        await kernelClient.request({ type: "fs.delete", data: { path: "/User/.volt/tmp", recursive: true } });
      }
      setCacheState((p) => ({ ...p, [name]: "cleared" }));
      setTimeout(() => setCacheState((p) => ({ ...p, [name]: "" })), 3000);
    } catch {
      setCacheState((p) => ({ ...p, [name]: "error" }));
    }
  };

  const isOnline = useKernelConnected();

  return (
    <div className="utilities">
      <div className="utilities__hero">
        <div className="utilities__heroBadge">Utilities</div>
        <h1>System Management</h1>
        <p>Diagnostics, storage, network, and security tools for Samaris OS.</p>
      </div>
      <div className="utilities__layout">
        <UtilitiesSidebar active={activePanel} onSelect={scrollTo} />
        <div className="utilities__content" ref={contentRef}>

          {/* ── 1. System Monitor ── */}
          <div id="uts-panel-monitor" className="utilities__panel">
            <div className="utilities__panelHead">
              <div className="utilities__panelTitle">System Monitor</div>
              <div className="utilities__panelSubtitle">Live view of the local Samaris session.</div>
            </div>
            <div className="utilities__panelBody">
              {!monitor.ready ? (
                <div className="utilities__empty"><Loader2 size={16} className="store__spin" /> Gathering metrics...</div>
              ) : (
                <>
                  <div className="uts-monitorGrid">
                    <div className="uts-monitorCard">
                      <div className="uts-monitorLabel">CPU</div>
                      <div className="uts-monitorValue">{monitor.snapshot.cpu.toFixed(1)}%</div>
                      <MiniChart values={monitor.cpuSeries} stroke="var(--volt-accent, #2563eb)" />
                    </div>
                    <div className="uts-monitorCard">
                      <div className="uts-monitorLabel">Memory</div>
                      <div className="uts-monitorValue">{monitor.snapshot.memory.toFixed(1)} MB</div>
                      <MiniChart values={monitor.memorySeries} stroke="#ef4444" />
                    </div>
                    <div className="uts-monitorCard">
                      <div className="uts-monitorLabel">Kernel</div>
                      <div className="uts-monitorValue" style={{ fontSize: 16 }}>{isOnline ? "Connected" : "Offline"}</div>
                      <div className="uts-monitorSub">ws://localhost:9999</div>
                    </div>
                  </div>
                  <div className="uts-monitorGrid" style={{ gridTemplateColumns: "repeat(3, minmax(0, 1fr))", marginTop: 0 }}>
                    <div className="uts-monitorCard"><div className="uts-monitorLabel">Processes</div><div className="uts-monitorValue">{monitor.snapshot.processes}</div></div>
                    <div className="uts-monitorCard"><div className="uts-monitorLabel">Windows</div><div className="uts-monitorValue">{monitor.snapshot.windows}</div></div>
                    <div className="uts-monitorCard"><div className="uts-monitorLabel">Runtimes</div><div className="uts-monitorValue">{monitor.snapshot.runtimes}</div></div>
                  </div>
                </>
              )}
            </div>
          </div>

          {/* ── 2. Processes ── */}
          <div id="uts-panel-processes" className="utilities__panel">
            <div className="utilities__panelHead">
              <div className="utilities__panelTitle">Processes</div>
              <div className="utilities__panelSubtitle">All running processes in this session.</div>
            </div>
            <div className="utilities__panelBody">
              {osState.processes.length === 0 ? (
                <div className="utilities__empty">No tracked processes right now.</div>
              ) : (
                <>
                  <div style={{ display: "flex", gap: 8, marginBottom: 10 }}>
                    <div style={{ position: "relative", flex: 1 }}>
                      <Search size={13} style={{ position: "absolute", left: 10, top: "50%", transform: "translateY(-50%)", color: "#9ca3af" }} />
                      <input className="uts-input" style={{ width: "100%", paddingLeft: 30 }} placeholder="Search processes..." value={processSearch} onChange={(e) => setProcessSearch(e.target.value)} />
                    </div>
                  </div>
                  <div className="uts-processTable">
                    <div className="uts-processHead">
                      {["appId", "pid", "cpu", "memory", "state"].map((col) => (
                        <span key={col} className="uts-processSortable" onClick={() => setProcessSort((prev) => ({ by: col, dir: prev.by === col && prev.dir === "asc" ? "desc" : "asc" }))} style={{ cursor: "pointer" }}>
                          {col === "appId" ? "App" : col === "pid" ? "PID" : col === "cpu" ? "CPU" : col === "memory" ? "Memory" : "State"}
                          {processSort.by === col ? (processSort.dir === "asc" ? " ▲" : " ▼") : ""}
                        </span>
                      ))}
                      <span>Actions</span>
                    </div>
                    {osState.processes
                      .filter((proc) => !processSearch || proc.appId.toLowerCase().includes(processSearch.toLowerCase()) || String(proc.pid).includes(processSearch))
                      .sort((a, b) => {
                        const dir = processSort.dir === "asc" ? 1 : -1;
                        const aVal = a[processSort.by as keyof typeof a];
                        const bVal = b[processSort.by as keyof typeof b];
                        if (typeof aVal === "string" && typeof bVal === "string") return aVal.localeCompare(bVal) * dir;
                        return ((aVal as number) - (bVal as number)) * dir;
                      })
                      .map((proc) => (
                      <div key={proc.pid} className="uts-processRow">
                        <span>{proc.appId}</span><span>{proc.pid}</span>
                        <span>{proc.cpu.toFixed(1)}%</span><span>{proc.memory.toFixed(1)} MB</span>
                        <span>{proc.state}</span>
                        <div className="uts-processActions">
                          {proc.windowId ? <button className="uts-btn" onClick={() => windowManager.focus(proc.windowId!)}>Focus</button> : null}
                          <button className="uts-btn uts-btn--icon" onClick={() => proc.state === "paused" ? processManager.resumeProcess(proc.pid) : processManager.pauseProcess(proc.pid)} title={proc.state === "paused" ? "Resume" : "Pause"}>
                            {proc.state === "paused" ? <Play size={13} /> : <Pause size={13} />}
                          </button>
                          <button className="uts-btn uts-btn--icon uts-btn--danger" onClick={() => processManager.killProcess(proc.pid)} title="Kill"><X size={13} /></button>
                        </div>
                      </div>
                    ))}
                  </div>
                </>
              )}
            </div>
          </div>

          {/* ── 3. Storage ── */}
          <div id="uts-panel-storage" className="utilities__panel">
            <div className="utilities__panelHead">
              <div className="utilities__panelTitle">Storage</div>
              <div className="utilities__panelSubtitle">Review and manage disk usage.</div>
            </div>
            <div className="utilities__panelBody">
              {diskInfo.length > 0 ? (
                <div className="uts-monitorCard" style={{ marginBottom: 14 }}>
                  <div className="uts-monitorLabel">Disk — {diskInfo[0].mounted}</div>
                  <div className="uts-monitorValue">{diskInfo[0].used} / {diskInfo[0].size}</div>
                  <div className="uts-storageBar">
                    <div className="uts-storageBarFill" style={{ width: diskInfo[0].usePercent }} />
                  </div>
                  <div className="uts-monitorSub">{diskInfo[0].usePercent} used &middot; {diskInfo[0].avail} free</div>
                </div>
              ) : null}
              <div className="uts-monitorCard" style={{ marginBottom: 14 }}>
                <div className="uts-monitorLabel">Total user data</div>
                <div className="uts-monitorValue">{formatBytes(disk.totalSize)}</div>
              </div>
              {disk.loading ? <div className="utilities__empty">Scanning workspace...</div> : (
                <div style={{ display: "grid", gap: 6, marginBottom: 14 }}>
                  {disk.nodes.map((n) => (
                    <div key={n.path} className="uts-diskRow">
                      <div className="uts-diskMain">
                        <div className="uts-diskTitle">{n.path}</div>
                        <div className="uts-diskMeta">{n.files} files &middot; {n.directories} dirs</div>
                      </div>
                      <div className="uts-diskSize">{formatBytes(n.size)}</div>
                    </div>
                  ))}
                </div>
              )}
              {storageDevices.length > 0 ? (
                <div style={{ marginBottom: 14 }}>
                  <div className="uts-monitorLabel" style={{ marginBottom: 6 }}>External Devices</div>
                  <div style={{ display: "grid", gap: 6 }}>
                    {storageDevices.map((dev: any, i: number) => (
                      <div key={i} className="uts-diskRow">
                        <div className="uts-diskMain">
                          <div className="uts-diskTitle">{dev.name || dev.device || `Device ${i + 1}`}</div>
                          <div className="uts-diskMeta">{dev.size || ""} {dev.fstype ? `(${dev.fstype})` : ""}</div>
                        </div>
                      </div>
                    ))}
                  </div>
                </div>
              ) : null}
              <div style={{ display: "grid", gap: 6 }}>
                {[
                  { key: "browser", label: "Peregrine cache", hint: "Cache, cookies, storage" },
                  { key: "appstore", label: "App Store repos", hint: "Cloned git repositories" },
                  { key: "temp", label: "Temp files", hint: "System temporary files" },
                ].map((item) => (
                  <div key={item.key} className="uts-diskRow">
                    <div className="uts-diskMain">
                      <div className="uts-diskTitle">{item.label}</div>
                      <div className="uts-diskMeta">{item.hint}</div>
                    </div>
                    <button className="uts-btn" disabled={cacheState[item.key] === "clearing" || cacheState[item.key] === "cleared"} onClick={() => clearCache(item.key)}>
                      {cacheState[item.key] === "clearing" ? <Loader2 size={12} className="store__spin" /> : cacheState[item.key] === "cleared" ? "Cleared" : "Clear"}
                    </button>
                  </div>
                ))}
              </div>
              <div style={{ marginTop: 10 }}>
                <button className="uts-btn" onClick={disk.refresh}><RefreshCw size={12} /> Refresh</button>
              </div>
            </div>
          </div>

          {/* ── 4. Network ── */}
          <div id="uts-panel-network" className="utilities__panel">
            <div className="utilities__panelHead">
              <div className="utilities__panelTitle">Network</div>
              <div className="utilities__panelSubtitle">Interfaces and connectivity.</div>
            </div>
            <div className="utilities__panelBody">
              <div className="uts-statusCard" style={{ marginBottom: 14 }}>
                <div className="uts-statusCardIcon"><Wifi size={20} /></div>
                <div className="uts-statusCardInfo">
                  <div className="uts-statusCardLabel">{isOnline ? "Connected" : "Offline"}</div>
                  <div className="uts-statusCardValue">Kernel bridge active</div>
                </div>
                <span style={{ fontSize: 11, fontWeight: 700, color: isOnline ? "#059669" : "#dc2626", padding: "4px 10px", borderRadius: 999, background: isOnline ? "rgba(5,150,105,0.1)" : "rgba(220,38,38,0.1)" }}>{isOnline ? "Online" : "Offline"}</span>
              </div>
              {netInterfaces.length > 0 ? (
                <div style={{ marginBottom: 14 }}>
                  <div className="uts-monitorLabel" style={{ marginBottom: 6 }}>Interfaces</div>
                  {netInterfaces.map((iface: any) => (
                    <div key={iface.id} className="uts-diskRow" style={{ marginBottom: 4 }}>
                      <div className="uts-diskMain">
                        <div className="uts-diskTitle">{iface.label || iface.name}</div>
                        <div className="uts-diskMeta">{iface.type} &middot; {iface.address || "unaddressed"}{iface.connected ? " &middot; Connected" : ""}</div>
                      </div>
                      <span style={{ fontSize: 11, color: iface.connected ? "#059669" : "#6b7d90" }}>{iface.connected ? "Up" : "Down"}</span>
                    </div>
                  ))}
                </div>
              ) : null}
              {[
                { label: "Wi-Fi", key: "wifiEnabled" },
                { label: "VPN", key: "vpnEnabled" },
                { label: "Proxy", key: "proxyEnabled" },
              ].map((item) => (
                <div key={item.key} className="uts-row">
                  <div className="uts-rowLabel">{item.label}</div>
                  <button className={`uts-toggle ${netPrefs[item.key] ? "uts-toggle--on" : ""}`} onClick={() => {
                    const next = !netPrefs[item.key];
                    updateNet({ [item.key]: next });
                    kernelClient.request({ type: "network.setConfig", data: { interfaceId: item.key.replace("Enabled", ""), enabled: next, id: item.key } }).catch(() => {});
                  }}>
                    <span className="uts-toggleKnob" />
                  </button>
                </div>
              ))}
              <div className="uts-row"><span className="uts-rowLabel">DNS servers</span><input className="uts-input" value={netPrefs.dnsServers || ""} placeholder="1.1.1.1, 8.8.8.8" onChange={(e) => {
                updateNet({ dnsServers: e.target.value });
                kernelClient.request({ type: "network.setConfig", data: { dnsPrimary: e.target.value.split(",")[0]?.trim() || "", dnsSecondary: e.target.value.split(",")[1]?.trim() || "" } }).catch(() => {});
              }} /></div>
              <div className="uts-row"><span className="uts-rowLabel">Proxy URL</span><input className="uts-input" value={netPrefs.proxyUrl || ""} placeholder="http://proxy:8080" onChange={(e) => updateNet({ proxyUrl: e.target.value })} /></div>
              <div className="uts-row" style={{ marginTop: 10 }}>
                <div>
                  <div className="uts-rowLabel">Firewall</div>
                  <div className="uts-rowHint">Open the Firewall app for advanced rules.</div>
                </div>
                <button className="uts-btn" onClick={() => void appLoader.openApp("firewall")}><ArrowRight size={13} /> Open</button>
              </div>
            </div>
          </div>

          {/* ── 5. Security ── */}
          <div id="uts-panel-security" className="utilities__panel">
            <div className="utilities__panelHead">
              <div className="utilities__panelTitle">Security</div>
              <div className="utilities__panelSubtitle">App permissions and system hardening.</div>
            </div>
            <div className="utilities__panelBody">
              <div className="uts-row" style={{ marginTop: 0 }}>
                <div>
                  <div className="uts-rowLabel">Permissions Manager</div>
                  <div className="uts-rowHint">Review and revoke app permissions.</div>
                </div>
                <button className="uts-btn uts-btn--primary" onClick={() => void appLoader.openApp("permissions-manager")}><Shield size={13} /> Open</button>
              </div>
              <div className="uts-row">
                <div>
                  <div className="uts-rowLabel">Firewall</div>
                  <div className="uts-rowHint">Inbound and outbound policies.</div>
                </div>
                <button className="uts-btn uts-btn--primary" onClick={() => void appLoader.openApp("firewall")}><Shield size={13} /> Open</button>
              </div>
              <div className="uts-row">
                <div>
                  <div className="uts-rowLabel">Encryption</div>
                  <div className="uts-rowHint">Check LUKS status and recovery.</div>
                </div>
                <button className="uts-btn uts-btn--primary" onClick={() => void appLoader.openApp("encryption")}><Shield size={13} /> Open</button>
              </div>
            </div>
          </div>

          {/* ── 6. Software Update ── */}
          <div id="uts-panel-updates" className="utilities__panel">
            <div className="utilities__panelHead">
              <div className="utilities__panelTitle">Software Update</div>
              <div className="utilities__panelSubtitle">Keep Samaris OS current.</div>
            </div>
            <div className="utilities__panelBody">
              <div className="uts-statusCard" style={{ marginBottom: 14 }}>
                <div className="uts-statusCardIcon"><CheckCircle2 size={20} /></div>
                <div className="uts-statusCardInfo">
                  <div className="uts-statusCardLabel">Samaris OS</div>
                  <div className="uts-statusCardValue">v1.0.0-alpha — Core{updateResult.checked ? (updateResult.upToDate ? " — Up to date" : "") : ""}</div>
                </div>
                <button className="uts-btn uts-btn--primary" disabled={updateChecking} onClick={() => void handleCheckUpdates()}>
                  {updateChecking ? <Loader2 size={12} className="store__spin" /> : "Check for Updates"}
                </button>
              </div>
              {updateResult.error ? <div style={{ fontSize: 12, color: "#dc2626", marginBottom: 10 }}>{updateResult.error}</div> : null}
              <div className="uts-row">
                <div className="uts-rowLabel">Auto-update OS</div>
                <button className={`uts-toggle ${netPrefs.autoUpdateOS !== false ? "uts-toggle--on" : ""}`} onClick={() => updateNet({ autoUpdateOS: !(netPrefs.autoUpdateOS !== false) })}>
                  <span className="uts-toggleKnob" />
                </button>
              </div>
            </div>
          </div>

          {/* ── 7. Print ── */}
          <div id="uts-panel-print" className="utilities__panel">
            <div className="utilities__panelHead">
              <div className="utilities__panelTitle">Print</div>
              <div className="utilities__panelSubtitle">Printers and print queue.</div>
            </div>
            <div className="utilities__panelBody">
              {printers.length === 0 ? (
                <div className="utilities__empty">No printers detected.<br />Print support requires CUPS or a network print server accessible from the kernel.</div>
              ) : (
                <div style={{ display: "grid", gap: 6, marginBottom: 14 }}>
                  {printers.map((printer) => (
                    <div key={printer.id} className="uts-diskRow">
                      <div className="uts-diskMain">
                        <div className="uts-diskTitle">{printer.name}</div>
                        <div className="uts-diskMeta">Status: {printer.status}</div>
                      </div>
                      <span style={{ fontSize: 11, color: printer.status === "ready" ? "#059669" : "#dc2626" }}>{printer.status}</span>
                    </div>
                  ))}
                </div>
              )}
              {printerQueue.length > 0 ? (
                <div>
                  <div className="uts-monitorLabel" style={{ marginBottom: 6 }}>Print Queue</div>
                  <div style={{ display: "grid", gap: 4 }}>
                    {printerQueue.map((job: any, i: number) => (
                      <div key={i} className="uts-diskRow">
                        <div className="uts-diskTitle" style={{ fontSize: 12 }}>{job.jobId}</div>
                        <div className="uts-diskMeta">{job.summary}</div>
                      </div>
                    ))}
                  </div>
                </div>
              ) : null}
            </div>
          </div>

        </div>
      </div>
    </div>
  );
}
