# Performance Guide — Volt Dynamic Worker Pool v2.0

## Running Benchmarks

The project uses Criterion.rs for benchmarks. Run all benchmarks:

```bash
cargo bench
```

Run a specific benchmark:

```bash
cargo bench --bench scheduling_bench
cargo bench --bench queue_bench
cargo bench --bench worker_spawn_bench
cargo bench --bench yield_point_bench
cargo bench --bench scaling_bench
cargo bench --bench orbit_burst_bench
```

Benchmarks are defined in `/benches/` with `harness = false`.

### Benchmark Descriptions

| Benchmark            | Measures                                              |
|----------------------|-------------------------------------------------------|
| `scheduling_bench`   | Job submission + dispatch throughput across priorities |
| `queue_bench`        | `MultiQueue` enqueue/dequeue latency at various depths |
| `worker_spawn_bench` | Worker thread spawn and state initialization           |
| `yield_point_bench`  | `yield_point()` overhead under varying budget states   |
| `scaling_bench`      | `AdaptiveScaler` decision latency with varied inputs   |
| `orbit_burst_bench`  | `OrbitBurstController` request throughput and cooldown |

## Metrics Measured

Runtime metrics are exported via `MetricsSnapshot` (accessible from CLI `status` or programmatically):

| Metric                      | Type    | Description                              |
|-----------------------------|---------|------------------------------------------|
| `total_jobs_submitted`      | u64     | Cumulative job submissions               |
| `total_jobs_completed`      | u64     | Cumulative completions                   |
| `total_jobs_failed`         | u64     | Cumulative failures                      |
| `total_jobs_cancelled`      | u64     | Cumulative cancellations                 |
| `total_jobs_timed_out`      | u64     | Cumulative timeouts                      |
| `active_workers`            | u32     | Currently busy workers                   |
| `idle_workers`              | u32     | Currently idle workers                   |
| `queue_depth`               | usize   | Total jobs across all priority queues    |
| `high_priority_queue_depth` | usize   | Jobs at priority ≥ High                  |
| `avg_completion_time_ms`    | f64     | Average job completion time              |
| `avg_queue_time_ms`         | f64     | Average job queue wait time              |
| `throughput_jobs_per_sec`   | f64     | Jobs completed per second                |
| `uptime_ms`                 | u64     | Pool uptime in milliseconds              |
| `yield_count`               | u64     | Total `yield_point()` calls              |
| `preemption_count`          | u64     | Total preemptions forced by budget       |
| `orbit_burst_count`         | u64     | Total orbit burst requests               |
| `scaling_events`            | u64     | Total scale up/down events               |
| `desktop_pressure`          | f64     | Current desktop frame pressure (0.0–1.0) |
| `thermal_throttle_count`    | u64     | Times thermal backoff was triggered      |
| `worker_pool_state`         | String  | Current lifecycle state                  |

### Optional Histograms

When `latency_histograms = true`:
- Per-priority-level completion time histograms
- Queue wait time histograms
- Yield point timing distribution

## Interpreting Results

### Throughput

```
Throughput = total_jobs_completed / (uptime_ms / 1000)
```

Measured in jobs/sec. Varies significantly based on:
- Job duration (benchmarks use synthetic micro-jobs)
- Worker count (more workers → higher throughput until CPU saturation)
- Priority mix (Critical-heavy loads have different throughput characteristics)
- Yield frequency (more yields = more overhead but better responsiveness)

### Latency

```
Avg Queue Time: higher values indicate pressure at the module's priority level
Avg Completion Time: dominated by job work duration, not scheduling overhead
```

Scheduling overhead per `yield_point()` call is typically < 1µs in the hot path (checked via `yield_point_bench`).

### Scaling Efficiency

```
Worker Utilization = active_workers / current_workers
```

Target range: 0.6–0.8. Below 0.6 suggests over-provisioning; above 0.8 suggests under-provisioning.

## Honest Limitations

### No 100% Efficiency Guarantee

This pool is designed for **safety and responsiveness**, not peak throughput:
- Desktop guard may deliberately leave workers idle to protect frame timing
- Cooldown timers prevent rapid scaling, causing transient queue buildup
- Strict priority scheduling can leave lower-priority queues starved until aging kicks in
- Thermal backoff trades throughput for hardware safety

### Cooperative Only

The pool **cannot preempt** running OS threads. A job that never yields will block its worker until completion:
- No watchdog kill of long-running jobs
- No enforced time-slicing
- Desktop pressure is advisory, not coercive

This means:
- All job code must call `yield_point()` regularly (every ≤ 5ms of work)
- A misbehaving job degrades the pool for everyone
- Real-time guarantees are not provided

### Single-Threaded Bottlenecks

- `MultiQueue` uses per-queue `Mutex` — under extreme contention, lock overhead appears
- `MetricsCounters` uses atomics — fast but not lock-free across all counter updates
- `ModuleRegistry` uses `RwLock` — read-heavy workloads are fine, but write storms block readers

### Configurable, Not Auto-Tuning

Default values are reasonable for a desktop-class system but may need tuning:
- `scale_cooldown_ms = 5000` is conservative (prevents thrashing but slow to react)
- `yield_budget_us = 50` is tuned for micro-task workloads
- `starvation_limit_ms = 1000` may be too short for bursty-but-rare low-priority work

### Benchmark Environment Dependence

Results vary significantly with:
- CPU generation and core count
- System thermal state during benchmark
- Kernel scheduler behavior (CFS, BFS, etc.)
- Other system load

Always benchmark on the target hardware with realistic job profiles.
