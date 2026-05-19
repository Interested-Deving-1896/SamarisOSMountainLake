# Installing the ISO

## Download

Download the latest ISO from the [official website](https://samaris.tech).

**File:** `Samaris-OS-Alpha-One-RC.iso`  
**Size:** ~4.2 GB  
**Architecture:** Dual-architecture ISO (bundles both x86_64 and aarch64 kernels)  
**SHA256:** Verify against the published checksum.

<br>

## Writing to USB

### macOS
```bash
# Identify your USB device
diskutil list

# Unmount (do not eject)
diskutil unmountDisk /dev/diskX

# Write the ISO
sudo dd if=Samaris-OS-Alpha-One-RC.iso of=/dev/rdiskX bs=1m status=progress
```

### Linux
```bash
sudo dd if=Samaris-OS-Alpha-One-RC.iso of=/dev/sdX bs=4M status=progress
```

### Windows
Use [Rufus](https://rufus.ie) in DD Image mode, or `balenaEtcher`.

<br>

## Booting

1. Insert the USB and restart
2. Enter boot menu (F12/F2/ESC/Option depending on hardware)
3. Select the USB drive (UEFI mode)
4. GRUB will detect your CPU architecture automatically and load the matching kernel

<br>

## System Requirements

| Component | Minimum | Recommended |
|-----------|---------|-------------|
| CPU | 2 cores, x86_64 or aarch64 | 4+ cores |
| RAM | 4 GB | 8 GB |
| Storage | 16 GB | 32 GB+ |
| GPU | Intel HD or better | Metal/Vulkan capable |

<br>

## Related

- [First Boot Guide](first-boot.md)
- [Getting Started](getting-started.md)
- [Hardware Matrix](../system/hardware-matrix.md)

<br>

---

[← Back: Documentation Index](../index.md)
