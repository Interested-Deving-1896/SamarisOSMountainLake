# Volt ASC Specification

## Hardware Profile

The `HardwareProfile` struct contains detected hardware information:
- CPU cores/threads/model/arch
- RAM total/available/swap
- GPU availability and vendor/model/memory
- Boot medium (USB/internal/network)
- Storage type (USB/HDD/SSD/NVMe)
- USB speed (2.0/3.0/4.0)
- VM detection
- Laptop/battery detection
- Detection confidence per component

## Machine Classification

Classes are cumulable (a machine can be multiple):

| Class | Rule |
|-------|------|
| `low_ram` | RAM < 4 GB |
| `high_memory` | RAM >= 32 GB |
| `server` | Cores >= 32 \|\| RAM >= 32 GB |
| `workstation` | Cores >= 16 \|\| RAM >= 16 GB |
| `standard_laptop` | Laptop && RAM < 16 GB |
| `performance_laptop` | Laptop && RAM >= 16 GB |
| `virtual_machine` | VM detected |
| `usb_boot` | Boot medium is USB |
| `cpu_only` | No GPU |
| `battery_powered` | Battery present |
| `thermal_sensitive` | Laptop \|\| battery \|\| thermal sensor |
| `desktop` | Not laptop, not VM, cores < 16, RAM < 16 GB |
| `constrained` | Cores <= 4 \|\| RAM < 4 GB |

## Policies

See `POLICIES.md` for per-module formulas.

## Budget System

- `samaris_budget_cap()`: RAM-based cap for total Samaris allocation
  - < 2 GB: 55%
  - < 8 GB: 65%
  - >= 8 GB: 75%
- Reconciliation order (when budget exceeds cap):
  1. VUM cache
  2. VRM cache
  3. VUM buffer
  4. Orbit quota
  5. Desktop (last resort, with warning)

## Profiles

See `PROFILES.md` for per-profile modifications.

## Safety

See `SAFETY.md` for invariants and constraints.
