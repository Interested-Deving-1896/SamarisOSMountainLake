# VOLT Documentation

**Samaris OS — Native Runtime & System Layer**

*Document Version 1.0 — Alpha Architecture Specification*
*Corresponds to codebase state: Alpha One*

---

## Scope

This document constitutes the complete architectural reference for **VOLT**, the native runtime spine of Samaris OS. It describes the system's modular architecture, component responsibilities, inter-module communication protocols, boot lifecycle, resource management subsystems, and integration with the underlying Linux platform. The documentation is written at the architectural level and does not expose detailed implementation code.

## How to Read This Document

The documentation is organized in four thematic clusters:

| Cluster | Chapters | Content |
|---------|----------|---------|
| Foundation | 1–2 | Executive summary, architectural philosophy, system overview |
| Core Runtime | 3–6 | Boot sequence, Kernel A orchestrator, Tesseract Engine, Volt Unifier |
| Subsystem Modules | 7–11 | Adaptive config, RAM, GPU, USB storage, worker pool |
| Desktop & Integration | 12–16 | Electron shell, Peregrine browser, terminal, SBP protocol, systemd |
| Operations | 17–24 | Watchdog, security, observability, states, profiles, Orbit AI, glossary, roadmap |

## Table of Contents

| # | Chapter | Description |
|---|---------|-------------|
| 1 | [Executive Summary](01-Executive-Summary.md) | What VOLT is, its role in Samaris OS, design principles |
| 2 | [Architectural Overview](02-Architectural-Overview.md) | Three-layer architecture, component map, data flow |
| 3 | [Boot Sequence](03-Boot-Sequence.md) | systemd boot chain, ASC execution, service orchestration |
| 4 | [Tesseract Engine](04-Tesseract-Engine.md) | Kernel B — native Rust daemon, boot coordination, IPC server |
| 5 | [Kernel A Orchestrator](05-Kernel-A-Orchestrator.md) | Node.js server, message router, handler architecture, service layer |
| 6 | [Volt Unifier](06-Volt-Unifier.md) | Module registry, health monitoring, metrics aggregation, bridges |
| 7 | [Adaptive System Config](07-Adaptive-System-Config.md) | Hardware detection, profile selection, resource budget calculation |
| 8 | [RAM Manager](08-RAM-Manager.md) | Compression tiers, deduplication, per-app quotas, pressure management |
| 9 | [GPU Manager](09-GPU-Manager.md) | VRAM tiering, shader cache, compute scheduling, thermal backoff |
| 10 | [USB Storage Manager](10-USB-Storage-Manager.md) | Journaled write cache, FUSE filesystem, device hotplug, IO scheduling |
| 11 | [Dynamic Worker Pool](11-Dynamic-Worker-Pool.md) | Cooperative scheduling, adaptive scaling, desktop guard, thermal integration |
| 12 | [Electron Desktop Shell](12-Electron-Desktop-Shell.md) | BrowserWindow, preload bridge, IPC handlers, security architecture |
| 13 | [Peregrine Browser](13-Peregrine-Browser.md) | WebContentsView tab manager, downloads, permissions, session management |
| 14 | [Native Terminal](14-Native-Terminal.md) | node-pty integration, xterm.js frontend, session lifecycle |
| 15 | [Samaris Binary Protocol](15-Samaris-Binary-Protocol.md) | SBP message format, opcode table, SBP-MEM memory extension |
| 16 | [Systemd Integration](16-Systemd-Integration.md) | Service units, dependency graph, readiness signalling, log architecture |
| 17 | [Watchdog and Recovery](17-Watchdog-and-Recovery.md) | Heartbeat monitoring, degraded mode, recovery procedures, crash handling |
| 18 | [Security Model](18-Security-Model.md) | Context isolation, path validation, capability guards, audit logging |
| 19 | [Observability and Metrics](19-Observability-and-Metrics.md) | Telemetry pipeline, health snapshots, dashboard feed, log aggregation |
| 20 | [System States](20-System-States.md) | State machine, transitions, per-state behaviour, system-wide coordination |
| 21 | [Hardware Profiles](21-Hardware-Profiles.md) | Detection parameters, profile definitions, adaptive configuration mapping |
| 22 | [Orbit AI Assistant](22-Orbit-AI-Assistant.md) | Local LLM orchestration, speech-to-text, text-to-speech, resource management |
| 23 | [Glossary](23-Glossary.md) | Terminology reference, acronyms, component names |
| 24 | [Roadmap](24-Roadmap.md) | Development phases, victory conditions, anti-objectives |

---

## Version Control

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | May 2026 | Initial alpha architecture specification |

## Related Documents

- Samaris OS Website: [samaris.tech](https://samaris.tech)
- ISO Builder Documentation: `builder/ISOGenerator/README.md`
- Kernel A Implementation: `overlay/opt/volt/kernel/`
- Tesseract Engine Source: `content/volt-kernel-b/`

---

*"VOLT is the native runtime spine of Samaris OS — the layer that turns a beautiful WebOS interface into a bootable, adaptive and usable operating system."*
