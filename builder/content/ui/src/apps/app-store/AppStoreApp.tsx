import React from "react";
import { appStoreKernel } from "../../services/kernel/appStore";
import { openInstalledWebApp } from "../../os/apps/installedWebApp";
import { ChevronDown, ChevronRight, ExternalLink, RefreshCw, Trash2, Wrench, Globe, Copy, Check, Loader2, GitBranch, Package, Hammer, Terminal, AlertCircle } from "lucide-react";

const TEMPLATES = [
  { label: "Vite React TS", url: "https://github.com/stackblitz/samaris-demo-vite-react", hint: "React + TypeScript" },
  { label: "Vanilla Vite", url: "https://github.com/stackblitz/samaris-demo-vanilla", hint: "Plain JS/TS" },
  { label: "Vite + Vue", url: "https://github.com/stackblitz/samaris-demo-vue", hint: "Vue 3 SPA" },
  { label: "Vite + Svelte", url: "https://github.com/stackblitz/samaris-demo-svelte", hint: "Svelte" },
];

const STEPS: Record<string, { icon: typeof Loader2; label: string }> = {
  "idle": { icon: Globe, label: "Idle" },
  "cloning": { icon: GitBranch, label: "Cloning repo..." },
  "installing": { icon: Package, label: "Installing dependencies..." },
  "building": { icon: Hammer, label: "Building app..." },
  "done": { icon: Check, label: "Ready" },
  "error": { icon: AlertCircle, label: "Failed" },
};

const APP_COLORS = [
  "#4d84ff", "#f472b6", "#34d399", "#fbbf24", "#a78bfa",
  "#fb923c", "#2dd4bf", "#e879f9", "#22d3ee", "#f87171"
];

function colorForAppId(appId: string) {
  let hash = 0;
  for (let i = 0; i < appId.length; i++) hash = appId.charCodeAt(i) + ((hash << 5) - hash);
  return APP_COLORS[Math.abs(hash) % APP_COLORS.length];
}

function AppIcon({ appId, name, size = 40 }: { appId: string; name: string; size?: number }) {
  const bg = colorForAppId(appId);
  const letter = (name || appId).charAt(0).toUpperCase();
  const fontSize = Math.round(size * 0.46);
  return (
    <span className="store__appIcon" style={{ width: size, height: size, borderRadius: size * 0.28, background: `linear-gradient(135deg, ${bg}, ${bg}cc)`, fontSize, lineHeight: `${size}px`, textAlign: "center", color: "#fff", fontWeight: 800, display: "inline-block", flexShrink: 0 }}>
      {letter}
    </span>
  );
}

function StatusBadge({ status, launchable }: { status: string; launchable: boolean }) {
  let cls = "store__badge";
  let label = status;
  if (launchable) { cls += " store__badge--ready"; label = "Ready"; }
  else if (status === "build_failed" || status === "clone_failed") { cls += " store__badge--error"; }
  else if (status === "cloned") { cls += " store__badge--cloned"; label = "Needs Build"; }
  else { cls += " store__badge--other"; }
  return <span className={cls}>{label}</span>;
}

export function AppStoreApp(_props: { windowId: string }) {
  const [url, setUrl] = React.useState("");
  const [apps, setApps] = React.useState<Awaited<ReturnType<typeof appStoreKernel.listInstalled>>>([]);
  const [loading, setLoading] = React.useState(true);
  const [busy, setBusy] = React.useState<string | null>(null);
  const [logs, setLogs] = React.useState<Record<string, string>>({});
  const [expandedLogs, setExpandedLogs] = React.useState<Set<string>>(new Set());
  const [copiedUrl, setCopiedUrl] = React.useState("");
  const [globalBusy, setGlobalBusy] = React.useState(false);
  const [globalLog, setGlobalLog] = React.useState("");
  const [step, setStep] = React.useState<string>("idle");
  const stepRef = React.useRef(step);
  React.useEffect(() => { stepRef.current = step; }, [step]);

  const refresh = React.useCallback(async () => {
    try { setApps(await appStoreKernel.listInstalled()); }
    catch (e) { setGlobalLog("Failed to load installed apps: " + (e instanceof Error ? e.message : "unknown error")); }
    finally { setLoading(false); }
  }, []);
  React.useEffect(() => { void refresh(); }, [refresh]);

  async function runAction(label: string, action: () => Promise<{ ok: boolean; logs?: string }>) {
    setBusy(label);
    try {
      const result = await action();
      if (result.logs) {
        setLogs((prev) => { const next = { ...prev }; next[label] = result.logs!; return next; });
        setExpandedLogs((prev) => new Set(prev).add(label));
      }
      await refresh();
    } catch (e) {
      const msg = e instanceof Error ? e.message : "Action failed";
      setLogs((prev) => ({ ...prev, [label]: msg }));
      setExpandedLogs((prev) => new Set(prev).add(label));
    } finally {
      setBusy(null);
    }
  }

  async function cloneAndBuild(repoUrl: string) {
    setGlobalBusy(true);
    setGlobalLog("");
    setStep("cloning");
    try {
      const cloneResult = await appStoreKernel.clone(repoUrl);
      if (cloneResult.logs) setGlobalLog(cloneResult.logs);
      if (!cloneResult.ok) {
        setGlobalLog((prev) => prev + "\n\nClone failed.");
        setStep("error");
        return;
      }
      if (!cloneResult.entry) {
        setGlobalLog((prev) => prev + "\n\nNo app entry after clone.");
        setStep("error");
        return;
      }
      setGlobalLog((prev) => prev + "\n\nClone OK. Installing dependencies...");
      setStep("installing");
      const buildResult = await appStoreKernel.build(cloneResult.entry.appId);
      if (buildResult.logs) setGlobalLog((prev) => prev + "\n\n" + buildResult.logs);
      setStep(buildResult.ok ? "done" : "error");
      await refresh();
    } catch (e) {
      const msg = e instanceof Error ? e.message : "Unknown error";
      setGlobalLog((prev) => prev + "\n\n" + msg);
      setStep("error");
    } finally {
      setGlobalBusy(false);
      setTimeout(() => {
        const current = stepRef.current;
        if (current === "done" || current === "error") setStep("idle");
      }, 3000);
    }
  }

  async function launch(app: (typeof apps)[number]) {
    try {
      await openInstalledWebApp(app);
    } catch (error) {
      setGlobalLog(error instanceof Error ? error.message : "Unable to launch");
    }
  }

  function copyUrl(appUrl: string) {
    void navigator.clipboard.writeText(appUrl).then(() => {
      setCopiedUrl(appUrl);
      setTimeout(() => setCopiedUrl(""), 2000);
    });
  }

  return (
    <div className="store">
      <div className="store__hero">
        <div className="store__heroInner">
          <div className="store__brand">
            <span className="store__brandIcon">S</span>
            <div>
              <div className="store__title">App Store</div>
              <div className="store__subtitle">Install apps from GitHub repos into your Samaris user space. Clone, build, launch.</div>
            </div>
          </div>
        </div>
      </div>

      <div className="store__inputRow">
        <div className="store__inputWrap">
          <Globe size={16} strokeWidth={2} className="store__inputIcon" />
          <input
            value={url}
            onChange={(event) => setUrl(event.target.value)}
            placeholder="https://github.com/user/repo"
            onKeyDown={(event) => {
              if (event.key === "Enter" && url.trim() && !globalBusy) {
                event.preventDefault();
                void cloneAndBuild(url.trim());
              }
            }}
          />
        </div>
        <button
          type="button"
          className="store__btn store__btn--primary"
          disabled={!url.trim() || globalBusy}
          onClick={() => void cloneAndBuild(url.trim())}
        >
          {globalBusy ? <Loader2 size={15} strokeWidth={2.5} className="store__spin" /> : <Copy size={15} strokeWidth={2.5} />}
          Clone & Build
        </button>
      </div>

      <div className="store__templates">
        {TEMPLATES.map((t) => (
          <button
            key={t.url}
            type="button"
            className="store__templateBtn"
            disabled={globalBusy}
            onClick={() => { setUrl(t.url); void cloneAndBuild(t.url); }}
          >
            <span className="store__templateLabel">{t.label}</span>
            <span className="store__templateHint">{t.hint}</span>
          </button>
        ))}
      </div>

      {globalLog ? (
        <div className="store__globalLog">
          <div className="store__globalLogHead">
            <span>Activity</span>
            <button type="button" className="store__logClose" onClick={() => setGlobalLog("")}>✕</button>
          </div>
          <pre className="store__logBody">{globalLog}</pre>
        </div>
      ) : null}

      {globalBusy && step !== "idle" ? (
        <div className="store__stepBar">
          <span className="store__stepIcon">{React.createElement(STEPS[step]?.icon || Loader2, { size: 14, strokeWidth: 2.5, className: step === "cloning" || step === "installing" || step === "building" ? "store__spin" : "" })}</span>
          <span className="store__stepLabel">{STEPS[step]?.label || step}</span>
        </div>
      ) : null}

      <div className="store__grid">
        {loading ? (
          <div className="store__empty">
            <Loader2 size={28} strokeWidth={1.5} className="store__spin" />
            <div className="store__emptyTitle" style={{ marginTop: 12 }}>Loading installed apps…</div>
          </div>
        ) : apps.length === 0 ? (
          <div className="store__empty">
            <div className="store__emptyIcon"><Wrench size={32} strokeWidth={1.5} /></div>
            <div className="store__emptyTitle">No apps installed</div>
            <div className="store__emptySub">Paste a GitHub URL above or pick a template to get started.</div>
          </div>
        ) : (
          <div className="store__appGrid">
            {apps.map((app) => {
              const appName = app.manifest?.displayName || app.repoName || app.appId;
              const isBusy = busy === app.appId || busy === `clone:${app.url}`;
              const anyBusy = isBusy || globalBusy;
              return (
                <div key={app.appId} className="store__card">
                  <div className="store__cardHead">
                    <AppIcon appId={app.appId} name={appName} size={42} />
                    <div className="store__cardInfo">
                      <div className="store__cardName">{appName}</div>
                      {app.manifest?.version ? <div className="store__cardVersion">v{app.manifest.version}</div> : null}
                    </div>
                    <StatusBadge status={app.status} launchable={Boolean(app.launchable)} />
                  </div>
                  {app.manifest?.description ? <div className="store__cardDesc">{app.manifest.description}</div> : null}
                  <div className="store__cardActions">
                    <button type="button" className="store__cardBtn store__cardBtn--primary" disabled={!app.launchable || anyBusy} onClick={() => void launch(app)}>
                      <ExternalLink size={14} strokeWidth={2.5} /> Launch
                    </button>
                    <button type="button" className="store__cardBtn" disabled={anyBusy} onClick={() => void runAction(app.appId, () => appStoreKernel.build(app.appId))}>
                      {busy === app.appId ? <Loader2 size={14} strokeWidth={2.5} className="store__spin" /> : <RefreshCw size={14} strokeWidth={2.5} />} Build
                    </button>
                    <button type="button" className="store__cardBtn" disabled={anyBusy} onClick={() => void runAction(app.appId, () => appStoreKernel.update(app.appId))}>
                      <RefreshCw size={14} strokeWidth={2.5} /> Update
                    </button>
                    <button type="button" className="store__cardBtn store__cardBtn--danger" disabled={anyBusy} onClick={() => void runAction(app.appId, () => appStoreKernel.remove(app.appId))}>
                      <Trash2 size={14} strokeWidth={2.5} /> Remove
                    </button>
                  </div>
                  {logs[app.appId] ? (
                    <div className="store__cardLogs">
                      <button type="button" className="store__cardLogsToggle" onClick={() => { const next = new Set(expandedLogs); if (next.has(app.appId)) next.delete(app.appId); else next.add(app.appId); setExpandedLogs(next); }}>
                        {expandedLogs.has(app.appId) ? <ChevronDown size={14} /> : <ChevronRight size={14} />} Build logs
                      </button>
                      {expandedLogs.has(app.appId) ? <pre className="store__cardLogsBody">{logs[app.appId]}</pre> : null}
                    </div>
                  ) : null}
                  <div className="store__cardFooter">
                    <span className="store__cardUrl" title={app.url}>{app.repoName || app.appId}</span>
                    <button type="button" className="store__copyBtn" onClick={() => copyUrl(app.url)}>
                      {copiedUrl === app.url ? <Check size={12} strokeWidth={3} /> : <Copy size={12} strokeWidth={2} />}
                    </button>
                  </div>
                </div>
              );
            })}
          </div>
        )}
      </div>
    </div>
  );
}
