import React, { useCallback, useEffect, useLayoutEffect, useMemo, useRef, useState, useSyncExternalStore } from "react";
import "./settings/settings.css";
import { Wifi, CheckCircle2, Shield, HardDrive, Download } from "lucide-react";
import { osStore } from "../os/core/osStore";
import { eventBus } from "../os/kernel/eventBus";
import { kernelClient } from "../os/kernel/kernelClient";
import { sessionPersistence } from "../system/session/sessionPersistence";
import { securityStore } from "../system/session/securityStore";
import { themeStore } from "../system/theme/themeStore";
import { wallpaperStore } from "../system/wallpaper/wallpaperStore";
import { connectivityStore } from "../system/connectivity/connectivityStore";
import { SettingsSection } from "./settings/components/SettingsSection";
import { SettingsToggle } from "./settings/components/SettingsToggle";
import { SettingsSidebar } from "./settings/components/SettingsSidebar";
import { SettingsSlider } from "./settings/components/SettingsSlider";
import { SettingsColorPicker } from "./settings/components/SettingsColorPicker";
import { SettingsDropdown } from "./settings/components/SettingsDropdown";
import { userKernel, type SamarisUser } from "../services/kernel/user";
import { firewallKernel, type FirewallState } from "../services/kernel/firewall";
import { storageKernel, type StorageDevice } from "../services/kernel/storage";
import { DND_FEATURE_FLAG } from "../os/dnd/constants";

const LS_KEY = "samaris-os/settings-prefs";

type SettingsPrefs = {
  accentColor: string; fontSize: number; glassOpacity: number;
  animations: boolean; reduceMotion: boolean;
  dockPosition: "bottom" | "left" | "right"; dockSize: number;
  dockMagnify: boolean; dockMagnifyScale: number; dockAutoHide: boolean;
  menuBar: boolean; desktopIcons: boolean;
  wallpaperFit: "fill" | "fit" | "stretch" | "center";
  lockScreen: boolean; lockWallpaper: boolean; loginMessage: string;
  autoLogin: boolean;
  wifiEnabled: boolean; vpnEnabled: boolean; proxyEnabled: boolean;
  proxyUrl: string; dnsServers: string; firewallEnabled: boolean;
  cameraAccess: boolean; micAccess: boolean; locationServices: boolean;
  analytics: boolean; autoUpdateSecurity: boolean;
  doNotDisturb: boolean; dndStart: string; dndEnd: string; showNotifOnLock: boolean;
  highContrast: boolean; boldText: boolean; screenReader: boolean;
  stickyKeys: boolean; pointerSize: number;
  language: string; dateFormat: string; timeFormat: string;
  firstDayOfWeek: string; tempUnit: string; keyboardLayout: string;
  autoUpdateOS: boolean; updateChannel: string;
  devMode: boolean; featureFlags: Record<string, boolean>;
};

function loadPrefs(): SettingsPrefs {
  try { return JSON.parse(localStorage.getItem(LS_KEY) || "{}"); } catch { return {} as SettingsPrefs; }
}
function savePrefs(partial: Partial<SettingsPrefs>) {
  try {
    const current = JSON.parse(localStorage.getItem(LS_KEY) || "{}");
    localStorage.setItem(LS_KEY, JSON.stringify({ ...current, ...partial }));
  } catch {}
}

const DEFAULT_PREFS: SettingsPrefs = {
  accentColor: "#2563eb", fontSize: 14, glassOpacity: 80,
  animations: true, reduceMotion: false,
  dockPosition: "bottom", dockSize: 48, dockMagnify: false, dockMagnifyScale: 1.5, dockAutoHide: false,
  menuBar: true, desktopIcons: true,
  wallpaperFit: "fill",
  lockScreen: true, lockWallpaper: true, loginMessage: "", autoLogin: false,
  wifiEnabled: true, vpnEnabled: false, proxyEnabled: false,
  proxyUrl: "", dnsServers: "", firewallEnabled: false,
  cameraAccess: true, micAccess: true, locationServices: false,
  analytics: false, autoUpdateSecurity: true,
  doNotDisturb: false, dndStart: "22:00", dndEnd: "07:00", showNotifOnLock: false,
  highContrast: false, boldText: false, screenReader: false, stickyKeys: false, pointerSize: 1,
  language: "en", dateFormat: "dd/mm/yyyy", timeFormat: "24h", firstDayOfWeek: "monday",
  tempUnit: "celsius", keyboardLayout: "us",
  autoUpdateOS: true, updateChannel: "stable",
  devMode: false, featureFlags: { [DND_FEATURE_FLAG]: true },
};

const SECTION_IDS = ["appearance","desktop","session","accounts","network","security","notifications","accessibility","storage","updates","language","developer","about"] as const;

const SECTION_LABELS: Record<string, string> = {
  appearance: "Appearance", desktop: "Desktop & Wallpaper", session: "Session & Lock",
  accounts: "Accounts", network: "Network", security: "Security & Privacy",
  notifications: "Notifications", accessibility: "Accessibility", storage: "Storage",
  updates: "Software Update", language: "Language & Region", developer: "Developer", about: "About",
};

function updateChannelLabel(channel: string): string {
  if (channel === "beta") return "Beta";
  if (channel === "dev") return "Dev";
  return "Stable";
}

export default function Settings(_props: { windowId: string }) {
  const osState = useSyncExternalStore((l) => osStore.subscribe(l), () => osStore.getState());
  const theme = useSyncExternalStore((l) => themeStore.subscribe(l), () => themeStore.getState());
  const wallpaperId = useSyncExternalStore((l) => wallpaperStore.subscribe(l), () => wallpaperStore.getState());
  const security = useSyncExternalStore((l) => securityStore.subscribe(l), () => securityStore.getState());

  const [restoreEnabled, setRestoreEnabled] = useState(sessionPersistence.restoreEnabled());
  const [kernelStatus, setKernelStatus] = useState<"online" | "offline">(kernelClient.connected() ? "online" : "offline");
  const [connectivity, setConnectivity] = useState(() => connectivityStore.getState());
  const [firewallState, setFirewallState] = useState<FirewallState | null>(null);
  const [storageDevices, setStorageDevices] = useState<StorageDevice[]>([]);
  const [fwError, setFwError] = useState("");
  const [users, setUsers] = useState<SamarisUser[]>([]);
  const [activeUser, setActiveUser] = useState<SamarisUser | null>(null);
  const [usersError, setUsersError] = useState("");
  const [showCreateUser, setShowCreateUser] = useState(false);
  const [newUserData, setNewUserData] = useState({ username: "", displayName: "", password: "" });
  const [createError, setCreateError] = useState("");
  const [secError, setSecError] = useState("");
  const [activeSection, setActiveSection] = useState("appearance");
  const [search, setSearch] = useState("");
  const [cacheClearing, setCacheClearing] = useState(false);
  const [cacheCleared, setCacheCleared] = useState(false);
  const [updateChecking, setUpdateChecking] = useState(false);
  const [updateChecked, setUpdateChecked] = useState(false);
  const [updateError, setUpdateError] = useState("");
  const [prefs, setPrefs] = useState<SettingsPrefs>(() => ({ ...DEFAULT_PREFS, ...loadPrefs() }));

  const updatePrefs = useCallback((partial: Partial<SettingsPrefs>) => {
    setPrefs((prev) => {
      const next = { ...prev, ...partial };
      savePrefs(next);
      return next;
    });
  }, []);

  // ── Apply prefs to DOM synchronously before paint ──
  useLayoutEffect(() => {
    const root = document.documentElement;
    root.style.setProperty("--volt-font-scale", String(prefs.fontSize));
    root.style.setProperty("--scale", String(prefs.fontSize / 14));
    root.dataset.reduceMotion = prefs.reduceMotion ? "true" : "false";
    root.dataset.highContrast = prefs.highContrast ? "true" : "false";
    root.dataset.boldText = prefs.boldText ? "true" : "false";
    root.dataset.disableAnimations = prefs.animations ? "false" : "true";
    root.dataset.pointerScale = String(prefs.pointerSize);
    root.style.setProperty("--volt-glass-opacity", String(prefs.glassOpacity / 100));
    root.style.setProperty("--glass-alpha", String(prefs.glassOpacity / 100));
    root.style.setProperty("--volt-accent", prefs.accentColor);

    const hex = prefs.accentColor.replace("#", "");
    const r = parseInt(hex.substring(0, 2), 16) / 255;
    const g = parseInt(hex.substring(2, 4), 16) / 255;
    const b = parseInt(hex.substring(4, 6), 16) / 255;
    const max = Math.max(r, g, b), min = Math.min(r, g, b);
    let h = 0, s = 0;
    const l = (max + min) / 2;
    if (max !== min) {
      const d = max - min;
      s = l > 0.5 ? d / (2 - max - min) : d / (max + min);
      if (max === r) h = ((g - b) / d + (g < b ? 6 : 0)) / 6;
      else if (max === g) h = ((b - r) / d + 2) / 6;
      else h = ((r - g) / d + 4) / 6;
    }
    root.style.setProperty("--volt-accent-h", String(Math.round(h * 360)));
    root.style.setProperty("--volt-accent-s", `${Math.round(s * 100)}%`);
    root.style.setProperty("--volt-accent-l", `${Math.round(l * 100)}%`);

    try {
      localStorage.setItem("samaris-dock/settings", JSON.stringify({
        position: prefs.dockPosition,
        size: prefs.dockSize,
        magnify: prefs.dockMagnify,
        magnifyScale: prefs.dockMagnifyScale,
        autoHide: prefs.dockAutoHide,
      }));
    } catch {}
  }, [prefs.fontSize, prefs.reduceMotion, prefs.highContrast, prefs.boldText, prefs.animations, prefs.glassOpacity, prefs.accentColor, prefs.dockPosition, prefs.dockSize, prefs.dockMagnify, prefs.dockAutoHide]);

  const windowState = osState.windows.find((e) => e.id === _props.windowId) || null;
  const sectionParam = (windowState?.params as any)?.section as string | undefined;

  const sectionRefs: Record<string, React.RefObject<HTMLElement>> = {};
  for (const id of SECTION_IDS) sectionRefs[id] = useRef<HTMLElement>(null);
  const contentRef = useRef<HTMLDivElement>(null);

  // ── Scroll spy ──
  useEffect(() => {
    const content = contentRef.current;
    if (!content) return;
    const observer = new IntersectionObserver(
      (entries) => {
        for (const entry of entries) {
          if (entry.isIntersecting) {
            const id = SECTION_IDS.find((s) => sectionRefs[s]?.current === entry.target);
            if (id) setActiveSection(id);
            break;
          }
        }
      },
      { root: content, rootMargin: "-10% 0px -75% 0px", threshold: 0 }
    );
    for (const id of SECTION_IDS) {
      const el = sectionRefs[id]?.current;
      if (el) observer.observe(el);
    }
    return () => observer.disconnect();
  }, []);

  const searchTerms = search.trim().toLowerCase().split(/\s+/).filter(Boolean);

  const showsSection = useCallback((id: string) => {
    if (searchTerms.length === 0) return true;
    const label = (SECTION_LABELS[id] || id).toLowerCase();
    return searchTerms.every((term) => label.includes(term) || id.includes(term));
  }, [searchTerms]);

  const hiddenSections = useMemo(() => {
    if (searchTerms.length === 0) return new Set<string>();
    return new Set(SECTION_IDS.filter((id) => !showsSection(id)));
  }, [searchTerms, showsSection]);

  useLayoutEffect(() => {
    const target = sectionParam || activeSection;
    const ref = sectionRefs[target];
    if (ref?.current) ref.current.scrollIntoView({ behavior: "smooth", block: "start" });
  }, [sectionParam]);

  useEffect(() => { wallpaperStore.refreshFit(); }, [prefs.wallpaperFit]);
  useEffect(() => { void refreshUsers(); }, []);
  useEffect(() => {
    const a = eventBus.on("kernel:connected", () => setKernelStatus("online"));
    const b = eventBus.on("kernel:disconnected", () => setKernelStatus("offline"));
    return () => { a(); b(); };
  }, []);
  useEffect(() => {
    const unsub = connectivityStore.subscribe(() => setConnectivity(connectivityStore.getState()));
    return () => { unsub(); };
  }, []);
  useEffect(() => {
    void firewallKernel.list().then((s) => setFirewallState(s)).catch(() => {});
  }, []);
  useEffect(() => {
    void storageKernel.status().then(() => storageKernel.devices().then(setStorageDevices).catch(() => {})).catch(() => {});
  }, []);

  const refreshUsers = async () => {
    setUsersError("");
    try { setUsers(await userKernel.list()); setActiveUser(await userKernel.active()); }
    catch (e) { setUsersError(e instanceof Error ? e.message : "Unable to load users"); }
  };
  const handleCreateUser = async () => {
    setCreateError("");
    if (!/^[a-z0-9][a-z0-9._-]{1,31}$/i.test(newUserData.username.trim())) { setCreateError("Username: 2-32 chars"); return; }
    if (newUserData.displayName.trim().length < 2) { setCreateError("Display name min 2 chars"); return; }
    if (newUserData.password.length < 4) { setCreateError("Password min 4 chars"); return; }
    try { await userKernel.create(newUserData.username.trim(), newUserData.displayName.trim(), newUserData.password); setShowCreateUser(false); setNewUserData({ username: "", displayName: "", password: "" }); await refreshUsers(); }
    catch (err) { setCreateError(err instanceof Error ? err.message : "Failed"); }
  };
  const handleDeleteUser = async (u: string) => { try { await userKernel.delete(u); await refreshUsers(); } catch (e) { setUsersError(e instanceof Error ? e.message : "Delete failed"); } };
  const handleSecuritySet = async (p: Partial<typeof security>) => { setSecError(""); try { await securityStore.set(p); } catch (e) { setSecError(e instanceof Error ? e.message : "Failed"); } };

  const handleClearCache = async () => {
    setCacheClearing(true); setCacheCleared(false);
    try {
      if (window.electronAPI?.browser.clearData) {
        await window.electronAPI.browser.clearData({ scope: "all", data: ["cache", "cookies", "storage", "serviceWorkers"] });
      }
      setCacheCleared(true);
      setTimeout(() => setCacheCleared(false), 3000);
    } catch { /* silent */ }
    finally { setCacheClearing(false); }
  };

  const handleCheckUpdates = async () => {
    setUpdateChecking(true); setUpdateChecked(false); setUpdateError("");
    try {
      const result = await kernelClient.request<{ latestVersion?: string; upToDate?: boolean }>(
        { type: "system.checkUpdate", data: {} },
        { timeoutMs: 10000 }
      );
      setUpdateChecked(true);
      if (!result.data?.upToDate && result.data?.latestVersion) {
        setUpdateError(`Update available: ${result.data.latestVersion}`);
      }
    } catch {
      setUpdateError("No update server available");
    } finally {
      setUpdateChecking(false);
    }
  };

  const selectSection = useCallback((id: string) => {
    setActiveSection(id);
    sectionRefs[id]?.current?.scrollIntoView({ behavior: "smooth", block: "start" });
  }, []);

  const isOnline = kernelStatus === "online";

  return (
    <div className="settings">
      {secError ? <div className="settings__errorBanner" onClick={() => setSecError("")}>{secError} <span>&times;</span></div> : null}
      <div className="settings__hero">
        <div className="settings__title">Settings</div>
        <div className="settings__subtitle">Shape the desktop, theme, and runtime behavior of this local Samaris OS session.</div>
      </div>
      <div className="settings__searchWrap">
        <input className="settings__search" placeholder="Search settings..." value={search} onChange={(e) => setSearch(e.target.value)} />
      </div>
      <div className="settings__layout">
        <SettingsSidebar activeSection={activeSection} onSelect={selectSection} hiddenSections={hiddenSections} />
        <div className="settings__content" ref={contentRef}>

          {/* ── 1. Appearance ── */}
          <SettingsSection ref={sectionRefs.appearance} title="Appearance" description="Choose your visual style.">
            {!showsSection("appearance") ? null : <>
              <div className="settings__row">
                <div><div className="settings__rowTitle">Theme</div><div className="settings__rowHint">Light or dark workspace.</div></div>
                <div className="settings__segmented">
                  <button className={`settings__segment ${theme === "light" ? "settings__segment--active" : ""}`} onClick={() => themeStore.setMode("light")}>Day</button>
                  <button className={`settings__segment ${theme === "dark" ? "settings__segment--active" : ""}`} onClick={() => themeStore.setMode("dark")}>Night</button>
                </div>
              </div>
              <SettingsColorPicker label="Accent color" hint="System-wide accent for buttons and highlights." value={prefs.accentColor} onChange={(c) => updatePrefs({ accentColor: c })} />
              <SettingsSlider label="Font size" hint="System font scale." min={12} max={20} value={prefs.fontSize} onChange={(v) => updatePrefs({ fontSize: v })} formatValue={(v) => `${v}px`} />
              <SettingsSlider label="Transparency" hint="Glassmorphism intensity." min={0} max={100} value={prefs.glassOpacity} onChange={(v) => updatePrefs({ glassOpacity: v })} formatValue={(v) => `${v}%`} />
              <SettingsToggle label="Animations" hint="Enable window and UI animations." checked={prefs.animations} onChange={(c) => updatePrefs({ animations: c })} />
              <SettingsToggle label="Reduce motion" hint="Minimize motion for comfort." checked={prefs.reduceMotion} onChange={(c) => updatePrefs({ reduceMotion: c })} />
            </>}
          </SettingsSection>

          {/* ── 2. Desktop & Wallpaper ── */}
          <SettingsSection ref={sectionRefs.desktop} title="Desktop &amp; Wallpaper" description="Pick a background and customize the dock.">
            {!showsSection("desktop") ? null : <>
              <div className="settings__wallpapers">
                {wallpaperStore.list().map((preset) => (
                  <button key={preset.id} type="button" className={`settings__wallpaper ${wallpaperId === preset.id ? "settings__wallpaper--active" : ""}`} onClick={() => wallpaperStore.setWallpaper(preset.id)}>
                    <span className="settings__wallpaperPreview" style={{ backgroundImage: `url(${preset.preview})`, backgroundColor: "#eef2f7" }} />
                    <span className="settings__wallpaperLabel">{preset.label}</span>
                  </button>
                ))}
              </div>
              <SettingsDropdown label="Wallpaper fit" value={prefs.wallpaperFit} options={[{ label: "Fill Screen", value: "fill" }, { label: "Fit", value: "fit" }, { label: "Stretch", value: "stretch" }, { label: "Center", value: "center" }]} onChange={(v) => updatePrefs({ wallpaperFit: v })} />
              <SettingsDropdown label="Dock position" value={prefs.dockPosition} options={[{ label: "Bottom", value: "bottom" }, { label: "Left", value: "left" }, { label: "Right", value: "right" }]} onChange={(v) => updatePrefs({ dockPosition: v })} />
              <SettingsSlider label="Dock size" min={32} max={80} value={prefs.dockSize} onChange={(v) => updatePrefs({ dockSize: v })} formatValue={(v) => `${v}px`} />
              <SettingsToggle label="Dock magnification" hint="Enlarge icons on hover." checked={prefs.dockMagnify} onChange={(c) => updatePrefs({ dockMagnify: c })} />
              <SettingsToggle label="Auto-hide dock" hint="Hide dock when not hovered." checked={prefs.dockAutoHide} onChange={(c) => updatePrefs({ dockAutoHide: c })} />
              <SettingsToggle label="Menu bar" checked={prefs.menuBar} onChange={(c) => updatePrefs({ menuBar: c })} />
              <SettingsToggle label="Desktop icons" checked={prefs.desktopIcons} onChange={(c) => updatePrefs({ desktopIcons: c })} />
            </>}
          </SettingsSection>

          {/* ── 3. Session & Lock ── */}
          <SettingsSection ref={sectionRefs.session} title="Session &amp; Lock" description="Control restore, guest mode, and auto-lock.">
            {!showsSection("session") ? null : <>
              <SettingsToggle label="Restore previous session" hint="Reopen windows on boot." checked={restoreEnabled} onChange={(c) => { sessionPersistence.setRestoreEnabled(c); setRestoreEnabled(c); }} />
              <SettingsToggle label="Guest mode" hint="Ephemeral workspace on logout." checked={security.guestMode} onChange={(c) => void handleSecuritySet({ guestMode: c })} />
              <SettingsToggle label="Lock screen" hint="Show lock screen when idle." checked={prefs.lockScreen} onChange={(c) => updatePrefs({ lockScreen: c })} />
              <div className="settings__row">
                <div><div className="settings__rowTitle">Auto-lock</div><div className="settings__rowHint">Lock after inactivity.</div></div>
                <div className="settings__segmented">
                  {[["Never", 0], ["5m", 5], ["10m", 10], ["30m", 30], ["1h", 60]].map(([l, v]) => (
                    <button key={v} className={`settings__segment ${security.lockAfterMinutes === (v as number) ? "settings__segment--active" : ""}`} onClick={() => void handleSecuritySet({ lockAfterMinutes: v as number })}>{l}</button>
                  ))}
                </div>
              </div>
              <SettingsToggle label="Show wallpaper on lock" checked={prefs.lockWallpaper} onChange={(c) => updatePrefs({ lockWallpaper: c })} />
              <div className="settings__row">
                <div><div className="settings__rowTitle">Login message</div></div>
                <input className="settings__textInput" value={prefs.loginMessage} placeholder="Optional..." onChange={(e) => updatePrefs({ loginMessage: e.target.value })} />
              </div>
            </>}
          </SettingsSection>

          {/* ── 4. Accounts ── */}
          <SettingsSection ref={sectionRefs.accounts} title="Accounts" description="Manage local users on this device.">
            {!showsSection("accounts") ? null : <>
              <div className="settings__accounts">
                {usersError ? <div className="settings__emptyUsers" style={{ color: "#dc2626" }}>{usersError}</div>
                : users.length === 0 ? <div className="settings__emptyUsers">No users yet.</div>
                : users.map((u) => (
                  <div key={u.username} className={`settings__userCard ${activeUser?.username === u.username ? "settings__userCard--active" : ""}`}>
                    <div className="settings__userAvatar">{u.displayName.charAt(0).toUpperCase()}</div>
                    <div className="settings__userInfo">
                      <div className="settings__userName">{u.displayName}</div>
                      <div className="settings__userUsername">@{u.username}{u.guest ? " (Guest)" : ""}</div>
                    </div>
                    {activeUser?.username === u.username && <div className="settings__userBadge">Active</div>}
                    {activeUser?.username !== u.username && (
                      <button className="settings__userDeleteBtn" onClick={() => { if (window.confirm(`Delete "${u.displayName}"?`)) void handleDeleteUser(u.username); }}>Delete</button>
                    )}
                  </div>
                ))}
              </div>
              <button className="settings__createUserBtn" onClick={() => setShowCreateUser(true)}>+ Create new user</button>
              <SettingsToggle label="Auto-login" hint="Skip login for current user." checked={prefs.autoLogin} onChange={(c) => updatePrefs({ autoLogin: c })} />
            </>}
          </SettingsSection>
          {/* ── 5. Network ── */}
          <SettingsSection ref={sectionRefs.network} title="Network" description="WiFi, VPN, and proxy configuration.">
            {!showsSection("network") ? null : <>
              <div className="settings__row">
                <div><div className="settings__rowTitle">WiFi</div><div className="settings__rowHint">{connectivity.wifiEnabled ? "On" : "Off"} &middot; {connectivity.currentNetworkLabel || "Not connected"}</div></div>
                <button type="button" className={`settings__switch ${connectivity.wifiEnabled ? "settings__switch--on" : ""}`} onClick={() => { void connectivityStore.toggleWifi(); }}><span className="settings__switchKnob" /></button>
              </div>
              {connectivity.capabilities.wifiToggle && connectivity.wifiEnabled && connectivity.networks.length > 0 ? (
                <div className="settings__row">
                  <div><div className="settings__rowTitle">Available networks</div></div>
                  <div style={{ display: "flex", flexDirection: "column", gap: 4, alignItems: "flex-end" }}>
                    {connectivity.networks.slice(0, 6).map((net) => (
                      <button key={net.id} type="button" className="settings__actionBtn" style={{ fontSize: 11, height: 28, padding: "0 10px" }}
                        onClick={() => {
                          if (net.connected) { void connectivityStore.disconnectNetwork(); }
                          else { void connectivityStore.connectNetwork(net.label); }
                        }}>
                        {net.label} {net.connected ? "\u2713" : ""}
                      </button>
                    ))}
                  </div>
                </div>
              ) : null}
              <SettingsToggle label="VPN" hint="Virtual private network tunnel." checked={prefs.vpnEnabled} onChange={(c) => { updatePrefs({ vpnEnabled: c }); void kernelClient.request({ type: "network.setConfig", data: { interfaceId: "vpn", enabled: c } }).catch(() => {}); }} />
              <SettingsToggle label="Proxy" hint="Route traffic through a proxy server." checked={prefs.proxyEnabled} onChange={(c) => updatePrefs({ proxyEnabled: c })} />
              {prefs.proxyEnabled && (
                <div className="settings__row">
                  <div><div className="settings__rowTitle">Proxy URL</div></div>
                  <input className="settings__textInput" value={prefs.proxyUrl} placeholder="http://proxy:8080" onChange={(e) => { updatePrefs({ proxyUrl: e.target.value }); void kernelClient.request({ type: "network.setConfig", data: { proxyUrl: e.target.value } }).catch(() => {}); }} />
                </div>
              )}
              <div className="settings__row">
                <div><div className="settings__rowTitle">DNS servers</div><div className="settings__rowHint">Comma-separated.</div></div>
                <input className="settings__textInput" value={prefs.dnsServers} placeholder="8.8.8.8, 1.1.1.1" onChange={(e) => { const v = e.target.value; updatePrefs({ dnsServers: v }); void kernelClient.request({ type: "network.setConfig", data: { dnsPrimary: v.split(",")[0]?.trim() || "", dnsSecondary: v.split(",")[1]?.trim() || "" } }).catch(() => {}); }} />
              </div>
            </>}
          </SettingsSection>

          {/* ── 6. Security & Privacy ── */}
          <SettingsSection ref={sectionRefs.security} title="Security &amp; Privacy" description="Firewall, permissions, and data policies.">
            {!showsSection("security") ? null : <>
              <div className="settings__row">
                <div><div className="settings__rowTitle">Firewall</div><div className="settings__rowHint">{firewallState?.enabled ? "Active" : "Disabled"}</div></div>
                <button type="button" className={`settings__switch ${firewallState?.enabled ? "settings__switch--on" : ""}`}
                  onClick={async () => {
                    if (!firewallState) return;
                    const nextEnabled = !firewallState.enabled;
                    setFirewallState({ ...firewallState, enabled: nextEnabled });
                    updatePrefs({ firewallEnabled: nextEnabled });
                    try {
                      const next = await firewallKernel.setEnabled(nextEnabled);
                      setFirewallState(next);
                      updatePrefs({ firewallEnabled: next.enabled });
                    } catch (e) {
                      setFirewallState({ ...firewallState, enabled: !nextEnabled });
                      updatePrefs({ firewallEnabled: !nextEnabled });
                      setFwError(e instanceof Error ? e.message : "Failed");
                    }
                  }}>
                  <span className="settings__switchKnob" />
                </button>
              </div>
              {fwError ? <div className="settings__row"><div className="settings__rowHint" style={{ color: "#dc2626" }}>{fwError}</div></div> : null}
              <SettingsToggle label="Camera access" hint="Allow apps to use the camera." checked={prefs.cameraAccess} onChange={(c) => updatePrefs({ cameraAccess: c })} />
              <SettingsToggle label="Microphone access" hint="Allow apps to use the microphone." checked={prefs.micAccess} onChange={(c) => updatePrefs({ micAccess: c })} />
              <SettingsToggle label="Location services" hint="Allow apps to determine your location." checked={prefs.locationServices} onChange={(c) => updatePrefs({ locationServices: c })} />
              <SettingsToggle label="Analytics" hint="Share anonymous usage data." checked={prefs.analytics} onChange={(c) => updatePrefs({ analytics: c })} />
              <SettingsToggle label="Auto-update security" hint="Apply security patches automatically." checked={prefs.autoUpdateSecurity} onChange={(c) => updatePrefs({ autoUpdateSecurity: c })} />
            </>}
          </SettingsSection>

          {/* ── 7. Notifications ── */}
          <SettingsSection ref={sectionRefs.notifications} title="Notifications" description="Control alerts and focus time.">
            {!showsSection("notifications") ? null : <>
              <SettingsToggle label="Do Not Disturb" hint="Silence all notifications." checked={prefs.doNotDisturb} onChange={(c) => updatePrefs({ doNotDisturb: c })} />
              {prefs.doNotDisturb && (
                <div className="settings__row">
                  <div><div className="settings__rowTitle">DND schedule</div></div>
                  <div style={{ display: "flex", gap: 6, alignItems: "center" }}>
                    <input className="sts-dropdown" style={{ width: 90 }} value={prefs.dndStart} onChange={(e) => updatePrefs({ dndStart: e.target.value })} />
                    <span style={{ fontSize: 12, color: "#6b7d90" }}>to</span>
                    <input className="sts-dropdown" style={{ width: 90 }} value={prefs.dndEnd} onChange={(e) => updatePrefs({ dndEnd: e.target.value })} />
                  </div>
                </div>
              )}
              <SettingsToggle label="Show on lock screen" hint="Notification preview on lock." checked={prefs.showNotifOnLock} onChange={(c) => updatePrefs({ showNotifOnLock: c })} />
            </>}
          </SettingsSection>

          {/* ── 8. Accessibility ── */}
          <SettingsSection ref={sectionRefs.accessibility} title="Accessibility" description="Make the system comfortable.">
            {!showsSection("accessibility") ? null : <>
              <SettingsToggle label="High contrast" hint="Boost contrast system-wide." checked={prefs.highContrast} onChange={(c) => updatePrefs({ highContrast: c })} />
              <SettingsToggle label="Reduce motion" hint="Disable animations." checked={prefs.reduceMotion} onChange={(c) => updatePrefs({ reduceMotion: c, animations: c ? false : prefs.animations })} />
              <SettingsToggle label="Bold text" hint="System-wide bold font." checked={prefs.boldText} onChange={(c) => updatePrefs({ boldText: c })} />
              <SettingsToggle label="Screen reader" hint="Voice narration." checked={prefs.screenReader} onChange={(c) => updatePrefs({ screenReader: c })} />
              <SettingsToggle label="Sticky keys" hint="Modifier key persistence." checked={prefs.stickyKeys} onChange={(c) => updatePrefs({ stickyKeys: c })} />
              <SettingsSlider label="Pointer size" min={1} max={3} step={0.25} value={prefs.pointerSize} onChange={(v) => updatePrefs({ pointerSize: v })} formatValue={(v) => `${v.toFixed(2)}x`} />
            </>}
          </SettingsSection>
          {/* ── 9. Storage ── */}
          <SettingsSection ref={sectionRefs.storage} title="Storage" description="Mounted volumes and usage.">
            {!showsSection("storage") ? null : <>
              {storageDevices.length === 0 ? (
                <div className="settings__row"><div className="settings__rowHint">No storage devices detected.</div></div>
              ) : storageDevices.map((dev) => (
                <div key={dev.id} className="settings__statusCard">
                  <div className="settings__statusCardIcon"><HardDrive size={18} /></div>
                  <div className="settings__statusCardInfo">
                    <div className="settings__statusCardLabel">{dev.label}</div>
                    <div className="settings__statusCardValue">{dev.filesystem} &middot; {dev.size}{dev.mounted ? ` &middot; ${dev.mountPath}` : ""}</div>
                    <div className="sts-storageBar" style={{ marginTop: 6, width: "100%", maxWidth: 200 }}>
                      <div className="sts-storageBarFill" style={{ width: dev.mounted ? "45%" : "0%" }} />
                    </div>
                  </div>
                  <button type="button" className="settings__actionBtn" style={{ flexShrink: 0 }}
                    onClick={() => {
                      if (dev.mounted) { void storageKernel.unmount(dev.path).then(() => storageKernel.devices().then(setStorageDevices).catch(() => {})); }
                      else { void storageKernel.mount(dev.path).then(() => storageKernel.devices().then(setStorageDevices).catch(() => {})); }
                    }}>
                    {dev.mounted ? "Unmount" : "Mount"}
                  </button>
                </div>
              ))}
            </>}
          </SettingsSection>

          {/* ── 10. Software Update ── */}
          <SettingsSection ref={sectionRefs.updates} title="Software Update" description="Check for OS and app updates.">
            {!showsSection("updates") ? null : <>
              <div className="sts-versionCard" style={{ flexDirection: "row", justifyContent: "space-between", padding: "16px 20px" }}>
                <div>
                  <div className="settings__rowTitle">Samaris OS</div>
                  <div className="settings__rowHint">v1.0.0-alpha &middot; {updateChannelLabel(prefs.updateChannel)} channel</div>
                  {updateChecked && !updateError ? <div className="settings__rowHint" style={{ color: "#059669" }}>Up to date</div> : null}
                  {updateError ? <div className="settings__rowHint" style={{ color: updateError.includes("available") ? "#d97706" : "#6b7d90" }}>{updateError}</div> : null}
                </div>
                <button className="settings__actionBtn settings__actionBtn--primary" disabled={updateChecking} onClick={() => void handleCheckUpdates()}>
                  {updateChecking ? "Checking\u2026" : "Check for Updates"}
                </button>
              </div>
              <SettingsToggle label="Automatic updates" hint="Download and install updates automatically." checked={prefs.autoUpdateOS} onChange={(c) => updatePrefs({ autoUpdateOS: c })} />
              <SettingsDropdown label="Update channel" value={prefs.updateChannel} options={[
                { label: "Stable", value: "stable" },
                { label: "Beta", value: "beta" },
                { label: "Dev", value: "dev" },
              ]} onChange={(v) => updatePrefs({ updateChannel: v })} />
            </>}
          </SettingsSection>

          {/* ── 11. Language & Region ── */}
          <SettingsSection ref={sectionRefs.language} title="Language &amp; Region" description="Localization and formatting.">
            {!showsSection("language") ? null : <>
              <SettingsDropdown label="Language" value={prefs.language} options={[{ label: "English", value: "en" }, { label: "Francais", value: "fr" }, { label: "Espanol", value: "es" }, { label: "Deutsch", value: "de" }]} onChange={(v) => updatePrefs({ language: v })} />
              <SettingsDropdown label="Date format" value={prefs.dateFormat} options={[{ label: "DD/MM/YYYY", value: "dd/mm/yyyy" }, { label: "MM/DD/YYYY", value: "mm/dd/yyyy" }, { label: "YYYY-MM-DD", value: "iso" }]} onChange={(v) => updatePrefs({ dateFormat: v })} />
              <SettingsDropdown label="Time format" value={prefs.timeFormat} options={[{ label: "24-hour", value: "24h" }, { label: "12-hour", value: "12h" }]} onChange={(v) => updatePrefs({ timeFormat: v })} />
              <SettingsDropdown label="First day of week" value={prefs.firstDayOfWeek} options={[{ label: "Monday", value: "monday" }, { label: "Sunday", value: "sunday" }]} onChange={(v) => updatePrefs({ firstDayOfWeek: v })} />
              <SettingsDropdown label="Temperature" value={prefs.tempUnit} options={[{ label: "Celsius", value: "celsius" }, { label: "Fahrenheit", value: "fahrenheit" }]} onChange={(v) => updatePrefs({ tempUnit: v })} />
              <SettingsDropdown label="Keyboard layout" value={prefs.keyboardLayout} options={[{ label: "US", value: "us" }, { label: "French AZERTY", value: "fr" }, { label: "German QWERTZ", value: "de" }]} onChange={(v) => updatePrefs({ keyboardLayout: v })} />
            </>}
          </SettingsSection>

          {/* ── 12. Developer ── */}
          <SettingsSection ref={sectionRefs.developer} title="Developer" description="Tools and experimental features.">
            {!showsSection("developer") ? null : <>
              <SettingsToggle label="Developer mode" hint="Enable dev tools and logs." checked={prefs.devMode} onChange={(c) => updatePrefs({ devMode: c })} />
              {prefs.devMode && <>
                <div className="settings__row"><div className="settings__rowTitle">Open DevTools</div><button className="settings__actionBtn settings__actionBtn--primary" onClick={() => { try { window.electronAPI?.browser.openDevTools(""); } catch {} }}>Open DevTools</button></div>
                {[
                  { id: DND_FEATURE_FLAG, name: "Drag and Drop v2", desc: "Use the transaction-safe OS drag and drop system.", defaultEnabled: true },
                  { id: "experimental-gpu", name: "GPU acceleration", desc: "Enable hardware-accelerated rendering." },
                  { id: "experimental-file-preview", name: "File previews", desc: "Show inline previews for images and PDFs in Finder." },
                  { id: "experimental-spotlight-ai", name: "Spotlight AI", desc: "Search with natural language in Spotlight." },
                  { id: "experimental-window-tabs", name: "Window tabs", desc: "Tabbed window groups like macOS tabs." },
                ].map((ff) => (
                  <div key={ff.id} className="settings__featureFlag">
                    <div className="settings__featureFlagInfo">
                      <div className="settings__featureFlagName">{ff.name}</div>
                      <div className="settings__featureFlagDesc">{ff.desc}</div>
                    </div>
                    <button type="button" className={`settings__switch ${(prefs.featureFlags[ff.id] ?? ff.defaultEnabled ?? false) ? "settings__switch--on" : ""}`} onClick={() => {
                      const current = prefs.featureFlags[ff.id] ?? ff.defaultEnabled ?? false;
                      updatePrefs({ featureFlags: { ...prefs.featureFlags, [ff.id]: !current } });
                    }}>
                      <span className="settings__switchKnob" />
                    </button>
                  </div>
                ))}
              </>}
              <div className="settings__dangerZone" style={{ marginTop: 8 }}>
                <div className="settings__dangerZoneLabel">Reset all settings to defaults</div>
                <button className="settings__dangerBtn" onClick={() => { if (window.confirm("Reset ALL settings? Cannot be undone.")) { localStorage.removeItem(LS_KEY); setPrefs({ ...DEFAULT_PREFS }); } }}>Reset All</button>
              </div>
            </>}
          </SettingsSection>

          {/* ── 13. About ── */}
          <SettingsSection ref={sectionRefs.about} title="About" description="System information.">
            {!showsSection("about") ? null : <>
              <div className="sts-versionCard">
                <div className="sts-versionLogo">S</div>
                <div className="sts-versionName">Samaris OS</div>
                <div className="sts-versionSub">v1.0.0-alpha -- Core</div>
                <div className="settings__metaGrid" style={{ width: "100%", marginTop: 8 }}>
                  <div className="settings__metaCard"><div className="settings__metaLabel">Kernel</div><div className="settings__metaStrong" style={{ fontSize: 14 }}>{kernelStatus}</div><div className="settings__metaValue">WebSocket bridge</div></div>
                  <div className="settings__metaCard"><div className="settings__metaLabel">Windows</div><div className="settings__metaStrong" style={{ fontSize: 14 }}>{osState.windows.length}</div><div className="settings__metaValue">Open</div></div>
                  <div className="settings__metaCard"><div className="settings__metaLabel">Runtime</div><div className="settings__metaStrong" style={{ fontSize: 14 }}>Electron</div><div className="settings__metaValue">{osState.runtimes.length} runtimes</div></div>
                </div>
              </div>
            </>}
          </SettingsSection>
        </div>
      </div>

      {showCreateUser ? (
        <div className="settings__modalOverlay" onClick={() => setShowCreateUser(false)}>
          <div className="settings__modal" onClick={(e) => e.stopPropagation()}>
            <div className="settings__modalTitle">Create new user</div>
            <div className="settings__modalField"><label>Display name</label><input value={newUserData.displayName} onChange={(e) => setNewUserData({ ...newUserData, displayName: e.target.value })} placeholder="Full name" autoFocus /></div>
            <div className="settings__modalField"><label>Username</label><input value={newUserData.username} onChange={(e) => setNewUserData({ ...newUserData, username: e.target.value })} placeholder="Username" autoCapitalize="none" autoCorrect="off" /></div>
            <div className="settings__modalField"><label>Password</label><input type="password" value={newUserData.password} onChange={(e) => setNewUserData({ ...newUserData, password: e.target.value })} placeholder="Minimum 4 characters" /></div>
            {createError ? <div className="settings__modalError">{createError}</div> : null}
            <div className="settings__modalActions">
              <button className="settings__modalBtn settings__modalBtn--ghost" onClick={() => setShowCreateUser(false)}>Cancel</button>
              <button className="settings__modalBtn settings__modalBtn--primary" onClick={() => void handleCreateUser()}>Create</button>
            </div>
          </div>
        </div>
      ) : null}
    </div>
  );
}
