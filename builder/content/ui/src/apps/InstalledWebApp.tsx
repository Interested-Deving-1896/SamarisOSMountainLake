import React from "react";
import { AlertCircle, Loader2, RefreshCw, Clock } from "lucide-react";
import { osStore } from "../os/core/osStore";
import { windowManager } from "../os/core/windowManager";
import "./installed-web-app.css";

const LOAD_TIMEOUT_MS = 20000;

type InstalledWebAppParams = {
  launchUrl?: string;
  title?: string;
  subtitle?: string;
  source?: string;
};

export default function InstalledWebApp(props: { windowId: string }) {
  const state = React.useSyncExternalStore((listener) => osStore.subscribe(listener), () => osStore.getState());
  const windowState = state.windows.find((entry) => entry.id === props.windowId) || null;
  const params = (windowState?.params || {}) as InstalledWebAppParams;
  const launchUrl = typeof params.launchUrl === "string" ? params.launchUrl : "";
  const title = typeof params.title === "string" ? params.title : "Installed App";
  const subtitle = typeof params.subtitle === "string" ? params.subtitle : "App Store";
  const [loading, setLoading] = React.useState(Boolean(launchUrl));
  const [failed, setFailed] = React.useState(false);
  const [timedOut, setTimedOut] = React.useState(false);
  const iframeRef = React.useRef<HTMLIFrameElement>(null);
  const retryCountRef = React.useRef(0);
  const loadingStartRef = React.useRef(0);
  const timerRef = React.useRef<ReturnType<typeof setTimeout> | null>(null);

  const clearTimer = React.useCallback(() => {
    if (timerRef.current) { clearTimeout(timerRef.current); timerRef.current = null; }
  }, []);

  const startTimer = React.useCallback(() => {
    clearTimer();
    loadingStartRef.current = Date.now();
    timerRef.current = setTimeout(() => {
      setTimedOut(true);
      setLoading(false);
      setFailed(true);
    }, LOAD_TIMEOUT_MS);
  }, [clearTimer]);

  const handleLoad = React.useCallback(() => {
    clearTimer();
    setLoading(false);
    setFailed(false);
    setTimedOut(false);
    try {
      const doc = iframeRef.current?.contentDocument;
      if (doc && doc.body) {
        const text = (doc.body.textContent || "").trim().substring(0, 200);
        console.log("[iwa] Loaded:", doc.URL, "| Body:", text);
      }
    } catch (e) {
      console.log("[iwa] Loaded (cross-origin):", launchUrl);
    }
  }, [clearTimer, launchUrl]);

  const handleError = React.useCallback(() => {
    clearTimer();
    setLoading(false);
    setFailed(true);
  }, [clearTimer]);

  React.useEffect(() => {
    if (!windowState) return;
    if (windowState.title === title && windowState.subtitle === subtitle) return;
    windowManager.updateLocal(props.windowId, { title, subtitle });
  }, [props.windowId, subtitle, title, windowState]);

  React.useEffect(() => {
    setLoading(Boolean(launchUrl));
    setFailed(false);
    setTimedOut(false);
    retryCountRef.current = 0;
    clearTimer();
    if (launchUrl) startTimer();
    return clearTimer;
  }, [launchUrl, clearTimer, startTimer]);

  function handleRetry() {
    setFailed(false);
    setTimedOut(false);
    setLoading(true);
    retryCountRef.current += 1;
    if (iframeRef.current) {
      iframeRef.current.src = launchUrl;
    }
  }

  if (!launchUrl) {
    return (
      <div className="iwa iwa--empty">
        <AlertCircle size={18} strokeWidth={2.2} />
        <div>
          <strong>Launch URL missing</strong>
          <span>This installed app is missing its local launch target.</span>
        </div>
      </div>
    );
  }

  return (
    <div className="iwa">
      {loading ? (
        <div className="iwa__overlay" aria-live="polite">
          <Loader2 size={18} strokeWidth={2.2} className="iwa__spinner" />
          <div>
            <strong>Loading {title}</strong>
            <span>{subtitle}</span>
          </div>
        </div>
      ) : null}
      {failed ? (
        <div className="iwa__overlay iwa__overlay--error" aria-live="polite">
          {timedOut ? <Clock size={18} strokeWidth={2.2} /> : <AlertCircle size={18} strokeWidth={2.2} />}
          <div>
            <strong>{timedOut ? "Timed out" : "Unable to load this app"}</strong>
            <span>{timedOut ? "The app server took too long to respond. Try retrying." : "The app server may not be running or the page failed to load."}</span>
            <button type="button" className="iwa__retry" onClick={handleRetry}>
              <RefreshCw size={14} strokeWidth={2.5} /> Retry
            </button>
          </div>
        </div>
      ) : null}
      <iframe
        ref={iframeRef}
        className="iwa__frame"
        title={title}
        src={launchUrl}
        key={`${launchUrl}-${retryCountRef.current}`}
        sandbox="allow-same-origin allow-scripts allow-forms allow-popups allow-modals allow-downloads allow-presentation"
        referrerPolicy="no-referrer"
        onLoad={handleLoad}
        onError={handleError}
      />
    </div>
  );
}
