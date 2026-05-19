import React from "react";
import type { DosFactory, DosInstance } from "../types";

declare global {
  interface Window {
    Dos?: DosFactory;
  }
}

const SCRIPT_ID = "volt-jsdos-script";
const STYLE_ID = "volt-jsdos-style";

function ensureStyle() {
  if (document.getElementById(STYLE_ID)) return;
  const link = document.createElement("link");
  link.id = STYLE_ID;
  link.rel = "stylesheet";
  link.href = "/vendor/js-dos/js-dos.css";
  document.head.appendChild(link);
}

function ensureScript() {
  return new Promise<void>((resolve, reject) => {
    if (window.Dos) {
      resolve();
      return;
    }

    const existing = document.getElementById(SCRIPT_ID) as HTMLScriptElement | null;
    if (existing) {
      existing.addEventListener("load", () => resolve(), { once: true });
      existing.addEventListener("error", () => reject(new Error("jsdos_load_failed")), { once: true });
      return;
    }

    const script = document.createElement("script");
    script.id = SCRIPT_ID;
    script.src = "/vendor/js-dos/js-dos.js";
    script.async = true;
    script.onload = () => resolve();
    script.onerror = () => reject(new Error("jsdos_load_failed"));
    document.head.appendChild(script);
  });
}

export function useDoomRuntime(containerRef: React.RefObject<HTMLDivElement>) {
  const [status, setStatus] = React.useState<"idle" | "loading" | "ready" | "error">("idle");
  const [error, setError] = React.useState<string | null>(null);

  React.useEffect(() => {
    const container = containerRef.current;
    if (!container) return;
    const host = container;

    let mounted = true;
    let instance: DosInstance | null = null;

    async function boot() {
      setStatus("loading");
      setError(null);
      try {
        ensureStyle();
        await ensureScript();
        if (!mounted || !window.Dos) return;
        host.innerHTML = "";
        instance = window.Dos(host, {
          url: "/games/doom/DOOM.jsdos",
          pathPrefix: "/vendor/js-dos/emulators/",
          autoStart: true,
          kiosk: true,
          noCloud: true,
          noNetworking: true,
          mouseCapture: true,
          renderBackend: "webgl",
          renderAspect: "fit",
          theme: "light"
        });
        if (mounted) setStatus("ready");
      } catch (runtimeError) {
        if (!mounted) return;
        setStatus("error");
        setError(runtimeError instanceof Error ? runtimeError.message : "Unable to load DOOM");
      }
    }

    void boot();
    return () => {
      mounted = false;
      if (instance?.stop) {
        void instance.stop();
      }
    };
  }, [containerRef]);

  return {
    status,
    error
  };
}
