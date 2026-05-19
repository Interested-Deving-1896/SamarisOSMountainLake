# 19. Observability and Metrics

## 19.1 Overview

Observability is a first-class concern in VOLT. Every component is designed to expose structured telemetry about its state, performance, and health. This data flows through a centralised aggregation pipeline in the Volt Unifier and is made available to the desktop UI, system administrators, and developers through multiple channels.

## 19.2 Telemetry Sources

| Source | Data | Collection Method |
|--------|------|-------------------|
| Tesseract Engine | CPU load, memory usage, task throughput, IPC latency | SBP heartbeat payloads, internal metrics |
| VRM | Compression ratios, tier utilisation, pressure zone, GC activity | SBP-MEM messages, internal metrics |
| VGM | VRAM usage, shader cache hit rate, thermal state, frame budget | SBP heartbeat payloads |
| VUM | Cache utilisation, writeback queue depth, journal health | SBP heartbeat payloads |
| DWP | Worker count, queue depth, thermal state, scheduler latency | SBP heartbeat payloads |
| Kernel A | Service status, connection count, request throughput | Internal metrics |
| Electron | Window state, renderer health, IPC latency | Runtime observation |

## 19.3 Metrics Aggregation

The Volt Unifier's MetricsAggregator collects and processes telemetry from all sources:

- **Polling**: periodically requests state snapshots from each module via SBP
- **Event-driven**: subscribes to asynchronous events (pressure changes, state transitions)
- **Normalisation**: converts module-specific data into a unified metric schema
- **Windowing**: maintains sliding windows for trend analysis

### Health Snapshot

The `getHealthSnapshot()` method produces a system-wide health assessment:

- **overallStatus**: healthy, degraded, critical, unknown
- Per-module status: online, offline, degraded, error
- Degradation reasons for modules in non-healthy state
- Timestamp and collection duration

### Dashboard Snapshot

The `getDashboardSnapshot()` method provides a detailed operational view:

- All module states with capabilities and metadata
- Aggregate resource utilisation estimates
- Error counts and patterns
- Latency metrics for IPC operations
- Recent health transitions

## 19.4 Dashboard Feed

The DashboardFeed transforms metrics into UI-consumable events:

- Produces structured, timestamped events for each metric change
- Normalises event format for consistent UI rendering
- Supports filtering by module, severity, and time range
- Pushes events to the desktop UI via WebSocket

## 19.5 REST Endpoints

Kernel A exposes observability endpoints:

| Endpoint | Content | Use Case |
|----------|---------|----------|
| `/health` | `{"ok": true, "service": "volt-kernel"}` | Liveness probe |
| `/api/unifier/health` | System health snapshot | Overview dashboard |
| `/api/unifier/snapshot` | Full metrics dashboard | Detailed monitoring |
| `/api/unifier/modules` | Per-module status listing | Module state inspection |

## 19.6 SBP Telemetry

Low-level telemetry flows through SBP messages:

- HEARTBEAT (0x02): periodic liveness with status payload
- MEM_PRESSURE (0x03), GPU_PRESSURE (0x05): resource pressure events
- WORKER_STATE (0x06): worker pool utilisation

SBP telemetry is binary-encoded for minimal overhead and is processed by the Unifier's SBP router before entering the metrics aggregation pipeline.

## 19.7 Logging Architecture

All VOLT components log to systemd journald:

- **Rust daemons**: structured tracing via `tracing` crate, output to journald
- **Kernel A**: custom `[SAMARIS]` prefix logger with info/warn/error levels
- **Electron**: standard `console.log` with `[samaris]` prefix

For development and debugging, additional detail is available via environment variables:

- `RUST_LOG=debug` — Rust daemon verbose logging
- `NODE_ENV=development` — Kernel A development mode with DevTools

## 19.8 Operational Commands

```bash
# Systemd service status
systemctl status volt-kernel-b.service
systemctl status volt-desktop.service

# Journal log inspection
journalctl -u volt-kernel-b.service -b
journalctl -u volt-desktop.service -b
journalctl -u volt-kernel.service -b

# Readiness signals
cat /run/volt-*.started

# Service failure inspection
systemctl --failed

# ASC hardware profile
cat /var/lib/samaris/asc/last-hardware-profile.json

# Check Kernel A health
curl http://127.0.0.1:9999/health
curl http://127.0.0.1:9999/api/unifier/snapshot
```
