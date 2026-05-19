# Electron Shell

The **Electron shell** transforms the React-based web interface into a genuine desktop environment by providing window management, system integration, filesystem access, peripheral control, and secure bridge APIs. It runs as the `volt-desktop.service` systemd unit.

<br>

## Process Architecture

```
┌─────────────────────────────────────────────────┐
│                Main Process (main.js)            │
│  Window management · Native menus · Kernel spawn │
├─────────────────────────────────────────────────┤
│             Renderer Process (React UI)          │
│  Vite (dev) / built bundle (prod)               │
├─────────────────────────────────────────────────┤
│          Peregrine BrowserView Tabs              │
│  Separate WebContentsView instances per tab      │
├─────────────────────────────────────────────────┤
│              Terminal Sessions                   │
│  node-pty via pty-helper.js                      │
└─────────────────────────────────────────────────┘
```

<br>

## Preload Bridge

The preload script (`preload.js`) exposes a controlled API surface to the renderer via `contextBridge.exposeInMainWorld` as `window.electronAPI`:

| Domain | Functions | Access |
|--------|-----------|--------|
| Window | minimize, maximize, close, isMaximized, onMaximizeChange | Desktop window control |
| Clipboard | readText, writeText | System clipboard |
| System | platform, arch, versions, getMemoryInfo, getGPUInfo | Hardware information |
| Browser | createTab, navigate, goBack, goForward, reload, stop, closeTab, getTabs, setBounds, activateTab, savePage, printPage, clearData, and 15+ more | Peregrine browser control |
| Terminal | create, write, resize, kill, onData, onExit | node-pty terminal sessions |
| Dialog | save, open | Native file dialogs |
| Downloads | onStarted, onProgress, onComplete, cancel | Download monitoring |
| Shell | openExternal, openPath, showItemInFolder | System shell actions |
| App | quit, restart, getVersion, isPackaged | Application lifecycle |

The renderer never receives raw Node.js or Electron API access.

<br>

## IPC Handlers

Registered in the `ipc/` directory:

- **window-controls.js**: window state management
- **clipboard.js**: clipboard read/write with sanitisation
- **system.js**: hardware and platform information queries
- **browser.js**: Peregrine tab management via WebContentsView
- **terminal.js**: PTY session lifecycle management

Security validation is applied at the handler level:
- Path access restricted to `~/.volt` and `~/Downloads` via whitelist
- URL protocols restricted to `http:`, `https:`, `mailto:` for external opening
- WebView creation is globally prevented

<br>

## Kernel Spawn

The kernel (Node.js server) is spawned as a child process by Electron:

```js
main.js → findNodeBinary()
  → spawn(node, ["server.js"], { cwd, env })
```

Kernel logs are piped to Electron's stdout. The main process retries the WebSocket connection up to **30 times** (200ms spacing) before declaring failure.

<br>

## Security Architecture

1. **contextIsolation**: renderer runs in isolated context, separate from Electron and Node.js APIs
2. **nodeIntegration**: disabled in the renderer
3. **Sandbox**: enabled by default (`--no-sandbox` for development)
4. **Preload bridge**: only explicitly exposed functions are available
5. **Path whitelist**: filesystem access restricted to user data directories
6. **WebView prevention**: `will-attach-webview` event is intercepted and blocked
7. **Protocol restriction**: `setWindowOpenHandler` intercepts popup creation and routes to Peregrine

<br>

## Lifecycle

```
Electron app.whenReady()
  → Wait for Kernel A health check (with retry)
  → Start Kernel A if not detected (Node.js server.js)
  → Initialise TerminalManager, DownloadManager, BrowserManager
  → Create frameless fullscreen BrowserWindow
  → Load preload bridge
  → Load React SPA (production: file://, dev: http://localhost:5173)
  → Register all IPC handlers
  → Ready for user interaction
```

<br>

## Related

- [IPC Reference](../apis/ipc-electron.md)
- [Window System](window-system.md)
- [Kernel A — Node.js](kernel-node.md)

<br>

---

[← Back: Architecture Overview](overview.md)
