# Tesseract Engine — Kernel B

**Native Rust acceleration daemon for Samaris OS.**

Kernel B (Tesseract Engine) is the low-level Rust daemon that provides system acceleration, boot coordination, and IPC infrastructure. It communicates with Kernel A via dual-protocol IPC (SBP v5 binary + JSON-RPC 2.0 legacy) over Unix sockets.

<br>

## Architecture

```
┌──────────────────────────────────────────────────┐
│               TESSERACT ENGINE                    │
│                                                   │
│  ┌──────────┐  ┌──────────┐  ┌────────────────┐  │
│  │ IPC Layer│  │Scheduler │  │ Thermal Watchdog│  │
│  │(SBP v5)  │  │ (RR+P)   │  │  (500ms check) │  │
│  └────┬─────┘  └────┬─────┘  └───────┬────────┘  │
│       │              │                │           │
│  ┌────▼──────────────▼────────────────▼────────┐  │
│  │              Dispatch Core                   │  │
│  │  ┌────────┐ ┌────────┐ ┌──────┐ ┌────────┐  │  │
│  │  │  GPU   │ │Compute │ │Media │ │System  │  │  │
│  │  │ Canvas │ │ Bridge │ │      │ │Metrics │  │  │
│  │  └────────┘ └────────┘ └──────┘ └────────┘  │  │
│  └────────────────┬─────────────────────────────┘  │
│                   │                                │
│  ┌────────────────▼─────────────────────────────┐  │
│  │  Security (sandbox, quotas, audit)            │  │
│  │  Telemetry (local-only metrics, profiler)     │  │
│  └──────────────────────────────────────────────┘  │
└────────────────────────────────────────────────────┘
```

<br>

## Modules

| Module | Description |
|--------|-------------|
| `core` | Configuration, error types, boot sequence |
| `protocol` | SBP v5 opcodes, headers, FlatBuffers serialisation, SPSC ring buffer |
| `ipc` | Unix Socket (primary), WebSocket (debug), Shared Memory (perf) |
| `scheduler` | Deterministic Round-Robin + Priority with 5 levels |
| `gpu_canvas` | GPU rendering commands with CPU fallback |
| `compute_bridge` | Compute tasks (hash, compress, encrypt) with buffer management |
| `media` | Video/audio processing and A/V synchronisation |
| `system` | CPU, memory, thermal, and process monitoring |
| `security` | Command sandboxing, per-app quotas, audit log |
| `telemetry` | Local-only performance metrics and profiler |
| `safety` | Thermal watchdog (85/95/100°C thresholds), resource limits |

<br>

## Dual-Protocol IPC

Kernel B supports two protocols, distinguished by the first byte:

| Protocol | Magic Byte | Use Case |
|----------|-----------|----------|
| **SBP v5** | `0x53` ('S') | Primary binary protocol (FastBuffers + FlatBuffers payload) |
| **JSON-RPC 2.0** | `0x4A` ('J') | Legacy health/ping only |

### SBP v5 Frame Format

```
[0..1]   magic:      0x56 0x4F ("VO")
[2]      version:    0x05
[3]      opcode:     uint8
[4..5]   flags:      uint16 (response, error, event bits)
[6]      priority:   uint8 (0=CRITICAL .. 4=IDLE)
[7..10]  app_id:     uint32
[11..14] payload_len: uint32 (little-endian)
[15]     checksum:   XOR of bytes 0..14
────────── 16-byte header ──────────
[16..]   payload:    FlatBuffer<Packet>
```

<br>

## Scheduler

5 priority levels with deterministic Round-Robin:

| Level | Max per cycle | Preempts |
|-------|:------------:|----------|
| CRITICAL (0) | unlimited | everything |
| HIGH (1) | 8 | NORMAL, LOW, IDLE |
| NORMAL (2) | 4 | LOW, IDLE |
| LOW (3) | 2 | IDLE |
| IDLE (4) | 1 | nothing |

<br>

## Opcodes (16)

| Code | Name | Description |
|:----:|------|-------------|
| 0x01 | GPU_RENDER | Render rectangle with border-radius, shadow, blur |
| 0x02 | GPU_COMPUTE | GPU compute shader operation |
| 0x03 | CPU_RESERVE | Reserve CPU cores for an app |
| 0x04 | CPU_RELEASE | Release reserved CPU cores |
| 0x05 | CPU_EXEC | Execute compute task (hash, compress, encrypt, image) |
| 0x06 | MEM_ALLOC | Allocate memory buffer |
| 0x07 | MEM_FREE | Free memory buffer |
| 0x08 | STREAM_VIDEO | Process video frame |
| 0x09 | STREAM_AUDIO | Process audio frame |
| 0x0A | QUERY_CORES | Query CPU core count and load |
| 0x0B | QUERY_GPU | Query GPU availability |
| 0x0C | HEARTBEAT | Liveness check |
| 0x0F | THERMAL_STATUS | Query thermal zone temperatures |
| 0x30 | CONTEXT_CREATE | Create compute context |
| 0x31 | CONTEXT_SHARE | Share context with another app |

<br>

## Thermal Watchdog

| Temperature | Action |
|:-----------:|--------|
| > 85°C | Throttle to 50% (skip IDLE and LOW tasks) |
| > 95°C | Release reserved cores, send CRITICAL alert |
| > 100°C | Emergency shutdown of Tesseract Engine |

Checked every 500ms via `/sys/class/thermal/thermal_zone*/temp`.

<br>

## Security

- **Command sandbox**: Every command validated (opcode, app_id, payload size)
- **Per-app quotas**: Memory, tasks/sec, concurrent tasks
- **Audit log**: Full audit trail with JSON export (10,000 entries default)

<br>

## Configuration

See [`kernel-b.toml`](../../config/kernel-b.toml.md) for the complete configuration reference.

```toml
socket_path = "/tmp/volt-kernel-b.sock"
websocket_port = 9998
max_workers = 4
thermal_throttle_celsius = 85.0
thermal_emergency_celsius = 95.0
thermal_critical_celsius = 100.0
scheduler_tick_ms = 1
max_total_memory_mb = 1024
quota_default_max_memory_mb = 256
```

<br>

## CLI

```bash
tesseract-engine --config /path/to/config.toml
tesseract-engine --debug --socket /tmp/custom.sock
```

<br>

## Build

```bash
cd builder/content/volt-kernel-b
cargo build --release
cargo test
cargo bench
cargo run --example basic_command
cargo run --example render_rect
```

<br>

## Integration

Kernel B is the primary SBP v5 peer for the Volt Unifier. It connects via the `KernelBClient` bridge and provides low-level system services to Kernel A and applications through the dispatch core.
