# VGM — Volt GPU Manager

**GPU orchestration layer for Samaris OS — detection, VRAM tiering, scheduling, shader caching, thermal monitoring.**

VGM manages GPU detection, backend selection, VRAM residency (T1/T2/T3 compression tiers), shader caching, compute/render scheduling, multi-GPU orchestration, and thermal monitoring. Communicates via the SBP-GPU binary protocol.

<br>

## Architecture

```
Applications (Orbit, Desktop, Kernel B)
        │
        ▼
┌─────────────────────────────────┐
│     Volt GPU Manager (vgmd)     │
│  ┌──────────┐  ┌────────────┐   │
│  │ SBP-GPU  │  │ Scheduler  │   │
│  │ Protocol │  │ (prio Q)   │   │
│  └────┬─────┘  └─────┬──────┘   │
│       │               │          │
│  ┌────▼───────────────▼──────┐   │
│  │      Backend Layer        │   │
│  │ wgpu / Vulkan / Metal /   │   │
│  │ Null / CpuFallback        │   │
│  └───────────┬───────────────┘   │
│  ┌───────────▼───────────────┐   │
│  │  VRAM Residency Manager   │   │
│  │  T1 → T2 → T3 + Quotas   │   │
│  └───────────────────────────┘   │
│  ┌──────────┐ ┌──────────────┐   │
│  │ Thermal  │ │ Multi-GPU    │   │
│  │ Watchdog │ │ Orchestrator │   │
│  └──────────┘ └──────────────┘   │
└───────────────────────────────────┘
```

<br>

## Backend System

| Backend | Feature Gate | Platform | Availability |
|---------|-------------|----------|-------------|
| **Wgpu** | default | Cross-platform (WebGPU) | Always enabled |
| **Vulkan** | `vulkan_backend` | Linux | Optional |
| **Metal** | `metal_backend` | macOS | Optional |
| **Null** | always | All | Stub when no GPU |
| **CpuFallback** | always | All | Software fallback |

Backend selection is automatic. Probes in order: Wgpu → Vulkan → Metal → Null. Config override via `gpu.backend` field.

<br>

## VRAM Residency (T1/T2/T3)

| Tier | Name | Location | Bindable | Compressible | Restore Required |
|------|------|----------|----------|-------------|-----------------|
| T1 | Active VRAM | GPU memory | Yes | Yes | No |
| T2 | Compressed VRAM | GPU memory (compressed pool) | No | No | Yes |
| T3 | Cold Fallback | System RAM | No | No | Yes |

### Operations
- **allocate_t1**: Allocate in active VRAM (quota checked)
- **compress_to_t2**: Compress T1 → T2 (checks compressibility)
- **restore_to_t1**: Decompress T2 → T1 (scratch budget verified)
- **evict_to_t3**: Evict to fallback (any tier)

### Scratch Budget
Before restoring T2 → T1, the system ensures: `free_vram >= reserved_bytes + resource_size + min_free_bytes`

### VRAM Quotas
- `max_allocations`: default 1024 per app
- `max_pinned_mb`: default 512 MB per app

### Compression Ratios
Typically 2:1 to 5:1 for textures, 1.5:1 to 3:1 for compute data.

<br>

## Shader Cache

- **12 builtin shaders** registered via `BuiltinShaderRegistry`
- **Cache**: Bounded (MB), insert/lookup with hit/miss counting
- **Builtins**: Blur, Shadow, Composite, Transform2D, MatMul, TexturePack, MipmapGenerate, VramCompress, VramDecompress, VideoAssist, and more

<br>

## Scheduler

Priority-ordered command queue with four levels: Critical > High > Normal > Idle.

| Priority | Batch Size | Affected by Frame Guard |
|----------|-----------|----------------------|
| Critical | 1 | No |
| High | 4 | No |
| Normal | 8 | Yes |
| Idle | 16 | Yes |

**DesktopFrameGuard**: Pauses Idle and Normal priorities under frame pressure.

<br>

## Thermal Watchdog

| Temp Range | State | Action |
|-----------|-------|--------|
| ≤70°C | Normal | Full performance |
| ≤75°C | Warm | Normal operation |
| ≤80°C | Hot | Reduce non-critical compute |
| ≤85°C | Throttle | Pause idle jobs, disable burst |
| ≤90°C | Critical | Only critical priority runs |
| >95°C | Emergency | CPU fallback, stop non-critical |
| Fatal | Shutdown | Immediate stop |

Polling interval: 1000ms default.

<br>

## SBP-GPU Protocol

Inter-process GPU control protocol. 15 opcodes (0x40–0x4E), 5 permission levels.

### Header (36 bytes)

| Offset | Size | Field |
|--------|------|-------|
| 0 | 4 | Magic (`0x47505542` = `"GPUB"`) |
| 4 | 1 | Opcode |
| 5 | 2 | Flags (Request/Response/Error/Event) |
| 7 | 8 | Request ID |
| 15 | 8 | Timestamp (µs) |
| 23 | 4 | Payload Length |
| 27 | 4 | CRC32 Checksum |
| 31 | 5 | Reserved |

### Key Opcodes

| Code | Name | Permission | Purpose |
|------|------|-----------|---------|
| 0x40 | GpuStatus | CAP_READ_STATUS | Query GPU status |
| 0x41 | GpuAllocResource | CAP_GPU_ALLOC | Allocate GPU resource |
| 0x43 | GpuExecCompute | CAP_GPU_COMPUTE | Execute compute job |
| 0x44 | GpuRenderFrame | CAP_GPU_RENDER | Render a frame |
| 0x45 | GpuThermalStatus | CAP_READ_STATUS | Query thermal status |
| 0x4B | GpuCompressResource | CAP_GPU_ALLOC | Compress VRAM resource |
| 0x4E | GpuMetricsSnapshot | CAP_READ_STATUS | Get metrics snapshot |

<br>

## Integration

- **Kernel B** (SBP-GPU): Applications communicate via Unix domain sockets
- **Kernel A**: VGM runs as `vgmd` system daemon, config at `/etc/volt-gpu-manager/config.toml`
- **Orbit**: Uses VGM for AI/matrix multiplication, image processing
- **Desktop**: Uses VGM for UI rendering, glyph atlas, frame budget

<br>

## Configuration

See [`vgm.toml`](../../config/vgm.toml.md) for the complete configuration reference.

Key settings:
- GPU backend selection
- VRAM pool sizes per tier
- Compression algorithm and levels
- Shader cache size
- Scheduler queue depth and batch sizes
- Thermal thresholds and polling interval
- Multi-GPU mode
