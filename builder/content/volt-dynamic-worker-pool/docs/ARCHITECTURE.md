# Architecture

## Component Diagram

```
+--------------------------------------------------------------------+
|                        DynamicWorkerPool                           |
|                                                                    |
|  +------------------+  +------------------+  +-------------------+ |
|  |   Lifecycle      |  |   Scheduler      |  |   MultiQueue      | |
|  |  (State Machine) |  | (Min/Max Workers)|  | (5 Priority Queues)| |
|  +------------------+  +------------------+  +-------------------+ |
|                                                                    |
|  +------------------+  +------------------+  +-------------------+ |
|  | AdaptiveScaler   |  | CooperativeSched |  | OrbitBurstControl | |
|  | (Queue/CPU/Therm)|  | (Yield/Preempt)  |  | (Cooldown/Limits) | |
|  +------------------+  +------------------+  +-------------------+ |
|                                                                    |
|  +------------------+  +------------------+  +-------------------+ |
|  | DesktopGuard     |  | ModuleRegistry   |  | MetricsCounters   | |
|  | (FramePressure)  |  | (Adapter Proxies)|  | (Snapshot/Histo)  | |
|  +------------------+  +------------------+  +-------------------+ |
|                                                                    |
|  +------------------+  +------------------+  +-------------------+ |
|  | HardwareProbe    |  | SafetyGuards     |  | ThermalMonitor    | |
|  | (CPU/RAM/Detect) |  | (Invariants)     |  | (Backoff/Sensor)  | |
|  +------------------+  +------------------+  +-------------------+ |
|                                                                    |
|  +--------------------------------------------------------------+ |
|  |                    Worker Pool (n workers)                    | |
|  |  +---------+  +---------+  +---------+  +---------+          | |
|  |  | Worker  |  | Worker  |  | Worker  |  | Worker  |  ...    | |
|  |  | (Idle)  |  | (Busy)  |  | (Drain) |  | (Stop)  |          | |
|  |  +---------+  +---------+  +---------+  +---------+          | |
|  +--------------------------------------------------------------+ |
+--------------------------------------------------------------------+
```

## Core Modules

### Pool (`core::pool::DynamicWorkerPool`)
Central orchestrator. Owns all subsystems. Exposes the public API:
- `start()` / `shutdown()` / `state()`
- `submit_job()` / `cancel_job()` / `yield_point()`
- `set_desktop_pressure()` / `request_orbit_burst()`
- `register_module()` / `metrics()`

### Scheduler (`core::scheduler::Scheduler`)
Tracks worker counts and desktop frame pressure. Provides `should_scale_up()` and `should_scale_down()` heuristics based on queue depth and CPU utilization thresholds.

### Worker (`worker/`)
Per-worker state machine (`Idle → Busy → Idle`, `Idle → Draining → Stopped`). Each worker has an ID, lifecycle tracker, and stats counters.

### Job (`job/`)
Represents a unit of work with a `JobId`, `PriorityLevel`, name, and context. Supports cancellation via `JobContext` and cooperative yield via budget tracking.

### Priority (`priority/`)
Five-level priority system (`Low`, `Normal`, `High`, `Critical`, `Realtime`). Implements:
- `MultiQueue` — per-priority `VecDeque` with strict priority dequeuing
- `AgingPolicy` — tracks wait time, boosts starved jobs
- `FairnessPolicy` — aggregates aging + starvation detection
- `StarvationGuard` — per-job wait tracking with configurable limit
- `PriorityBoost` — temporary priority elevation with expiry

### Preemption (`preemption/`)
Cooperative preemption model. `CooperativeScheduler` manages yield budgets and force-preempt after configurable duration. `YieldPoint` is called by jobs to check cancellation and budget exhaustion. No kernel-level preemption.

### Scaling (`scaling/`)
`AdaptiveScaler` uses queue depth, CPU utilization, thermal state, and desktop pressure to decide `ScaleUp`/`ScaleDown`/`NoChange`. Cooldown prevents thrashing. `HardwareProbe` auto-detects CPU cores and RAM (with override support). `ThermalMonitor` reads `/sys/class/thermal/thermal_zone0/temp` when the `thermal` feature is enabled.

### Desktop Guard (`desktop_guard/`)
Protects desktop UI responsiveness. `FramePressure` is a five-level enum (`None`→`Critical`). `LatencyGuard` tracks frame times (rolling window of 60 samples). `DesktopProtection` ties frame pressure to orbit burst blocking and background worker reduction.

### Orbit Burst (`orbit/`)
Controls temporary priority escalation for Orbit inference. `OrbitBurstController` enforces cooldown, concurrent burst limits, and per-minute rate limits. `OrbitBurstDecision` can be `Accepted`, `RejectedCooldown`, `RejectedLimitReached`, `RejectedNoCapacity`, or a custom rejection.

### Metrics (`metrics/`)
`MetricsCounters` tracks submissions, completions, failures, cancellations, preemptions, yields, orbit bursts, scaling events, desktop pressure, and thermal throttles. `MetricsSnapshot` provides a point-in-time view. Optional latency histograms.

### Safety (`safety/`)
`SafetyGuards` wraps policy decisions: can scale down, can scale up, can burst, can kill worker. `InvariantChecker` validates worker count bounds, desktop minimums, no-busy-kill, state transitions, and non-negative metrics.

### Adapters (`adapters/`)
Module-specific adapter stubs (`kernel_b`, `kernel_a`, `desktop`, `orbit`, `vrm`, `vum`, `vgm`, `background`). Each implements `WorkerPoolAdapter`. Currently all stubs; real implementations in Phase 3.

## Data Flow

```
Job Submission:
  External Module → submit_job(job) → MultiQueue.enqueue()
    → AdaptiveScaler.should_scale_up() → spawn worker if needed

Worker Dispatch:
  MultiQueue.dequeue() → [highest priority non-empty queue]
    → Worker picks job, transitions Busy
    → Job runs with periodic yield_point() calls

Yield/Preempt:
  yield_point() → check cancellation → check budget → check preemption
    → if preempted: preempt_count++ → resume later

Desktop Pressure:
  set_desktop_pressure(pressure) → update Scheduler.desktop_frame_pressure
    → DesktopGuard checks throttling → may block orbit bursts
    → AdaptiveScaler reads pressure in scale decisions

Orbit Burst:
  request_orbit_burst(req) → check desktop pressure (< 0.6)
    → OrbitBurstController checks cooldown/limits → Accept/Reject

Scaling:
  AdaptiveScaler.should_scale_up(queue, cpu, thermal, desktop)
    → returns ScaleUp(n) / ScaleDown(n) / NoChange(reason)

Metrics Snapshot:
  metrics() → gathers scheduler state, queue depth, counters
    → returns MetricsSnapshot for CLI or adapter consumption

Shutdown:
  shutdown() → Lifecycle: Running → Draining → Shutdown
    → Workers drain current jobs → stop
```

## State Machines

### Pool Lifecycle

```
Uninitialized → Starting → Running ──────────────────────────┐
                   │            │  │   │   │                  │
                   │            v  v   v   v                  │
                   └──→ Error   ScalingUp ScalingDown Draining │
                                        │         │          │
                                        v         v          v
                                     Running    Running   Shutdown
                                                          │
                                                          v
                                                     Uninitialized
                                                      (restart)
```

### Worker Lifecycle

```
        ┌─────────────────────────────────────────┐
        │                                         │
        v                                         │
     Idle ──→ Busy ──→ Idle                       │
      │         │                                  │
      │         v                                  │
      └──→ Draining ──→ Stopped                    │
      │                                            │
      v                                            │
    Error ──→ Idle (recovery)                      │
      │                                            │
      └──→ Stopped                                 │
                                                   │
        (Worker can also go Idle → Stopped directly)
```

### Job State

```
Pending → Running → Completed
             │
             v
         Cancelled / Failed / TimedOut
```

### Orbit Burst Decision

```
Request → DesktopPressure > 0.6? → Rejected("desktop_pressure")
  ↓
  Cooldown active? → RejectedCooldown
  ↓
  Max concurrent reached? → RejectedNoCapacity
  ↓
  Rate limit per minute? → RejectedLimitReached
  ↓
  Accepted
```
