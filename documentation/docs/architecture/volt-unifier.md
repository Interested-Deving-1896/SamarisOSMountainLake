# Volt Unifier & SBP Protocol

The **Volt Unifier** is the health monitoring, module registry, and coordination subsystem of Samaris OS. Unlike a standalone service, it is deeply embedded within Kernel A as a Node.js library, running in-process with the main orchestrator. A separate `volt-unifier.service` systemd unit runs a lightweight health-check watchdog that polls the Unifier's REST endpoint.

The Unifier solves a fundamental architectural problem: without central coordination, each VOLT module optimises locally, potentially at the expense of overall system stability.

<br>

## Architecture

```
Volt Unifier
├── ModuleRegistry
├── SbpRouter (binary message routing)
├── HealthMonitor (checks every 2s)
├── HeartbeatManager
├── Bridges (8)
│   ├── DesktopBridge  — general desktop health events
│   ├── FinderBridge   — filesystem navigation events
│   ├── SettingsBridge — system configuration sync
│   ├── OrbitBridge    — AI assistant state and inference
│   ├── AirBarBridge   — system bar status indicators
│   ├── DevToolsBridge — development and debugging
│   ├── ServiceBridge  — service lifecycle events
│   └── AudioBridge    — audio playback state and volume
├── Clients (11)
│   ├── KernelBClient      — SBP v5 (Tesseract Engine)
│   ├── VrmClient          — SBP-MEM (RAM Manager)
│   ├── DwpClient          — SBP (Worker Pool)
│   ├── DwpLocalFallback   — Local fallback (no daemon)
│   ├── VgmClient          — SBP-GPU (GPU Manager)
│   ├── VumClient          — SBP-USB (USB Manager)
│   ├── AscClient          — File read + subprocess (ASC)
│   ├── BootClient         — Boot readiness coordination
│   ├── ServiceHealthClient— Health check polling
│   └── baseClient         — Shared connection lifecycle
└── Safety & Observability
    ├── AuditLog — per-action audit trail
    ├── CapabilityGuard — opcode permission enforcement
    ├── LocalOnlyGuard — network origin restriction
    └── Metrics — latency histograms, error counts, snapshots
```

<br>

## Core Responsibilities

### Module Registry

Maintains the authoritative state table for all VOLT modules. Each module is registered with:

- Module identifier (e.g., `kernel-b`, `vrm`, `vgm`, `vum`, `dwp`, `asc`)
- Connection status (online, offline, degraded, error)
- Capabilities list (e.g., `scheduling`, `adaptive-scaling`, `orbit-burst`)
- Health metadata: last heartbeat, error count, reconnect count
- Assigned IPC client reference

### Health Monitoring

- Periodic connection probes to all registered modules
- Heartbeat reception and timeout detection (configurable, default 10s)
- Degradation assessment based on error frequencies
- System-level health snapshot generation

### Metrics Aggregation

Collects and summarises telemetry from all modules: state snapshots, latency histograms for SBP operations, error counts, and resource utilisation summaries. The aggregated data feeds a dashboard endpoint (`/api/unifier/snapshot`) accessible to the desktop UI.

### Bridge Registry (8)

Bridges connect desktop-side events to the SBP protocol layer, enabling two-way communication between the React UI and Rust daemons:

| Bridge | Events Forwarded | Direction |
|--------|-----------------|-----------|
| **DesktopBridge** | Workspace state, focus changes, session status | UI ↔ Unifier |
| **FinderBridge** | Directory navigation, file selection, search queries | UI ↔ FS Service |
| **SettingsBridge** | Config changes, theme toggles, preference sync | UI ↔ Config |
| **OrbitBridge** | Inference state, model load progress, token stream | UI ↔ Orbit Runtime |
| **AirBarBridge** | System tray icons, clock, network status, battery | UI ↔ System Services |
| **DevToolsBridge** | Logs, metrics, debug commands, inspector data | UI ↔ DevTools |
| **ServiceBridge** | Service start/stop/restart events, health transitions | UI ↔ Service Manager |
| **AudioBridge** | Playback state, volume changes, device switches | UI ↔ Audio Service |

### Client Registry (11)

Clients manage IPC connections to all Rust daemons and system services:

| Client | Target | Protocol | Socket |
|--------|--------|----------|--------|
| **KernelBClient** | Tesseract Engine (SBP v5) | SBP v5 binary | `/run/samaris/volt-kernel-b.sock` |
| **VrmClient** | RAM Manager | SBP-MEM | `/run/samaris/volt-ram-manager.sock` |
| **DwpClient** | Dynamic Worker Pool | SBP | `/run/samaris/volt-worker-pool.sock` |
| **DwpLocalFallback** | Local scheduler (no daemon) | In-process | N/A — simulated |
| **VgmClient** | GPU Manager | SBP-GPU | `/run/samaris/volt-gpu-manager.sock` |
| **VumClient** | USB Manager | SBP-USB | `/run/samaris/volt-usb-manager.sock` |
| **AscClient** | Adaptive System Config | File read + subprocess | `/run/samaris/adaptive.generated.toml` |
| **BootClient** | Boot readiness | SBP event | Unix socket |
| **ServiceHealthClient** | Health checks | HTTP poll | Kernel A REST |
| **baseClient** | Base class | Shared lifecycle | — |

Each client handles:
- Connection lifecycle (connect, reconnect with backoff, disconnect)
- Message serialisation/deserialisation
- Heartbeat and timeout detection
- Error classification and recovery

### SBP Routing

Manages binary protocol communication between Kernel A and all Rust daemons:

- Message encoding and decoding with CRC32 checksum verification
- Request-response correlation with configurable timeouts
- Event subscription for asynchronous module notifications
- Capability-based permission checking for sensitive opcodes
- Stream reassembly for fragmented SBP messages

### Safety and Audit

- **Audit log**: records security-relevant actions with module ID, action type, allowed/denied status, and reason
- **Capability guard**: prevents modules from issuing SBP opcodes they are not authorised for
- **Local-only guard**: restricts certain operations to local (non-network) origins

<br>

## SBP Binary Format

| Offset | Size | Field | Description |
|--------|------|-------|-------------|
| 0 | 4 | Magic | Protocol identifier (`SBP_MAGIC`) |
| 4 | 1 | Version | Protocol version number |
| 5 | 1 | Opcode | Message type identifier |
| 6 | 2 | Flags | Bitfield: REQUEST, RESPONSE, ERROR, EVENT |
| 8 | 8 | RequestId | Unique 64-bit request identifier |
| 16 | 8 | TimestampUs | Message creation timestamp (microseconds) |
| 24 | 4 | PayloadLen | Payload length in bytes (0–65536) |
| 28 | 4 | Checksum | CRC32 of bytes 0–27 + payload |
| 32 | N | Payload | Message body (opaque byte sequence) |

### Opcodes

| Opcode | Name | Purpose |
|--------|------|---------|
| 0x01 | HELLO | Connection handshake |
| 0x02 | HEARTBEAT | Service liveness signal |
| 0x03 | MEM_PRESSURE | Memory pressure notification |
| 0x04 | MEM_QUOTA | Per-application quota state |
| 0x05 | GPU_PRESSURE | GPU/VRAM pressure signal |
| 0x06 | WORKER_STATE | Worker pool utilisation report |
| 0x07 | BOOT_READY | Service readiness signal |
| 0x08 | RECOVERY | Recovery mode activation |
| 0x09 | SHUTDOWN | Graceful shutdown request |

### Flags

- **REQUEST** (bit 0): message expects a response
- **RESPONSE** (bit 1): message is a response to a prior request
- **ERROR** (bit 2): message indicates an error condition
- **EVENT** (bit 3): asynchronous event notification

### SBP-MEM Extension

SBP-MEM is a protocol extension for memory management communication between VRM and the Unifier. It defines structured messages for memory pressure levels, per-application quota utilisation, compression pool statistics, page migration events between tiers, and recommended system actions.

<br>

## Communication Flow

```
Kernel A ↔ Volt Unifier ↔ SBP Router ↔ Unix Socket ↔ Rust Daemon
     │                             │
     │  REST/WS                    │  subscription/event
     ▼                             ▼
  Desktop UI               Event Bus → Bridges → WebSocket push
```

<br>

## Related

- [SBP Protocol Reference](../apis/sbp-protocol.md)
- [Volt Daemons](volt-daemons.md)
- [Kernel A — Node.js](kernel-node.md)

<br>

---

[← Back: Architecture Overview](overview.md)
