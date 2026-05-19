# Safety — Volt Dynamic Worker Pool v2.0

## Core Safety Invariants

The pool enforces these invariants at runtime via `safety::invariants::InvariantChecker`:

### 1. Worker Count Bounds
```
min_workers <= current_workers <= max_workers
```
Checked before and after every scaling operation. Violation returns `WorkerPoolError::InternalInvariantViolation`.

### 2. Desktop Minimum Reservation
```
desktop_workers >= desktop_min_workers
```
The desktop always has at least `desktop_min_workers` (default 1) workers available. No scaling-down or draining can reduce desktop-allocated workers below this threshold.

### 3. No Busy Worker Kill
```
worker.state != Busy  // precondition for killing/removing a worker
```
A worker can only be stopped or drained when it is `Idle`, `Draining`, or `Error`. Busy workers must complete their current job or be yielded cooperatively. There is **no force-kill** mechanism for busy workers.

### 4. Valid Lifecycle Transitions
All pool state transitions must pass `WorkerPoolState::can_transition_to()`:
```
Uninitialized → Starting → Running ↔ ScalingUp/ScalingDown → Draining → Shutdown
                                                            ↘ Error
```
Invalid transitions (e.g., `Uninitialized → Running`) return `WorkerPoolError::InvalidStateTransition`.

### 5. Non-Negative Metrics
All metric values in `MetricsSnapshot` must be ≥ 0.0. Negative values indicate a counting bug and trigger `InternalInvariantViolation`.

## Desktop Protection Invariants

### Frame Budget Hard Limit
The `DesktopGuard` tracks frame times via `LatencyGuard` with a rolling window of 60 samples. When the average frame time exceeds `frame_budget_ms - latency_guard_ms`, the system is considered under frame pressure.

### Guard Actions
- **Background reduction:** When `DesktopProtection.reduce_background()` returns true, the pool prioritizes desktop worker availability over background work. Background jobs (Low priority) may be delayed or deprioritized.
- **Orbit burst blocking:** When `DesktopProtection.block_orbit_burst()` returns true (triggered by `UiSignal::CriticalPressure`), all new orbit burst requests are rejected with reason `"desktop_pressure"`. This is checked in `DynamicWorkerPool.request_orbit_burst()` before the burst controller is consulted.
- Both `reduce_orbit_on_frame_pressure` and `reduce_background_on_frame_pressure` must be enabled in config for these actions to take effect.

## Thermal Backoff

### Thermal State Machine
```
Normal → Warm → Hot → Critical
```
Read from `/sys/class/thermal/thermal_zone0/temp` when the `thermal` feature is enabled. Falls back to `Normal` if the file is unavailable or the feature is disabled.

### Backoff Rules
| State    | Scale Up | Scale Down | Orbit Burst | Reasoning                                           |
|----------|----------|------------|-------------|------------------------------------------------------|
| Normal   | Allowed  | Allowed    | Allowed     | No thermal concern                                   |
| Warm     | Allowed  | Allowed    | Allowed     | Mild, no action needed                               |
| Hot      | Blocked  | Allowed    | Blocked     | Thermal pressure; avoid adding load                  |
| Critical | Blocked  | Blocked    | Blocked     | Critical; avoid all new work, prevent scale-down to keep CPU available |

### Config Control
- `thermal_backoff_enabled` — master switch for all thermal backoff behavior
- `scale_down_on_thermal_pressure` — if true, permits scale-down during thermal events (except Critical)
- `disable_orbit_burst_on_thermal_pressure` — if true, orbit bursts are blocked during Hot and Critical

## Safety Guard Checks

`safety::guards::SafetyGuards` aggregates safety decisions:

```rust
can_scale_down(high_backlog: bool)  → !high_backlog
can_scale_up(thermal_active: bool)  → !(thermal_backoff && thermal_active)
can_burst(desktop_pressure, thermal) → desktop < 0.6 && !thermal_active
can_kill_worker(state)              → state == Idle
```

## Shutdown Safety

### Clean Shutdown Procedure
1. `Lifecycle.transition(Draining)` — signals all workers to drain
2. Workers currently `Busy` complete their current job
3. Workers transition to `Stopped` via `Draining → Stopped`
4. `Lifecycle.transition(Shutdown)` — marks pool as shut down
5. New job submissions return `Err(WorkerPoolError::PoolNotStarted)` after shutdown

### No Force-Shutdown of Busy Workers
The pool will **not** terminate busy workers during shutdown. It waits for cooperative completion. If a job never yields or completes, shutdown blocks indefinitely.

### Idempotent Shutdown
Calling `shutdown()` on an already-shutdown pool returns `Err(WorkerPoolError::PoolNotStarted)`. State transitions enforce single shutdown.

## No Indefinite Orbit Monopoly

Orbit bursts are bounded by three independent limiters:
1. **Cooldown** — minimum 2000ms between bursts (`orbit_burst_cooldown_ms`)
2. **Concurrent limit** — max 5 simultaneous bursts (`max_concurrent_bursts`)
3. **Rate limit** — max 10 bursts per minute (`max_bursts_per_minute`)
4. **Desktop pressure gate** — blocked when pressure > 0.6
5. **Thermal gate** — blocked when thermal state is Hot/Critical and `disable_orbit_burst_on_thermal_pressure` is true

These ensure that even under sustained orbit demand, the pool remains fair to other modules.

## No Forced Integration While Standalone

- The config field `integration_mode` controls adapter binding
- When `integration_mode = "standalone"`, all adapters are decoupled stubs
- Module registration (`register_module()`) is purely informational in standalone mode — it tracks which modules *would* connect but does not bind to any external process
- No Samaris OS modules are called, spawned, or affected when integration_mode is `"standalone"`
- Switching to `"full_runtime"` without actual adapters present results in no-op (safe degradation)
