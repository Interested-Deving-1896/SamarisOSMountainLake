# Integration Plan — Volt Dynamic Worker Pool into Samaris OS

## Phase 1: Standalone (Current)

The pool runs independently. No Samaris module integration.

**Status:** Complete.

**Capabilities:**
- CLI simulations: `simulate-load`, `simulate-orbit-burst`, `simulate-desktop-pressure`, `simulate-scaling`, `simulate-adapters`
- Config validation via `check-config`
- Metrics snapshot via `status`
- All adapters are stubs

**Config:** `integration_mode = "standalone"`

**Verification:**
```bash
cargo run --release -- check-config
cargo run --release -- simulate-load
cargo run --release -- simulate-orbit-burst
```

## Phase 2: Adapter-Ready

Enable the `adapters` feature and connect each Samaris module through a `WorkerPoolAdapter` stub.

**Status:** Stubs exist; real implementations pending.

**Steps:**

1. Build with `--features adapters`:
   ```bash
   cargo build --release --features adapters
   ```
2. Register each module's adapter:
   ```bash
   cargo run --release --features adapters -- simulate-adapters
   ```
3. Verify adapter contract: each `WorkerPoolAdapter` must implement `submit_job()`, `cancel_job()`, `metrics()`.
4. Tests should validate `ModuleRegistry` registration and conflict detection.

**Module → Adapter mapping:**

| Module    | Adapter File            | Status |
|-----------|-------------------------|--------|
| kernel_b  | `adapters/kernel_b_adapter.rs` | stub   |
| kernel_a  | `adapters/kernel_a_adapter.rs` | stub   |
| desktop   | `adapters/desktop_adapter.rs`  | stub   |
| orbit     | `adapters/orbit_adapter.rs`    | stub   |
| vrm       | `adapters/vrm_adapter.rs`      | stub   |
| vum       | `adapters/vum_adapter.rs`      | stub   |
| vgm       | `adapters/vgm_adapter.rs`      | stub   |
| background| `adapters/background_adapter.rs`| stub   |

## Phase 3: Partial Integration

Replace stubs with real module connections.

### Kernel B / Tesseract Engine

1. Replace `KernelBAdapter` stub with adapter that calls `pool.submit_job()` for compute tasks
2. Priority: `High` (`kernel_b = "high"`)
3. Kernel B submits compute scheduling jobs; the pool acts as the worker supervisor
4. Adapter should read `MetricsSnapshot` and feed back scaling hints

### Desktop

1. Replace `DesktopAdapter` stub with adapter that calls `pool.set_desktop_pressure()` from Compositor frame timings
2. Desktop sends frame pressure signals derived from actual frame render times
3. Desktop Guard responds by reducing background work and blocking orbit bursts

### Orbit

1. Replace `OrbitAdapter` stub with adapter that calls `pool.request_orbit_burst()` for inference jobs
2. Orbit inference jobs are submitted as `Critical` priority
3. Burst control provides backpressure: cooldown, rate limits, concurrent limits
4. Orbit jobs should call `yield_point()` regularly (every ≤ 5ms) to enable cooperative yields

### VRM, VUM, VGM (Optional)

- **VRM** — submit compression/dedup tasks at `Low` priority via `vrm_adapter`
- **VUM** — submit flush/journal tasks at `Normal` priority via `vum_adapter`
- **VGM** — submit GPU-helper tasks at `Normal` priority via `vgm_adapter`

### Background

- Submit maintenance/cleanup tasks at `Low` priority
- First to be reduced when desktop pressure rises

## Phase 4: Full Runtime Integration

The pool becomes the central scheduler for all Samaris OS async work.

**Config:** `integration_mode = "full_runtime"`

**Changes:**
1. `volt-dynamic-worker-pool` is started by the Samaris runtime (not as a standalone binary)
2. All modules start their adapters on pool startup and stop on pool shutdown
3. Old per-module thread pools are deprecated and removed
4. Pool lifecycle is tied to runtime lifecycle

**Benefits:**
- Unified metrics and observability
- Consistent priority enforcement across all modules
- Single scaling policy instead of per-module heuristics
- Desktop guard protects all background work, not just pool-internal work

**Risks:**
- A single misbehaving module can affect the entire system (mitigated by priorities, adapters, and safety invariants)
- Pool becomes a single point of failure (mitigated by clean shutdown and error recovery)

## Migration Guide

### From Standalone to Adapter-Ready

```diff
- cargo run --release
+ cargo build --release --features adapters
```

No code changes required. All adapters are stubs.

### From Adapter-Ready to Partial Integration

For each module:

1. Implement the `WorkerPoolAdapter` trait for the module
2. Replace the stub in `src/adapters/<module>_adapter.rs`
3. Register the adapter before pool start:

```rust
let profile = ModuleProfile::new(ModuleId::new("orbit"), PriorityLevel::Critical);
pool.register_module(profile).expect("register orbit adapter");
```

### From Partial to Full Runtime

```rust
// In Samaris runtime bootstrap
let config = load_config("runtime.toml");
config.worker_pool.integration_mode = "full_runtime".into();
let pool = DynamicWorkerPool::new(config);
pool.start()?;

// Register all adapters
register_all_adapters(&pool);

// Runtime loop polls pool metrics and feeds desktop pressure
loop {
    pool.set_desktop_pressure(compositor.current_frame_pressure());
    let metrics = pool.metrics();
    runtime.observe(metrics);
    thread::sleep(Duration::from_millis(16)); // ~60fps tick
}
```

### Module-Specific Adapter Details

| Module    | API to Call                          | Priority  | Notes                              |
|-----------|--------------------------------------|-----------|------------------------------------|
| Orbit     | `request_orbit_burst()`, `submit_job()` | Critical | Burst for inference; regular jobs for prep |
| Desktop   | `set_desktop_pressure()`             | High      | Frame timing → pressure signal     |
| VRM       | `submit_job()`                       | Low       | Compression, dedup                 |
| VUM       | `submit_job()`                       | Normal    | Flush, journal                     |
| VGM       | `submit_job()`                       | Normal    | GPU-helper tasks                    |
| Kernel B  | `submit_job()`                       | High      | Compute scheduler supervisor       |
| Kernel A  | `submit_job()`                       | High      | Electron, kernel tasks             |
| Background| `submit_job()`                       | Low       | Maintenance, cleanup               |

### Rollback Plan

1. Set `integration_mode = "standalone"` via config
2. Revert to per-module thread pools
3. Restart runtime
4. Pool runs independently with no module binding
