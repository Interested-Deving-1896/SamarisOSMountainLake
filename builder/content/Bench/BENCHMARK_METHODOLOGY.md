# Benchmark Methodology

## Reproducibility Rules

Every benchmark result must be reproducible. To ensure reproducibility:

1. **Warmup runs**: At least 1 warmup iteration before measurement begins. Warmup results are not included in the score.
2. **Multiple iterations**: Quick mode runs 3 iterations. Full mode runs 5 iterations. More iterations = higher confidence.
3. **Median over mean**: The final score uses the **median** of all iteration scores, not the mean. Median is more robust against outliers.
4. **Variance tracking**: Standard deviation is reported alongside the score. High variance triggers a `HIGH_VARIANCE` reliability flag.

## Hardware Class

Every result is tagged with a hardware class to prevent cross-class comparisons. Hardware classes are:

| Class | Description |
|-------|-------------|
| `low_power_usb` | USB-booted on low-power hardware (e.g., old laptop from USB stick) |
| `old_intel_desktop` | Pre-2015 Intel desktop or laptop |
| `modern_laptop` | Post-2020 laptop |
| `high_end_desktop` | Desktop with dedicated GPU, >16 GB RAM |
| `raspberry_pi` | ARM SBC |
| `virtual_machine` | VM (VMware, VirtualBox, QEMU) |
| `unknown` | Hardware could not be classified |

Hardware class is detected by:
- CPU model name
- RAM size
- Storage type (NVMe, SSD, HDD, USB)
- GPU model
- DMI/system manufacturer info

## Same-Hardware Comparison Rule

Bench may only claim superiority over another OS if the comparison was measured on the **same physical hardware**. Imported baselines are always marked `reference_only` regardless of how similar the hardware appears to be.

**Valid comparison modes:**
- Same machine, different OS: `comparison_validity = "same_hardware"`
- Imported baseline, different machine: `comparison_validity = "reference_only"`
- No baseline imported: `comparison_validity = "none"`

## Environment-Dependent Metrics

Some metrics depend heavily on external conditions. These are tagged `environment_dependent` and weighted at half the value of `system_controlled` metrics.

| Metric | Classification | Reason |
|--------|---------------|--------|
| RAM idle | system_controlled | Reflects OS memory management |
| CPU idle | system_controlled | Reflects OS scheduling efficiency |
| UI FPS | system_controlled | Reflects rendering pipeline performance |
| App launch times | system_controlled | Reflects app/OS initialization |
| Finder listing speed | system_controlled | Reflects filesystem integration |
| VRM compression ratio | system_controlled | Reflects memory management quality |
| IPC latency | system_controlled | Reflects kernel B responsiveness |
| SBP latency | system_controlled | Reflects module IPC efficiency |
| Network throughput | environment_dependent | Depends on hardware, ISP, WiFi |
| Download speed | environment_dependent | Depends on remote server |
| Disk read/write IOPS | environment_dependent | Depends on SSD/HDD model |
| Boot time | environment_dependent | Depends on USB speed for boot drive |
| Thermal throttling events | environment_dependent | Depends on cooling, ambient temp |

## Reliability Flags

| Flag | Meaning | Trigger |
|------|---------|---------|
| `LOW_ITERATION_COUNT` | Fewer than 3 valid iterations | < 3 iterations completed |
| `HIGH_VARIANCE` | Standard deviation > 5% of score | stddev / median > 0.05 |
| `THERMAL_THROTTLING` | CPU throttling detected during run | thermal events > 0 |
| `NETWORK_UNSTABLE` | Network throughput variance > 20% | stddev of net > 20% mean |
| `DISK_TOO_SLOW` | Disk throughput < expected minimum | seq read < 10 MB/s |
| `VM_ENVIRONMENT` | Running inside a virtual machine | DMI or /proc/cpuinfo hint |
| `USB_BOOT` | System booted from USB | /sys/block/sd*/removable or mount info |
| `MISSING_COLLECTOR` | One or more collectors failed | collector returned None |
| `ESTIMATED_METRIC` | A metric was estimated due to missing data | fallback value used |
| `BASELINE_IMPORTED` | Comparison uses an imported baseline | baseline was not self-measured |
| `BASELINE_NOT_SAME_HARDWARE` | Baseline was imported from different HW | hardware class mismatch |

If any reliability flag is present, the score is still computed but the UI displays a warning.

## Confidence Levels

| Level | Criterion |
|-------|-----------|
| `high` | ≥ 3 iterations, stddev ≤ 3%, no THROTTLING or UNSTABLE flags |
| `medium` | 2–3 iterations OR stddev ≤ 8%, no critical flags |
| `low` | 1 iteration OR stddev > 8% OR any critical flag present |
