# Volt Daemons — Rust Backend

A suite of six native **Rust daemons** managing system resources with fine-grained control, memory safety, and zero-cost abstractions. They communicate with Kernel A via the Samaris Binary Protocol (SBP) over Unix sockets.

<br>

## Tesseract Engine (Kernel B)

The native Rust daemon providing low-level system acceleration, boot coordination, and IPC infrastructure.

| Feature | Value |
|---------|-------|
| Socket | `/run/samaris/volt-kernel-b.sock` |
| Protocol | JSON-RPC 2.0 (prefixed with "J" byte) |
| Scheduler tick | 16ms (aligned to 60fps) |
| Watchdog interval | 2000ms |
| Thermal throttle | 85°C (emergency 95°C, critical 100°C) |
| Scheduling | `Nice=-20`, `IOSchedulingClass=realtime` |
| Orbit reservation | 4 workers, 512 MB memory |

**Config:** [`kernel-b.toml`](../config/kernel-b.toml.md) | **VOLT spec:** [ch.4](../spec/04-Tesseract-Engine.md)

<br>

## VRM — Volt RAM Manager

Deterministic memory manager with adaptive compression, content-aware deduplication, per-application memory quotas, and pressure-responsive allocation.

| Feature | Value |
|---------|-------|
| Pipeline | Observer → Classifier → Compressor → Deduplicator → Quota Engine → Pressure Controller → GC Coordinator |
| Monitoring interval | 1000ms |
| Memory tiers | T1 (hot, no compression), T2 (warm, LZ4/ZSTD L1), T3 (cold, ZSTD L3+) |
| Inactivity timeout | 5000ms before demotion |
| Compression | ZSTD (levels 1–5), LZ4 for binary buffers |
| Deduplication | SHA-256 truncated 64-bit, verify on match |
| Pressure zones | Green <65% → Yellow ≥70% → Orange ≥85% → Red ≥95% |
| Orbit quota | 2048 MB (critical priority, T1 tier) |
| Desktop quota | 256 MB (critical priority) |

**Config:** [`vrm.toml`](../config/vrm.toml.md) | **VOLT spec:** [ch.8](../spec/08-RAM-Manager.md)

<br>

## DWP — Dynamic Worker Pool

Cooperative adaptive priority scheduler with desktop frame guard, Orbit burst support, and starvation prevention.

| Feature | Value |
|---------|-------|
| Max workers | 48 |
| Yield budget | 50 µs (cooperative) |
| Desktop frame budget | 16ms (60 FPS target) |
| Latency guard | 8ms (preemptive load reduction) |
| Priority levels | Critical, High, Normal, Idle |
| Fairness aging | Priority boost after 200ms |
| Starvation limit | 1000ms (guaranteed scheduling) |
| Orbit priority | Critical |
| Orbit burst | 100ms window, 75% workers, 3 max consecutive |
| Scaling cooldown | 5000ms (prevents oscillation) |
| Thermal backoff | Enabled; disables orbit burst on thermal pressure |

**Config:** [`dwp.toml`](../config/dwp.toml.md) | **VOLT spec:** [ch.11](../spec/11-Dynamic-Worker-Pool.md)

<br>

## VGM — Volt GPU Manager

Graphics adaptation layer with VRAM tiering, shader compilation cache, compute scheduling, and thermal-aware backoff.

| Feature | Value |
|---------|-------|
| Backends | Metal, Vulkan, wgpu, CPU fallback |
| VRAM tiers | Active (uncompressed) → Warm (LZ4) → Cold (ZSTD L1) → CPU-side |
| VRAM scratch | 128 MB |
| Shader cache | 64 MB (precompiled at boot) |
| Max concurrent compute jobs | 4 |
| Frame budget | 16ms |
| Thermal normal | <80°C |
| Thermal reduce | ≥80°C (reduce shader complexity, throttle compute) |
| Thermal CPU fallback | ≥100°C |

**Config:** [`vgm.toml`](../config/vgm.toml.md) | **VOLT spec:** [ch.9](../spec/09-GPU-Manager.md)

<br>

## VUM — Volt USB Manager

Journaled storage manager with write caching, device hotplug detection, and FUSE integration for removable storage.

| Feature | Value |
|---------|-------|
| Read cache | 256 MB (LRU eviction at 90% utilisation) |
| Write buffer | 128 MB |
| Flush interval | 5000ms |
| Poll interval | 2000ms (udev + polling) |
| Journal | WAL with fsync on record, CRC32 checksums |
| Journal path | `/var/lib/samaris/volt-usb-manager/journal.wal` |
| IO alignment | 128 KiB (NAND-aware) |
| Max concurrent flushes | 2 |
| Write durability modes | journaled (full safety), buffered (performance) |

**Config:** [`vum.toml`](../config/vum.toml.md) | **VOLT spec:** [ch.10](../spec/10-USB-Storage-Manager.md)

<br>

## ASC — Adaptive System Config

Hardware-aware policy compiler. Runs once at boot as a oneshot service before any other VOLT component.

| Detection | Source | Output |
|-----------|--------|--------|
| CPU vendor, cores, arch | `/proc/cpuinfo`, `uname -m` | Profile selection |
| Total RAM | `/proc/meminfo`, `sysinfo()` | Budget calculation |
| GPU availability, VRAM | `/dev/dri`, `lspci`, `wgpu` | Rendering profile |
| Storage type, boot medium | `/sys/block`, `/proc/cmdline` | Cache configuration |
| VM detection | CPUID, `dmidecode` | Virtualisation profile |
| Laptop detection | ACPI battery, chassis type | Powersave profile |
| Network interfaces | `/sys/class/net` | Interface configuration |

Generates output to `/run/samaris/adaptive.generated.toml`, consumed by all VOLT modules.

**Config:** [`asc.toml`](../config/asc.toml.md) | **VOLT spec:** [ch.7](../spec/07-Adaptive-System-Config.md)

<br>

<br>

## VDM — VOLT Display Manager

Automatic display detection, configuration, and adaptation layer. Detects connected screens, computes optimal layouts, applies configuration through xrandr, watches for hotplug events, and recovers gracefully from bad configurations.

| Feature | Value |
|---------|-------|
| Backend | xrandr (Xorg), wlr-randr (Wayland — Beta) |
| Layout modes | auto, single, mirror, extend |
| Scale detection | Resolution-based DPI classification (1.0x / 1.5x / 2.0x) |
| Hotplug | udev DRM events + 2s polling fallback |
| Debounce | 300–800ms event coalescing |
| Safe mode | 1280×720, 1.0x, single screen |
| Rollback | Snapshots last good config before apply |
| Generated files | `/run/samaris/display.generated.toml`, `display.event.json` |
| User config | `~/.config/samaris/display.toml` |
| Systemd | user services with `After=graphical-session.target` |

**Module doc:** [`volt-display-manager`](../modules/daemons/vdm.md) | **Source:** `builder/content/volt-display-manager/`

<br>

## Bench — Performance Measurement Suite

Built-in performance measurement, scoring, and certification layer. Collects metrics across the full stack and produces a score out of 10,000.

| Feature | Value |
|---------|-------|
| Collectors | 12 (System, Boot, Memory, CPU, Disk, Network, VRM, VGM, DWP, VUM, Kernel B, UI apps) |
| Scoring | 9 categories, weighted, 0–10,000 |
| Modes | quick, full, stress, watch, ci |
| Methodology | Median of 3–5 iterations, warmup, variance tracking |
| Reliability | 11 reliability flags (THERMAL, MISSING, VARIANCE, etc.) |
| Output | `/var/lib/samaris/bench/latest.json`, `history.json`, `optimizer-input.json` |
| AutoOptimizer | Generates structured input for future self-tuning |

**Module doc:** [`bench`](../modules/bench.md) | **Source:** `builder/content/Bench/`

<br>

## Profiles

| Profile | Target | Workers | Compression | GPU | Display | Benchmark |
|---------|--------|---------|-------------|-----|---------|-----------|
| **Safe** | Unknown hardware | 1–2 | Moderate | Software | 1280×720, 1.0x | Disabled |
| **Low RAM** | ≤2 GB | 1–2 | Aggressive (ZSTD L5) | CPU fallback | Auto-detect, 1.0x | Quick only |
| **Balanced** | 4–8 GB, 4+ cores | 2–4 | Enabled (ZSTD L3) | Hardware accelerated | Auto-detect, HiDPI aware | Full |
| **Performance** | 16+ GB, 6+ cores | 4–8 | Conservative (ZSTD L1) | Premium, full effects | Auto-detect, Retina | Full + stress |
| **Powersave** | Laptop, battery | Reduced | Moderate | Balanced, lower clock | Auto-detect, lower res | Quick only |
| **VM** | Virtualised | 1–2 | Moderate | CPU fallback or virtio | Safe mode fallback | Quick only |
| **USB Boot** | Live USB | Adaptive | Enabled | Hardware dependent | Auto-detect, persistent config | Full |
| **Debug** | Development | As detected | Configurable | As detected | Auto-detect, verbose logging | All modes |

<br>

---

[← Back: Architecture Overview](overview.md)
