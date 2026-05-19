# ASC — Volt Adaptive System Configuration

**Hardware-aware policy compiler for Samaris OS. One OS. All hardware. Zero manual tuning.**

ASC is the hardware calibration layer of Samaris OS. At boot, it detects the machine's hardware, classifies the machine type, computes safe system budgets for all Samaris modules, generates a coherent configuration, validates safety constraints, and explains every decision.

<br>

## Role & Boot Sequence

ASC runs once at boot as a **oneshot systemd service** before any other VOLT component:

```
Boot → ASC service (oneshot) → adaptive.generated.toml → consumed by all VOLT modules
                                                              │
                                    ┌─────────────────────────┼─────────────────────────┐
                                    ▼                         ▼                         ▼
                                Kernel B                    VRM                      DWP
                              (workers, IPC)           (quotas, cache)          (min/max workers)
                                    ▼                                                 ▼
                                    VGM                                              VUM
                              (VRAM, backends)                                   (cache, buffers)
```

<br>

## Hardware Detection

| Component | Source | What's Detected |
|-----------|--------|----------------|
| CPU | `/proc/cpuinfo`, `uname -m` | Vendor, cores, threads, model, architecture |
| RAM | `/proc/meminfo`, `sysinfo()` | Total, available, swap |
| GPU | `/dev/dri`, `lspci`, `wgpu` | Availability, vendor, model, VRAM |
| Storage | `/sys/block`, `lsblk` | Type (USB/HDD/SSD/NVMe), capacity |
| Boot medium | `/proc/cmdline` | USB / internal / network |
| VM | CPUID, `dmidecode` | Virtualisation detection |
| Laptop | ACPI battery, chassis type | Battery presence, thermal sensitivity |
| Network | `/sys/class/net` | Interfaces, type, link status |

All detections include a confidence metric per component.

<br>

## Machine Classification

Classes are cumulative — a machine can match multiple:

| Class | Rule | Effect |
|-------|------|--------|
| `low_ram` | RAM < 4 GB | Aggressive compression, reduced caches |
| `high_memory` | RAM ≥ 32 GB | Large caches, performance profile |
| `server` | Cores ≥ 32 or RAM ≥ 32 GB | High worker counts, stable config |
| `workstation` | Cores ≥ 16 or RAM ≥ 16 GB | Balanced performance |
| `standard_laptop` | Laptop && RAM < 16 GB | Powersave profile |
| `performance_laptop` | Laptop && RAM ≥ 16 GB | Performance with battery awareness |
| `virtual_machine` | VM detected | Conservative resource allocation |
| `usb_boot` | Boot medium is USB | Reduced write amplification |
| `cpu_only` | No GPU | CPU fallback for graphics |
| `battery_powered` | Battery present | Thermal sensitivity, power saving |
| `thermal_sensitive` | Laptop or battery or thermal sensor | Temperature-aware scheduling |
| `desktop` | Not laptop, not VM | Standard configuration |
| `constrained` | Cores ≤ 4 or RAM < 4 GB | Minimum resource budget |

<br>

## Budget System

### Global Budget Cap
| RAM | Cap |
|-----|-----|
| < 2 GB | 55% of total RAM |
| < 8 GB | 65% of total RAM |
| ≥ 8 GB | 75% of total RAM |

### Reconciliation Order
When total budget exceeds cap, resources are reclaimed in order:
1. VUM cache
2. VRM cache
3. VUM buffer
4. Orbit quota
5. Desktop (last resort, with warning)

<br>

## Per-Module Policies

### Kernel B
```
workers = (cpu_cores × 3/4).max(2).min(48)
VM:      (cpu_cores / 2).max(2).min(8)
```

### Dynamic Worker Pool
```
min_workers = (cpu_cores / 3).max(2).min(12)
max_workers = (cpu_cores × 3/4).max(min).min(48)
desktop_min = 1
system_min  = cores ≥ 8 ? 2 : 1
orbit_max   = dwp_max × 3/4
orbit_burst_max = dwp_max
orbit_burst_window_ms = laptop/battery ? 250 : 500
```

### VRM
```
desktop_quota_mb = (RAM/16).max(64).min(512)
cache_mb = RAM/16 (capped by total RAM tiers)
```
Pressure policy has 3 tiers based on total RAM (< 2 GB, < 8 GB, ≥ 8 GB).

### VUM
```
cache_mb: RAM/16, adjusted by storage type (USB higher, NVMe lower)
buffer_mb = cache / 2
flush_interval_ms: USB=5000, HDD=10000, SSD=15000, NVMe=30000
batch_size_kb: USB3+=256, else 128
prefetch_boot_assets: true if USB boot
```

<br>

## Output Files

| File | Path | Purpose |
|------|------|---------|
| Generated config | `/run/samaris/adaptive.generated.toml` | Consumed by all VOLT modules |
| Hardware profile | `/var/lib/samaris/asc/last-hardware-profile.json` | Hardware detection results |
| Explain report | `/var/lib/samaris/asc/last-explain-report.md` | Human-readable decision log |

<br>

## CLI Commands

```bash
volt-asc probe        # Probe hardware and show profile
volt-asc generate     # Generate full config
volt-asc explain      # Explain all decisions
volt-asc dry-run      # Show config without writing
volt-asc check        # Validate config
volt-asc write        # Write generated config to disk
volt-asc --profile safe generate  # Force a specific profile
```

<br>

## Profiles

| Profile | Target | Workers | Compression | GPU |
|---------|--------|---------|-------------|-----|
| **Safe** | Unknown hardware | 1–2 | Moderate | Software |
| **Low RAM** | ≤2 GB | 1–2 | Aggressive (ZSTD L5) | CPU fallback |
| **Balanced** | 4–8 GB, 4+ cores | 2–4 | Enabled (ZSTD L3) | Hardware accelerated |
| **Performance** | 16+ GB, 6+ cores | 4–8 | Conservative (ZSTD L1) | Premium, full effects |
| **Powersave** | Laptop, battery | Reduced | Moderate | Balanced, lower clock |
| **VM** | Virtualised | 1–2 | Moderate | CPU fallback or virtio |
| **USB Boot** | Live USB | Adaptive | Enabled | Hardware dependent |
| **Debug** | Development | As detected | Configurable | As detected |
