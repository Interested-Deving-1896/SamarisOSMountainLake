# Kernel B (Tesseract Engine) Configuration

**Path:** `/opt/volt/kernel-b/config.toml`

This configuration is defined in the **VOLT specification (ch.4 — Tesseract Engine)**.

```toml
# Tesseract Engine — Kernel B Configuration
socket_path = "/run/samaris/volt-kernel-b.sock"
websocket_port = 9998
max_workers = 8
thermal_throttle_celsius = 85.0
thermal_emergency_celsius = 95.0
thermal_critical_celsius = 100.0
metrics_interval_ms = 1000
watchdog_interval_ms = 2000
scheduler_tick_ms = 16
max_total_memory_mb = 1024

[orbit_reservation]
workers = 4
memory_mb = 512
priority = "critical"
burst_window_ms = 100
```

<br>

## Key Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `socket_path` | `/run/samaris/volt-kernel-b.sock` | Unix domain socket for SBP communication |
| `websocket_port` | `9998` | Internal WebSocket port |
| `max_workers` | `8` | Max concurrent worker threads |
| `thermal_throttle_celsius` | `85.0` | Throttling begins at this temperature |
| `thermal_emergency_celsius` | `95.0` | Emergency thermal state |
| `thermal_critical_celsius` | `100.0` | Critical thermal shutdown threshold |
| `metrics_interval_ms` | `1000` | Metrics collection interval |
| `watchdog_interval_ms` | `2000` | Health watchdog check interval |
| `scheduler_tick_ms` | `16` | Scheduler tick aligned to 60 fps frame budget |
| `max_total_memory_mb` | `1024` | Maximum total memory for all workers |

<br>

## Orbit Reservation

| Parameter | Value | Description |
|-----------|-------|-------------|
| `workers` | 4 | Dedicated workers for Orbit AI |
| `memory_mb` | 512 | Reserved memory for Orbit contexts |
| `priority` | `critical` | Scheduling priority tier |
| `burst_window_ms` | 100 | Burst allocation window |

<br>

## Related

- [Kernel A — Node.js](../architecture/kernel-node.md)
- [Volt Daemons — Tesseract](../architecture/volt-daemons.md)
- [SBP Binary Protocol](../apis/sbp-protocol.md)

<br>

---

[← Back: Documentation Index](../index.md)
