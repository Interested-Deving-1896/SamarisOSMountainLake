# Specification — Volt Dynamic Worker Pool v2.0

## Priority Levels

| Level     | u8  | Samaris Modules      | Behavior                                  |
|-----------|-----|----------------------|-------------------------------------------|
| Realtime  | 4   | _(reserved)_         | Highest; always dequeued first            |
| Critical  | 3   | orbit                | Dequeued before High, Normal, Low         |
| High      | 2   | desktop, kernel_b, kernel_a | Dequeued before Normal, Low        |
| Normal    | 1   | electron, vum, vgm   | Default priority                          |
| Low       | 0   | vrm, background      | Only dequeued when higher queues are empty|

Jobs are dequeued in strict priority order: the scheduler scans from Realtime (index 4) down to Low (index 0) and pops the first available job. **No weighted fair queuing** — higher priority queues are always drained first.

## Scaling Rules

The `AdaptiveScaler` evaluates four inputs on each tick:

1. **Queue depth** — total pending jobs across all priority levels
2. **CPU utilization** — system-wide or process-level CPU usage (0.0–1.0)
3. **Thermal state** — `Normal`, `Warm`, `Hot`, `Critical`
4. **Desktop pressure** — frame-time-derived pressure (0.0–1.0)

### Scale Up Decision

Condition for `ScaleUp`:

- `queue_depth > current_workers * scale_up_queue_factor` (default 2.0)
- AND `cpu_util < scale_up_cpu_threshold` (default 0.80)
- AND scale up cooldown elapsed (default 5000ms)
- AND thermal state is **not** throttled (`Hot` or `Critical`)
- AND `current_workers < max_workers`

Action: `current_workers += 1`

### Scale Down Decision

Condition for `ScaleDown`:

- `current_workers > min_workers`
- AND `queue_depth < current_workers * scale_down_queue_factor` (default 1.0)
- AND `cpu_util < scale_down_cpu_threshold` (default 0.30)
- AND scale down cooldown elapsed
- AND thermal state is **not** `Critical` (prevents scale down during thermal crisis)
- AND critical backlog absent: `queue_depth < current_workers`

Action: `current_workers -= 1`

### Min/Max Workers

Derived from hardware probe unless overridden:

- **No override:** `min = max(2, cpu_cores/3).clamp(..12)`, `max = max(min, cpu_cores*3/4).clamp(..48)`
- **Override:** `min_workers_override` and `max_workers_override` from config take precedence
- **Cap:** `max_workers_cap` from top-level config (default 48) is always the absolute maximum

## Cooperative Preemption Model

The pool uses **cooperative preemption only**. No kernel or OS thread preemption.

- Each job has a **yield budget** (`yield_budget_us`, default 50µs per yield quantum)
- Jobs call `yield_point()` periodically, which checks:
  1. Is the job cancelled? → return `Cancelled`
  2. Is the budget exhausted? → return `BudgetExhausted`
  3. Has `force_preempt_after_us` elapsed? → return `Preempted`
- On preemption, the job is **not** killed; its context is saved and it is re-queued or rescheduled
- Preemption tracking: `MetricsCounters.record_preemption()`

**Yield budget is consumed but not automatically replenished across context switches.** Each call to `yield_point()` resets the budget timer on preemption.

## Desktop Guard Protection

Protects desktop UI responsiveness by monitoring frame times and back-pressuring the scheduler.

### Frame Pressure Levels

| Level    | Range      | Action                                              |
|----------|------------|-----------------------------------------------------|
| None     | ≤ 0.10     | No restriction                                      |
| Low      | 0.11–0.30  | Normal operation                                    |
| Medium   | 0.31–0.60  | Latency guard may trigger protection                |
| High     | 0.61–0.85  | Background reduced, orbit burst may be blocked      |
| Critical | 0.86–1.00  | Orbit burst blocked, background reduced              |

### Latency Guard

Tracks a rolling window of 60 frame times. Triggers protection when average frame time exceeds `frame_budget_ms - latency_guard_ms` (default: 16ms - 8ms = 8ms).

### Protection Actions

When `DesktopProtection.should_protect()` is true:
- `reduce_background()` — return true, allowing pool to shed low-priority work
- `block_orbit_burst()` — return true when pressure is `CriticalPressure`, preventing new orbit bursts

## Orbit Burst Control

Orbit inference can request a temporary burst. The `OrbitBurstController` enforces:

1. **Cooldown** — minimum time between bursts (default 2000ms)
2. **Concurrent burst limit** — maximum simultaneous active bursts (default 5, configured as `max_concurrent_bursts`)
3. **Per-minute rate limit** — maximum bursts in a rolling minute (default 10)
4. **Desktop pressure gate** — bursts rejected if `desktop_pressure > 0.6`
5. **Thermal gate** — bursts disabled when thermal state is throttled (`Hot` or `Critical`) and `disable_orbit_burst_on_thermal_pressure` is true

Bursts are tracked atomically with `AtomicU64` counters and a cooldown timestamp.

## Fairness and Anti-Starvation

### Aging

When `aging_enabled = true`, the `AgingPolicy` tracks how long a job has been waiting:
- `aging_after_ms` (default 200ms) — time after which aging starts tracking
- `starvation_limit_ms` (default 1000ms) — time after which a job is considered starved

### Priority Boost on Starvation

When `priority_boost_on_starvation = true` and a job has waited ≥ `starvation_limit_ms`, the `FairnessPolicy` issues a `PriorityBoost` that temporarily elevates the job to `High` priority. The boost expires after its duration (configurable via `PriorityBoost.duration_ms`).

### FairnessSnapshot

Exported via `FairnessPolicy.snapshot()`:
- `starvation_count` — number of jobs that hit starvation limit
- `boost_count` — number of priority boosts issued
- `max_wait_time_ms` — upper bound before forced intervention

## Module Registry

Modules register via `DynamicWorkerPool.register_module(ModuleProfile)`:
- `ModuleId` — string identifier (`orbit`, `desktop`, `kernel_b`, etc.)
- `ModuleProfile` — contains `ModuleId` and associated `PriorityLevel`

The `ModuleRegistry` is a `HashMap<ModuleId, ModuleProfile>` behind an `RwLock`. Registration fails if the module ID already exists.

## Safety Invariants

All invariants in `safety::invariants::InvariantChecker`:

### Worker Count
- `current_workers >= min_workers` — never below configured minimum
- `current_workers <= max_workers` — never above configured maximum

### Desktop Minimum
- `desktop_workers >= desktop_min_workers` — desktop reservation always honored

### No Busy Worker Kill
- `state != Busy` — idle/draining/stopped workers only

### Valid State Transitions
- Every lifecycle transition must pass `WorkerPoolState::can_transition_to()`

### Non-Negative Metrics
- `avg_completion_time_ms >= 0`
- `avg_queue_time_ms >= 0`
- `throughput_jobs_per_sec >= 0`
- `desktop_pressure >= 0`

### Thermal Safety
- Scale up blocked when thermal state is throttled
- Scale down blocked when thermal state is critical (preventing CPU starvation during crisis)
- Orbit bursts disabled under thermal pressure (configurable via `disable_orbit_burst_on_thermal_pressure`)
