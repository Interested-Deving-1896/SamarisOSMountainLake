# DWP — Worker Pool Configuration

**Path:** `/opt/volt/worker-pool/config.toml`

This configuration is defined in the **VOLT specification (ch.11 — DWP)**.

```toml
[worker_pool]
scheduler = "cooperative_adaptive_priority"
max_workers_cap = 48
yield_budget_us = 50
idle_timeout_ms = 5000

[worker_pool.scaling]
mode = "adaptive"
scale_cooldown_ms = 5000

[worker_pool.reservations]
desktop_min_workers = 1
orbit_default_fraction = 0.75
orbit_burst_window_ms = 100
orbit_max_consecutive_bursts = 3

[worker_pool.desktop_guard]
frame_budget_ms = 16

[worker_pool.priorities]
orbit = "critical"
desktop = "high"
kernel_b = "high"
background = "idle"
```

<br>

## Scheduler

The DWP uses a **cooperative adaptive priority** scheduler that balances workloads across available workers with yield-based preemption.

<br>

## Scaling

| Setting | Value | Description |
|---------|-------|-------------|
| Mode | `adaptive` | Worker count scales with load |
| Scale cooldown | 5000 ms | Minimum time between scale events |

<br>

## Reservations

| Reservation | Value | Description |
|-------------|-------|-------------|
| Desktop min workers | 1 | Always at least one worker for desktop |
| Orbit default fraction | 0.75 | 75% of workers reserved for Orbit by default |
| Orbit burst window | 100 ms | Burst allocation time window |
| Orbit max consecutive bursts | 3 | Limit consecutive burst allocations |

<br>

## Desktop Guard

| Setting | Value | Description |
|---------|-------|-------------|
| Frame budget | 16 ms | Maximum time for desktop frame processing |

Ensures desktop UI remains responsive by limiting Orbit worker usage when frame pressure is detected.

<br>

## Priorities

| Priority | Assignees |
|----------|-----------|
| **Critical** | Orbit AI |
| **High** | Desktop, Kernel B |
| **Idle** | Background tasks |

<br>

## Related

- [VRM — RAM Manager Configuration](vrm.toml.md)
- [Orbit AI Configuration](orbit-config.md)
- [Kernel B Configuration](kernel-b.toml.md)

<br>

---

[← Back: Documentation Index](../index.md)
