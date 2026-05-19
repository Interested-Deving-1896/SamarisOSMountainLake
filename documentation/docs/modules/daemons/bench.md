# volt-bench — Rust Benchmark Engine

**The measurement engine that powers the Bench suite. Collects, scores, and reports system performance metrics.**

`volt-bench` is the Rust CLI binary that runs benchmarks and produces scores. It sits below the Bench design layer (`builder/content/Bench/`) and the React UI — this crate is the actual data-gathering and scoring engine.

<br>

## Architecture

```
Runner (orchestrator)
    │
    ▼
CollectorRegistry (11 collectors)
    │
    ▼
Scorer (9 categories → 0–10,000)
    │
    ▼
Reporter + Storage (JSON files)
    │
    ▼
OptimizerInput (for AutoOptimizer)
```

<br>

## Collector Registry

11 collectors registered in `CollectorRegistry`:

| Collector | Source | Metrics |
|-----------|--------|---------|
| `system` | `/proc/meminfo`, `/proc/stat` | RAM idle, CPU idle, process count |
| `boot` | `systemd-analyze time`, `/proc/uptime` | Firmware, kernel, userspace boot times |
| `memory` | `/proc/pressure/memory`, `/proc/swaps` | Memory pressure, swap usage, available RAM |
| `cpu` | `/proc/cpuinfo`, `/sys/class/thermal/` | Model, cores, frequency, thermal throttling |
| `disk` | `/proc/diskstats`, timed I/O | Read/write throughput, IOPS |
| `network` | `/proc/net/dev`, ping | Throughput, RX/TX rates, latency |
| `vrm` | SBP IPC → VRM socket | Compression ratio, dedup, tiers, pressure zone |
| `vgm` | SBP IPC → VGM socket | VRAM used, shader cache hit, frame budget |
| `dwp` | SBP IPC → DWP socket | Active workers, queue depth, burst usage |
| `vum` | SBP IPC → VUM socket | Cache hit rate, writeback queue, journal replays |
| `kernel_b` | SBP IPC → Kernel B socket | SBP latency, IPC queue depth, daemon health |

Each collector implements the `Collector` trait and fails independently without blocking others.

<br>

## Runner

The `BenchRun` struct orchestrates the benchmark lifecycle:

```rust
BenchRun {
    mode: String,           // quick, full, stress, watch, ci
    iterations: u32,        // 3 (quick) or 5 (full)
    warmup_iterations: u32, // 1 (quick) or 2 (full)
    collectors: CollectorRegistry,
    scorer: Scorer,
    hardware: HardwareInfo,
    environment: EnvironmentInfo,
}
```

Modes:
- **quick**: 3 iterations, 1 warmup. All collectors, time-bounded (~20s)
- **full**: 5 iterations, 2 warmup. All collectors
- **stress**: 3 iterations under load. Detects throttling
- **watch**: Continuous system + cpu + memory monitoring (5s refresh)
- **ci**: Same as quick, strict exit code on regression

<br>

## Scorer

Converts raw metrics into a 0–10,000 Samaris Score.

### Categories & Weights

| Category | Weight | Source Metrics |
|----------|--------|---------------|
| system | 20% | Boot time, RAM/CPU idle, process count |
| ui | 20% | FPS, app launch, resize latency |
| memory | 15% | VRM: compression ratio, dedup, reclaim latency |
| kernel | 10% | Kernel B: SBP latency, daemon response |
| graphics | 10% | VGM: VRAM used, shader cache hit, frame budget |
| ai | 10% | Orbit: tokens/sec, inference latency |
| browser | 5% | Peregrine: page load, tab switch |
| filesystem | 5% | Finder: listing, search; VUM: cache hit rate |
| stability | 5% | Crashes, restarts, thermal events |

### Scoring Formula

```
internal_score (0-100) = Σ(category_score × weight)
samaris_score (0-10000) = round(internal_score × 100)
```

Median of all iteration scores (robust against outliers). Standard deviation tracked for confidence.

<br>

## Reporter

Writes structured results:
- `RunResult` → JSON serialisation
- `OptimizerInput` → for AutoOptimizer consumption
- Console output for CLI mode

<br>

## Storage

```
/var/lib/samaris/bench/latest.json      # Most recent result
/var/lib/samaris/bench/history.json      # Historical runs (max 100)
/var/lib/samaris/bench/baselines/*.json  # Imported baselines
/var/lib/samaris/bench/optimizer-input.json  # AutoOptimizer payload
```

<br>

## CI / CLI Usage

```bash
# Run from builder/content/volt-bench
cargo run --release -- --quick
cargo run --release -- --full
cargo run --release -- --ci      # strict exit code on regression
cargo run --release -- --watch   # continuous monitoring
cargo run --release -- --export json
cargo run --release -- --history
```

<br>

## Build

```bash
cargo build --release
cargo test
cargo bench
```

The binary (`bench`) is installed to `/opt/volt/bin/` during ISO build (step 03.11).
