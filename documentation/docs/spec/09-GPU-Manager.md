# 9. GPU Manager

## 9.1 Overview

The Volt GPU Manager (VGM) is the graphics adaptation layer of Samaris OS. Written in Rust as an independent daemon (`volt-gpu-manager`), it manages GPU resource allocation, VRAM tiering, shader compilation, compute scheduling, and thermal-aware behaviour. The VGM ensures that Samaris OS presents a stable graphical interface regardless of GPU capability, from software-rendered virtual machines to high-performance dedicated graphics hardware.

## 9.2 Architecture

The VGM is structured around several coordinated subsystems:

### Device and Backend Layer

Detects and abstracts the underlying graphics hardware:

- Probes available GPU devices via wgpu/WebGPU enumeration
- Determines hardware acceleration capability
- Selects appropriate rendering backend (Vulkan, Metal, D3D12, or CPU fallback)
- Supports multi-GPU configurations
- Safe mode forces CPU fallback when GPU issues are detected

### VRAM Tier Management

Manages video memory through a tiered approach:

| Tier | Content | Strategy |
|------|---------|----------|
| Active VRAM | Visible textures, active framebuffers | Always resident, uncompressed |
| Warm VRAM | Minimised window surfaces, icon caches | Compressed with fast algorithm (LZ4) |
| Cold VRAM | Inactive textures, wallpaper layers | Compressed with ZSTD level 1 |
| CPU-side | Thumbnails, cold snapshot data | Evicted from VRAM, reconstructible |

VRAM compression targets only inactive or reconstructible resources. Visible GPU resources remain uncompressed and immediately accessible. The compression subsystem verifies checksums on decompression.

### Shader Compilation Cache

- Precompiles critical shaders at boot time to reduce runtime jank
- Maintains an on-disk cache (configurable size, default 64 MiB)
- Supports just-in-time compilation for dynamically generated shaders
- Cache is shared across application and compositor rendering

### Compute Scheduling

Manages GPU compute resources for:

- Concurrent compute job execution (max 4 by default)
- Priority scheduling: desktop rendering has critical priority
- Frame budget enforcement (default: 16 ms for 60 FPS target)
- Integration with the Dynamic Worker Pool for coordinated resource management

### Thermal Backoff

Monitors GPU thermal state and responds proportionally:

| Temperature | Response |
|-------------|----------|
| < 80°C | Normal operation |
| ≥ 80°C | Reduce shader complexity, throttle compute jobs |
| ≥ 100°C | Fall back to CPU software rendering |

### Metrics and Monitoring

The VGM exposes:

- VRAM utilisation per tier
- Compression ratios and operation latency
- Shader cache hit rates
- Compute job queue depth
- Thermal state and backoff events
- Frame budget adherence

## 9.3 Graphics Profiles

The VGM supports four rendering profiles, typically selected by ASC based on hardware capability:

| Profile | Hardware | Effects | Rendering |
|---------|----------|---------|-----------|
| Ultra Light | VM, software GPU | Minimal animations | CPU fallback, safe mode |
| Balanced | Integrated GPU | Moderate effects | Hardware accelerated, medium quality |
| Premium | Dedicated GPU | Full effects, animations | Hardware accelerated, high quality |
| Recovery | Failing GPU | All effects disabled | CPU fallback, guaranteed boot |

## 9.4 Quality of Service

The VGM encludes:

- Per-application VRAM quotas (desktop: 128 MiB, Orbit: 512 MiB, default: 64 MiB)
- Desktop rendering always has critical priority
- Animation guard prevents resource-intensive operations during frame rendering

## 9.5 Configuration

The VGM is configured via `/opt/volt/gpu-manager/config.toml`. Key sections include:

- GPU mode (auto/manual), backend selection, safe mode
- VRAM tiers, compression enablement, scratch space allocation
- Scheduler: frame budget, priority scheduling
- Shader cache: size, precompile, JIT settings
- Compute: max concurrent jobs
- Thermal: watchdog, backoff thresholds, CPU fallback
- Per-application VRAM quotas
