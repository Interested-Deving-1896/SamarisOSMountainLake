# Volt Dynamic Worker Pool v2.0

**Cooperative Adaptive Priority Scheduler for Samaris OS**

A Rust-based dynamic worker pool that implements cooperative multi-queue priority scheduling with adaptive scaling, desktop latency protection, orbit burst control, and thermal backoff. Designed to replace per-module thread pools in Samaris OS with a single, safe, instrumented scheduler.

## Quick Start

```bash
cargo build --release
```

### CLI Usage

```bash
# Run with default config
cargo run --release

# Run with custom config
cargo run --release -- --config /path/to/config.toml

# Validate config
cargo run --release -- check-config --config config.example.toml

# Print pool status
cargo run --release -- status

# Simulate load (100 normal + 10 idle jobs)
cargo run --release -- simulate-load

# Simulate orbit burst
cargo run --release -- simulate-orbit-burst

# Simulate desktop frame pressure
cargo run --release -- simulate-desktop-pressure

# Show scaling/hardware info
cargo run --release -- simulate-scaling

# Test adapter registration
cargo run --release -- simulate-adapters

# Print integration plan
cargo run --release -- integration-plan

# Print version
cargo run --release -- version
```

### Feature Flags

| Feature        | Default | Description                              |
|----------------|---------|------------------------------------------|
| `adaptive`     | yes     | Enable adaptive scaling                   |
| `metrics`      | yes     | Enable metrics and histograms             |
| `desktop_guard`| yes     | Enable desktop frame pressure protection  |
| `thermal`      | no      | Enable thermal sensor reading             |
| `adapters`     | no      | Enable Samaris module adapter stubs       |
| `devtools`     | no      | Enable dev/debug tooling                  |

## Configuration Overview

See `config.example.toml` for the full schema. Key sections:

- **worker_pool** — Scheduler mode, preemption, yield budget, integration mode
- **worker_pool.scaling** — Adaptive scaling thresholds and cooldown
- **worker_pool.hardware** — Auto-detection of CPU cores, RAM, thermal
- **worker_pool.reservations** — Min worker guarantees for desktop/system
- **worker_pool.desktop_guard** — Frame budget and latency protection
- **worker_pool.priorities** — Module-to-priority mapping
- **worker_pool.fairness** — Aging and anti-starvation parameters
- **worker_pool.thermal** — Thermal backoff behavior
- **adapters** — Module adapter backends (stub by default)
- **metrics** — Latency histograms, utilization, queue tracking

## Integration with Samaris OS

This pool operates in **standalone mode** by default (Phase 1). It can be run independently for testing and simulation. Integration with Samaris OS follows four phases:

1. **Standalone** — Pool runs with CLI simulations, no module binding
2. **Adapter-Ready** — Enable `adapters` feature, register module adapters
3. **Partial Integration** — Kernel B, Desktop, Orbit connect via adapters
4. **Full Runtime** — All modules submit work through the pool

See `INTEGRATION_PLAN.md` for details.

## Documentation

| Document          | Description                               |
|-------------------|-------------------------------------------|
| `ARCHITECTURE.md` | Component diagram, modules, data flow     |
| `SPEC.md`         | Priority levels, scaling rules, invariants|
| `SCHEDULING.md`   | Multi-queue algorithm, aging, yield model |
| `INTEGRATION_PLAN.md` | Phased Samaris OS integration plan    |
| `PERFORMANCE.md`  | Benchmarks, metrics, limitations          |
| `SAFETY.md`       | Safety invariants, thermal backoff        |

## License

Samaris OS — Volt Dynamic Worker Pool
