# 1. Executive Summary

**VOLT** is the native runtime and system orchestration layer of Samaris OS. It transforms Samaris from a web-based interface into a bootable, adaptive, hardware-aware operating system capable of managing real machine resources. VOLT is not an application, nor merely a backend service — it is the system-level substrate that coordinates boot, memory, graphics, storage, computation, process scheduling, inter-process communication, and desktop lifecycle across the entire Samaris stack.

## 1.1 Core Function

VOLT provides a deterministic, modular intermediate layer positioned between the Linux operating system and the Samaris desktop environment. It translates low-level hardware capabilities into adaptive system policies, routes messages between distributed components, manages resource allocation across competing subsystems, and maintains system stability through continuous health monitoring and graceful degradation.

## 1.2 Scope of Coordination

VOLT coordinates the following domains:

- **Boot orchestration**: hardware detection, capability assessment, policy generation, service activation, desktop launch
- **Memory management**: multi-tier compression, content-aware deduplication, per-application quotas, pressure-responsive allocation
- **Graphics adaptation**: VRAM-aware tiering, shader compilation caching, compute scheduling, thermal-aware backoff
- **Storage management**: journaled write caching, FUSE integration, device hotplug handling, IO scheduling
- **Background computation**: cooperative priority scheduling, adaptive worker scaling, desktop frame guard
- **Inter-process communication**: typed binary protocol, structured message routing, event bus, system-wide addressing
- **Desktop integration**: Electron shell, preload bridge, native window management, filesystem access, peripheral control
- **System health**: heartbeat monitoring, crash detection, recovery procedures, audit logging, metrics aggregation

## 1.3 Design Principles

1. **Deterministic adaptation**: the system must detect its environment and configure itself without manual tuning
2. **Progressive enhancement**: behaviour scales with hardware capability, from lightweight VM deployments to high-performance desktop configurations
3. **Graceful degradation**: resource pressure triggers proportional responses rather than catastrophic failure
4. **Modular separability**: each subsystem can be independently developed, tested, and deployed
5. **Observability by default**: all components expose structured metrics, logs, and health signals
6. **Security through isolation**: the UI layer never has direct access to native APIs; all communication passes through typed, validated bridges

## 1.4 Relationship to the Operating System

VOLT does not replace the Linux kernel. Linux provides process scheduling, device drivers, memory virtualization, and hardware abstraction. VOLT builds upon this foundation to provide Samaris-specific services: boot coordination, adaptive resource policies, a real-time binary communication protocol, and a stable runtime environment for web-based desktop applications. The relationship is symbiotic: Linux provides the substrate; VOLT provides the identity, orchestration, and native experience.

## 1.5 Distinction from Web Applications

Samaris OS presents as a web-based desktop environment, but VOLT fundamentally distinguishes it from a standard web application. Where a web app runs in a browser with limited system access, VOLT provides:

- Boot-time hardware detection and adaptive configuration
- Direct filesystem access via typed and permissioned APIs
- Native process lifecycle management
- Real hardware resource monitoring and control
- Systemd-integrated service management
- Desktop-level window, clipboard, and peripheral management
- Offline-first architecture with persistent state

## 1.6 Current Status

VOLT is under active development at Alpha stage. The core architecture is implemented and functional: the boot chain, all six native Rust daemons, the Node.js orchestration layer, the Electron shell, the SBP communication protocol, and the systemd integration. The system can boot from a live ISO into a functional desktop environment with WiFi, filesystem access, browser, terminal, and application launching capabilities.
