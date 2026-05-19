# 2. Architectural Overview

## 2.1 Three-Layer Architecture

VOLT operates across three complementary layers that together form the complete Samaris OS software stack.

### Layer A — Interface Layer

The user-facing presentation layer. Built with React, it renders the desktop environment, application windows, system dialogs, and peripheral interfaces (dock, AirBar, Finder, settings, Peregrine browser, terminal). This layer communicates exclusively through a typed preload bridge and has no direct access to native system resources.

**Technologies**: React, xterm.js, js-dos (DOSBox emulation)

### Layer B — Runtime Layer

The native application host. Built with Electron and Node.js, it provides the desktop window, manages the preload bridge, hosts the Kernel A orchestrator, coordinates IPC between the interface and native subsystems, and exposes controlled access to system resources through validated APIs.

**Technologies**: Electron, Node.js, WebSocket, node-pty

### Layer C — System Layer (VOLT Kernel Layer)

The native system services layer. Built with Rust, it comprises six independent daemons that manage memory, GPU, storage, computation, adaptive configuration, and low-level system acceleration. These daemons communicate via the Samaris Binary Protocol (SBP) over Unix sockets and are coordinated by the Volt Unifier module running in Layer B.

**Technologies**: Rust, tokio, wgpu, SBP, Unix sockets

## 2.2 Component Map

```
┌──────────────────────────────────────────────────────────────┐
│                    SAMARIS OS DESKTOP UI                     │
│   React / Peregrine Browser / Terminal / Apps / AirBar      │
└──────────────────────────────┬───────────────────────────────┘
                               │  contextBridge / WebSocket
┌──────────────────────────────▼───────────────────────────────┐
│                  ELECTRON NATIVE SHELL                       │
│       BrowserWindow / Preload Bridge / IPC Handlers          │
└──────────────────────────────┬───────────────────────────────┘
                               │
┌──────────────────────────────▼───────────────────────────────┐
│                  KERNEL A ORCHESTRATOR                       │
│         Node.js Server (port 9999) / Message Router          │
│          ~35 Services / 25+ Message Handlers                 │
│                        │                                      │
│              ┌─────────┴──────────┐                          │
│              │                    │                           │
│     ┌────────▼────────┐   ┌──────▼───────────┐              │
│     │  VOLT UNIFIER   │   │  VOLT SUBSYSTEMS │               │
│     │  Health Watchdog │   │  IPC Clients      │              │
│     │  Module Registry │   │  SBP Router       │              │
│     │  Metrics         │   └──────┬───────────┘              │
│     └─────────────────┘          │                            │
│                                  │ SBP over Unix Sockets     │
└──────────────────────────────────┼───────────────────────────┘
                                   │
       ┌───────────────────────────┼───────────────────────────┐
       │                           │                           │
  ┌────▼────┐  ┌────▼────┐  ┌─────▼─────┐  ┌─────▼─────┐    │
  │Tesseract │  │   VRM   │  │    VGM    │  │    VUM    │     │
  │ Engine   │  │  RAM    │  │   GPU     │  │   USB     │     │
  │ (Kernel B)│  │ Manager  │  │  Manager  │  │  Manager  │     │
  └─────────┘  └─────────┘  └───────────┘  └───────────┘     │
       │                           │                           │
  ┌────▼────┐  ┌─────────────────────────────────────────┐    │
  │   ASC   │  │   Dynamic Worker Pool (DWP)              │    │
  │ Adaptive│  │   Cooperative Priority Scheduler          │    │
  │  Config │  └─────────────────────────────────────────┘    │
  └─────────┘                                                 │
       │                                                       │
┌──────┴───────────────────────────────────────────────────────┘
│                       LINUX BASE SYSTEM
│            systemd / NetworkManager / Kernel / Drivers
└──────────────────────────────────────────────────────────────
```

## 2.3 Data Flow Patterns

### Request-Response (UI → System)

```
React App  →  preload (contextBridge)  →  Electron IPC  →
  Kernel A (WebSocket)  →  Handler  →  Service  →  Rust Daemon (SBP)
  →  Response  →  Reverse path
```

### Event Broadcast (System → UI)

```
Rust Daemon (SBP event)  →  Volt Unifier  →  Event Bus  →
  Kernel A  →  WebSocket  →  React state update
```

### Direct Native Access (Electron → System)

```
Electron Main Process  →  IPC Handler  →  node-pty (terminal)
  →  Linux shell process  →  Streaming PTY output
```

## 2.4 Namespace Conventions

| Entity | Naming Convention | Example |
|--------|-------------------|---------|
| Rust daemon packages | `volt-{function}-manager` | `volt-ram-manager` |
| Tesseract Engine | `tesseract-engine` | Package name for Kernel B |
| SBP opcodes | `0x{NN}` hex | `0x01` (HELLO), `0x03` (MEM_PRESSURE) |
| Systemd services | `volt-{name}.service` | `volt-kernel-b.service` |
| Readiness signals | `/run/volt-{name}.started` | `/run/volt-kernel-b.started` |
| Configuration | `/opt/volt/{module}/config.toml` | `/opt/volt/ram-manager/config.toml` |

## 2.5 Inter-Module Dependencies

The six Rust daemons and two Node.js processes are not fully meshed. Communication follows a hub-and-spoke pattern centered on Kernel A and the Volt Unifier:

- **Tesseract Engine** (Kernel B) communicates directly with Kernel A via Unix socket
- **VRM, VGM, VUM, DWP** each communicate with Kernel A → Volt Unifier via SBP over Unix sockets
- **ASC** is a oneshot generator; its output is read by all other modules from a shared filesystem path
- **Rust modules do not communicate directly with each other** — all cross-module coordination passes through the Volt Unifier's SBP router or through shared configuration files
