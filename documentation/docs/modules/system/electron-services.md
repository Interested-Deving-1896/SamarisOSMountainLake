# Electron Shell Services

**Native desktop shell services running in the Electron main process**

The Electron shell (`builder/content/electron/`) provides the native window, browser, terminal, and download orchestration that bridges the React UI to the operating system.

<br>

## Architecture

```
Electron Main Process
├── main.js                    Entry point — window creation, kernel lifecycle
├── launcher.js                Pre-window setup, screen mode selection
├── preload.js                 Context bridge (contextBridge API)
├── ipc/                       IPC handler modules
│   ├── index.js               Handler registration hub
│   ├── browser.js             Browser IPC (tab management, navigation)
│   ├── terminal.js            Terminal IPC (pty sessions)
│   ├── window-controls.js     Window state (minimize, maximize, close)
│   ├── system.js              System IPC (notifications, clipboard)
│   ├── bench.js               Benchmark IPC (trigger, stream results)
│   └── clipboard.js           Clipboard read/write
└── services/                  Managed native services
    ├── browser-manager.js     Peregrine WebContentsView manager
    ├── terminal-manager.js    PTY terminal session manager
    └── download-manager.js    File download orchestration
```

<br>

## Services

### Browser Manager (`services/browser-manager.js`)
Manages Peregrine browser tabs using Electron's `WebContentsView` API:

| Feature | Description |
|---------|-------------|
| Tab creation | New tab with URL validation and navigation |
| Tab lifecycle | Create, activate, close, discard (after 10min inactivity) |
| Navigation | Back, forward, reload, stop, URL bar management |
| Security | URL sanitisation, path traversal prevention, allowed file roots |
| Downloads | Integrated with DownloadManager for save-as dialogs |
| DevTools | Per-tab DevTools toggle |
| Session isolation | Per-window session partitioning |

Supports custom protocols (`samaris://`), view-source, file:// with restricted roots.

### Terminal Manager (`services/terminal-manager.js`)
Manages pseudo-terminal sessions via a PTY helper process:

| Feature | Description |
|---------|-------------|
| Session creation | Spawns `pty-helper.js` per terminal tab |
| I/O streaming | Bidirectional stdin/stdout with resize support |
| Security | CWD restricted to `~/.volt/` tree, ID sanitisation |
| Multi-session | Multiple independent terminal sessions |
| Cleanup | `killAll()` on app quit |

Each session runs as a child process of Electron, with configurable columns/rows, 256-color TERM support, and truecolor enabled.

### Download Manager (`services/download-manager.js`)
Orchestrates file downloads with user interaction:

| Feature | Description |
|---------|-------------|
| Save dialog | Native "Save As" dialog with default to `~/Downloads` |
| Progress tracking | Real-time progress events to renderer |
| Lifecycle | Start, progress, complete, cancel, fail |
| Storage | Files saved under `~/.volt/user/Downloads/` |
| UX integration | Download bar in UI with progress indicators |

<br>

## IPC Handlers

| Handler | Module | Channels |
|---------|--------|----------|
| **Browser** | `ipc/browser.js` | Peregrine tab lifecycle, navigation, URL bar |
| **Terminal** | `ipc/terminal.js` | PTY create, resize, stdin, kill |
| **Window Controls** | `ipc/window-controls.js` | Minimize, maximize, close, fullscreen |
| **System** | `ipc/system.js` | Notifications, open external links, platform info |
| **Bench** | `ipc/bench.js` | Trigger benchmark, stream progress/results |
| **Clipboard** | `ipc/clipboard.js` | Read/write clipboard text |

All IPC is secured via `contextBridge` with `contextIsolation: true`.

<br>

## Kernel Lifecycle

The Electron shell manages Kernel A lifecycle:

1. On startup, polls `http://127.0.0.1:9999/health` (up to 30 retries, 200ms delay)
2. If kernel not detected, spawns it from `builder/content/volt-kernel-a/server.js`
3. Waits for kernel readiness before creating the browser window
4. Integrates with VDM (Display Manager) for screen detection

<br>

## VDM Integration

The shell detects VDM display configuration on startup:

```javascript
// Reads display events to determine safe mode or screen layout
findVdmEventPath() → parse JSON → apply safe mode or fullscreen
```

If safe mode is active, the window opens at 1280×720 with a frame title bar instead of fullscreen kiosk mode.

<br>

## Build

```bash
# Built as part of the ISO pipeline (step 11-electron)
# Builder: builder/ISOGenerator/steps/11-electron.sh

# The Electron app is installed to /opt/volt/desktop/ in the ISO
# Launched via /usr/local/bin/volt-desktop
```
