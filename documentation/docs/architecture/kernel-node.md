# Kernel A — Node.js

The central orchestration layer written in **Node.js**. Acts as the HTTP and WebSocket server bound to `127.0.0.1:9999` and serves as the central communication hub between the React desktop interface, the Electron native shell, and the native Rust daemons. Kernel A is distinct from the Tesseract Engine (Kernel B) — it operates at a higher abstraction level, managing application logic, user sessions, service orchestration, and business logic.

<br>

## Four-Layer Architecture

### Core Layer

Provides foundational services:

- **Kernel class**: the central runtime singleton that initialises and owns all subsystems
- **Logger**: structured logging with `[SAMARIS]` prefix, supporting info, warn, and error levels
- **Event bus**: publish-subscribe event system for internal component communication
- **IPC utilities**: helper functions for inter-process message handling
- **Authentication**: session management and credential validation
- **Scheduler**: internal process scheduling and task queuing

### Router Layer

The message router parses incoming WebSocket messages by `type` and dispatches them to the appropriate handler module. It supports:

- Request-response patterns with optional `requestId` correlation
- Event streaming via the stream channel
- Error handling and structured error responses
- Rate limiting: **60 messages/second/client**

### Handler Layer

Each message type is handled by a dedicated handler module:

| Handler Domain | File | Purpose |
|----------------|------|---------|
| App lifecycle | `handlers/app.js` | Launch, suspend, resume, terminate applications |
| Archive | `handlers/archive.js` | Create, extract, inspect compressed archives |
| Audio playback | `handlers/audio.js` | Stream control, volume, metadata |
| Battery | `handlers/battery.js` | Charge level, status, estimated time |
| Browser | `handlers/browser.js` | Peregrine browser integration, tab management |
| Device enumeration | `handlers/device.js` | Connected peripherals, hardware inventory |
| Disk | `handlers/disk.js` | Partition info, usage statistics |
| Display | `handlers/display.js` | Screen configuration queries, VDM integration |
| Encryption | `handlers/encryption.js` | File and volume encryption management |
| Filesystem | `handlers/fs.js` | Read/write/delete, directory listing, MIME type detection |
| Firewall | `handlers/firewall.js` | Rules, ports, network filtering |
| Mail | `handlers/mail.js` | IMAP/SMTP email client integration |
| Media metadata | `handlers/media.js` | Audio, video, and image metadata extraction |
| Network | `handlers/network.js` | WiFi scanning, connection management, interface status |
| Onboarding | `handlers/onboarding.js` | First-boot setup wizard |
| Orbit AI | `handlers/orbit.js` | LLM inference requests, context management |
| Power management | `handlers/power.js` | Shutdown, reboot, suspend |
| Printing | `handlers/print.js` | Print job management |
| Process management | `handlers/process.js` | Process listing, termination, resource usage |
| Runtime lifecycle | `handlers/runtime.js` | Service start/stop/restart |
| Search | `handlers/search.js` | Full-text search across files and metadata |
| Session management | `handlers/session.js` | User session lifecycle |
| Storage | `handlers/storage.js` | Block device and mount management |
| Speech-to-text | `handlers/stt.js` | Audio transcription via Whisper |
| System information | `handlers/system.js` | OS, kernel, hardware queries |
| Text-to-speech | `handlers/tts.js` | TTS synthesis via OuteTTS |
| User management | `handlers/user.js` | User accounts, settings, preferences |
| Wine compatibility | `handlers/wine.js` | Windows app execution via Wine |

### Service Layer

Handlers delegate to specialised service modules that encapsulate business logic: **33 service modules** manage domains including filesystem virtualisation, app store (GitHub-backed), browser orchestration, window management, permission enforcement, system metrics, audio, encryption, connectivity, mail, session persistence, and more.

### Volt Unifier Integration

Kernel A embeds the **Volt Unifier**, which provides:

| Component | Count | Purpose |
|-----------|-------|---------|
| **Bridges** | 8 | Desktop, Finder, Settings, Orbit, AirBar, DevTools, Service, Audio — bridge desktop events to SBP |
| **Clients** | 11 | KernelB, VRM, VGM, VUM, DWP, ASC, Boot, ServiceHealth, DWP Local Fallback — IPC to all Rust daemons |
| **Module Registry** | 1 | Tracks all registered modules with status, capabilities, and health |
| **SBP Router** | 1 | Binary message routing with CRC32, timeouts, event subscriptions |
| **Health Monitor** | 1 | Periodic probes, heartbeat detection, degradation assessment |
| **Metrics** | 1 | Latency histograms, error counts, resource utilisation summaries |

<br>

## Initialization

```
Kernel A initialisation:
  1. Kernel instance created
  2. All service modules registered (app, fs, network, orbit, etc.)
  3. Volt Unifier instantiated and initialised
  4. Unifier connects to Rust daemon clients
  5. ASC-generated config checked (generated if missing)
  6. HTTP/WebSocket server bound to port 9999
  7. Kernel ready for connections
```

<br>

## Communication Channels

| Direction | Transport | Protocol | Endpoint |
|-----------|-----------|----------|----------|
| UI → Kernel A | WebSocket | JSON | `ws://127.0.0.1:9999` |
| Kernel A → Rust daemons | Unix socket | SBP binary | `/run/samaris/*.sock` |
| Electron → Kernel A | HTTP REST | JSON | `http://127.0.0.1:9999/*` |

<br>

## Desktop REST Endpoints

| Endpoint | Purpose |
|----------|---------|
| `/health` | Service liveness check |
| `/api/unifier/health` | System health snapshot |
| `/api/unifier/snapshot` | Full metrics and module state dashboard |
| `/api/unifier/modules` | Per-module status listing |
| `/api/peregrine/open` | Browser URL launch |
| `/api/peregrine/status` | Browser availability status |
| `/api/fs/read-file` | File streaming for media playback with range support |

<br>

## Complete Service Reference (33 services)

| Service | File | Role |
|---------|------|------|
| App Store | `appStoreService.js` | GitHub-backed app installation and updates |
| App Static Server | `appStaticServer.js` | Serves built-in app static assets |
| Archive | `archiveService.js` | Archive creation, extraction, inspection |
| Audio | `audioService.js` | Volume control, output switching |
| Battery | `batteryService.js` | Battery status, charging, level |
| Browser | `browserService.js` | Peregrine browser orchestration, tab management |
| Connectivity | `connectivityService.js` | WiFi scan/connect/disconnect, Bluetooth |
| Dev State | `devStateService.js` | Development mode state and tooling |
| Disk | `diskService.js` | Partition info, mounting, usage statistics |
| Encryption | `encryptionService.js` | File and volume encryption management |
| Filesystem | `fileSystem.js` | Virtual filesystem with overlay + user volumes |
| Firewall | `firewallService.js` | Firewall rules, ports, network filtering |
| Kernel B Client | `kernelBClient.js` | SBP client bridge to Tesseract Engine |
| Mail | `mailService.js` | IMAP/SMTP email client integration |
| Media | `mediaService.js` | Audio/video/image metadata extraction |
| Network | `networkService.js` | Interface IP config (DHCP / Manual) |
| Onboarding | `onboardingService.js` | First-boot setup wizard logic |
| Orbit Runtime | `orbitRuntime.js` | LLM inference via llama-server (Qwen3) |
| Permission Manager | `permissionManager.js` | App permission grants and enforcement |
| Power | `powerService.js` | Power management, shutdown, reboot, suspend |
| Print | `printService.js` | Print job management |
| Process Manager | `processManager.js` | App process lifecycle |
| Runtime Manager | `runtimeManager.js` | Service start/stop/restart |
| Search | `searchService.js` | Full-text search across files and metadata |
| Session Features | `sessionFeaturesService.js` | Session persistence and feature flags |
| Speech-to-Text | `sttService.js` | Audio transcription via Whisper |
| Storage | `storageService.js` | Block device and mount management |
| System Metrics | `systemMetricsService.js` | CPU, memory, thermal, process metrics |
| Text-to-Speech | `ttsService.js` | TTS synthesis via OuteTTS / llama-tts |
| User | `userService.js` | User accounts, settings, preferences |
| Vault | `vaultService.js` | Encrypted credential storage |
| Window Manager | `windowManager.js` | Window position, state, z-index |
| Wine | `wineService.js` | Windows app execution via Wine |

<br>

## Related

- [Data Flow](data-flow.md)
- [Volt Unifier & SBP](volt-unifier.md)
- [Kernel WebSocket Protocol](../apis/kernel-ws.md)

<br>

---

[← Back: Architecture Overview](overview.md)
