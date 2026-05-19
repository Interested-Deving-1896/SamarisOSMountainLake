# Orbit AI Configuration

This configuration is defined in the **VOLT specification (ch.22 — Orbit AI)**.

<br>

## Model Paths

Orbit searches for the GGUF model file in order:

1. `$VOLT_ORBIT_MODEL_PATH` environment variable
2. `/opt/volt/ai-models/Qwen3-1.7B-Q8_0.gguf` (default)

All models are stored under `/opt/volt/ai-models/`.

<br>

## Runtime Configuration

| Parameter | Value | Description |
|-----------|-------|-------------|
| `idle_shutdown_ms` | 45000 | Shuts down model after 45 s of inactivity |
| `gpu_layers` | -1 | Offload all layers to GPU (-1 = all) |
| `threads` | 4 | CPU inference threads |
| `batch_size` | 512 | Token batch size |
| `ctx_size` | 8192 | Context window size |

<br>

## Mode Parameters

| Mode | Temperature | top_p | top_k | n_predict |
|------|-------------|-------|-------|-----------|
| **Fast** | 0.7 | 0.80 | 20 | 1024 |
| **Smart** | 0.6 | 0.95 | 20 | 4096 |
| **Code** | 0.5 | 0.95 | 20 | 4096 |
| **Data-science** | 0.5 | 0.95 | 20 | 4096 |

<br>

## Worker Configuration

Orbit uses dedicated worker reservations managed by DWP:

| Parameter | Value |
|-----------|-------|
| Dedicated workers | 4 |
| VRM quota | 2048 MB (Critical, Tier 1) |
| Reserved memory | 512 MB |
| Burst window | 100 ms |
| Burst worker fraction | 75% |
| Max consecutive bursts | 3 |

These reservations are enforced cooperatively by the DWP scheduler (see [DWP Configuration](../config/dwp.toml.md)).

<br>

## Related

- [DWP — Worker Pool Configuration](dwp.toml.md)
- [VRM — RAM Manager Configuration](vrm.toml.md)
- [VGM — GPU Manager Configuration](vgm.toml.md)

<br>

---

[← Back: Documentation Index](../index.md)
