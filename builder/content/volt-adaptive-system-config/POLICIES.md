# Volt ASC Policies

## Kernel B

`kernel_b_workers(hw) = (cpu_cores * 3/4).max(2).min(48)`

VM: `(cpu_cores / 2).max(2).min(8)`

| Hardware | Cores | Workers |
|----------|-------|---------|
| Pi 5 | 4 | 3 |
| Laptop | 8 | 6 |
| Desktop | 16 | 12 |
| Workstation | 32 | 24 |
| Server | 64 | 48 |
| VM | 4 | 2 |

## Dynamic Worker Pool

- `dwp_min_workers = (cpu_cores / 3).max(2).min(12)`
- `dwp_max_workers = (cpu_cores * 3/4).max(min).min(48)` (VM: capped at 8)
- `desktop_min_workers = 1`
- `system_min_workers = cores >= 8 ? 2 : 1`
- `orbit_default_max = dwp_max * 3/4`
- `orbit_burst_max = dwp_max`
- `orbit_burst_window_ms = laptop/battery ? 250 : 500`

## VRM

- `desktop_quota_mb = (ram/16).max(64).min(512)`
- `orbit_quota_mb` depends on RAM, VM, GPU (see code)
- `cache_mb = (ram/16)` with caps by total RAM

Pressure policy has 3 tiers based on total RAM (<2GB, <8GB, >=8GB).

## VUM

- `cache_mb`: RAM/16, adjusted by storage type (USB higher, NVMe lower)
- `buffer_mb = cache / 2`
- `flush_interval_ms`: USB=5000, HDD=10000, SSD=15000, NVMe=30000
- `batch_size_kb`: USB3+=256, else 128
- `prefetch_boot_assets`: true if USB boot

## Global Budget

`samaris_budget_cap`:
- RAM < 2 GB: 55%
- RAM < 8 GB: 65%
- RAM >= 8 GB: 75%
