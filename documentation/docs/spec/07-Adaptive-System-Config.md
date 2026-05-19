# 7. Adaptive System Configuration

## 7.1 Overview

The Adaptive System Configuration (ASC) subsystem is the hardware-aware policy compiler of Samaris OS. It runs as a oneshot systemd service (`volt-asc.service`) early in the boot sequence, before any other VOLT component. ASC detects the machine's hardware capabilities and generates a comprehensive configuration policy that governs the behaviour of all subsequent VOLT modules. Its fundamental purpose is to eliminate hardcoded machine assumptions — the same ISO image adapts to run on diverse hardware without manual tuning.

## 7.2 Hardware Detection

ASC probes the following hardware dimensions:

| Parameter | Detection Method | Values |
|-----------|-----------------|--------|
| CPU vendor | /proc/cpuinfo | Intel, AMD, ARM, unknown |
| Core count | OS enumeration | 1–N (detected) |
| Architecture | uname -m | x86_64, aarch64 |
| Total RAM | /proc/meminfo or sysinfo | MiB (detected) |
| GPU availability | /dev/dri, lspci fallback | available, unavailable |
| VRAM estimate | heuristics from GPU class | MiB (estimated) |
| Hardware acceleration | DRI device presence | yes, no |
| Storage type | /sys/block heuristics | SSD, HDD, NVMe, unknown |
| Boot medium | /proc/cmdline + udev | USB, SATA, NVMe, VM |
| Virtualisation | CPUID + dmidecode | VM, physical |
| Laptop detection | acpi, battery presence | yes, no |
| Network interfaces | /sys/class/net | interface list |

## 7.3 Profile Selection

Based on detected hardware, ASC selects one of eight available profiles:

| Profile | Target | RAM | CPU | GPU | Use Case |
|---------|--------|-----|-----|-----|----------|
| safe | Any | minimal | 1+ | any | Fallback when detection fails |
| low_ram | Low-end | ≤2 GB | 2+ | any | Very constrained machines |
| balanced | Standard | 4–8 GB | 4+ | integrated | General desktop use |
| performance | High-end | 16+ GB | 6+ | dedicated | Power-user workstations |
| powersave | Laptop | any | any | any | Battery optimisation |
| vm | Virtualised | any | any | virtio | Virtual machine guests |
| usb_boot | USB live | any | any | any | Live USB operation |
| debug | Development | any | any | any | Tracing and verbose logging |

## 7.4 Policy Generation

The selected profile drives a budget calculator that determines:

- **Worker pool**: minimum and maximum worker counts, desktop reservation, orbit burst limits
- **Memory management**: compression enablement, target compression ratio, pressure thresholds
- **GPU configuration**: rendering profile, animation quality, hardware acceleration settings
- **Storage management**: cache size, buffer allocation, journal mode
- **UI density**: scale factor, desktop density mode

The generated policy is written as a TOML configuration file to `/run/samaris/adaptive.generated.toml`, which is then consumed by all VOLT modules at initialisation time.

## 7.5 Explanation Feature

ASC includes an optional report generator that produces a human-readable explanation of its decisions. When enabled, it writes a markdown report to `/var/lib/samaris/asc/last-explain-report.md`, documenting:

- Detected hardware parameters
- Selected profile and its rationale
- Budget allocations for each subsystem
- Configuration overrides and their justification

This feature is designed to aid debugging and system tuning during development.

## 7.6 Output Artifacts

| Path | Content |
|------|---------|
| `/run/samaris/adaptive.generated.toml` | Active configuration for all VOLT modules |
| `/var/lib/samaris/asc/last-hardware-profile.json` | Raw hardware detection results |
| `/var/lib/samaris/asc/last-generated-config.toml` | Archived copy of last generated configuration |
| `/var/lib/samaris/asc/last-explain-report.md` | Human-readable decision explanation |
| `/run/volt-asc.complete` | Readiness signal created by systemd ExecStartPost |

## 7.7 Failure Behaviour

If ASC fails to execute or produces an invalid configuration, subsequent VOLT modules fall back to their built-in default configurations. The system continues to boot but operates with conservative assumptions (safe profile). The Unifier detects the missing ASC configuration and may trigger a regeneration attempt.
