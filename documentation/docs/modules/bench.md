# Bench — Performance Measurement Suite

**The built-in performance measurement, scoring, and certification layer of Samaris OS.**

Bench collects metrics across the full stack — from hardware detection to UI frame rates, from VRM memory compression to Kernel B IPC latency — and produces a single honest score out of 10,000 with full transparency on methodology, uncertainty, and hardware context.

<br>

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│  React UI (Bench app — launched from Dock)                   │
│  Shows score, badges, history, charts                        │
├─────────────────────────────────────────────────────────────┤
│  Kernel A bridge (Node.js REST + WebSocket)                  │
│  Proxies requests, runs `bench` CLI, streams results         │
├─────────────────────────────────────────────────────────────┤
│  volt-bench (Rust CLI — the measurement engine)              │
│  Runner → Collectors → Scorer → Reporter → Storage           │
└─────────────────────────────────────────────────────────────┘
```

### Runner

The orchestrator that:
- Parses CLI arguments (mode, target collectors, export format)
- Selects collectors based on mode (quick, full, stress, watch, ci)
- Manages warmup and measurement iterations
- Tracks elapsed time and progress
- Passes collected metrics to the Scorer

### Collectors

| Collector | Data Source | What It Measures |
|-----------|-------------|-----------------|
| **System** | `/proc/meminfo`, `/proc/stat` | RAM idle, CPU idle, process count |
| **Boot** | `systemd-analyze time` | Firmware, bootloader, kernel, userspace times |
| **Memory** | `/proc/pressure/memory` | Memory pressure, swap usage, available RAM |
| **CPU** | `/proc/cpuinfo`, `/sys/class/thermal/` | Model, cores, frequency, thermal throttling |
| **Disk** | `/proc/diskstats`, timed I/O | Read/write throughput, IOPS |
| **Network** | `/proc/net/dev`, ping | Throughput, latency |
| **VRM** (SBP IPC) | Unix socket → VRM | Compression ratio, dedup ratio, tier counts, pressure zone |
| **VGM** (SBP IPC) | Unix socket → VGM | VRAM used, shader cache hit rate, frame budget |
| **DWP** (SBP IPC) | Unix socket → DWP | Active workers, queue depth, burst usage |
| **VUM** (SBP IPC) | Unix socket → VUM | Cache hit rate, writeback queue, journal replays |
| **Kernel B** (SBP IPC) | Unix socket → Kernel B | SBP latency, IPC queue depth, daemon health |
| **Orbit / Peregrine / Finder** | Electron IPC via Kernel A | Inference speed, page load, search latency |

### Scorer

Two-stage scoring:

1. Each raw metric is normalised to a 0–100 score using target ranges
2. Categories are weighted and summed
3. Result (0–100) × 100 = **Samaris Score** (0–10,000)

| Category | Weight | Example Metrics |
|----------|--------|-----------------|
| System | 20% | Boot time, RAM idle, CPU idle |
| UI | 20% | FPS, app launch, resize latency |
| Memory | 15% | Compression ratio, dedup ratio, reclaim latency |
| Kernel | 10% | SBP latency, daemon response time |
| Graphics | 10% | VRAM used, shader cache hit, frame budget |
| AI | 10% | Tokens/sec, inference latency |
| Browser | 5% | Page load time, tab switch latency |
| Filesystem | 5% | Listing speed, cache hit rate |
| Stability | 5% | Crash count, service health, thermal events |

### Badge Thresholds

| Score | Badge |
|-------|-------|
| 9500–10000 | Legendary |
| 9000–9499 | Exceptional |
| 8500–8999 | Excellent |
| 8000–8499 | Very Good |
| 7000–7999 | Good |
| 6000–6999 | Needs Optimization |
| < 6000 | Critical Optimization Needed |

<br>

## CLI Commands

```bash
bench --run                       # Run default (quick) benchmark
bench --quick                     # Short benchmark (10–30 sec)
bench --full                      # Complete benchmark, multiple iterations
bench --stress                    # Thermal/memory pressure test
bench --watch                     # Continuous monitoring (5s refresh)
bench --ci                        # CI mode — strict, non-zero exit on regression
bench --score                     # Recompute score from latest.json
bench --system                    # Run system collector only
bench --export json               # Export latest result as JSON
bench --history                   # Show recent benchmark runs
bench --compare                   # Compare against imported baseline
bench --optimizer-export          # Generate AutoOptimizer input
```

<br>

## Output Files

| Path | Description |
|------|-------------|
| `/var/lib/samaris/bench/latest.json` | Most recent benchmark result |
| `/var/lib/samaris/bench/history.json` | Historical runs (max 100) |
| `/var/lib/samaris/bench/baselines/` | Imported baseline files |
| `/var/lib/samaris/bench/optimizer-input.json` | Data for AutoOptimizer |

<br>

## Methodology

Bench follows strict reproducibility rules:
- **Warmup runs**: At least 1 warmup iteration before measurement
- **Multiple iterations**: Quick mode = 3, Full mode = 5
- **Median over mean**: Final score uses median of all iteration scores
- **Variance tracking**: Standard deviation reported; high variance triggers `HIGH_VARIANCE` flag

### Reliability Flags

| Flag | Meaning |
|------|---------|
| `THERMAL_THROTTLING` | CPU throttling detected during run |
| `MISSING_COLLECTOR` | One or more collectors failed |
| `HIGH_VARIANCE` | Standard deviation > 5% of score |
| `VM_ENVIRONMENT` | Running inside a virtual machine |
| `BASELINE_NOT_SAME_HARDWARE` | Baseline from different HW |

### Same-Hardware Comparison Rule

Bench never claims superiority over another OS unless measured on the **same physical hardware**. Imported baselines are always marked `reference_only`.

<br>

## AutoOptimizer Integration

Bench generates structured optimizer input for the planned AutoOptimizer module (Beta):

```json
{
  "fitness_score": 7797,
  "bottlenecks": ["high_memory_pressure", "low_shader_cache_hit"],
  "recommendations": ["increase_vram_t1_pool", "enable_shader_precache"],
  "unstable_metrics": ["fps_animation"],
  "regression_alerts": []
}
```

Bench is the sensor. AutoOptimizer is the actuator.

<br>

## Kernel A Bridge

| Endpoint | Purpose |
|----------|---------|
| `GET /api/bench/latest` | Latest result |
| `GET /api/bench/history` | Historical data |
| `POST /api/bench/run` | Trigger benchmark |
| `POST /api/bench/import-baseline` | Import external baseline |

WebSocket events: `bench.started`, `bench.progress`, `bench.metric`, `bench.completed`, `bench.failed`, `bench.optimizer.ready`
