# Peregrine Browser

Web browser built on **Electron's WebContentsView** API (replaces the deprecated BrowserView) with full tab management, session isolation, and crash resilience.

<br>

## Features

- Multiple tabs (each backed by a separate `WebContentsView`)
- URL bar with navigation controls (back, forward, reload, stop)
- Tab switching with instant activation
- **Session management**: default persistent session + ephemeral session for private/incognito tabs
- **Popup handling**: intercepted via `setWindowOpenHandler` — popups open as new tabs
- Download manager (intercepts `will-download` events)
- Start page with quick links and search
- **Crash isolation**: a crashed tab does not bring down the browser or other tabs

<br>

## Architecture

```
PeregrineApp (React)
├── PeregrineToolbar (tab strip + URL bar + nav buttons)
├── PeregrineViewport (WebContentsView container)
│   └── TabCrashOverlay (shown when a tab crashes)
├── PeregrineStartPage (new tab)
├── PeregrineContextMenu (right-click)
└── SessionManager (default + ephemeral sessions)
```

Browser communication via **Electron IPC**:

| Channel | Direction |
|---------|-----------|
| `browser:create-tab` → `browser:tab-update` | Bidirectional |
| `browser:navigate` | Renderer → Main |
| `browser:close-tab` | Renderer → Main |
| `browser:tab-crashed` | Main → Renderer |

<br>

## Session Isolation

| Session Type | Storage | Use Case |
|--------------|---------|----------|
| **Default** | Persistent cookies, cache, localStorage | Normal browsing |
| **Ephemeral** | In-memory only, discarded on close | Private tabs, incognito |

<br>

## Related

- [VOLT Architecture — Chapter 13: Peregrine Spec](../architecture/volt-ch13-peregrine.md)
- [VOLT Architecture — WebContentsView Migration](../architecture/volt-webview.md)
- [Kernel WebSocket — Browser Integration](../apis/kernel-ws.md#browser)

<br>

---

[← Back: Documentation Index](../index.md)
