# SAMARIS OS Builder

SAMARIS OS Builder creates a Debian-based live ISO that boots into the native Electron desktop shell:

```text
/usr/local/bin/volt-desktop
```

It is not a Linux distribution from scratch. It is a modular, reproducible OS builder that uses `debootstrap`, Debian packages, module overlays, `squashfs`, `live-boot`, ISOLINUX/GRUB boot files, and `xorriso`.

## Boot Pipeline

```text
BIOS/UEFI
-> GRUB/ISOLINUX
-> Linux kernel
-> initrd/live-boot
-> filesystem.squashfs
-> systemd
-> volt-kernel.service
-> volt-kernel-b.service (optional Rust accelerator)
-> graphical session
-> Electron native shell
```

## Architecture

```text
build.sh                  Compatibility wrapper for ISOGenerator
config.env                Project, Debian, ISO, and module settings
ISOGenerator/             Canonical modular ISO pipeline
content/modules/*         Package and overlay contributions by module
overlay/                  Global files copied into the image
work/rootfs/x86_64        Generated x86_64 Debian root filesystem
work/rootfs/aarch64       Generated ARM64 Debian root filesystem
work/iso/                 Generated universal ISO tree
cache/                    Reusable local builder data
output/                   Final ISO output
```

## Module System

Modules are enabled through `ENABLED_MODULES` in `config.env`.

Each module may provide:

- `packages.list`: one Debian package per line; comments and blank lines are ignored.
- `overlay/`: files copied into the root filesystem with permissions preserved.

To add a module:

1. Create `modules/60-my-module/packages.list`.
2. Create `modules/60-my-module/overlay/` if it needs files.
3. Add `60-my-module` to `ENABLED_MODULES` in `config.env`.

Numeric prefixes define ordering.

## Build

Run on a Debian host, VM, or container with Linux mount/chroot support:

```bash
sudo ./Generate-final-alpha-iso.sh
```

The final ISO is written to:

```text
output/Samaris-OS-Alpha-One-RC.iso
```

For the universal Alpha One release candidate, use:

```bash
sudo ./Generate-final-alpha-iso.sh
```

### Fast Iteration (Incremental)

By default `ISOGenerator` uses `work/` for generated root filesystems and ISO trees, and `cache/` for reusable local data.

Examples:

```bash
./ISOGenerator/generator.sh check
./ISOGenerator/generator.sh iso
./ISOGenerator/generator.sh qemu
```

### Fast Inner Loop (Dev ISO)

The compatibility scripts in `content/scripts/` delegate to `ISOGenerator` so there is one source of build logic.

### macOS (Docker Desktop)

On macOS, the recommended path is to build inside a Debian `amd64` container:

```bash
../run.sh iso --docker
```

## Test

If QEMU is installed:

```bash
./ISOGenerator/generator.sh qemu
```

The test script uses 2G RAM and enables KVM only when `/dev/kvm` is available.

## Safety Notes

- Build scripts stop on errors.
- `01-clean.sh` resets only `work/` paths inside the builder.
- `cache/` and `output/` are preserved.
- Package installation happens inside the generated rootfs chroot.
- The builder does not install host dependencies automatically.

## Root And Sudo Notes

Most build steps need root because they use `debootstrap`, `chroot`, mount namespaces, device binds, and filesystem ownership. Use:

```bash
sudo ./build.sh
```

The live image creates a passwordless sudo user named `user` by default. Change `LIVE_USER` in `config.env` if needed.

## macOS Note

macOS is a good place to edit this project, but the build should preferably run in a Debian VM or container. The core tools are Linux-native: `debootstrap`, `chroot`, proc/sys/dev mounts, kernel packages, ISOLINUX, GRUB EFI helpers, `mksquashfs`, and `xorriso`.

## Desktop Runtime

The user experience is the React desktop packaged in the Electron shell. The Node kernel provides local services over WebSocket, and Kernel B in Rust accelerates critical system operations when available. Wi-Fi and Mail secrets are encrypted per user under `.volt/users/{username}/`.

## Host Dependencies

Required:

- `debootstrap`
- `chroot`
- `mksquashfs`
- `xorriso`
- `rsync`
- ISOLINUX/SYSLINUX boot files

Optional:

- `grub-mkrescue`
- `grub-mkstandalone`
- `mtools`
- `qemu-system-x86_64`
- `cargo`
- `curl`
- `cmake`

Debian/Ubuntu install hint:

```bash
sudo apt update
sudo apt install debootstrap squashfs-tools xorriso rsync isolinux syslinux-common grub-pc-bin grub-efi-amd64-bin grub-efi-arm64-bin mtools qemu-system-x86 curl git cmake cargo rustup gcc-aarch64-linux-gnu g++-aarch64-linux-gnu qemu-user-static binfmt-support
```
