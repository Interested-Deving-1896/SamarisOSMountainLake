# Systemd Services

The boot chain is managed by **systemd** with explicit service ordering for optimal startup.

<br>

## Service Units

| Service | Type | Description |
|---------|------|-------------|
| `volt-asc.service` | oneshot | Adaptive System Configuration — hardware detection and policy generation |
| `volt-kernel-b.service` | notify | Tesseract Engine — native Rust daemon, boot coordination |
| `volt-ram-manager.service` | simple | VRM — deterministic memory manager |
| `volt-usb-manager.service` | simple | VUM — journaled storage manager |
| `volt-gpu-manager.service` | simple | VGM — GPU orchestration layer |
| `volt-worker-pool.service` | simple | DWP — cooperative priority scheduler |
| `volt-kernel.service` | simple | Kernel A — Node.js orchestrator, port 9999 |
| `volt-unifier.service` | simple | Volt Unifier health watchdog |
| `volt-fs.service` | simple | Filesystem service |
| `volt-desktop.service` | simple | Electron desktop shell |

<br>

## Dependency Graph

```
sysinit.target
  ├── volt-asc.service (Before=sysinit.target)
  └── volt-kernel-b.service (Before=sysinit.target)
        │
        ├── volt-usb-manager.service (After=kernel-b)
        ├── volt-ram-manager.service (After=kernel-b)
        │     └── volt-gpu-manager.service (After=ram-manager)
        │
        ├── volt-worker-pool.service (After=kernel-b, asc)
        │
        └── volt-kernel.service (After=kernel-b, network.target)
              │
              ├── volt-unifier.service (After=kernel)
              ├── volt-fs.service
              │
              └── volt-desktop.service (After=kernel, Requires=kernel)
                    └── (Wants=graphical.target)
```

<br>

## Service Ordering Rationale

1. **ASC first**: hardware detection is required by all subsequent modules
2. **Tesseract Engine second**: provides native acceleration and IPC substrate
3. **VRM, VUM, VGM**: depend on Tesseract's IPC infrastructure
4. **DWP**: requires both ASC (sizing) and Tesseract (coordination)
5. **Kernel A**: depends on all native daemons plus network for WebSocket
6. **Unifier**: health monitor depends on Kernel A
7. **Desktop**: last in the chain, fulfils the graphical session

<br>

## Service Details

| Service | Type | Boot Order | Startup (VM) |
|---------|------|-----------|-------------|
| `volt-asc.service` | oneshot | 1 | **0.506s** |
| `volt-kernel-b.service` | notify | 2 | **12.823s** |
| `volt-ram-manager.service` | simple | 3 | ~0.2s |
| `volt-gpu-manager.service` | simple | 4 | ~0.3s |
| `volt-usb-manager.service` | simple | 5 | ~0.2s |
| `volt-worker-pool.service` | simple | 6 | ~0.8s |
| `volt-kernel.service` | simple | 7 | ~0.5s |
| `volt-desktop.service` | simple | 8 | inactive (nodm) |
| `volt-unifier.service` | simple | 9 | ~0.3s |
| `volt-fs.service` | simple | 10 | ~0.2s |

> Timings measured on QEMU x86_64 VM (4 GB RAM, 4 vCPU), Debian Trixie.

<br>

## Measured Boot Timeline

```
BIOS ──── GRUB ──── Kernel (10.741s) ──── Userspace (37.763s)
                                               ├── live-config: 19.610s
                                               ├── volt-kernel-b: 12.823s
                                               ├── ldconfig: 10.322s
                                               ├── NetworkManager: 2.879s
                                               └── volt-asc: 0.506s
```

<br>

## Readiness Signalling

Three mechanisms signal service readiness:
1. **systemd Type=notify**: the Tesseract Engine uses sd_notify
2. **ExecStartPost touch files**: shell wrappers create `/run/volt-*.started` files
3. **Health polling**: the Volt Unifier verifies module connectivity via SBP heartbeats

<br>

## Failure and Recovery Policies

All services use `Restart=on-failure`:
- Tesseract Engine: fast restart (1s), real-time scheduling priority
- Rust daemons: fast restart (1s), elevated nice (-10)
- Kernel A: fast restart (1s)
- Desktop: restart with 3s delay

The Tesseract Engine runs with `Nice=-20`, `IOSchedulingClass=realtime`, and `LimitMEMLOCK=infinity`.

<br>

## Non-Volt Services Active at Boot

`accounts-daemon`, `acpid`, `avahi-daemon`, `cups`, `dbus`, `live-config`, `ModemManager`, `NetworkManager`, `nodm`, `Plymouth`, `polkit`, `rtkit-daemon`, `systemd-journald`, `systemd-logind`, `udisks2`, `upower`, `wpa_supplicant`.

<br>

## Related

- [Desktop Session](desktop-session.md)
- [Boot Splash](boot-splash.md)
- [ISO Boot Benchmark](../../assets/benchmarks/iso-boot-qemu.md)

<br>

---

[← Back: Documentation Index](../index.md)
