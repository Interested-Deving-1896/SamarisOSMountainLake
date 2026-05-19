# Hardware Matrix

System requirements and tested hardware configurations.

<br>

## Requirements

| Component | Minimum | Recommended | Optimal |
|-----------|---------|-------------|---------|
| **CPU** | 2 cores, x86_64 | 4 cores | 8+ cores |
| **RAM** | 4 GB | 8 GB | 16 GB |
| **Storage** | 16 GB | 32 GB SSD | 64 GB+ NVMe |
| **GPU** | Intel HD / basic | Metal / CUDA capable | Dedicated GPU |
| **WiFi** | Any supported | Intel AX200+ | Intel AX210 |
| **Display** | 1280×720 | 1920×1080 | 2560×1440+ |

<br>

## Architecture Support

| Architecture | Status | Notes |
|-------------|--------|-------|
| **x86_64** (amd64) | ✅ Full | Primary target |
| **aarch64** (arm64) | ✅ Full | Raspberry Pi 4/5, Apple Silicon (VM) |

<br>

## ASC Hardware Profiles

The Adaptive System Configuration (ASC) subsystem detects your hardware at boot and selects the optimal profile:

| Profile | Target | Workers | RAM | GPU Mode |
|---------|--------|---------|-----|----------|
| **Safe** | Unknown/unreliable hardware | 1–2 | Any | Software rendering |
| **Low RAM** | ≤2 GB RAM | 1–2 | ≤2 GB | CPU fallback |
| **Balanced** | Standard desktop | 2–4 | 4–8 GB | Hardware accelerated |
| **Performance** | High-end workstation | 4–8 | 16+ GB | Premium, full effects |
| **Powersave** | Laptop, battery | Reduced | Any | Lower clock, reduced |
| **VM** | Virtualised | 1–2 | Any | CPU fallback or virtio |
| **USB Boot** | Live USB | Adaptive | Any | Hardware dependent |
| **Debug** | Development | As detected | Any | As detected |

<br>

## Tested Hardware

| Device | Config | Status |
|--------|--------|--------|
| Apple MacBook Air **M3** | 16 GB RAM | ✅ Primary dev machine |
| Intel **i5-12400** desktop | 32 GB RAM, RTX 3060 | ✅ |
| Intel **i7-13700H** laptop | 16 GB RAM, Iris Xe | ✅ |
| **Raspberry Pi 5** | 8 GB RAM (arm64) | ⚠️ Limited testing |
| Intel **Mac Mini 2012** | 16 GB RAM, Ivy Bridge | ✅ Real-hardware boot validated |

<br>

---

[← Back: Documentation Index](../index.md)
