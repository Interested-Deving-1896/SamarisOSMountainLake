import React, { useEffect, useSyncExternalStore, useState, useCallback } from "react";
import { Grid3X3, Info, FilePlus2, FolderPlus, Image, RotateCw, Settings2, ClipboardPaste } from "lucide-react";
import { osBootstrap } from "../os/core/bootstrap";
import { osStore } from "../os/core/osStore";
import { windowManager as osWindowManager } from "../os/core/windowManager";
import { appLoader } from "../os/apps/appLoader";
import { processManager } from "../os/core/processManager";
import { sessionPersistence } from "../system/session/sessionPersistence";
import { useFs } from "../services/fs/useFs";
import { onboardingKernel } from "../services/kernel/onboarding";
import { Dock } from "./Dock";
import { LockScreen } from "./LockScreen";
import { AirBar } from "../modules/airbar";
import { Spotlight } from "./Spotlight";
import { Window } from "./Window";
import { DesktopIcons } from "./DesktopIcons";
import { ContextMenu } from "./ContextMenu";
import { ConfirmModal, PromptModal } from "./PromptModal";
import type { AppWindow } from "../shell/windowing/types";
import { dockStore } from "../system/dock/dockStore";
import { windowCloseGuards } from "../system/windowing/windowCloseGuards";
import { OnboardingShell } from "../modules/onboarding";
import { applyDevResetIfNeeded } from "../system/dev/devReset";
import { downloadStore } from "../system/downloads/downloadStore";
import { DownloadToast } from "../apps/downloads/DownloadToast";
import { systemClipboard } from "../os/filesystem/clipboard";
import { pasteFromClipboard } from "../os/filesystem/fileActions";
import { AppErrorBoundary } from "./AppErrorBoundary";
import { useGenieManager } from "../effects/genie/GenieManager";
import { useFileDrop } from "../apps/shared/useFileDrop";
import { commitFileDrop } from "../os/dnd";

export function Desktop() {
  const fs = useFs();
  const [ready, setReady] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [onboardingComplete, setOnboardingComplete] = useState<boolean | null>(null);
  const [deskMenu, setDeskMenu] = useState<{ x: number; y: number } | null>(null);
  const [deskDialog, setDeskDialog] = useState<
    | { kind: "newFolder" }
    | { kind: "newTextFile" }
    | { kind: "newNote" }
    | null
  >(null);
  const [desktopRefreshToken, setDesktopRefreshToken] = useState(0);
  const [desktopRearrangeToken, setDesktopRearrangeToken] = useState(0);
  const [downloadItems, setDownloadItems] = useState(() => downloadStore.getItems());
  const [showDemoTip, setShowDemoTip] = useState(false);
  const [showDesktopIcons, setShowDesktopIcons] = useState(() => {
    try {
      const prefs = JSON.parse(localStorage.getItem("samaris-os/settings-prefs") || "{}");
      return prefs.desktopIcons !== false;
    } catch { return true;
    }
  });
  const [showMenuBar, setShowMenuBar] = useState(() => {
    try {
      const prefs = JSON.parse(localStorage.getItem("samaris-os/settings-prefs") || "{}");
      return prefs.menuBar !== false;
    } catch { return true; }
  });
  useEffect(() => {
    const sync = () => {
      try {
        const prefs = JSON.parse(localStorage.getItem("samaris-os/settings-prefs") || "{}");
        setShowDesktopIcons(prefs.desktopIcons !== false);
        setShowMenuBar(prefs.menuBar !== false);
      } catch {}
    };
    window.addEventListener("storage", sync);
    return () => window.removeEventListener("storage", sync);
  }, []);
  const genie = useGenieManager();
  const desktopDrop = useFileDrop({
    target: { id: "desktop", label: "Desktop", path: "/User/Desktop", kind: "folder" },
    allowedChoices: ["copy", "move", "link", "import"],
    recommendedAction: "copy",
    ignoreSourceIds: ["desktop-icons"],
    onDrop: async (_files, context) => {
      if (!fs) return;
      await commitFileDrop(fs, context.plan, context.decision);
      setDesktopRefreshToken((t) => t + 1);
    }
  });
  const state = useSyncExternalStore(
    (listener) => osStore.subscribe(listener),
    () => osStore.getState()
  );

  const sortedWindows = React.useMemo(
    () => [...state.windows].sort((a, b) => a.z - b.z),
    [state.windows]
  );

  useEffect(() => {
    let cancelled = false;
    (async () => {
      try {
        await applyDevResetIfNeeded();
        await osBootstrap.init();
        const onboarding = await onboardingKernel.get();
        if (!cancelled) {
          setOnboardingComplete(Boolean(onboarding.completed));
        }
        if (!cancelled) setReady(true);
      } catch (err) {
        if (cancelled) return;
        setError(err instanceof Error ? err.message : "Kernel unavailable");
        setReady(true);
      }
    })();
    return () => {
      cancelled = true;
    };
  }, []);

  useEffect(() => downloadStore.subscribe(setDownloadItems), []);

  React.useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (!e.metaKey || !["ArrowLeft", "ArrowRight", "ArrowUp"].includes(e.key)) return;
      const focused = state.windows.find((w) => w.focused);
      if (!focused) return;
      e.preventDefault();
      if (e.key === "ArrowLeft") osWindowManager.snap(focused.id, "left");
      else if (e.key === "ArrowRight") osWindowManager.snap(focused.id, "right");
      else if (e.key === "ArrowUp") osWindowManager.toggleMaximize(focused.id);
    };
    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [state.windows]);

  const windowSummary = React.useMemo(() => state.windows.map((w) => `${w.id}:${w.appId}:${w.minimized}`).join(","), [state.windows]);

  useEffect(() => {
    const timeoutId = window.setTimeout(() => {
      sessionPersistence.save();
    }, 2000);
    return () => window.clearTimeout(timeoutId);
  }, [windowSummary, state.processes.length, state.runtimes.length]);

  if (!ready) {
    return <div style={{ width: "100vw", height: "100vh" }} />;
  }

  if (onboardingComplete === false) {
    return (
      <div style={{ width: "100vw", height: "100vh" }}>
        <OnboardingShell
          onCompleted={() => {
            setOnboardingComplete(true);
          }}
        />
      </div>
    );
  }

  return (
    <div
      className={desktopDrop.isDragging ? "desktop--drop-target" : ""}
      style={{ width: "100vw", height: "100vh" }}
      {...desktopDrop.dragProps}
      onContextMenu={(event) => {
        const target = event.target as HTMLElement | null;
        if (target?.closest(".samaris-window, .samaris-airbar-shell, .samaris-airbar, .airbar-panel, .win, .dock, .topbar")) return;
        event.preventDefault();
        setDeskMenu({ x: event.clientX + 1, y: event.clientY + 1 });
      }}
      onPointerDown={(event) => {
        const target = event.target as HTMLElement | null;
        if (target?.closest(".cm")) return;
        setDeskMenu(null);
        if (target?.closest(".samaris-window, .samaris-airbar-shell, .samaris-airbar, .airbar-panel, .win, .dock, .topbar")) return;
        const active = document.activeElement as HTMLElement | null;
        active?.blur?.();
      }}
    >
      {showMenuBar ? <AirBar /> : null}
      <Spotlight />
      <LockScreen />
      {showDesktopIcons && <DesktopIcons refreshToken={desktopRefreshToken} rearrangeToken={desktopRearrangeToken} />}
      <canvas ref={genie.canvasRef} style={{ position: "fixed", inset: 0, zIndex: 9999, pointerEvents: "none", display: "none" }} />
      {genie.isAnimating && (
        <div style={{
          position: "fixed", inset: 0, zIndex: 9998,
          background: "rgba(0,0,0,0.15)",
          pointerEvents: "none",
          transition: "opacity 0.3s ease",
        }} />
      )}
      {sortedWindows.map((window) => (
          <AppErrorBoundary key={window.id} name={window.title} onClose={() => processManager.killProcessByWindow(window.id)}>
          <Window
            window={window}
            onFocus={(id) => { osWindowManager.focus(id); }}
            onMinimize={async (id) => {
              const win = osStore.getState().windows.find((w) => w.id === id);
              if (win && !genie.isAnimating) {
                await genie.minimize(win);
              }
              osWindowManager.minimize(id);
            }}
            onMaximize={(id) => { osWindowManager.toggleMaximize(id); }}
            onClose={async (id) => {
              const canClose = await windowCloseGuards.canClose(id);
              if (!canClose) return;
              processManager.killProcessByWindow(id);
            }}
            onUpdate={(id, partial) => { osWindowManager.updateLocal(id, partial as Partial<AppWindow>); }}
            onSnap={(id, target) => { osWindowManager.snap(id, target); }}
            onRestoreWindow={(id) => { osWindowManager.restoreWindowState(id); }}
            onDuplicate={(appId, params) => { void appLoader.openApp(appId, { windowParams: params, forceNewWindow: true }); }}
          />
          </AppErrorBoundary>
        ))}
      <Dock
        store={dockStore}
        onLaunch={async (appId) => {
          const minimized = state.windows.filter((w) => w.appId === appId && (w.minimized || w.minimizing));
          if (minimized.length > 0 && !genie.isAnimating) {
            const win = minimized[0];
            await genie.restore(win, appId);
            osWindowManager.restore(win.id);
            return;
          }
          await appLoader.openApp(appId);
        }}
        onLaunchChild={async (appId) => {
          await appLoader.openApp(appId);
        }}
      />

      {deskMenu ? (
        <ContextMenu
          x={deskMenu.x}
          y={deskMenu.y}
          ariaLabel="Desktop menu"
          onClose={() => setDeskMenu(null)}
          items={[
            {
              id: "new-folder",
              label: "New Folder",
              icon: FolderPlus,
              onSelect: () => setDeskDialog({ kind: "newFolder" })
            },
            {
              id: "new-text",
              label: "New Text File",
              icon: FilePlus2,
              onSelect: () => setDeskDialog({ kind: "newTextFile" })
            },
            {
              id: "refresh",
              label: "Refresh",
              icon: RotateCw,
              onSelect: () => window.location.reload()
            },
            {
              id: "arrange",
              label: "Rearrange Icons",
              icon: Grid3X3,
              disabled: false,
              onSelect: () => {
                setDesktopRearrangeToken((value) => value + 1);
              }
            },
            {
              id: "paste",
              label: "Paste",
              icon: ClipboardPaste,
              disabled: !systemClipboard.readFiles(),
              onSelect: () => {
                void pasteFromClipboard(fs, "/User/Desktop").then(() => {
                  setDesktopRefreshToken((value) => value + 1);
                }).catch(() => {});
              }
            },
            {
              id: "settings",
              label: "Settings",
              icon: Settings2,
              onSelect: () => void appLoader.openApp("settings")
            },
            {
              id: "wallpaper",
              label: "Change Wallpaper",
              icon: Image,
              onSelect: () => void appLoader.openApp("settings", { windowParams: { section: "appearance" } })
            },
            {
              id: "about",
              label: "About Samaris OS",
              icon: Info,
              onSelect: () => void appLoader.openApp("about")
            }
          ]}
        />
      ) : null}

      {deskDialog?.kind === "newFolder" ? (
        <PromptModal
          title="New Folder"
          subtitle="/User/Desktop"
          placeholder="Folder name"
          defaultValue="New Folder"
          confirmLabel="Create"
          onCancel={() => setDeskDialog(null)}
          onConfirm={(value) => {
            setDeskDialog(null);
            if (!value) return;
            void fs
              .mkdir(`/User/Desktop/${value}`.replace(/\/+/g, "/"))
              .finally(() => setDesktopRefreshToken((t) => t + 1));
          }}
        />
      ) : null}

      {deskDialog?.kind === "newTextFile" ? (
        <PromptModal
          title="New Text File"
          subtitle="/User/Desktop"
          placeholder="File name"
          defaultValue="untitled.txt"
          confirmLabel="Create"
          onCancel={() => setDeskDialog(null)}
          onConfirm={(value) => {
            setDeskDialog(null);
            if (!value) return;
            void fs
              .write(`/User/Desktop/${value}`.replace(/\/+/g, "/"), "")
              .finally(() => setDesktopRefreshToken((t) => t + 1));
          }}
        />
      ) : null}

      {deskDialog?.kind === "newNote" ? (
        <PromptModal
          title="New Note"
          subtitle="/User/Desktop"
          placeholder="Note name"
          defaultValue="New Note.md"
          confirmLabel="Create"
          onCancel={() => setDeskDialog(null)}
          onConfirm={(value) => {
            setDeskDialog(null);
            if (!value) return;
            const fileName = value.toLowerCase().endsWith(".md") ? value : `${value}.md`;
            const path = `/User/Desktop/${fileName}`.replace(/\/+/g, "/");
            void fs
              .write(path, "# New Note\n\n")
              .then(() => {
                setDesktopRefreshToken((t) => t + 1);
                return appLoader.openApp("notes", { windowParams: { path } });
              })
              .catch(() => appLoader.openApp("notes", { windowParams: { path } }));
          }}
        />
      ) : null}

      {error ? (
        <div
          style={{
            position: "fixed",
            right: 16,
            bottom: 112,
            padding: "10px 12px",
            borderRadius: 10,
            background: "rgba(20,24,36,0.82)",
            border: "1px solid rgba(255,255,255,0.12)",
            color: "rgba(255,255,255,0.86)",
            fontSize: 12,
            zIndex: 100000
          }}
        >
          Kernel unavailable: {error}
        </div>
      ) : null}
    </div>
  );
}
