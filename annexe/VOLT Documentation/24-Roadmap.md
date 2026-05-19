# 24. Roadmap

## 24.1 Development Phases

### Alpha One (Current)

**Objective**: Bootable, usable desktop environment with core functionality.

**Priorities**:
- Boot chain complete (systemd → ASC → Tesseract → Kernel A → Desktop)
- Electron shell with context bridge and window management
- WiFi connectivity via NetworkManager
- Filesystem UX (browse, open, create, delete)
- Peregrine native browser (WebContentsView)
- Native terminal (node-pty + xterm.js)
- Per-application menus and window management
- Systemd service integration with journald logging
- Watchdog and basic recovery mode
- Demo folder with sample content
- All six Rust daemons functional and stable

**Completion criteria**:
- Samaris boots from ISO directly into desktop
- WiFi connects and displays in AirBar
- Files open on double-click by MIME type
- Peregrine acts as a genuine browser (tabs, navigation, downloads)
- Terminal executes real shell commands
- System does not freeze under normal usage
- Services are visible in systemd with readable logs
- Desktop recovers from minor crashes

### Alpha Two

**Objective**: Adaptive, robust system with intelligent resource management.

**Priorities**:
- Full ASC profile system with all eight profiles
- VRM v1 stable (compression, dedup, quotas, pressure zones)
- VGM v1 stable (VRAM tiers, shader cache, thermal backoff)
- DWP v1 stable (adaptive scaling, desktop frame guard, Orbit burst)
- VUM v1 stable (journal, writeback, FUSE)
- Volt Unifier complete (all clients, bridges, event bus)
- Hardware profile selection and system reconfiguration
- Improved IPC security (capability enforcement)
- Crash recovery and state persistence
- System settings with real configuration persistence
- Bluetooth connectivity stable
- ISO packaging with reproducible builds

### Beta

**Objective**: Publicly presentable operating system.

**Priorities**:
- Desktop UI polish (animations, transitions, theming)
- Complete documentation (this document and component-level docs)
- Installer (optional, alongside live USB mode)
- Software updater
- Permission system enforcement
- App sandboxing (iframes, Chromium runtime)
- Performance benchmarks
- Public website and demo videos
- Release notes and changelog

## 24.2 Victory Conditions

VOLT is considered successful when:

1. Samaris boots directly into its desktop environment without manual intervention
2. WiFi connectivity works from the desktop UI
3. Bluetooth device management is functional
4. Files open with the correct application on double-click
5. Peregrine browser provides a full browser experience
6. Terminal executes real Linux commands
7. System remains responsive under moderate load
8. All services are visible in systemd with meaningful status
9. Logs provide actionable information in failure scenarios
10. The desktop recovers from crashes without data loss
11. System resources are adaptively managed based on hardware capability
12. The user experience feels like an operating system, not a web page

## 24.3 Anti-Objectives

VOLT is explicitly not designed to be:

- A replacement for the Linux kernel
- A hypervisor or virtualisation platform
- A hard real-time system
- A military-grade security platform
- A mechanism for infinite memory (compression has limits)
- A clone of macOS, Windows, or any other desktop environment
- A simple Electron application (it is a system layer, not an app)
- A portfolio mockup (it must boot real hardware)

## 24.4 Strategic Position

VOLT occupies a specific architectural niche: it provides the system-level identity, orchestration, and native experience that transforms a web-based interface into a genuine operating system environment. Linux provides the hardware abstraction and process model. VOLT provides the adaptive policy engine, the inter-module communication fabric, the desktop integration layer, and the native runtime for web applications. This division of responsibility allows Samaris OS to benefit from Linux's stability and driver ecosystem while delivering a custom, adaptive, and modern user experience.

---

*"VOLT is the native runtime spine of Samaris OS — the layer that turns a beautiful WebOS interface into a bootable, adaptive and usable operating system."*
