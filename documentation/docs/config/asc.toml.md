# ASC — Adaptive System Config

**Path:** `/opt/volt/asc/config.toml`

ASC runs once at boot to detect hardware and generate the adaptive configuration. Defined in the **VOLT specification (ch.7 — ASC)**.

```toml
[detection]
cpu_cores = true
ram_total = true
gpu_available = true
vram_total = true
storage_type = true
boot_medium = true
vm_state = true
laptop_detection = true
network_available = true

[policy]
max_memory_percent = 75
low_ram_threshold_mb = 4096
```

<br>

## Hardware Detection

ASC detects the following hardware parameters at boot:

| Parameter | Description |
|-----------|-------------|
| CPU cores | Number of physical/logical cores |
| RAM total | Total system memory |
| GPU | Available GPU (discrete/integrated) |
| VRAM | GPU video memory |
| Storage type | SSD, NVMe, HDD |
| Boot medium | Disk, USB, network |
| VM detection | Is running in a virtual machine |
| Laptop | Is a laptop/portable system |
| Network | Network interface availability |

<br>

## Profiles

ASC selects from 8 hardware profiles:

| Profile | Use Case |
|---------|----------|
| **safe** | Minimum feature set, conservative limits |
| **low_ram** | Systems with less than 4 GB RAM |
| **balanced** | General-purpose desktops/laptops |
| **performance** | High-RAM, multi-core systems |
| **powersave** | Laptops on battery |
| **vm** | Virtualized environment |
| **usb_boot** | Booting from USB drive |
| **debug** | Development/testing with verbose logging |

<br>

## Output

Profile parameters are written to `/run/samaris/adaptive.generated.toml`, which is consumed by:

- **VRM** — memory limits and pressure thresholds
- **DWP** — worker pool sizing
- **VGM** — GPU backend selection
- **Kernel B** — Tesseract worker count and thermal config

<br>

## Related

- [VRM — RAM Manager Configuration](vrm.toml.md)
- [DWP — Worker Pool Configuration](dwp.toml.md)
- [VGM — GPU Manager Configuration](vgm.toml.md)
- [Kernel B Configuration](kernel-b.toml.md)

<br>

---

[← Back: Documentation Index](../index.md)
