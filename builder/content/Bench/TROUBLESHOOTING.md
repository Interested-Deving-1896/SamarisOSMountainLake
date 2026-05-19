# Troubleshooting

## Missing Permissions

**Symptom**: `Permission denied` when reading `/proc/` or `/sys/`

**Fix**: Bench uses standard system files. If running inside a container, ensure the `/proc` and `/sys` filesystems are mounted. For SBP IPC sockets, ensure the `samaris` group has access.

## Missing Collectors

**Symptom**: A category score is 0 with `MISSING_COLLECTOR` flag

**Common causes:**
- VRM not running: start `volt-ram-manager.service`
- VGM not running: start `volt-gpu-manager.service`
- DWP not running: start `volt-worker-pool.service`
- VUM not running: start `volt-usb-manager.service`
- Kernel B not running: start `volt-kernel-b.service`

All missing collectors produce a reliability flag but do not crash the benchmark.

## No systemd-analyze

**Symptom**: Boot collector fails

**Fix**: `systemd-analyze` is not available in non-systemd environments or containers. The boot collector falls back to reading `/proc/uptime`. The `MISSING_COLLECTOR` flag is added.

## No Network

**Symptom**: Network collector hangs or fails

**Fix**: The network collector has a 5-second timeout. If no network is available, it reports 0 throughput and adds the `NETWORK_UNSTABLE` flag. The benchmark does not fail.

## No VRM Socket

**Symptom**: VRM collector fails

**Fix**: Ensure `volt-ram-manager.service` is running. The Unix socket should be at `/run/samaris/volt-ram-manager.sock`. If the socket path is different, update the config file.

## No Kernel B SBP

**Symptom**: Kernel B collector fails

**Fix**: Ensure `volt-kernel-b.service` is running. Socket at `/run/samaris/volt-kernel-b.sock`.

## Running in a Virtual Machine

**Symptom**: `VM_ENVIRONMENT` flag set

**Impact**: Scores are compared only against other VM baselines. Cross-class comparison is automatically marked `reference_only`. Hardware detection may be less accurate.

## Running from USB

**Symptom**: `USB_BOOT` flag set

**Impact**: Boot time and disk I/O metrics are affected by USB speed. These metrics are tagged as `environment_dependent` and weighted lower. The flag warns users that results may not reflect internal storage performance.

## High Score Variance

**Symptom**: `HIGH_VARIANCE` flag set, stddev > 5%

**Possible causes:**
- Thermal throttling during the run
- Background processes consuming resources
- Network instability affecting browser tests
- USB boot causing inconsistent I/O

**Fix**: Run `bench --full` which uses more iterations. Check for background processes with `top` or `htop`. Ensure adequate cooling.

## Fake Baseline Prevention

**Symptom**: A baseline comparison shows `reference_only` when you expected `same_hardware`

**Causes:**
- The baseline was imported from a different machine
- The hardware class of the baseline differs from current hardware
- The baseline file does not contain matching hardware info

**Policy**: Bench never assumes same hardware. If the hardware data matches exactly (model, CPU, RAM, GPU, storage), Bench MAY mark it as `same_hardware` if the user confirms. Otherwise, it is always `reference_only`.
