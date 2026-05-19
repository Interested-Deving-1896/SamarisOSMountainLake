# VGM — GPU Manager Configuration

**Path:** `/opt/volt/gpu-manager/config.toml`

This configuration is defined in the **VOLT specification (ch.9 — VGM)**.

```toml
[gpu]
backend = "null"
cpu_fallback = true
frame_budget_ms = 16

[vram]
compressed_vram_enabled = true
decompression_scratch_mb = 128

[shaders]
cache_size_mb = 64
precompile_at_boot = true

[quotas]
desktop_mb = 128
orbit_mb = 512
default_mb = 64
```

<br>

## GPU Backends

| Backend | Platform |
|---------|----------|
| `null` | No GPU (CPU fallback) — default |
| `wgpu` | Cross-platform (WebGPU) |
| `Metal` | macOS |
| `Vulkan` | Linux |

<br>

## VRAM

| Setting | Value | Description |
|---------|-------|-------------|
| Compressed VRAM | Enabled | VRAM pages compressed when idle |
| Decompression scratch | 128 MB | Scratch buffer for decompression |

<br>

## Shaders

| Setting | Value | Description |
|---------|-------|-------------|
| Cache size | 64 MB | Compiled shader cache |
| Precompile at boot | Enabled | Precompile known shaders on startup |

<br>

## Quotas

| App | VRAM Quota |
|-----|------------|
| **Orbit** | 512 MB |
| **Desktop** | 128 MB |
| **Default** | 64 MB |

<br>

## Related

- [VRM — RAM Manager Configuration](vrm.toml.md)
- [Kernel B Configuration](kernel-b.toml.md)
- [SBP Binary Protocol](../apis/sbp-protocol.md)

<br>

---

[← Back: Documentation Index](../index.md)
