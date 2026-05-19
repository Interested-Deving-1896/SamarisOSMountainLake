# Volt GPU Manager — Architecture

## Overview

Volt GPU Manager (VGM) is the GPU orchestration layer for Samaris OS. It manages GPU
detection, backend selection, VRAM residency (compressed tiers), shader caching,
compute/render scheduling, multi-GPU orchestration, thermal monitoring, and the
SBP-GPU inter-process protocol.

```
┌─────────────────────────────────────────────────────────┐
│                   Samaris OS Applications               │
├────────────┬───────────┬──────────┬─────────────────────┤
│   Orbit    │  Desktop  │  Kernel  │  3rd-party (SBP)    │
└─────┬──────┴─────┬─────┴────┬────┴──────────┬──────────┘
      │            │         │               │
      ▼            ▼         ▼               ▼
┌─────────────────────────────────────────────────────────┐
│              Volt GPU Manager (vgmd)                    │
│                                                         │
│  ┌──────────┐  ┌──────────┐  ┌──────────────────────┐  │
│  │ SBP-GPU  │  │ Scheduler│  │   Shader Cache       │  │
│  │ Protocol │  │ (prio Q) │  │   + Builtins         │  │
│  └────┬─────┘  └────┬─────┘  └──────────┬───────────┘  │
│       │             │                   │              │
│  ┌────▼─────────────▼───────────────────▼───────────┐  │
│  │              Backend Layer                        │  │
│  │  [wgpu] [Vulkan] [Metal] [Null] [CpuFallback]    │  │
│  └───────────────────────┬──────────────────────────┘  │
│                          │                             │
│  ┌───────────────────────▼──────────────────────────┐  │
│  │            VRAM Residency Manager                │  │
│  │  T1 (Active) → T2 (Compressed) → T3 (Fallback)  │  │
│  │  + CompressedPool + ScratchBudget + Quotas       │  │
│  └──────────────────────────────────────────────────┘  │
│                                                         │
│  ┌──────────────────┐  ┌────────────────────────────┐  │
│  │  Thermal Watchdog│  │  Multi-GPU Orchestrator    │  │
│  └──────────────────┘  └────────────────────────────┘  │
└─────────────────────────────────────────────────────────┘
```

## Backend System

The backend layer abstracts GPU access. Each backend implements `GpuBackend` trait:

- **Wgpu** (default): Cross-platform via WebGPU. Enabled by default.
- **Vulkan**: Linux-focused, raw Vulkan. Gated by `vulkan_backend` feature.
- **Metal**: macOS/iOS. Gated by `metal_backend` feature.
- **Null**: Stub backend; always available. Used when no real GPU is found.
- **CpuFallback**: Software fallback for compression when no GPU is available.

Backend selection is automatic. If no real GPU is detected, NullBackend is used.

## VRAM Tiers

| Tier | Name | Location | Bindable | Compressible |
|------|------|----------|----------|-------------|
| T1 | Active VRAM | GPU memory | Yes | Yes |
| T2 | Compressed VRAM | GPU memory (compressed) | No | No |
| T3 | Cold Fallback | System RAM | No | No |

## Scheduler

Priority-ordered command queue with four levels: Critical > High > Normal > Idle.
The DesktopFrameGuard pauses Idle and Normal priorities under frame pressure.
Each priority level has configurable batch sizes (Critical=1, High=4, Normal=8, Idle=16).

## Compute / Render Flow

1. Application submits compute job or render command
2. Scheduler enqueues by priority
3. Frame budget monitor tracks timing
4. Desktop guard adjusts priority during frame pressure
5. Batch dequeue processes by priority order
6. Backend executes the command

## Multi-GPU

Multi-GPU support includes device routing, power management, fallback, and fusion.
Gated by `multi_gpu` feature.

## Thermal

Temperature monitoring drives thermal policy:
- Normal (≤70°C): Full performance
- Warm (≤75°C): Normal operation
- Hot (≤80°C): Reduce non-critical compute
- Throttle (≤85°C): Pause idle jobs, disable burst
- Critical (≤90°C): Only critical priority runs
- Emergency (>95°C): Stop all non-critical, CPU fallback
- Fatal: Immediate shutdown

## SBP-GPU Protocol

Inter-process GPU control protocol. 15 opcodes (0x40-0x4E), 5 permission levels.
Header: 36 bytes (magic + opcode + flags + request_id + timestamp + payload_len + checksum).
