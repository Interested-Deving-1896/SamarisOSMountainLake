# Architecture Overview

Samaris OS is a layered operating system combining a **Debian Trixie** base with a **three-layer architecture**: the Interface Layer (React), the Runtime Layer (Electron + Node.js), and the System Layer (six Rust daemons).

<br>

## Three-Layer Architecture

### Layer A — Interface Layer

The user-facing presentation layer. Built with React, it renders the desktop environment, application windows, system dialogs, and peripheral interfaces. This layer communicates exclusively through a typed preload bridge and has no direct access to native system resources.

**Technologies**: React, xterm.js, js-dos

### Layer B — Runtime Layer

The native application host. Built with Electron and Node.js, it provides the desktop window, manages the preload bridge, hosts the Kernel A orchestrator, coordinates IPC between the interface and native subsystems, and exposes controlled access to system resources through validated APIs.

**Technologies**: Electron, Node.js, WebSocket, node-pty

### Layer C — System Layer (VOLT Kernel Layer)

The native system services layer. Built with Rust, it comprises six independent daemons that manage memory, GPU, storage, computation, adaptive configuration, and low-level system acceleration. These daemons communicate via the Samaris Binary Protocol (SBP) over Unix sockets and are coordinated by the Volt Unifier in Layer B.

**Technologies**: Rust, tokio, wgpu, SBP, Unix sockets

<br>

## Component Map

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
  ┌────▼────┐  ┌────▼────┐  ┌─────▼─────┐  ┌─────▼─────┐     │
  │Tesseract │  │   VRM   │  │    VGM    │  │    VUM    │      │
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

<br>

## Communication Flow

| Direction | Transport | Protocol | Endpoint |
|-----------|-----------|----------|----------|
| UI → Kernel A | WebSocket | JSON | `ws://127.0.0.1:9999` |
| Kernel A → Rust daemons | Unix socket | SBP binary | `/run/samaris/*.sock` |
| Electron → Kernel A | HTTP REST | JSON | `http://127.0.0.1:9999/*` |
| Daemon → Daemon | None (hub-and-spoke) | — | All via Unifier |

<br>

## Data Flow Patterns

### Request-Response (UI → System)

```
React App → preload (contextBridge) → Electron IPC →
  Kernel A (WebSocket) → Handler → Service → Rust Daemon (SBP)
  → Response → Reverse path
```

### Event Broadcast (System → UI)

```
Rust Daemon (SBP event) → Volt Unifier → Event Bus →
  Kernel A → WebSocket → React state update
```

<br>

## Namespace Conventions

| Entity | Convention | Example |
|--------|-----------|---------|
| Rust daemon packages | `volt-{function}-manager` | `volt-ram-manager` |
| Tesseract Engine | `tesseract-engine` | Package name for Kernel B |
| SBP opcodes | `0x{NN}` hex | `0x01` (HELLO), `0x03` (MEM_PRESSURE) |
| Systemd services | `volt-{name}.service` | `volt-kernel-b.service` |
| Readiness signals | `/run/volt-{name}.started` | `/run/volt-kernel-b.started` |
| Configuration | `/opt/volt/{module}/config.toml` | `/opt/volt/ram-manager/config.toml` |

<br>

## Key Design Decisions

| Decision | Rationale |
|----------|-----------|
| **WebSocket** for UI | Enables future remote access (PWA, mobile client) |
| **SBP binary** protocol | Low-latency, typed, CRC32-protected IPC |
| **Rust daemons** | Memory safety, zero-cost abstractions, fine-grained control |
| **Node.js kernel** | Rapid prototyping, abundant libraries, same language as UI |
| **GGUF model format** | Consistent inference across LLM + TTS via llama.cpp |
| **No Python runtime** in ISO | Avoids 500+ MB dependency chain, keeps ISO lean |

<br>

## Deep Dives

- [Kernel A — Node.js](kernel-node.md)
- [Kernel B — Tesseract Engine (Rust)](kernel-rust.md)
- [Volt Daemons — VRM / DWP / VGM / VUM / ASC](volt-daemons.md)
- [Volt Unifier & SBP Protocol](volt-unifier.md)
- [AI Stack](ai-stack.md)
- [Electron Shell](electron-shell.md)
- [Window System](window-system.md)
- [AirBar System Panel](airbar.md)
- [Filesystem](filesystem.md)
- [Data Flow](data-flow.md)
- [Security](security.md)

<br>

---

[← Back: Documentation Index](../index.md)
