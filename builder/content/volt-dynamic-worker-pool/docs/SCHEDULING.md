# Scheduling Algorithm — Volt Dynamic Worker Pool v2.0

## Multi-Queue Priority Scheduling

The pool uses a **strict priority multi-queue** model. Five internal `VecDeque<Job>` buffers (one per priority level) are stored in an array indexed by `PriorityLevel.as_u8()`.

### Enqueue
A job is pushed to the back of its priority queue:
```
queue_index = job.priority.as_u8()  // 0=Low, 1=Normal, 2=High, 3=Critical, 4=Realtime
queues[queue_index].push_back(job)
```

### Dequeue
The scheduler scans from highest to lowest priority, returning the first available job:
```
for i in (0..5).rev():
    if !queues[i].is_empty():
        return queues[i].pop_front()
return None  // all queues empty
```

This guarantees:
- Real-time jobs always run before Critical
- Critical jobs always run before High
- High jobs always run before Normal
- Normal jobs always run before Low

**There is no weighted fair queuing or proportional sharing.** A sustained stream of Critical jobs will starve all lower priorities until the aging mechanism kicks in.

### Priority Queue Helpers

- `has_high_priority_jobs()` — returns true if any queue at index ≥ 2 is non-empty
- `has_jobs_above(priority)` — returns true if any queue above the given priority has jobs
- `queue_depth_by_priority(priority)` — length of a specific queue
- `iter_priority()` — iterate over non-empty priority levels from highest to lowest

## Aging and Starvation Prevention

Each job has an implicit wait timer (tracked via `StarvationGuard`). The `AgingPolicy` periodically evaluates:

```
fn compute_boost(wait_ms):
    if wait_ms >= starvation_limit_ms (default 1000ms):
        return Some(PriorityLevel::High)
    return None
```

When a `PriorityBoost` is issued:
1. A `PriorityBoost` struct records the original level, boosted level, expiry, and reason
2. On the next dequeue, the boosted job is treated at its new priority level
3. After expiry (`is_expired()`), the boost reverts via `revert()`

This prevents indefinite starvation: even if Critical jobs are continuously submitted, a Normal job waiting ≥ 1000ms will be elevated to High and get scheduled.

### Limitations of Anti-Starvation

- Boost only elevates to `High`, not `Critical` or `Realtime` — starved jobs cannot preempt orbit or real-time work
- Aging is tracked per-job, not per-module — a module flooding jobs may see individual jobs boosted but the flood still dominates
- No decay mechanism after boost expiry — a starved job that was queued again may need to wait another full starvation period

## Cooperative Yield Points

The pool relies entirely on cooperative yields. The `yield_point()` workflow:

```
Job calls pool.yield_point(&mut ctx):
  1. metrics.record_yield()
  2. if ctx.is_cancelled() → return false (job should stop)
  3. elapsed = ctx.elapsed_ms() * 1000  // microseconds
  4. if preemption.should_preempt(elapsed) → preempt_count++, ctx.reset_yield_budget(), return true (yield)
  5. return true (continue)
```

- `should_preempt()` returns true when `elapsed_us >= force_preempt_after_us` (default 5000µs)
- `should_yield()` returns `BudgetExhausted` when `budget_remaining == 0`

### Yield Budget Model

Each job receives a **yield budget** of `yield_budget_us` (default 50µs per yield quantum). Every call to `yield_point()` consumes from this budget. When the budget reaches 0, `YieldResult::BudgetExhausted` is returned, and the scheduler may preempt the job.

**Important:** Budget exhaustion does not guarantee preemption — the scheduler first checks `force_preempt_after_us`. Budget exhaustion is advisory; the hard preemption trigger is wall-clock time.

## Adaptive Scaling Decisions

Scaling decisions are made by `AdaptiveScaler` on each tick (driven by external loop or adapter call). The scaler is stateless between decisions except for the cooldown timer.

### Decision Inputs

| Input             | Source                    | Range       |
|-------------------|---------------------------|-------------|
| `queue_depth`     | `MultiQueue.queue_depth()`| 0..N        |
| `cpu_util`        | caller-provided           | 0.0–1.0     |
| `thermal`         | `ThermalMonitor.state()`  | enum        |
| `desktop_pressure`| `DynamicWorkerPool`       | 0.0–1.0     |

### Scale Up Heuristic (default thresholds)

```
queue_depth > current_workers * 2.0
AND cpu_util < 0.80
AND cooldown elapsed (5000ms)
AND thermal not throttled
AND current_workers < max_workers
```

### Scale Down Heuristic (default thresholds)

```
current_workers > min_workers
AND queue_depth < current_workers * 1.0
AND cpu_util < 0.30
AND cooldown elapsed (5000ms)
AND thermal not critical
AND no critical backlog (queue_depth >= current_workers → prevent scale down)
```

### Cooldown

Both scaling directions have a shared cooldown (`scale_cooldown_ms`, default 5000ms). This prevents rapid oscillations. After `record_scale_up()` or `record_scale_down()`, the scaler stores `Instant::now()` and checks it against the cooldown on subsequent calls.

### Hardware Probe

On startup, `HardwareProbe` determines:
- `cpu_cores` — from `std::thread::available_parallelism()` or `default_cpu_cores`
- `min_workers` — derived or overridden
- `max_workers` — derived or overridden, capped at `max_workers_cap`

## Limitations

### Cooperative Only, No Kernel Preemption

This is the most important limitation. The pool **cannot** forcibly preempt a running OS thread:
- A job that never calls `yield_point()` will run indefinitely until completion
- The pool relies on well-behaved jobs that yield periodically (every ≤ 5000µs recommended)
- Malicious or buggy jobs can monopolize a worker thread
- Desktop guard signals are advisory — they can be ignored by non-cooperative jobs

### No Real-Time Guarantees

- `Realtime` priority exists in the enum but has no special kernel-level scheduling
- No CPU pinning, no interrupt masking, no deadline scheduling
- Latency is minimized via yield budget sizing but not guaranteed

### Single-Pool Scope

- All jobs share the same worker pool — there is no per-module isolation
- One module flooding the pool can affect all others (mitigated by priority but not eliminated)

### No NUMA Awareness

- Workers are not pinned to specific cores or NUMA nodes
- Cache locality is not managed
