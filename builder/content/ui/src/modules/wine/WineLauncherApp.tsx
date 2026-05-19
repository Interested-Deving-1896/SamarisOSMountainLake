import React from "react";
import { FolderOpen, Play, RefreshCw, Settings2, Square } from "lucide-react";
import { osStore } from "../../os/core/osStore";
import { PromptModal } from "../../components/PromptModal";
import { wineService } from "./WineService";
import { appendSessionLog, statusLabel, upsertSession } from "./wineProcess";
import type { WineSession, WineStatus } from "./WineTypes";
import "./wine.css";

const DEFAULT_EXE_PATH = "/User/Downloads/app.exe";

function formatDate(value: string | null) {
  if (!value) return "—";
  try {
    return new Date(value).toLocaleString();
  } catch {
    return value;
  }
}

function statusText(status: WineStatus | null) {
  if (!status) return "Checking";
  return status.installed ? "Installed" : "Missing";
}

export function WineLauncherApp(props: { windowId: string }) {
  const osState = React.useSyncExternalStore(
    (listener) => osStore.subscribe(listener),
    () => osStore.getState()
  );
  const windowParams = osState.windows.find((window) => window.id === props.windowId)?.params;
  const paramPath = (windowParams?.path as string | undefined) || "";
  const autoLaunch = Boolean(windowParams?.autoLaunch);

  const [status, setStatus] = React.useState<WineStatus | null>(null);
  const [draftPath, setDraftPath] = React.useState(paramPath || DEFAULT_EXE_PATH);
  const [selectedSessionId, setSelectedSessionId] = React.useState<string | null>(null);
  const [sessionLogs, setSessionLogs] = React.useState<Record<string, string[]>>({});
  const [note, setNote] = React.useState<string>("");
  const [loading, setLoading] = React.useState(true);
  const [launching, setLaunching] = React.useState(false);
  const [pickerOpen, setPickerOpen] = React.useState(false);
  const autoLaunchRef = React.useRef<string>("");

  const refresh = React.useCallback(async () => {
    setLoading(true);
    try {
      const next = await wineService.getStatus();
      setStatus(next);
      return next;
    } catch (error) {
      setNote(error instanceof Error ? error.message : "wine_status_failed");
    } finally {
      setLoading(false);
    }
  }, []);

  React.useEffect(() => {
    refresh().then((next) => {
      if (next?.sessions[0]) {
        setSelectedSessionId((current) => current || next.sessions[0].sessionId);
      }
    });
  }, [refresh]);

  React.useEffect(() => {
    if (paramPath) {
      setDraftPath(paramPath);
    }
  }, [paramPath]);

  React.useEffect(() => {
    const offUpdate = wineService.onSessionUpdate(({ session }) => {
      setStatus((current) => {
        if (!current) return current;
        return {
          ...current,
          sessions: upsertSession(current.sessions, session)
        };
      });
      setSelectedSessionId((current) => current || session.sessionId);
    });
    const offLog = wineService.onSessionLog((event) => {
      setSessionLogs((current) => appendSessionLog(current, event));
    });
    return () => {
      offUpdate();
      offLog();
    };
  }, []);

  React.useEffect(() => {
    if (!autoLaunch || !paramPath || autoLaunchRef.current === paramPath) {
      return;
    }
    autoLaunchRef.current = paramPath;
    void handleLaunch(paramPath);
  }, [autoLaunch, paramPath]);

  async function loadLogs(sessionId: string) {
    try {
      const payload = await wineService.getSessionLogs(sessionId);
      setSessionLogs((current) => ({
        ...current,
        [sessionId]: payload.logs
      }));
    } catch (error) {
      setNote(error instanceof Error ? error.message : "wine_logs_failed");
    }
  }

  async function handleLaunch(exePath: string) {
    if (launching) return;
    setLaunching(true);
    setNote("");
    try {
      const session = await wineService.launchExe(exePath);
      setSelectedSessionId(session.sessionId);
      setDraftPath(exePath);
      await refresh();
      await loadLogs(session.sessionId);
    } catch (error) {
      setNote(error instanceof Error ? error.message : "wine_launch_failed");
    } finally {
      setLaunching(false);
    }
  }

  async function handleOpenConfig() {
    setLaunching(true);
    setNote("");
    try {
      const session = await wineService.openConfig();
      setSelectedSessionId(session.sessionId);
      await refresh();
      await loadLogs(session.sessionId);
    } catch (error) {
      setNote(error instanceof Error ? error.message : "wine_config_failed");
    } finally {
      setLaunching(false);
    }
  }

  async function handleStop(sessionId: string) {
    try {
      await wineService.stopSession(sessionId);
      await refresh();
    } catch (error) {
      setNote(error instanceof Error ? error.message : "wine_stop_failed");
    }
  }

  const selectedSession =
    status?.sessions.find((session) => session.sessionId === selectedSessionId) || status?.sessions[0] || null;
  const logs = selectedSession ? sessionLogs[selectedSession.sessionId] || [] : [];

  return (
    <div className="wine">
      <div className="wine__column">
        <section className="wine__card wine__hero">
          <div className="wine__eyebrow">Windows compatibility</div>
          <div className="wine__title">Wine Launcher</div>
          <div className="wine__subtitle">
            Launch Windows executables through the native Linux Wine runtime, with a controlled user prefix and live logs.
          </div>
        </section>

        <section className="wine__card">
          <div className="wine__sectionTitle">Runtime status</div>
          <div className="wine__statusGrid" style={{ marginTop: 14 }}>
            <div className="wine__stat">
              <div className="wine__statLabel">Wine</div>
              <div className="wine__statValue">{loading ? "Checking…" : statusText(status)}</div>
            </div>
            <div className="wine__stat">
              <div className="wine__statLabel">Version</div>
              <div className="wine__statValue">{status?.version || "Unavailable"}</div>
            </div>
            <div className="wine__stat" style={{ gridColumn: "1 / -1" }}>
              <div className="wine__statLabel">Default prefix</div>
              <div className="wine__statValue">{status?.prefixPath || "Resolving…"}</div>
            </div>
          </div>
        </section>

        <section className="wine__card">
          <div className="wine__sectionTitle">Launch an .exe</div>
          <div className="wine__meta" style={{ marginTop: 10 }}>
            Allowed locations: Samaris user folders under <code>/User</code>.
          </div>
          <div style={{ marginTop: 14 }}>
            <input
              className="wine__pathInput"
              value={draftPath}
              onChange={(event) => setDraftPath(event.target.value)}
              placeholder={DEFAULT_EXE_PATH}
              spellCheck={false}
            />
          </div>
          <div className="wine__actions" style={{ marginTop: 14 }}>
            <button
              type="button"
              className="wine__btn wine__btn--primary"
              onClick={() => void handleLaunch(draftPath)}
              disabled={launching || !status?.installed}
            >
              <Play size={16} /> Run .exe
            </button>
            <button type="button" className="wine__btn" onClick={async () => {
              if (window.electronAPI) {
                const result = await window.electronAPI.dialog.open({
                  properties: ["openFile"],
                  filters: [{ name: "Windows Executables", extensions: ["exe"] }],
                });
                if (!result.canceled && result.filePaths.length > 0) {
                  setDraftPath(`/User/${result.filePaths[0].split("/User/").pop() || result.filePaths[0]}`);
                }
              } else {
                setPickerOpen(true);
              }
            }} disabled={launching}>
              <FolderOpen size={16} /> Browse
            </button>
            <button type="button" className="wine__btn" onClick={() => void handleOpenConfig()} disabled={launching || !status?.installed}>
              <Settings2 size={16} /> winecfg
            </button>
            <button type="button" className="wine__btn" onClick={() => void refresh()} disabled={loading}>
              <RefreshCw size={16} /> Refresh
            </button>
          </div>
          {note ? <div className="wine__note" style={{ marginTop: 14 }}>{note}</div> : null}
        </section>

        <section className="wine__card">
          <div className="wine__sectionTitle">Recent executables</div>
          <div className="wine__list" style={{ marginTop: 14 }}>
            {status?.recentExecutables.length ? (
              status.recentExecutables.map((entry) => (
                <button
                  key={entry}
                  type="button"
                  className="wine__recentItem"
                  onClick={() => {
                    setDraftPath(entry);
                    void handleLaunch(entry);
                  }}
                >
                  <div className="wine__path">{entry}</div>
                  <Play size={16} />
                </button>
              ))
            ) : (
              <div className="wine__empty">No executables launched yet.</div>
            )}
          </div>
        </section>
      </div>

      <div className="wine__column">
        <section className="wine__card">
          <div className="wine__row">
            <div>
              <div className="wine__sectionTitle">Running sessions</div>
              <div className="wine__meta">Statuses update live while Wine processes are active.</div>
            </div>
          </div>
          <div className="wine__list" style={{ marginTop: 14 }}>
            {status?.sessions.length ? (
              status.sessions.map((session) => (
                <div
                  key={session.sessionId}
                  className={`wine__sessionItem ${selectedSession?.sessionId === session.sessionId ? "wine__sessionItem--selected" : ""}`}
                  role="button"
                  tabIndex={0}
                  onClick={() => {
                    setSelectedSessionId(session.sessionId);
                    void loadLogs(session.sessionId);
                  }}
                  onKeyDown={(event) => {
                    if (event.key === "Enter" || event.key === " ") {
                      event.preventDefault();
                      setSelectedSessionId(session.sessionId);
                      void loadLogs(session.sessionId);
                    }
                  }}
                >
                  <div style={{ textAlign: "left", minWidth: 0 }}>
                    <div className="wine__statusPill" data-status={session.status}>
                      {statusLabel(session.status)}
                    </div>
                    <div className="wine__path" style={{ marginTop: 10 }}>{session.exePath}</div>
                    <div className="wine__sessionMeta" style={{ marginTop: 6 }}>
                      {session.command} • PID {session.pid ?? "—"} • Exit {session.exitCode ?? "—"}
                    </div>
                    <div className="wine__sessionMeta" style={{ marginTop: 4 }}>
                      Started {formatDate(session.startedAt)}
                    </div>
                  </div>
                  <div className="wine__sessionActions">
                    {(session.status === "running" || session.status === "starting" || session.status === "stopping") ? (
                      <button
                        type="button"
                        className="wine__btn wine__btn--danger"
                        onClick={(event) => {
                          event.stopPropagation();
                          void handleStop(session.sessionId);
                        }}
                      >
                        <Square size={16} /> Stop
                      </button>
                    ) : null}
                  </div>
                </div>
              ))
            ) : (
              <div className="wine__empty">No Wine sessions are active right now.</div>
            )}
          </div>
        </section>

        <section className="wine__card wine__logs">
          <div>
            <div className="wine__sectionTitle">Logs</div>
            <div className="wine__logMeta" style={{ marginTop: 4 }}>
              {selectedSession ? `${selectedSession.command} • ${selectedSession.exePath}` : "Select a session to inspect logs."}
            </div>
          </div>
          <div className="wine__logPanel">
            {logs.length ? logs.join("\n") : "No logs yet."}
          </div>
        </section>
      </div>

      {pickerOpen ? (
        <PromptModal
          title="Open .exe"
          subtitle="Enter a Samaris virtual path such as /User/Downloads/app.exe"
          defaultValue={draftPath || DEFAULT_EXE_PATH}
          confirmLabel="Run"
          onCancel={() => setPickerOpen(false)}
          onConfirm={(value) => {
            setPickerOpen(false);
            setDraftPath(value);
            void handleLaunch(value);
          }}
        />
      ) : null}
    </div>
  );
}
