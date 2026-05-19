import React from "react";
import { MoonStar, Power, RotateCcw } from "lucide-react";
import { systemSounds } from "../sounds/systemSounds";

type SamarisSystemHooks = {
  sleep?: () => void | Promise<void>;
  restart?: () => void | Promise<void>;
  shutdown?: () => void | Promise<void>;
};

function systemHooks(): SamarisSystemHooks {
  const win = window as typeof window & { samaris?: { system?: SamarisSystemHooks } };
  return win.samaris?.system || {};
}

export function PowerControls(props: { onFallbackLock: () => void }) {
  async function run(kind: "sleep" | "restart" | "shutdown") {
    const hooks = systemHooks();
    try {
      if (kind === "sleep" && hooks.sleep) {
        await hooks.sleep();
        return;
      }
      if (kind === "restart" && hooks.restart) {
        systemSounds.play("logout");
        await hooks.restart();
        return;
      }
      if (kind === "shutdown" && hooks.shutdown) {
        systemSounds.play("shutdown");
        await hooks.shutdown();
        return;
      }
    } catch {}

    if (kind === "restart") {
      systemSounds.play("logout");
      window.setTimeout(() => window.location.reload(), 120);
      return;
    }

    if (kind === "shutdown") {
      systemSounds.play("shutdown");
    }
    props.onFallbackLock();
  }

  return (
    <div className="samaris-login__power" aria-label="Power controls">
      <button type="button" className="samaris-login__powerBtn" title="Sleep" onClick={() => void run("sleep")}>
        <MoonStar size={18} strokeWidth={2.1} />
      </button>
      <button type="button" className="samaris-login__powerBtn" title="Restart" onClick={() => void run("restart")}>
        <RotateCcw size={18} strokeWidth={2.1} />
      </button>
      <button type="button" className="samaris-login__powerBtn" title="Shutdown" onClick={() => void run("shutdown")}>
        <Power size={18} strokeWidth={2.1} />
      </button>
    </div>
  );
}
