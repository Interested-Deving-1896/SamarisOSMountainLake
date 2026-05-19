# Build System

**Modular, reproducible ISO builder for Samaris OS**

The build system creates a Debian-based live ISO that boots into the native Electron desktop shell. It uses `debootstrap`, Debian packages, module overlays, `squashfs`, `live-boot`, ISOLINUX/GRUB boot files, and `xorriso`.

<br>

## Boot Pipeline

```
BIOS/UEFI
→ GRUB/ISOLINUX
→ Linux kernel
→ initrd/live-boot
→ filesystem.squashfs
→ systemd
→ volt-kernel.service
→ volt-kernel-b.service (optional Rust accelerator)
→ graphical session
→ Electron native shell
```

<br>

## Directory Structure

```
builder/
├── build.sh                    Compatibility wrapper for ISOGenerator
├── config.env                  Project, Debian, ISO, and module settings
├── ISOGenerator/               Canonical modular ISO pipeline
│   ├── generator.sh            Main orchestrator (check/iso/status/qemu)
│   ├── steps/                  26 build steps (00-check to 18-report)
│   ├── lib/                    Shared utilities, toolchain helpers
│   └── templates/              GRUB/ISOLINUX boot config templates
├── content/
│   ├── modules/                ISO build layers (packages + overlays)
│   ├── volt-ram-manager/       Rust daemon source
│   ├── volt-gpu-manager/       Rust daemon source
│   ├── volt-usb-manager/       Rust daemon source
│   ├── volt-dynamic-worker-pool/ Rust daemon source
│   ├── volt-adaptive-system-config/ Rust daemon source
│   ├── volt-kernel-b/          Rust daemon source (Tesseract Engine)
│   ├── volt-kernel-a/          Node.js kernel services source
│   ├── volt-display-manager/   Rust display manager source
│   ├── volt-bench/             Rust benchmark engine source
│   ├── Bench/                  Benchmark suite (docs + schemas)
│   ├── electron/               Electron shell source
│   ├── ui/                     React desktop UI source
│   ├── backend-audit/          Node.js audit framework
│   ├── theme/                  Plymouth boot theme assets
│   └── scripts/                Build helper scripts
├── overlay/                    Global files copied into the image
├── work/rootfs/x86_64          Generated x86_64 Debian root filesystem
├── work/rootfs/aarch64         Generated ARM64 Debian root filesystem
├── work/iso/                   Generated universal ISO tree
├── cache/                      Reusable local builder data
└── output/                     Final ISO output
```

<br>

## Build Steps (26 steps)

The ISOGenerator pipeline is checkpointed — every successful step writes a `.done` marker.

| # | Step | Description |
|---|------|-------------|
| 00 | `check-env` | Validate environment, dependencies, step inventory |
| 01 | `clean` | Reset work paths, preserve cache and output |
| 02 | `ai-assets` | Download/verify AI models (Qwen3, Whisper, OuteTTS) |
| 03 | `rust-kernel` | Build Tesseract Engine (Kernel B) for x86_64 + aarch64 |
| 03.5 | `ram-manager` | Build VRM for both architectures |
| 03.6 | `usb-manager` | Build VUM for both architectures |
| 03.7 | `gpu-manager` | Build VGM for both architectures |
| 03.8 | `worker-pool` | Build DWP for both architectures |
| 03.9 | `adaptive-config` | Build ASC for both architectures |
| 03.10 | `display-manager` | Build VDM for both architectures |
| 03.11 | `bench` | Build volt-bench for both architectures |
| 04 | `rootfs-bootstrap` | debootstrap Debian Trixie base |
| 05 | `packages` | Install Debian packages |
| 06 | `modules` | Apply module overlays (packages + files) |
| 07 | `boot-theme` | Install Plymouth boot splash theme |
| 08 | `overlay` | Copy global overlay files |
| 09 | `ui` | Build React desktop UI (Vite) |
| 10 | `kernel-a` | Install Node.js kernel and dependencies |
| 11 | `electron` | Install Electron shell and services |
| 12 | `squashfs` | Create compressed SquashFS filesystem |
| 13 | `iso-tree` | Assemble ISO directory tree (BIOS + UEFI) |
| 14 | `iso-image` | Generate final ISO with xorriso |
| 15 | `checksums` | Compute SHA256 checksums for all step outputs |
| 16 | `validate` | Validate ISO structure and boot paths |
| 17 | `qemu` | Boot ISO in QEMU for smoke testing |
| 18 | `report` | Generate build summary report |

<br>

## Module System (`content/modules/`)

Modules are ISO build layers. Each module may provide:

- `packages.list`: one Debian package per line
- `packages.amd64.list` / `packages.arm64.list`: architecture-specific packages
- `overlay/`: files copied into the root filesystem with permissions preserved

Numeric prefixes define ordering:

| Module | Purpose | Key Packages |
|--------|---------|-------------|
| **00-base** | Core system | systemd, sudo, bash, coreutils, network tools |
| **10-kernel** | Linux kernel | linux-image, firmware-linux, GRUB |
| **20-hardware** | Hardware support | GPU firmware, WiFi/BT firmware, thermal drivers |
| **22-drivers** | Additional drivers | Filesystem drivers, RAID, crypto modules |
| **25-boot-splash** | Plymouth theme | plymouth, plymouth-themes |
| **30-display-xorg** | X11 display | xorg, xserver-xorg, xinit, xrandr |
| **40-browser-chromium** | Web runtime | chromium, chromium-l10n |
| **50-runtimes** | App runtimes | nodejs, python3, wine, dosbox |
| **90-volt-shell** | Voltaic shell | Electron, VOLT daemon binaries |
| **99-demo** | Demo content | Sample files, wallpapers, preinstalled apps |

Modules are enabled through `ENABLED_MODULES` in `config.env`.

<br>

## Configuration (`config.env`)

| Variable | Purpose | Default |
|----------|---------|---------|
| `PROJECT_NAME` | ISO project name | `Samaris OS 1.0 Mountain Lake Alpha One` |
| `DEBIAN_SUITE` | Debian release | `trixie` |
| `DEBIAN_MIRROR` | Package mirror | `https://deb.debian.org/debian` |
| `ARCH` | Primary architecture | `amd64` |
| `SAMARIS_ARCHES` | Multi-arch targets | `x86_64 aarch64` |
| `LIVE_USER` | Default live user | `user` |
| `ENABLED_MODULES` | Active build layers | `00-base 10-kernel ... 99-demo` |
| `BUILD_AI_POSTINSTALL` | Download AI models | `0` |
| `OUTPUT_ISO` | Output filename | `Samaris-OS-Alpha-One-RC.iso` |

<br>

## Build Commands

```bash
# Full build (requires root/sudo on Linux, or Docker on macOS)
sudo ./builder/Generate-final-alpha-iso.sh

# Incremental build (checkpoint-aware)
./builder/ISOGenerator/generator.sh check
./builder/ISOGenerator/generator.sh iso
./builder/ISOGenerator/generator.sh qemu     # Boot in QEMU for testing

# Docker build (macOS)
./run.sh iso --docker

# Skip to specific step
./run.sh iso --docker --from 09-ui
# Run only one step
./run.sh iso --docker --only 11-electron

# Status and navigation
./run.sh status --docker
./run.sh next --docker
```

<br>

## Host Dependencies

Required: `debootstrap`, `chroot`, `mksquashfs`, `xorriso`, `rsync`, ISOLINUX/SYSLINUX

Optional: `grub-mkrescue`, `mtools`, `qemu-system-x86_64`, `cargo`, `cmake`

<br>

## Output

The final ISO is written to `builder/output/Samaris-OS-Alpha-One-RC.iso` (~4.2 GB).

Contains both x86_64 and aarch64 kernels. GRUB detects your CPU at boot and loads the right path.
