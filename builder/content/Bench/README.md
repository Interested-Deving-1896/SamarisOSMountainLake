# BENCH — Samaris OS Benchmark Suite

Bench measures everything.
Bench compares carefully.
Bench certifies honestly.
Samaris OS does not say it is fast.
It proves it.

Numbers. Not marketing.
Truth.

## What is Bench?

Bench is the built-in performance measurement, scoring, and certification layer of Samaris OS. It collects metrics across the full stack — from hardware detection to UI frame rates, from VRM memory compression to Kernel B IPC latency — and produces a single honest score out of 10,000.

Bench does not claim Samaris is fast. Bench measures it, scores it, and displays the result — including uncertainty, reliability flags, and hardware context.

## Why does Bench exist?

Most operating systems have no built-in performance measurement layer. Users rely on third-party benchmarks that may not reflect real system performance. Samaris OS includes Bench as a first-party module because:

- Performance must be measured, not marketed.
- Optimizations must be driven by data, not intuition.
- The future AutoOptimizer module needs a trusted sensor.
- Users deserve an honest, reproducible score.

## How does Bench fit into Samaris OS?

Bench sits across three layers:

```
┌─────────────────────────────────────────────────┐
│  React UI (Bench app — launched from Dock)       │
│  Shows score, badges, history, charts            │
├─────────────────────────────────────────────────┤
│  Kernel A bridge (Node.js REST + WebSocket)      │
│  Proxies requests, runs `bench` CLI, streams     │
├─────────────────────────────────────────────────┤
│  volt-bench (Rust CLI — the measurement engine)  │
│  Collectors → Scorer → Reporter → Storage        │
└─────────────────────────────────────────────────┘
```

## Quick Start

```bash
# Run a quick benchmark and view the score
bench --quick

# Run a full benchmark with all collectors
bench --full

# View the latest result
bench --latest

# Watch live system metrics
bench --watch

# Open the Bench app from the Dock (click the Bench icon)
```

## CLI Commands

| Command | Description |
|---------|-------------|
| `bench --run` | Run default (quick) benchmark |
| `bench --quick` | Short benchmark (10–30 sec) |
| `bench --full` | Complete benchmark, multiple iterations |
| `bench --stress` | Thermal/memory pressure test |
| `bench --watch` | Continuous monitoring (5s refresh) |
| `bench --ci` | CI mode — strict, non-zero exit on regression |
| `bench --score` | Recompute score from latest.json |
| `bench --system` | Run system collector only |
| `bench --ui` | Run UI collector only (via Kernel A) |
| `bench --vrm` | Run VRM collector only |
| `bench --vgm` | Run VGM collector only |
| `bench --dwp` | Run DWP collector only |
| `bench --vum` | Run VUM collector only |
| `bench --orbit` | Run Orbit AI collector only |
| `bench --peregrine` | Run Peregrine collector only |
| `bench --finder` | Run Finder collector only |
| `bench --kernel` | Run Kernel B collector only |
| `bench --export json` | Export latest result as JSON |
| `bench --export csv` | Export latest result as CSV |
| `bench --history` | Show recent benchmark runs |
| `bench --latest` | Show latest benchmark result |
| `bench --compare` | Compare against imported baseline |
| `bench --import-baseline <file>` | Import external baseline |
| `bench --optimizer-export` | Generate AutoOptimizer input |
| `bench --dump` | Dump all raw metrics |
| `bench --version` | Show version |

## Output Files

| Path | Description |
|------|-------------|
| `/var/lib/samaris/bench/latest.json` | Most recent benchmark result |
| `/var/lib/samaris/bench/history.json` | Historical runs (max 100) |
| `/var/lib/samaris/bench/baselines/` | Imported baseline files |
| `/var/lib/samaris/bench/export/` | Exported CSV/JSON |
| `/var/lib/samaris/bench/optimizer-input.json` | Data for AutoOptimizer |

## Score Explanation

Bench produces a score out of **10,000** (internally computed as a weighted 0–100 score multiplied by 100).

| Range | Badge |
|-------|-------|
| 9500–10000 | Legendary |
| 9000–9499 | Exceptional |
| 8500–8999 | Excellent |
| 8000–8499 | Very Good |
| 7000–7999 | Good |
| 6000–6999 | Needs Optimization |
| < 6000 | Critical Optimization Needed |

Category weights:
- System: 20% — UI: 20% — Memory: 15% — Kernel: 10% — Graphics: 10%
- AI: 10% — Browser: 5% — Filesystem: 5% — Stability: 5%

## Comparison Validity Warning

Bench never claims Samaris is faster than another OS unless the baseline was measured on the **same hardware**. Imported baselines are always marked as `reference_only`. Always check the `comparison_validity` field in the result JSON.

## AutoOptimizer Integration

Bench generates structured optimizer input at `/var/lib/samaris/bench/optimizer-input.json`. This file contains:
- `fitness_score` — the overall score
- `bottlenecks` — list of detected performance issues
- `recommendations` — actionable optimization suggestions
- `unstable_metrics` — high-variance metrics
- `regression_alerts` — score changes from previous runs

The AutoOptimizer module (planned for Beta) consumes this input to adjust system parameters automatically.

## React UI Overview

The Bench app opens in a window from the Dock. It displays:
- Global score (0–10000) with badge
- Normalized score (0–100)
- Category scores as a radar or bar chart
- Raw metrics table
- History chart (score over time)
- Baseline comparison table
- Optimizer recommendations panel
- Run button with mode selector

The UI is read-only. Actual benchmarks are triggered via CLI or from the UI (which sends a POST to Kernel A).
