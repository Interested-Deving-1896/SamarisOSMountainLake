# 13. Peregrine Browser

## 13.1 Overview

Peregrine is the integrated native browser of Samaris OS. Unlike its predecessor (which relied on fragile iframe-based webviews), the current implementation is built on Electron's `WebContentsView` API. Peregrine provides real tab management, stable navigation, native download handling, session persistence, and comprehensive permission control — functioning as a genuine web browser within the desktop environment.

## 13.2 Architecture

Peregrine is implemented in the Electron service layer as `BrowserManager`, with a `WebContentsView` instance per open tab. The UI communicates with the browser via the preload bridge (`window.electronAPI.browser.*`).

### Tab Management

- Each tab is backed by an independent `WebContentsView`
- Tabs support: navigation, reload, stop, go back, go forward
- Tab ordering, activation, and closure are managed through IPC
- Tabs can be private (sandboxed session without persistent storage)
- Zoom factor is adjustable per tab

### Session Management

- Tabs share a default session with persistent cookies and cache
- Private tabs use ephemeral sessions with no disk writes
- Session metadata (URL, title, favicon, loading state) is synchronised to the UI via snapshot events
- Tab updates (navigation, title changes, icon loading) are pushed in real-time

### Permissions

Permission requests from web content (location, notifications, media, etc.) are intercepted and forwarded to the desktop UI for user decision. The permission model is:

```
Web Content → WebContents permission handler → IPC to UI → User decision → Response
```

## 13.3 Browser Capabilities

| Feature | Implementation |
|---------|---------------|
| URL navigation | WebContents.loadURL() |
| Tab management | Create, close, reorder, activate tabs |
| Navigation controls | Back, forward, reload, stop |
| Downloads | Native Electron download manager with progress events |
| Page saving | Save page as HTML via WebContents |
| Printing | Print dialog via WebContents.print() |
| Fullscreen | Per-tab fullscreen support |
| Favicons | Extracted from page metadata |
| History | Session-based (in-memory during Alpha) |
| DevTools | Electrum DevTools per tab (toggled via IPC) |

## 13.4 Popup Handling

All popup window requests from web content are intercepted by the Electron shell's `setWindowOpenHandler`. Instead of allowing new OS windows, popups are routed to new Peregrine tabs within the existing desktop environment. This prevents window spawning abuse and maintains desktop coherence.

## 13.5 Crash Handling

The `WebContentsView` abstraction provides inherent crash isolation — a crashed tab does not affect other tabs or the main desktop UI. The browser manager detects crash conditions and reports tab state changes through the snapshot event system.

## 13.6 Integration

Peregrine is launched through two mechanisms:

1. **Desktop UI actions**: user clicks a link or opens the browser from the dock
2. **External URL requests**: applications call `kernel.browser.launch(url)` via the Kernel A API

Browser state is exposed to the desktop UI through:

- `/api/peregrine/status` — health and availability
- `/api/peregrine/session/open`, `sync`, `close` — attached session lifecycle
- `browser:snapshot` and `browser:tab-updated` IPC events — real-time state synchronisation
