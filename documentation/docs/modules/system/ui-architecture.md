# Desktop UI Architecture

**React 18 + TypeScript desktop shell — glass-design UI, window manager, dock**

The Samaris OS desktop UI is a single-page React application that renders a fullscreen desktop experience. It communicates with Kernel A via WebSocket and with the Electron shell via a `contextBridge` IPC API.

<br>

## Source Layout

```
ui/src/
├── main.tsx                Entry point — initialises all system stores, renders Desktop
├── App.tsx                 Root component with routing
│
├── components/             Shared UI components
│   ├── Desktop.tsx         Root shell component
│   ├── DesktopIcons.tsx    Desktop icon grid
│   ├── Dock.tsx            Application dock / taskbar
│   ├── Window.tsx          Window chrome (title bar, resize, controls)
│   ├── LockScreen.tsx      Authentication screen
│   ├── Spotlight.tsx       Spotlight search overlay
│   ├── ContextMenu.tsx     Right-click context menu
│   └── PromptModal.tsx     System prompt dialogs
│
├── shell/                  Shell layout components
│   ├── dock/               Dock layout and styling
│   ├── topbar/             Top bar (menu bar, clock, status)
│   └── windowing/          Window management layer
│
├── apps/                   50+ built-in application components
│   ├── Finder.tsx          Filesystem browser
│   ├── Peregrine.tsx       Web browser
│   ├── Terminal.tsx        Terminal emulator
│   ├── Settings.tsx        System settings
│   ├── Orbit/              AI assistant
│   ├── Bench.tsx           Performance benchmark UI
│   ├── Music.tsx           Music player
│   ├── Photos.tsx          Photo viewer
│   ├── Videos.tsx          Video player
│   ├── Mail.tsx            Email client
│   ├── PdfViewer.tsx       PDF viewer
│   ├── AppStore.tsx        Application store
│   ├── Network.tsx         WiFi/BT manager
│   ├── Firewall.tsx        Firewall config
│   ├── Encryption.tsx      Encryption manager
│   ├── PermissionsManager.tsx  App permissions
│   ├── TextEditor.tsx      Text/code editor
│   ├── Notes.tsx           Note-taking app
│   ├── Archive.tsx         Archive manager
│   ├── Doom.tsx            DOOM (jsdos)
│   ├── Downloads.tsx       Download manager
│   ├── Trash/              Trash bin
│   ├── Print.tsx           Print manager
│   └── ...                 (30+ more apps)
│
├── modules/                System modules
│   ├── icons/              Samaris icon system (providers, sets)
│   ├── airbar/             AirBar system panel
│   ├── onboarding/         First-boot onboarding wizard
│   ├── user-menu/          User menu dropdown
│   ├── system-panels/      System status panels
│   ├── window-system/      Window manager logic (z-order, focus)
│   └── wine/               Wine integration UI
│
├── system/                 System state stores (Zustand)
│   ├── theme/              Theme system (light/dark, accent, density)
│   ├── windowing/          Window state, sizing, close guards
│   ├── boot/               Boot sequence, splash, readiness
│   ├── audio/              Audio playback state
│   ├── battery/            Battery monitoring
│   ├── connectivity/       Network/BT state
│   ├── cursor/             Custom cursor engine
│   ├── session/            Session, login, security
│   ├── clipboard/          Clipboard management
│   ├── dock/               Dock state and configuration
│   ├── downloads/          Download tracking
│   ├── wallpaper/          Wallpaper management
│   ├── sounds/             System sounds
│   ├── update/             Update checking
│   └── dev/                Developer tools
│
├── os/                     OS abstraction layer
│   ├── kernel/             WebSocket client → Kernel A
│   ├── apps/               App registry and lifecycle
│   ├── filesystem/         Virtual filesystem client
│   ├── dnd/                Drag and drop provider
│   └── core/               Core OS abstractions
│
├── hooks/                  React hooks
│   ├── useClipboard.ts     Clipboard hook
│
├── services/               UI-level service clients
│   ├── fs/                 Filesystem API client
│   └── kernel/             Kernel API client
│
├── windows/                Window content components
│   └── finder/             Finder window implementation
│
├── assets/                 Images, icons, fonts
├── effects/                Visual effects (blur, glass, transitions)
├── styles/                 Global CSS (global.css, cursors.css)
└── test/                   Test setup and utilities
```

<br>

## Initialization Sequence

```
main.tsx:
  1. kernelClient.connect()       — WebSocket → Kernel A (:9999)
  2. themeStore.init()            — Load saved theme preference
  3. wallpaperStore.init()        — Load wallpaper config
  4. connectivityStore.init()     — Start network monitoring
  5. audioStore.init()            — Init audio system
  6. batteryStore.init()          — Start battery polling
  7. securityStore.init()         — Session security checks
  8. systemSounds.init()          — Load system sound effects
  9. installSam harnessisSystemApi()  — Install window.__samaris__
  10. initScaleEngine()           — Init VDM scale/density engine
  11. downloadStore.init()        — Download tracking (Electron only)
  12. ReactDOM render:
      └─ RootErrorBoundary
         └─ BootProvider
            └─ CursorEngine
               └─ SamarisIconProvider
                  └─ DndProvider
                     └─ Desktop
```

<br>

## Desktop Component Structure

```
<Desktop>
  ├─ <LockScreen />               Authentication overlay
  ├─ <DesktopIcons />             File/folder icons on desktop
  ├─ <Dock />                     Application dock / taskbar
  ├─ <TopBar />                   Menu bar, clock, status icons
  ├─ <WindowManager>              Z-order, focus management
  │   ├─ <Window>                 Application windows
  │   │   └─ <AppContent />       App-specific React component
  │   └─ ...
  ├─ <Spotlight />                Search overlay (Cmd+Space)
  ├─ <AirBar />                   System settings panel
  ├─ <ContextMenu />              Right-click context menu
  └─ <PromptModal />              System prompt dialogs
```

<br>

## Kernel Communication

The UI communicates with Kernel A via a persistent WebSocket connection:

```typescript
// ui/src/os/kernel/kernelClient.ts
kernelClient.send("fs.readDir", { path: "/home/user" })
  .then((result) => console.log(result));
```

- **Transport**: WebSocket to `ws://127.0.0.1:9999`
- **Protocol**: JSON messages with `type`, `payload`, and optional `requestId`
- **Events**: Kernel A pushes events (display.changed, network.status, apps.state)
- **Rate limit**: 60 messages/second/client

In non-Electron (browser dev) mode, the UI falls back to direct HTTP calls.

<br>

## Window System

The window system (`modules/window-system/`) manages:
- **Z-order**: Stacking order with focus tracking
- **Resizing**: Desktop-sizing engine with respect to dock/airbar
- **Snapping**: Window snap regions
- **Minimisation**: Window state preservation
- **Animations**: Open/close/minimize transitions
- **Window guards**: Prevents close on unsaved work

<br>

## Theme System

The theme system (`system/theme/`) provides:
- **Color scheme**: Light, dark, auto (follows system)
- **Accent color**: User-selectable accent
- **Density**: Compact, comfortable, spacious
- **Font scale**: 0.8x to 1.5x in 0.1 increments
- **CSS custom properties**: All tokens exposed via `--samaris-*` variables
- **VDM integration**: Scale factor from VDM display detection influences CSS `--scale`

<br>

## Icons

The icon system (`modules/icons/`) provides:
- **SamarisIconProvider**: Context provider wrapping all icon usage
- **Icon sets**: Built-in SVG icons with automatic variant switching
- **Theme-aware**: Icons adapt to light/dark theme
- **Performance**: SVG sprite sheets with lazy loading
