# Tesseract Engine — Kernel B

Samaris OS Kernel B — Native Rust Acceleration Daemon.

## Architecture

```
┌─────────────────────────────────────────────────┐
│                  TESSERACT ENGINE                │
│                                                   │
│  ┌──────────┐  ┌──────────┐  ┌────────────────┐ │
│  │ IPC Layer │  │Scheduler │  │ Thermal Watchdog│ │
│  │ (SBP v5)  │  │  (RR+P)  │  │  (500ms check) │ │
│  └────┬─────┘  └────┬─────┘  └───────┬────────┘ │
│       │              │                │           │
│  ┌────▼──────────────▼────────────────▼────────┐ │
│  │              Dispatch Core                   │ │
│  │  ┌────────┐ ┌────────┐ ┌──────┐ ┌────────┐  │ │
│  │  │  GPU   │ │Compute │ │Media │ │System  │  │ │
│  │  │ Canvas │ │ Bridge │ │      │ │Metrics │  │ │
│  │  └────────┘ └────────┘ └──────┘ └────────┘  │ │
│  └────────────────┬─────────────────────────────┘ │
│                   │                               │
│  ┌────────────────▼─────────────────────────────┐ │
│  │  Security (sandbox, quotas, audit)            │ │
│  │  Telemetry (local-only metrics, profiler)     │ │
│  └───────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────┘
```

### Dual-Protocol IPC
- First byte determines protocol:
  - `0x53` ('S') — SBP v5 binary protocol (FastBuffers + FlatBuffers payload)
  - `0x4A` ('J') — JSON-RPC 2.0 (legacy health/ping only)

### SBP v5 Frame Format
```
[0..1]   magic:      0x56 0x4F ("VO")
[2]      version:    0x05
[3]      opcode:     uint8
[4..5]   flags:      uint16 (bit 0=response, bit 1=error)
[6]      priority:   uint8 (0=CRITICAL .. 4=IDLE)
[7..10]  app_id:     uint32
[11..14] payload_len: uint32 (little-endian)
[15]     checksum:   uint8 (XOR of bytes 0..14)
────────── 16-byte header ──────────
[16..]   payload:    FlatBuffer<Packet>
```

## Modules

| Module | Description |
|--------|-------------|
| `core` | Configuration, error types, boot sequence |
| `protocol` | SBP v5 opcodes, headers, FlatBuffers serialization, SPSC ring buffer |
| `ipc` | Unix Socket (primary), WebSocket (debug), Shared Memory (perf) |
| `scheduler` | Deterministic Round-Robin + Priority with 5 levels |
| `gpu_canvas` | GPU rendering commands with CPU fallback |
| `compute_bridge` | Compute tasks (hash, compress, encrypt) with buffer management |
| `media` | Video/audio processing and A/V synchronization |
| `system` | CPU, memory, thermal, and process monitoring |
| `security` | Command sandboxing, per-app quotas, audit log |
| `telemetry` | Local-only performance metrics and profiler |
| `safety` | Thermal watchdog (85/95/100°C thresholds), resource limits |

## Scheduler

5 priority levels with deterministic Round-Robin:

| Level | Max per cycle | Preempts |
|-------|:------------:|----------|
| CRITICAL (0) | unlimited | everything |
| HIGH (1) | 8 | NORMAL, LOW, IDLE |
| NORMAL (2) | 4 | LOW, IDLE |
| LOW (3) | 2 | IDLE |
| IDLE (4) | 1 | nothing |

## Opcodes

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

## Build

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Run benchmarks
cargo bench

# Run examples
cargo run --example basic_command
cargo run --example render_rect
cargo run --example compute_task
cargo run --example media_task
```

## Configuration

Config file (TOML): `/opt/volt/kernel-b/config.toml`

```toml
socket_path = "/tmp/volt-kernel-b.sock"
websocket_port = 9998
debug_mode = false
max_workers = 4
thermal_throttle_celsius = 85.0
thermal_emergency_celsius = 95.0
thermal_critical_celsius = 100.0
scheduler_tick_ms = 1
max_total_memory_mb = 1024
quota_default_max_memory_mb = 256
quota_default_max_tasks = 4
quota_default_max_commands_per_sec = 100
```

## CLI

```bash
tesseract-engine --config /path/to/config.toml
tesseract-engine --debug --socket /tmp/custom.sock
```

## Protocol Detection

The first byte of any message determines the protocol:

- `'S'` (0x53) — SBP v5 binary protocol. Read 15 more bytes for header, then payload.
- `'J'` (0x4A) — JSON-RPC 2.0. Read newline-delimited JSON. Only `health` and `ping` are supported.

### JSON-RPC Legacy

```json
--> {"jsonrpc":"2.0","id":"1","method":"health"}
<-- {"jsonrpc":"2.0","id":"1","result":{"ok":true,"service":"tesseract-engine","version":"1.0.0-alpha"}}

--> {"jsonrpc":"2.0","id":"2","method":"ping"}
<-- {"jsonrpc":"2.0","id":"2","result":{"pong":true}}
```

## Thermal Watchdog

| Temperature | Action |
|:-----------:|--------|
| > 85°C | Throttle to 50% (skip IDLE and LOW tasks) |
| > 95°C | Release reserved cores, send CRITICAL alert |
| > 100°C | Emergency shutdown of Tesseract Engine |

Checked every 500ms via `/sys/class/thermal/thermal_zone*/temp`.

## Security

- Every command is validated by `CommandSandbox` (opcode, app_id, payload size)
- Per-app resource quotas (memory, tasks/sec, concurrent tasks)
- Full audit log with JSON export (configurable, default 10,000 entries)

## Telemetry

All metrics stay **local-only**. No external telemetry.

- Commands processed per second
- Average execution time (µs)
- IPC latency histogram (10 buckets: 0-10µs to >500ms)
- Per-opcode counters
- Error count

## License

Samaris OS — Mountain Lake Alpha One
