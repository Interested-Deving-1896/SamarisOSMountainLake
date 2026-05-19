# Scoring Model

## Overview

Bench uses a two-stage scoring system:

1. **Internal score**: 0–100 (weighted normalized score)
2. **Samaris score**: 0–10,000 (internal score × 100)

## Formula

```
internal_score (0-100) = Σ(category_score × category_weight)

samaris_score (0-10000) = round(internal_score × 100)
```

### Example

If internal score is 92.25:
```
samaris_score = round(92.25 × 100) = 9225
```

## Category Weights

| Category | Weight | Description |
|----------|--------|-------------|
| system | 0.20 (20%) | Boot time, RAM/CPU idle, responsiveness |
| ui | 0.20 (20%) | FPS, app launch, resize latency |
| memory | 0.15 (15%) | VRM metrics, memory pressure, reclaim |
| kernel | 0.10 (10%) | Kernel B IPC, SBP latency, daemon health |
| graphics | 0.10 (10%) | VGM metrics, frame budget, shader cache |
| ai | 0.10 (10%) | Orbit inference, tokens/sec, load time |
| browser | 0.05 (5%) | Peregrine page load, tab switching |
| filesystem | 0.05 (5%) | Finder listing, search, VUM cache |
| stability | 0.05 (5%) | Crashes, restarts, thermal events |

**Total: 1.0**

## Category Score Calculation

Each category contains multiple metrics. Each metric is normalized to a 0–100 score using a target range.

### Normalization Function

For metrics where **lower is better** (latency, boot time, etc.):
```
score = clamp(100 × (max_val - raw_val) / (max_val - min_val), 0, 100)
```

For metrics where **higher is better** (FPS, throughput, etc.):
```
score = clamp(100 × (raw_val - min_val) / (max_val - min_val), 0, 100)
```

### Scoring Tables

#### System Metrics

| Metric | Target (min) | Target (max) | Direction | Weight |
|--------|-------------|-------------|-----------|--------|
| boot_time_seconds | 0 | 60 | lower is better | 0.20 |
| ram_idle_percent | 20 | 80 | higher is better | 0.25 |
| cpu_idle_percent | 50 | 95 | higher is better | 0.25 |
| process_count | 50 | 300 | lower is better | 0.15 |
| system_responsiveness | 0 | 100 | higher is better | 0.15 |

#### UI Metrics

| Metric | Target (min) | Target (max) | Direction | Weight |
|--------|-------------|-------------|-----------|--------|
| fps_idle | 30 | 60 | higher is better | 0.20 |
| fps_animation | 20 | 60 | higher is better | 0.25 |
| fps_scroll | 20 | 60 | higher is better | 0.20 |
| app_launch_finder_ms | 200 | 3000 | lower is better | 0.15 |
| app_launch_peregrine_ms | 200 | 4000 | lower is better | 0.10 |
| window_resize_latency_ms | 8 | 60 | lower is better | 0.10 |

#### Memory (VRM) Metrics

| Metric | Target (min) | Target (max) | Direction | Weight |
|--------|-------------|-------------|-----------|--------|
| compression_ratio | 1.0 | 4.0 | higher is better | 0.25 |
| dedup_ratio | 1.0 | 3.0 | higher is better | 0.20 |
| swap_avoidance_score | 0 | 100 | higher is better | 0.25 |
| memory_reclaim_latency_ms | 0 | 50 | lower is better | 0.15 |
| pressure_zone | 0 | 3 | lower is better | 0.15 |

#### Kernel B Metrics

| Metric | Target (min) | Target (max) | Direction | Weight |
|--------|-------------|-------------|-----------|--------|
| sbp_latency_us | 0 | 1000 | lower is better | 0.30 |
| daemon_response_time_ms | 0 | 10 | lower is better | 0.25 |
| ipc_queue_depth | 0 | 20 | lower is better | 0.20 |
| service_restart_count | 0 | 5 | lower is better | 0.25 |

#### Graphics (VGM) Metrics

| Metric | Target (min) | Target (max) | Direction | Weight |
|--------|-------------|-------------|-----------|--------|
| vram_used_mb | 0 | 4096 | lower is better | 0.20 |
| shader_cache_hit_rate | 50 | 100 | higher is better | 0.25 |
| frame_budget_ms | 8 | 33 | lower is better | 0.30 |
| thermal_backoff_events | 0 | 10 | lower is better | 0.25 |

#### AI (Orbit) Metrics

| Metric | Target (min) | Target (max) | Direction | Weight |
|--------|-------------|-------------|-----------|--------|
| model_load_time_ms | 0 | 10000 | lower is better | 0.15 |
| inference_first_ms | 0 | 5000 | lower is better | 0.25 |
| inference_subsequent_ms | 0 | 2000 | lower is better | 0.25 |
| tokens_per_second | 0 | 100 | higher is better | 0.20 |
| tts_synthesis_ms | 0 | 5000 | lower is better | 0.15 |

#### Browser (Peregrine) Metrics

| Metric | Target (min) | Target (max) | Direction | Weight |
|--------|-------------|-------------|-----------|--------|
| page_load_youtube_ms | 0 | 5000 | lower is better | 0.25 |
| tab_switch_latency_ms | 0 | 500 | lower is better | 0.25 |
| memory_per_tab_mb | 50 | 500 | lower is better | 0.25 |
| cold_start_ms | 0 | 3000 | lower is better | 0.25 |

#### Filesystem (Finder + VUM) Metrics

| Metric | Target (min) | Target (max) | Direction | Weight |
|--------|-------------|-------------|-----------|--------|
| listing_1k_items_ms | 0 | 1000 | lower is better | 0.20 |
| search_latency_ms | 0 | 2000 | lower is better | 0.20 |
| cache_hit_rate | 50 | 100 | higher is better | 0.25 |
| fs_read_latency_ms | 0 | 20 | lower is better | 0.20 |
| fs_write_latency_ms | 0 | 30 | lower is better | 0.15 |

#### Stability Metrics

| Metric | Score | Condition |
|--------|-------|-----------|
| crash_free | 100 | 0 crashes during benchmark |
| crash_free | 50 | 1 crash |
| crash_free | 0 | ≥ 2 crashes |
| service_health | 100 | all services healthy |
| service_health | 50 | 1–2 degraded services |
| service_health | 0 | any failed service |
| thermal_stable | 100 | 0 thermal backoff events |
| thermal_stable | 50 | 1–3 events |
| thermal_stable | 0 | > 3 events |

## Badge Thresholds

| Samaris Score | Badge |
|---------------|-------|
| 9500–10000 | Legendary |
| 9000–9499 | Exceptional |
| 8500–8999 | Excellent |
| 8000–8499 | Very Good |
| 7000–7999 | Good |
| 6000–6999 | Needs Optimization |
| < 6000 | Critical Optimization Needed |

## Worked Example

```
Raw metrics:
  boot_time: 12s
  ram_idle: 65%
  cpu_idle: 92%
  fps_animation: 45
  app_launch_finder: 450ms
  compression_ratio: 2.8

System score:
  boot: clamp(100 × (60-12)/(60-0), 0, 100) = 80.0
  ram_idle: clamp(100 × (65-20)/(80-20), 0, 100) = 75.0
  cpu_idle: clamp(100 × (92-50)/(95-50), 0, 100) = 93.3
  system = 80.0 × 0.20 + 75.0 × 0.25 + 93.3 × 0.25 = 58.1
  (other metrics omitted for brevity → assume proportional)

UI score: 82.0
Memory score: 90.0
Kernel score: 85.0
Graphics score: 78.0
AI score: 70.0
Browser score: 75.0
Filesystem score: 88.0
Stability score: 100.0

internal_score = 58.1 × 0.20 + 82.0 × 0.20 + 90.0 × 0.15 + 85.0 × 0.10
                + 78.0 × 0.10 + 70.0 × 0.10 + 75.0 × 0.05 + 88.0 × 0.05
                + 100.0 × 0.05
              = 11.62 + 16.40 + 13.50 + 8.50 + 7.80 + 7.00 + 3.75 + 4.40 + 5.00
              = 77.97

samaris_score = round(77.97 × 100) = 7797  →  "Good"
```

## Important Notes

- Missing metrics due to collector failure are scored as **0** for that metric.
- The scorer marks missing metrics with `ESTIMATED_METRIC` reliability flag.
- The scorer never throws. All divisions are checked for zero.
- The scorer is deterministic: same inputs → same outputs.
