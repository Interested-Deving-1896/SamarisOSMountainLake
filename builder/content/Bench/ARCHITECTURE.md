# BENCH Architecture

Bench has four major layers that data flows through sequentially during each benchmark run.

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│   Runner    │────▶│  Collectors │────▶│   Scorer    │────▶│   Reporter  │
│ (orchestr.) │     │ (gather)    │     │ (normalize) │     │ (persist)   │
└─────────────┘     └─────────────┘     └─────────────┘     └─────────────┘
                                                                    │
                                                                    ▼
                                                              ┌─────────────┐
                                                              │   Storage   │
                                                              │ (JSON files)│
                                                              └─────────────┘
                                                                    │
                                          ┌─────────────────────────┤
                                          │                         │
                                          ▼                         ▼
                                   ┌──────────────┐        ┌──────────────┐
                                   │ Kernel A REST │        │AutoOptimizer │
                                   │ + WebSocket   │        │ (consumes)   │
                                   └──────┬───────┘        └──────────────┘
                                          │
                                          ▼
                                   ┌──────────────┐
                                   │ React UI app │
                                   └──────────────┘
```

## 1. Runner

The Runner is the orchestrator. It:

- Parses CLI arguments (mode, target collectors, export format)
- Selects which collectors to run based on mode:
  - `quick`: system + boot + memory + cpu + UI (time-bounded, ~20s)
  - `full`: all available collectors, 3–5 iterations, warmup runs
  - `stress`: runs collectors repeatedly under load, detects throttling
  - `watch`: runs system + cpu + memory collectors every 5 seconds
  - `ci`: same as quick but with strict exit code on regression
  - category-specific: runs only the specified collector(s)
- Manages warmup iterations and measurement iterations
- Tracks elapsed time and progress
- Passes collected metrics to the Scorer

## 2. Collectors

Each collector gathers metrics from a specific subsystem. Collectors are independent modules — a failure in one does not affect others.

### System Collector
- RAM: reads `/proc/meminfo` or `sysinfo` crate
- CPU: reads `/proc/stat` for idle/utilization
- Processes: counts `/proc` entries
- Disk: reads `/proc/diskstats` for I/O
- Network: reads `/proc/net/dev` for throughput

### Boot Collector
- Runs `systemd-analyze time` if available
- Parses firmware, bootloader, kernel, and userspace times
- Falls back to reading `/proc/uptime` minus systemd timestamps

### Memory Collector
- Reads memory pressure from `/proc/pressure/memory`
- Checks swap usage from `/proc/swaps`
- Reports available/used RAM

### CPU Collector
- Reads CPU model, cores, frequency from `/proc/cpuinfo`
- Measures context switches from `/proc/stat`
- Detects thermal throttling from `/sys/class/thermal/`

### Disk Collector
- Measures read/write I/O from `/proc/diskstats`
- Runs timed sequential/random read tests against a temp file
- Reports throughput and IOPS

### Network Collector
- Measures throughput by downloading a known payload
- Reports RX/TX rates from `/proc/net/dev`
- Pings a known host for latency

### VRM Collector (SBP IPC)
- Connects to `/run/samaris/volt-ram-manager.sock`
- Requests status: compression ratio, dedup ratio, tier counts, pressure zone, reclaim latency
- Timeout: 5 seconds. On failure: adds `MISSING_COLLECTOR` flag.

### VGM Collector (SBP IPC)
- Connects to `/run/samaris/volt-gpu-manager.sock`
- Requests status: VRAM used, shader cache hit rate, frame budget, thermal events
- Timeout: 5 seconds.

### DWP Collector (SBP IPC)
- Connects to `/run/samaris/volt-worker-pool.sock`
- Requests status: active workers, queue depth, burst usage, task completion latency
- Timeout: 5 seconds.

### VUM Collector (SBP IPC)
- Connects to `/run/samaris/volt-usb-manager.sock`
- Requests status: cache hit rate, writeback queue, journal replays, FS latency
- Timeout: 5 seconds.

### Kernel B Collector (SBP IPC)
- Connects to `/run/samaris/volt-kernel-b.sock`
- Requests status: SBP latency histogram, IPC queue depth, daemon uptime, restart count, service health
- Timeout: 5 seconds.

### Orbit / Peregrine / Finder Collectors
These run in the Electron layer (not Rust). The `volt-bench` CLI sends an IPC request to Kernel A, which triggers the measurements in the renderer and returns results.

## 3. Scorer

The Scorer converts raw metrics into normalized scores:

1. Each raw metric is clamped to realistic bounds
2. Each metric is normalized to a 0–100 score using a target range
3. Metrics missing due to collector failure are scored as 0
4. Sub-scores are averaged per category
5. Category scores are weighted using the global weight table
6. The weighted score (0–100) is multiplied by 100 to produce the Samaris score (0–10000)

See `SCORING_MODEL.md` for the complete formula.

## 4. Reporter

The Reporter:
- Writes `latest.json` to `/var/lib/samaris/bench/`
- Appends to `history.json` (max 100 entries)
- Generates optimizer-input.json
- Exports CSV/JSON on request
- Prints console output when running in CLI mode

## 5. Storage

Storage paths:
- `/var/lib/samaris/bench/latest.json`
- `/var/lib/samaris/bench/history.json`
- `/var/lib/samaris/bench/baselines/*.json`
- `/var/lib/samaris/bench/export/*.csv`
- `/var/lib/samaris/bench/optimizer-input.json`

## 6. Kernel A Bridge

The Kernel A bridge exposes Bench data via REST and real-time WebSocket events:

- `GET /api/bench/latest` — latest.json
- `GET /api/bench/history` — history.json
- `GET /api/bench/baselines` — list of imported baselines
- `POST /api/bench/run` — trigger a benchmark run
- `POST /api/bench/import-baseline` — import a baseline file
- `GET /api/bench/export/json` — export as JSON
- `GET /api/bench/export/csv` — export as CSV
- `GET /api/bench/optimizer-input` — optimizer payload

WebSocket events: `bench.started`, `bench.progress`, `bench.metric`, `bench.completed`, `bench.failed`, `bench.warning`, `bench.optimizer.ready`

## 7. React UI

The Bench app (launched from the Dock) subscribes to Bench data via the Kernel A bridge. See `frontend/README.md` for component details.

## 8. AutoOptimizer Flow

```
Bench (measure) → Scorer (score) → Reporter (persist) → Optimizer Input
                                                              │
                                                              ▼
                                                   AutoOptimizer (adjust)
                                                              │
                                                              ▼
                                                   Bench (re-measure) → ...
```

Bench is the sensor. AutoOptimizer is the actuator. Bench measures before and after each optimization to confirm improvement.
