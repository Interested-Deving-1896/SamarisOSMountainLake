# Volt GPU Manager

**GPU Orchestration Layer for Samaris OS**

VGM manages GPU detection, backend selection, VRAM residency (T1/T2/T3 compression
tiers), shader caching, compute/render scheduling, multi-GPU orchestration, thermal
monitoring, and the SBP-GPU inter-process protocol.

## Build

```bash
# Build with default features (wgpu + compressed_vram + shader_cache + metrics)
cargo build

# Build all features
cargo build --all-features

# Build release
cargo build --release
```

## CLI Commands

```bash
# Run the daemon with default config
cargo run

# Run with custom config
cargo run -- --config /path/to/config.toml

# Run all tests
cargo test --all-features

# Run benchmarks
cargo bench

# Lint and format check
cargo clippy --all-targets --all-features
cargo fmt --check
```

## Configuration

See `config.example.toml` for the complete configuration schema. All sections:

```toml
[gpu]
backend = "wgpu"           # wgpu, vulkan, metal, null, auto
frame_budget_ms = 16

[gpu.vram]
max_vram_percent = 90
reserved_mb = 256
t1_pool_size_mb = 1024
t2_pool_size_mb = 4096
scratch_budget_mb = 64

[gpu.vram.compression]
algorithm = "zstd"
level = 3

[gpu.vram.deduplication]
enabled = false
block_size = 4096

[gpu.scheduler]
queue_depth = 64
max_inflight = 4

[gpu.shaders]
cache_size = 128

[gpu.compute]
max_workgroups = 65535

[gpu.multi_gpu]
mode = "single"

[gpu.thermal]
throttle_temp = 85.0
emergency_temp = 95.0
poll_interval_ms = 1000

[gpu.quotas]
max_allocations = 1024
max_pinned_mb = 512
```

## Architecture Overview

```
Applications (Orbit, Desktop, Kernel B)
        │
        ▼
┌─────────────────────────┐
│   Volt GPU Manager      │
│  ┌───────────────────┐  │
│  │  SBP-GPU Protocol │  │
│  ├───────────────────┤  │
│  │  Scheduler        │  │
│  │  (Priority Queue) │  │
│  ├───────────────────┤  │
│  │  Backend Layer    │  │
│  │  wgpu/Vulkan/Metal│  │
│  ├───────────────────┤  │
│  │  VRAM Residency   │  │
│  │  T1→T2→T3 Pool   │  │
│  ├───────────────────┤  │
│  │  Shader Cache     │  │
│  │  + Builtins       │  │
│  ├───────────────────┤  │
│  │  Thermal Watchdog │  │
│  └───────────────────┘  │
└─────────────────────────┘
```

## Integration with Samaris OS

### Kernel B (SBP-GPU Protocol)

Applications communicate with VGM via the SBP-GPU protocol over Unix domain
sockets. Messages use a 36-byte header with CRC32 checksums. 15 GPU opcodes
(0x40-0x4E) map to GPU operations with 5 permission levels.

### Kernel A (System Daemon)

VGM runs as a system daemon (`vgmd`) managed by Kernel A's init system.
Configuration is loaded from `/etc/volt-gpu-manager/config.toml` by default.

### Orbit (Compute Engine)

Orbit's compute pipeline uses VGM for:
- AI/matrix multiplication workloads (`ai_matmul` shader)
- Image processing (blur, shadow, gradient)
- Texture packing and atlas generation

### Desktop (Render Engine)

The Desktop compositor uses VGM for:
- UI rendering and compositing
- Glyph atlas management
- Frame budget monitoring and scheduling
- Desktop frame compression protection

## License

Samaris OS — Proprietary
