# 3. Boot Sequence

## 3.1 Overview

The VOLT boot sequence transforms a running Linux system into a fully operational Samaris OS desktop environment. It is a staged, dependency-driven process managed by systemd, with explicit readiness signalling at each stage. The sequence performs hardware detection, adaptive policy generation, native service initialisation, and finally desktop shell launch.

## 3.2 Boot Stages

### Stage 0: Linux Kernel Boot

The physical or virtual machine boots via GRUB/isolinux into a Debian-based Linux system. The Samaris ISO uses a custom boot splash, but the underlying boot process is standard: kernel loading, device driver initialisation, root filesystem mounting, and systemd PID 1 launch.

### Stage 1: Adaptive System Configuration (ASC)

The first VOLT-specific stage. The `volt-asc.service` unit (type: oneshot) runs before any other VOLT component. The ASC executable:

- Detects CPU vendor, core count, architecture (x86_64 / aarch64)
- Measures total physical RAM
- Probes GPU availability, VRAM estimate, hardware acceleration support
- Determines storage type (SSD, HDD, USB, NVMe) and boot medium
- Detects virtualisation environment
- Identifies laptop form factor and battery presence
- Queries network interface availability

Based on these parameters, ASC selects an appropriate hardware profile and generates a system-wide configuration policy at `/run/samaris/adaptive.generated.toml`. This configuration informs the behaviour of every subsequent VOLT module.

### Stage 2: Tesseract Engine Initialisation (Kernel B)

The `volt-kernel-b.service` unit starts the Tesseract Engine, the native Rust daemon. It:

- Executes the VOLT BOOT sequence (`BootSequence::run()`) which performs accelerated boot checks
- Initialises the Unix socket-based IPC server
- Registers system signal handlers
- Loads module-specific configuration from `/opt/volt/kernel-b/config.toml`
- Creates readiness signal at `/run/volt-kernel-b.started`

### Stage 3: VOLT Subsystem Initialisation

With the adaptive configuration in place and the Tesseract Engine running, the remaining Rust daemons initialise in dependency order:

1. **VRM** (volt-ram-manager.service) — memory allocator, compression engine, pressure monitor
2. **VGM** (volt-gpu-manager.service) — GPU detection, VRAM tiers, shader compilation (depends on VRM)
3. **VUM** (volt-usb-manager.service) — journaled storage, device hotplug detection (depends on VRM)
4. **DWP** (volt-dynamic-worker-pool.service) — cooperative scheduler, adaptive scaling (depends on ASC + Tesseract)

Each daemon loads its configuration from its respective directory under `/opt/volt/{module}/config.toml`.

### Stage 4: Kernel A Orchestrator

The `volt-kernel.service` unit starts the Kernel A Node.js server:

- Binds to `127.0.0.1:9999` with HTTP and WebSocket support
- Initialises the Kernel instance, loading all service modules
- Connects to the Tesseract Engine via Unix socket
- Starts the Volt Unifier, which probes and connects to all Rust daemon clients
- Waits for readiness confirmation from the Unifier
- Begins accepting WebSocket connections from the UI

### Stage 5: Desktop Shell

The `volt-desktop.service` unit starts the Electron shell as the desktop user:

- Launches the Kernel A orchestrator if not already running (with health check retry)
- Creates the frameless fullscreen BrowserWindow
- Loads the preload bridge with context isolation enabled
- Connects to Kernel A via WebSocket (`ws://127.0.0.1:9999`)
- Renders the React desktop SPA
- Displays the boot splash (`/opt/volt/boot.html`) during initialisation

## 3.3 Complete Dependency Chain

```
systemd (PID 1)
  │
  ├── volt-asc.service (oneshot, Before=sysinit.target)
  │     └── Generates /run/samaris/adaptive.generated.toml
  │
  ├── volt-kernel-b.service (Before=sysinit.target)
  │     └── Unix socket /run/samaris/volt-kernel-b.sock
  │
  ├── volt-usb-manager.service (After=kernel-b)
  ├── volt-ram-manager.service (After=kernel-b)
  ├── volt-gpu-manager.service (After=ram-manager)
  │
  ├── volt-worker-pool.service (After=kernel-b, asc)
  │
  ├── volt-kernel.service (After=kernel-b, network.target)
  │     └── WebSocket port 9999
  │
  ├── volt-unifier.service (After=kernel, Requires=kernel)
  │     └── Health watchdog process
  │
  ├── volt-fs.service (After=network.target)
  │
  └── volt-desktop.service (After=kernel, Requires=kernel)
        └── Electron shell on display :0
```

## 3.4 Readiness Signalling

Each service signals availability through two mechanisms:

1. **systemd notify** (Type=notify for Tesseract Engine): the daemon sends readiness notification directly to systemd
2. **Filesystem signals**: shell scripts touch `/run/volt-{service}.started` after successful initialisation

The Volt Unifier independently verifies module readiness by connecting to each daemon's SBP endpoint and sending HEARTBEAT messages.

## 3.5 Failure Handling

If any stage fails:

- **ASC failure**: the system falls back to a conservative "safe" profile with minimal workers, disabled compression, and software rendering
- **Tesseract Engine failure**: subsequent services are not blocked; Kernel A operates in degraded mode with reduced native acceleration
- **Rust daemon failure**: individual daemons restart independently (Restart=on-failure in systemd); the Unifier marks them as degraded
- **Kernel A failure**: triggers Electron watchdog; desktop shows recovery interface
- **Desktop failure**: systemd restarts the desktop service up to configured limits; repeated failure triggers recovery mode

## 3.6 Boot Measurement

At Alpha stage, boot time is monitored via:

- Kernel A startup log timestamps
- Tesseract Engine boot sequence elapsed time
- Volt Unifier module connection latencies
- systemd `systemd-analyze` for service-level timing
