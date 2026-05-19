# VRAM Residency Model

## Overview

VRAM in Volt GPU Manager is managed through a three-tier residency model designed
to maximize GPU memory utilization while ensuring safe, predictable operation.

## Tiers

### T1 — Active VRAM

Resources in T1 are actively bound to the GPU and can be used for rendering or
compute operations. Allocations go through quota checking.

- Location: GPU memory
- Bindable: Yes
- Compressible: Yes
- Requires restore: No

### T2 — Compressed VRAM (GPU-to-GPU)

Resources in T2 are stored in a compressed format within GPU memory. They cannot
be directly bound for rendering/compute — they must first be restored to T1.
Compression is GPU-to-GPU (no system RAM involved).

- Location: GPU memory (compressed pool)
- Bindable: No
- Compressible: No (already compressed)
- Requires restore: Yes

### T3 — Cold Fallback

Resources in T3 have been evicted from GPU memory to system RAM. This is the
coldest tier, used only when T1 and T2 pools are under pressure.

- Location: System RAM
- Bindable: No
- Compressible: No
- Requires restore: Yes

## Scratch Budget

Before a resource can be restored from T2 to T1, the system checks the scratch
budget to ensure sufficient free space is available. The scratch budget is a
reserved portion of VRAM that guarantees restore operations can complete without
OOM.

Formula: `can_restore = (free_vram >= reserved_bytes + resource_size + min_free_bytes)`

## Quotas

Per-application quotas limit:
- Maximum number of allocations (`max_allocations`: default 1024)
- Maximum pinned memory (`max_pinned_mb`: default 512 MB)

Quotas are enforced at allocation time in `VramResidencyManager::allocate_t1`.

## Invariants

| Invariant | Enforced By |
|-----------|------------|
| T2 is in VRAM, not RAM | `InvariantChecker::check_t2_is_vram_not_ram` |
| No restore without scratch budget | `InvariantChecker::check_no_restore_without_scratch` |
| No Desktop frame compression | `InvariantChecker::check_no_desktop_compression` |
| No current frame compression | `InvariantChecker::check_no_current_frame_compression` |
| No fake compression ratio | `InvariantChecker::check_no_fake_compression_ratio` |
| No panic on missing GPU | `InvariantChecker::check_no_panic_on_missing_gpu` |

## Compression Ratios

Compression ratios are tracked by `GpuCompressionRatioTracker`:
- Tracks raw vs. compressed bytes across all compression operations
- Reports average ratio and total bytes saved
- Ratios from real GPU-accelerated compression typically range from 2:1 to 5:1
  for textures, 1.5:1 to 3:1 for compute data
