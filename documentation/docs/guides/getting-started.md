# Getting Started

## Prerequisites

- A computer with **x86_64** or **aarch64** CPU
- At least **4 GB RAM** (8 GB recommended)
- **16 GB** free storage (32 GB+ recommended)
- USB drive (8 GB+) for installation media

<br>

## Quick Start

### 1. Download the ISO

```
Samaris-OS-Alpha-One-RC.iso (4.2 GB — dual-architecture)
```

### 2. Write to USB

```bash
# macOS
sudo dd if=Samaris-OS-Alpha-One-RC.iso of=/dev/rdiskX bs=1m status=progress

# Linux
sudo dd if=Samaris-OS-Alpha-One-RC.iso of=/dev/sdX bs=4M status=progress
```

### 3. Boot

Insert the USB, restart, and enter the boot menu (F12/F2/ESC/Option). Select the USB drive in UEFI mode.

### 4. Explore

- **AirBar** — Top-right system panel (WiFi, BT, Sound, Battery)
- **Orbit AI** — Press `Cmd/Ctrl + Space` to launch the AI assistant
- **Finder** — Browse files on the desktop
- **Peregrine** — Built-in web browser
- **Terminal** — Full shell access via xterm.js

<br>

## First Time?

See the [First Boot Guide](first-boot.md) for what to expect on initial startup and the 6-stage boot sequence.

<br>

## Need Help?

- [Debugging Guide](debugging.md)
- [Hardware Matrix](../system/hardware-matrix.md)
- [Installing the ISO](installing-iso.md)

<br>

---

[← Back: Documentation Index](../index.md)
