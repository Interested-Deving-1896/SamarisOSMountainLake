# 5. Kernel A Orchestrator

## 5.1 Overview

Kernel A is the Node.js-based orchestration layer of Samaris OS. It operates as an HTTP and WebSocket server bound to `127.0.0.1:9999` and serves as the central communication hub between the React desktop interface, the Electron native shell, and the native Rust daemons. Kernel A is distinct from the Tesseract Engine (Kernel B) — it operates at a higher abstraction level, managing application logic, user sessions, service orchestration, and business logic.

## 5.2 Architecture

Kernel A is structured in four layers:

### Core Layer

Provides foundational services:

- **Kernel class**: the central runtime singleton that initialises and owns all subsystems
- **Logger**: structured logging with `[SAMARIS]` prefix, supporting info, warn, and error levels
- **Event bus**: publish-subscribe event system for internal component communication
- **IPC utilities**: helper functions for inter-process message handling
- **Authentication**: session management and credential validation
- **Scheduler**: internal process scheduling and task queuing

### Router Layer

The message router parses incoming WebSocket messages by type and dispatches them to the appropriate handler module. It supports:

- Request-response patterns with optional `requestId` correlation
- Event streaming via the stream channel
- Error handling and structured error responses
- Rate limiting (60 messages/second/client)

### Handler Layer

Each message type is handled by a dedicated handler module. The following domains are covered:

- **App lifecycle**: launch, suspend, resume, terminate applications
- **Archive operations**: create, extract, inspect compressed archives
- **Audio playback**: stream control, volume, metadata
- **Battery monitoring**: charge level, status, estimated time
- **Device enumeration**: connected peripherals, hardware inventory
- **Disk operations**: partition info, usage statistics
- **Encryption**: file and volume encryption management
- **Filesystem**: file read/write/delete, directory listing, MIME type detection
- **Firewall**: rules, ports, network filtering
- **Mail**: IMAP/SMTP email client integration
- **Media metadata**: audio, video, and image metadata extraction
- **Network**: WiFi scanning, connection management, interface status
- **Onboarding**: first-boot setup wizard
- **Orbit AI**: LLM inference requests, context management
- **Power management**: shutdown, reboot, suspend
- **Printing**: print job management
- **Process management**: process listing, termination, resource usage
- **Runtime lifecycle**: service start/stop/restart
- **Search**: full-text search across files and metadata
- **Session management**: user session lifecycle
- **Storage**: block device and mount management
- **Speech-to-text**: audio transcription via Whisper
- **System information**: OS, kernel, hardware queries
- **Text-to-speech**: text-to-speech synthesis via OuteTTS
- **User management**: user accounts, settings, preferences
- **Wine compatibility**: Windows application execution via Wine

### Service Layer

Handlers delegate to specialised service modules that encapsulate specific business logic:

- 30+ service modules manage domains including filesystem virtualisation, app store (GitHub-backed), browser orchestration, window management, permission enforcement, and system metrics collection

## 5.3 Communication Channels

| Direction | Transport | Protocol | Endpoint |
|-----------|-----------|----------|----------|
| UI → Kernel A | WebSocket | JSON | `ws://127.0.0.1:9999` |
| Kernel A → Rust daemons | Unix socket | SBP binary | `/run/samaris/*.sock` |
| Electron → Kernel A | HTTP REST | JSON | `http://127.0.0.1:9999/*` |

## 5.4 Desktop Bridges

Kernel A exposes REST endpoints for desktop-facing functionality:

- `/health` — service liveness check
- `/api/unifier/health` — Volt Unifier system health snapshot
- `/api/unifier/snapshot` — full metrics and module state dashboard
- `/api/unifier/modules` — per-module status listing
- `/api/peregrine/open` — browser URL launch
- `/api/peregrine/status` — browser availability status
- `/api/fs/read-file` — file streaming for media playback with range support

## 5.5 Startup Sequence

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
