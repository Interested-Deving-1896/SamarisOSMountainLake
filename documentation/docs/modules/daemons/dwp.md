# DWP — Volt Dynamic Worker Pool

**Cooperative adaptive priority scheduler for Samaris OS.**

A Rust-based dynamic worker pool that implements cooperative multi-queue priority scheduling with adaptive scaling, desktop latency protection, Orbit burst control, and thermal backoff. Replaces per-module thread pools with a single, safe, instrumented scheduler.

<br>

## Architecture

```
DynamicWorkerPool
├── Scheduler (min/max workers, scale heuristics)
├── MultiQueue (5 priority levels: Realtime/Critical/High/Normal/Low)
├── AdaptiveScaler (queue depth, CPU utilization, thermal, desktop pressure)
├── CooperativeScheduler (yield budget, preemption)
├── DesktopGuard (frame pressure, latency guard, protection)
├── OrbitBurstControl (cooldown, concurrent limits, rate limits)
├── ModuleRegistry (adapter proxies for Samaris modules)
├── ThermalMonitor (sensor reading, backoff)
├── SafetyGuards (invariants, bounds checking)
├── MetricsCounters (throughput, latency histograms, events)
└── Worker Pool (n workers: Idle → Busy → Idle / Draining → Stopped)
```

<br>

## Priority Levels

| Level | Value | Samaris Modules | Behaviour |
|-------|-------|----------------|-----------|
| Realtime | 4 | (reserved) | Highest; always dequeued first |
| Critical | 3 | orbit | Before High, Normal, Low |
| High | 2 | desktop, kernel_b, kernel_a | Before Normal, Low |
| Normal | 1 | electron, vum, vgm | Default priority |
| Low | 0 | vrm, background | Only when higher queues empty |

Strict priority dequeuing — no weighted fair queuing.

<br>

## Adaptive Scaling

The `AdaptiveScaler` evaluates four inputs on each tick:

1. **Queue depth** — total pending jobs
2. **CPU utilization** — system-wide or process-level (0.0–1.0)
3. **Thermal state** — Normal, Warm, Hot, Critical
4. **Desktop pressure** — frame-time-derived (0.0–1.0)

### Scale Up Condition
- Queue depth > workers × 2.0 AND CPU < 80%
- Cooldown elapsed (5000ms) AND thermal not throttled
- Workers below maximum

### Scale Down Condition
- Workers above minimum AND queue depth < workers × 1.0
- CPU < 30% AND cooldown elapsed
- Thermal not Critical

### Worker Bounds
- **Default**: `min = max(2, cores/3)`, `max = max(min, cores × 3/4)`
- **Override**: Config `min_workers_override`, `max_workers_override`
- **Cap**: 48 workers absolute maximum

<br>

## Desktop Guard

Protects UI responsiveness by monitoring frame times and back-pressuring the scheduler.

### Frame Pressure Levels

| Level | Range | Action |
|-------|-------|--------|
| None | ≤ 0.10 | No restriction |
| Low | 0.11–0.30 | Normal operation |
| Medium | 0.31–0.60 | Latency guard may trigger |
| High | 0.61–0.85 | Background reduced, orbit burst may block |
| Critical | 0.86–1.00 | Orbit burst blocked, background reduced |

**Latency Guard**: Tracks 60-frame rolling window. Triggers when average frame time exceeds `frame_budget - latency_guard_ms`.

<br>

## Orbit Burst Control

Orbit AI inference can request temporary priority escalation:

| Constraint | Default | Behaviour |
|-----------|---------|-----------|
| Cooldown | 2000ms | Minimum between bursts |
| Concurrent | 5 max | Simultaneous active bursts |
| Rate limit | 10/min | Max bursts per rolling minute |
| Desktop gate | > 0.6 blocked | Frame pressure protects desktop |
| Thermal gate | Hot/Critical blocked | Temperature safety |

<br>

## Cooperative Preemption

No kernel-level preemption. Jobs call `yield_point()` periodically:

1. Check cancellation → return `Cancelled`
2. Budget exhausted → return `BudgetExhausted`
3. Force preempt elapsed → return `Preempted`
4. Context saved, job re-queued or rescheduled

Yield budget: 50µs per quantum. Force preempt after configurable duration.

<br>

## Fairness & Anti-Starvation

- **Aging**: Tracks wait time after 200ms
- **Starvation limit**: 1000ms — guaranteed scheduling
- **Priority boost**: Starved jobs temporarily elevated to High priority
- **Fairness snapshot**: starvation_count, boost_count, max_wait_time_ms

<br>

## Safety Invariants

| Invariant | Description |
|-----------|-------------|
| Workers >= min | Never below configured minimum |
| Workers <= max | Never above configured maximum |
| Desktop >= min | Desktop reservation always honoured |
| No busy kill | Only idle/draining/stopped workers killed |
| Valid transitions | Pool lifecycle validates all state changes |
| Non-negative metrics | All counters ≥ 0 |
| Thermal safety | Scale-up blocked when throttled |

<br>

## Integration with Samaris OS

The pool operates in four phases:

1. **Standalone** — CLI simulations, no module binding
2. **Adapter-Ready** — Module adapters registered (kernel_b, kernel_a, desktop, orbit, vrm, vum, vgm, background)
3. **Partial Integration** — Kernel B, Desktop, Orbit connect via adapters
4. **Full Runtime** — All modules submit work through the pool

<br>

## Configuration

See [`dwp.toml`](../../config/dwp.toml.md) for the complete configuration reference.

Key settings:
- Min/max workers and scaling thresholds
- Desktop frame budget and latency guard
- Orbit burst cooldown, concurrent limits, rate limits
- Thermal backoff behaviour
- Priority mappings per module
- Fairness and aging parameters
- Metrics and latency histogram configuration
