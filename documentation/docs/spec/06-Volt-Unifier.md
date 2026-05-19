# 6. Volt Unifier

## 6.1 Overview

The Volt Unifier is the health monitoring, module registry, and coordination subsystem of Samaris OS. Unlike a standalone service, it is deeply embedded within Kernel A as a Node.js library, running in-process with the main orchestrator. A separate `volt-unifier.service` systemd unit runs a lightweight health-check watchdog that polls the Unifier's REST endpoint, but the Unifier's core logic lives inside Kernel A.

The Unifier solves a fundamental architectural problem: without central coordination, each VOLT module optimises locally, potentially at the expense of overall system stability. The Unifier provides global observability and coordinated response.

## 6.2 Core Responsibilities

### Module Registry

The registry maintains the authoritative state table for all VOLT modules. Each module is registered with:

- Module identifier (e.g., `kernel-b`, `vrm`, `vgm`, `vum`, `dwp`, `asc`)
- Connection status (online, offline, degraded, error)
- Capabilities list (e.g., `scheduling`, `adaptive-scaling`, `orbit-burst` for DWP)
- Health metadata: last heartbeat, error count, reconnect count, degraded reason
- Assigned IPC client reference

### Health Monitoring

The health monitor continuously evaluates system-wide status:

- Periodic connection probes to all registered modules
- Heartbeat reception and timeout detection
- Degradation assessment based on error frequencies and missing heartbeats
- System-level health snapshot generation

### Metrics Aggregation

The metrics aggregator collects and summarises telemetry from all modules:

- Module state snapshots
- Latency histograms for SBP operations
- Error counts and patterns
- Resource utilisation summaries

The aggregated data feeds a dashboard endpoint (`/api/unifier/snapshot`) accessible to the desktop UI for real-time system monitoring.

### SBP Routing

The SBP router manages binary protocol communication between Kernel A and all Rust daemons:

- Message encoding and decoding with CRC32 checksum verification
- Request-response correlation with configurable timeouts
- Event subscription for asynchronous module notifications
- Capability-based permission checking for sensitive opcodes
- Stream reassembly for fragmented SBP messages

### Bridge Layer

Bridges translate between the Unifier's internal state and the desktop UI's communication patterns. Six bridges are defined:

- **DesktopBridge**: general desktop health and status
- **FinderBridge**: filesystem navigation events
- **SettingsBridge**: system configuration synchronisation
- **OrbitBridge**: AI assistant state and inference events
- **DevToolsBridge**: development and debugging utilities
- **AirBarBridge**: system bar status indicators

### Lifecycle Management

The lifecycle manager coordinates controlled startup and shutdown:

- Module readiness tracking with dependency awareness
- Ordered shutdown with timeout enforcement
- Connection retry with exponential backoff
- Graceful handling of module disconnection and reconnection

### Safety and Audit

The safety subsystem provides:

- **Audit log**: records security-relevant actions with module ID, action type, allowed/denied status, and reason
- **Capability guard**: prevents modules from issuing SBP opcodes they are not authorised for
- **Local-only guard**: restricts certain operations to local (non-network) origins

## 6.3 Module IPC Clients

The Unifier maintains direct IPC client connections to each Rust daemon:

| Client | Target | Protocol | Purpose |
|--------|--------|----------|---------|
| KernelBClient | Tesseract Engine | SBP Unix socket | System acceleration, boot status |
| VrmClient | RAM Manager | SBP Unix socket | Memory pressure, quotas, compression state |
| VumClient | USB Manager | SBP Unix socket | Storage status, journal health |
| VgmClient | GPU Manager | SBP Unix socket | GPU profile, VRAM pressure, thermal state |
| DwpClient | Worker Pool | SBP Unix socket | Worker utilisation, queue depth, thermal backoff |
| AscClient | ASC (CLI) | File read + subprocess | Hardware config profile, generated policy |

## 6.4 Communication Flow

```
Kernel A ↔ Volt Unifier ↔ SBP Router ↔ Unix Socket ↔ Rust Daemon
     │                             │
     │  REST/WS                    │  subscription/event
     ▼                             ▼
  Desktop UI               Event Bus → Bridges → WebSocket push
```

## 6.5 Integration Pattern

The Unifier is not a daemon — it is a library instantiated by Kernel A. It shares the Kernel A process space and event loop. The separate `volt-unifier.service` is a simple shell script that periodically checks the Unifier's HTTP health endpoint and reports failures to journald, providing an additional layer of process-level monitoring.
