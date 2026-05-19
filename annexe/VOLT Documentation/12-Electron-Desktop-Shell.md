# 12. Electron Desktop Shell

## 12.1 Overview

The Electron Desktop Shell is the native desktop container of Samaris OS. It transforms the React-based web interface into a genuine desktop environment by providing window management, system integration, filesystem access, peripheral control, and secure bridge APIs. The shell runs as the `volt-desktop.service` systemd unit and is the primary interface between the user-facing web UI and the VOLT native layer.

## 12.2 Architecture

### Main Process

The Electron main process (`main.js`) manages:

- **BrowserWindow**: a frameless, fullscreen window spanning the primary display
- **Kernel A lifecycle**: starts the Node.js orchestrator if not already running, waits for health confirmation with retry
- **Display management**: detects all monitors, selects primary display, configures window geometry
- **Process isolation**: enforces `contextIsolation: true`, `nodeIntegration: false`, optional sandbox mode
- **Window controls**: maximise, minimise, close, fullscreen toggle

### Preload Bridge

The preload script (`preload.js`) exposes a controlled API surface to the renderer process via `contextBridge.exposeInMainWorld`. The exposed API is structured as `window.electronAPI` with the following domains:

| Domain | Functions | Access |
|--------|-----------|--------|
| Window | minimize, maximize, close, isMaximized, onMaximizeChange | Desktop window control |
| Clipboard | readText, writeText | System clipboard |
| System | platform, arch, versions, getMemoryInfo, getGPUInfo | Hardware information |
| Browser | createTab, navigate, goBack, goForward, reload, stop, closeTab, getTabs, setBounds, activateTab, savePage, printPage, clearData, and 15+ more | Full Peregrine browser control |
| Terminal | create, write, resize, kill, onData, onExit | node-pty terminal sessions |
| Dialog | save, open | Native file dialogs |
| Downloads | onStarted, onProgress, onComplete, cancel | Download monitoring |
| Shell | openExternal, openPath, showItemInFolder | System shell actions |
| App | quit, restart, getVersion, isPackaged | Application lifecycle |

The preload bridge exposes typed IPC invocations only — the renderer never receives raw Node.js or Electron API access.

### IPC Handlers

The IPC layer (`ipc/`) registers handlers for all preload-exposed functions:

- **window-controls.js**: window state management
- **clipboard.js**: clipboard read/write with sanitisation
- **system.js**: hardware and platform information queries
- **browser.js**: Peregrine tab management via WebContentsView
- **terminal.js**: PTY session lifecycle management

Security validation is applied at the handler level:

- Path access restricted to `~/.volt` and `~/Downloads` via whitelist
- URL protocols restricted to `http:`, `https:`, `mailto:` for external opening
- WebView creation is globally prevented

### Service Layer

Three managed service classes:

- **BrowserManager**: manages WebContentsView instances for the Peregrine browser, including tab creation, navigation, snapshot generation, zoom, and permission handling
- **TerminalManager**: manages node-pty pseudoterminal processes, including creation, data streaming, resize handling, and cleanup
- **DownloadManager**: manages native Electron download items with progress tracking and cancellation

## 12.3 Security Architecture

The Electron shell implements a defence-in-depth security model:

1. **contextIsolation**: renderer process runs in an isolated context, separate from Electron and Node.js APIs
2. **nodeIntegration**: disabled in the renderer
3. **Sandbox**: enabled by default, with `--no-sandbox` flag available for development
4. **Preload bridge**: only explicitly exposed functions are available to the renderer
5. **Path whitelist**: filesystem access restricted to user data directories
6. **WebView prevention**: `will-attach-webview` event is intercepted and blocked
7. **Protocol restriction**: `setWindowOpenHandler` intercepts popup creation and routes them to the Peregrine browser

## 12.4 Lifecycle

```
Electron app.whenReady()
  → Wait for Kernel A health check (with retry)
  → Start Kernel A if not detected (Node.js server.js)
  → Initialise TerminalManager, DownloadManager, BrowserManager
  → Create frameless fullscreen BrowserWindow
  → Load preload bridge
  → Load React SPA (production: file://, development: http://localhost:5173)
  → Register all IPC handlers
  → Ready for user interaction
```

## 12.5 Configuration

The Electron shell uses environment variables for configuration:

- `NODE_ENV`: development/production mode selection
- `PORT`: Kernel A port (default: 9999)

The shell package (`package.json`) includes build configuration for Linux AppImage and macOS DMG distribution, with extra resources mapping for the Kernel A server.
