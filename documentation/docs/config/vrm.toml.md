# VRM — RAM Manager Configuration

**Path:** `/opt/volt/ram-manager/config.toml`

This configuration is defined in the **VOLT specification (ch.8 — VRM)**.

```toml
[manager]
workers = 8
shm_size_mb = 64
enable_compression = true
enable_deduplication = true

[pressure]
green_max = 65
yellow_enter = 70
orange_enter = 85
red_enter = 95

[compression]
default_algorithm = "zstd"
lz4_for_binary_buffers = true
background_threads = 2

[apps.orbit]
quota_mb = 2048
priority = "critical"

[apps.desktop]
quota_mb = 256
priority = "critical"

[apps.peregrine]
quota_mb = 512
priority = "high"

[apps.default]
quota_mb = 128
priority = "normal"
```

<br>

## Pressure Zones

| Zone | Enter Threshold | Action |
|------|----------------|--------|
| **Green** | < 65% | Normal operation |
| **Yellow** | ≥ 70% | Start proactive compression |
| **Orange** | ≥ 85% | Aggressive reclaim, background eviction |
| **Red** | ≥ 95% | Critical — OOM killer active |

<br>

## Compression

| Setting | Value | Description |
|---------|-------|-------------|
| Default algorithm | `zstd` | General-purpose compression |
| Binary buffers | `lz4` | Fast compression for binary data |
| Background threads | 2 | Dedicated compression threads |

<br>

## App Quotas

| App | Quota | Priority |
|-----|-------|----------|
| **Orbit** | 2048 MB | Critical |
| **Desktop** | 256 MB | Critical |
| **Peregrine** | 512 MB | High |
| **Default** | 128 MB | Normal |

<br>

## Related

- [DWP — Worker Pool Configuration](dwp.toml.md)
- [VGM — GPU Manager Configuration](vgm.toml.md)
- [SBP Binary Protocol](../apis/sbp-protocol.md)

<br>

---

[← Back: Documentation Index](../index.md)
