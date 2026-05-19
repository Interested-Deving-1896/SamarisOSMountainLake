# Kernel B — Tesseract Engine (Rust)

The Tesseract Engine (package name: `tesseract-engine`) is the native Rust daemon at the core of VOLT's system layer, referred to architecturally as Kernel B. It provides low-level system acceleration, boot coordination, secure IPC infrastructure, and resource management primitives that are not efficiently expressible in JavaScript. It operates as a userspace daemon with elevated scheduling priority and direct memory access capabilities.

<br>

## Role and Responsibilities

- **Boot coordinator**: executes the VOLT BOOT sequence, performing accelerated hardware checks and initialising the shared memory ring buffer
- **IPC server**: hosts Unix socket and WebSocket endpoints for structured communication with Kernel A and other modules
- **System telemetry collector**: gathers CPU, memory, and thermal metrics at configurable intervals
- **Security boundary**: isolates native memory operations from JavaScript-facing APIs
- **Command execution engine**: dispatches typed commands to registered system handlers
- **Orbit AI resource reservation**: reserves dedicated workers and memory for local LLM inference

<br>

## Communication

```
Socket:  /run/samaris/volt-kernel-b.sock
Protocol: JSON-RPC 2.0 (prefixed with "J" byte)
Debug:    WebSocket port 9998
```

**Key Methods:** `health()`, `queryCores()`, `queryGpu()`, `heartbeat()`, `thermalStatus()`, `execCpu()`, `renderGpu()`

<br>

## Subsystems

### Boot Module

Implements the deterministic VOLT BOOT sequence. Initialises the GPU canvas subsystem, allocates the shared memory ring buffer, preloads critical assets, and reports elapsed boot time and subsystem readiness. Supports Fast and Full boot modes.

### IPC Module

Three transport mechanisms:
1. **Unix sockets** (primary): SBP binary protocol with the Volt Unifier and Kernel A
2. **Shared memory** (SHM ring buffer): high-frequency telemetry data exchange
3. **WebSocket** (port 9998): debug and development connectivity

### Scheduler

A lightweight cooperative scheduler managing concurrent tasks within the Tesseract Engine. Distinct from the Dynamic Worker Pool (DWP) — the Tesseract scheduler manages internal engine tasks only.

### Safety and Security

- Emergency stop detection and propagation
- Per-task memory and command rate quotas
- Audit trail recording for security-relevant operations
- Thermal monitoring with configurable thresholds

<br>

## Architecture

```rust
struct TesseractEngine {
    scheduler: SchedulerImpl,
    safety: SafetyMonitor,
    telemetry: TelemetryCollector,
}

// Scheduler tick:  16ms (aligned to 60fps frame budget)
// Watchdog:        2000ms
// Thermal throttle: 85°C
```

<br>

## Configuration

Loaded from `/opt/volt/kernel-b/config.toml`:

| Parameter | Default | Description |
|-----------|---------|-------------|
| `socket_path` | `/run/samaris/volt-kernel-b.sock` | Unix socket location |
| `websocket_port` | 9998 | Debug/dev WebSocket port |
| `scheduler_tick_ms` | 16 | Tick rate aligned to 60fps frame budget |
| `watchdog_interval_ms` | 2000 | Health watchdog interval |
| `max_workers` | 8 | Max concurrent worker threads |
| `thermal_throttle_celsius` | 85.0 | Thermal throttling threshold |
| `thermal_emergency_celsius` | 95.0 | Emergency thermal threshold |
| `thermal_critical_celsius` | 100.0 | Critical thermal threshold |
| `max_total_memory_mb` | 1024 | Maximum total memory for engine |
| `orbit_reservation.workers` | 4 | Dedicated workers for Orbit LLM |
| `orbit_reservation.memory_mb` | 512 | Reserved memory for Orbit |
| `orbit_reservation.burst_window_ms` | 100 | Burst window for inference spikes |

<br>

## Boot Chain Integration

Kernel B is the **first** Rust daemon to start:

```
volt-kernel-b.service
  ├── volt-ram-manager.service (VRM)
  ├── volt-gpu-manager.service  (VGM)
  └── volt-usb-manager.service  (VUM)
```

The Tesseract Engine runs with `Nice=-20`, `IOSchedulingClass=realtime`, and `LimitMEMLOCK=infinity` for deterministic low-level performance.

<br>

## Relationship to Other Modules

- **Kernel A** connects as a client over Unix socket SBP
- **Volt Unifier** monitors health via heartbeat messages
- **ASC** configuration influences worker count and memory limits
- **DWP** and **VRM** operate independently but coordinate through the Unifier

<br>

## Related

- [Volt Daemons — VRM / DWP / VGM / VUM / ASC](volt-daemons.md)
- [Kernel B Configuration](../config/kernel-b.toml.md)

<br>

---

[← Back: Architecture Overview](overview.md)
