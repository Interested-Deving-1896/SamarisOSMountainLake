# 16. Systemd Integration

## 16.1 Overview

Samaris OS integrates with the Linux init system through a set of well-defined systemd service units. Each VOLT component runs as a separate systemd service with explicit dependency ordering, restart policies, and journald logging. This integration provides standardised lifecycle management, failure handling, and observability through familiar Linux administration tools.

## 16.2 Service Units

The following systemd service units are defined:

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

## 16.3 Dependency Graph

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

## 16.4 Service Ordering Rationale

1. **ASC first**: hardware detection is required by all subsequent modules for adaptive configuration
2. **Tesseract Engine second**: provides native acceleration and IPC substrate for other daemons
3. **VRM, VUM, VGM**: memory and storage managers depend on Tesseract's IPC infrastructure
4. **DWP**: worker pool requires both ASC (for sizing) and Tesseract (for coordination)
5. **Kernel A**: orchestrator depends on all native daemons plus network for WebSocket
6. **Unifier**: health monitor depends on Kernel A (it polls the HTTP health endpoint)
7. **Desktop**: last in the chain, fullfils the graphical session requirement

## 16.5 Readiness Signalling

Three mechanisms signal service readiness:

1. **systemd Type=notify**: the Tesseract Engine uses sd_notify to signal readiness
2. **ExecStartPost touch files**: shell wrappers create `/run/volt-*.started` files after successful start
3. **Health polling**: the Volt Unifier independently verifies module connectivity via SBP heartbeats

## 16.6 Failure and Recovery Policies

All services use `Restart=on-failure` with `RestartSec=1–3` depending on criticality:

| Service | Restart Behaviour |
|---------|-------------------|
| Tesseract Engine | Fast restart (1s), real-time scheduling priority |
| Rust daemons (VRM, VGM, etc.) | Fast restart (1s), elevated nice (-10) |
| Kernel A | Fast restart (1s) |
| Desktop | Restart with 3s delay, graphical target dependency |

The Tesseract Engine runs with `Nice=-20`, `IOSchedulingClass=realtime`, and `LimitMEMLOCK=infinity` to ensure deterministic low-level performance.

## 16.7 Logging Architecture

All VOLT services log to systemd's journald:

| Log Source | Command |
|------------|---------|
| All VOLT services | `journalctl -u volt-{name}.service -b` |
| Failed services | `systemctl --failed` |
| Service status | `systemctl status volt-{name}.service` |

Rust daemons use structured tracing (`RUST_LOG=info` by default), with output directed to journald via `StandardOutput=journal`. Kernel A uses a custom `[SAMARIS]` logger prefix for easy log filtering.

## 16.8 Display Manager Integration

Samaris OS uses nodm (non-display manager) for X11 session management, configured via `/etc/default/nodm`. The nodm configuration:

- Auto-login as the desktop user
- Uses the custom X session script (`samaris-xsession`)
- Allocates VT 7 for the desktop
- Requires minimum 60-second session time
