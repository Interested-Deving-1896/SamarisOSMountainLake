# Volt GPU Manager — Feature Specification

## 1. GPU Detection

The system probes available GPU backends in order: Wgpu, Vulkan, Metal.
If no real GPU is found, the NullBackend is used as fallback.
Detection never panics on missing GPU hardware.

## 2. Backend Selection

Backend is selected from config (`gpu.backend` field in `VgmConfig`):
- `"wgpu"` — WebGPU backend (default)
- `"vulkan"` — Vulkan backend
- `"metal"` — Metal backend
- `"null"` — Null stub backend
- `"auto"` — Probe and select first available

Each backend exposes capabilities via `GpuCapabilities`:
- `compressed_vram`: T2 VRAM compression support
- `native_texture_compression`: Hardware texture compression
- `gpu_zstd` / `gpu_lz4`: GPU-accelerated compression algorithms
- `multi_gpu`: Multi-GPU support
- `shader_cache`: Shader caching support
- `compute`: Compute shader support

## 3. VRAM Residency (T1/T2/T3)

Three-tier VRAM residency model:

**T1 — Active VRAM**: Resources actively bound to the GPU. Bindable, compressible.
**T2 — Compressed VRAM**: Resources compressed in GPU memory. Not bindable without restore.
**T3 — Cold Fallback**: Evicted to system RAM. Used only when T1/T2 are full.

Key operations:
- `allocate_t1`: Allocate in active VRAM (checks quotas)
- `compress_to_t2`: Compress T1 → T2 (checks compressibility)
- `restore_to_t1`: Decompress T2 → T1 (checks scratch budget)
- `evict_to_t3`: Evict to fallback (any tier)

## 4. Compressed VRAM Pool

The compressed pool stores `CompressedVramBlock` entries with:
- Original and compressed sizes
- Compression algorithm
- CRC32 checksum
- Creation and last-restore timestamps

Pool capacity is bounded; full pool returns `CompressedPoolFull` error.

## 5. Shader Cache

Builtin shaders (12 total) are registered via `BuiltinShaderRegistry`.
The `ShaderCache` provides insert/lookup with hit/miss counting.
Cache size is bounded in megabytes; exceeding capacity triggers eviction.

## 6. Compute Engine

Compute jobs (`GpuComputeJob`) support: Blur, Shadow, Composite, Transform2D,
MatMul, TexturePack, MipmapGenerate, VramCompress, VramDecompress, VideoAssist.

## 7. Render Engine

Render frame management via `DesktopFrameGuard` and `FrameBudget`:
- Configurable frame budget (default 16ms / 60 FPS)
- Miss tracking and frame pressure detection
- Priority suspension under pressure

## 8. Multi-GPU Orchestration

Features (gated by `multi_gpu` feature):
- Device routing: Assign workloads to specific GPUs
- Power management: Dynamic GPU activation/deactivation
- Fallback: Automatic failover between GPUs
- Fusion: Combine multiple GPUs for single workload

## 9. Thermal Watchdog

Thermal monitoring with configurable thresholds:
- Polling interval: 1000ms default
- Throttle temperature: 85°C
- Emergency temperature: 95°C

Policy actions: Priority blocking, compute reduction, CPU fallback.

## 10. SBP-GPU Protocol

Inter-process GPU control protocol. See `SBP_GPU.md` for full specification.
Opcode range: 0x40–0x4E. Header: 36 bytes fixed-size. Permission model with 5 roles.
