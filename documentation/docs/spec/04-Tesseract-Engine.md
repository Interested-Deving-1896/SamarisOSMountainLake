# 4. Tesseract Engine

## 4.1 Overview

The Tesseract Engine (package name: `tesseract-engine`) is the native Rust daemon at the core of VOLT's system layer, referred to architecturally as Kernel B. It provides low-level system acceleration, boot coordination, secure IPC infrastructure, and resource management primitives that are not efficiently expressible in JavaScript. The Tesseract Engine does not replace the Linux kernel — it operates as a userspace daemon with elevated scheduling priority and direct memory access capabilities.

## 4.2 Role and Responsibilities

The Tesseract Engine serves as:

- **Boot coordinator**: executes the VOLT BOOT sequence, performing accelerated hardware checks and initialising the shared memory ring buffer
- **IPC server**: hosts Unix socket and WebSocket endpoints for structured communication with Kernel A and other modules
- **System telemetry collector**: gathers CPU, memory, and thermal metrics at configurable intervals
- **Security boundary**: isolates native memory operations from JavaScript-facing APIs
- **Command execution engine**: dispatches typed commands to registered system handlers
- **Orbit AI resource reservation**: reserves dedicated workers and memory for local LLM inference

## 4.3 Subsystems

### Boot Module

The boot subsystem implements the deterministic VOLT BOOT sequence. It initialises the GPU canvas subsystem, allocates the shared memory ring buffer, preloads critical assets, and reports elapsed boot time and subsystem readiness. It supports multiple boot modes including Fast and Full, where Fast mode skips non-critical initialisation steps.

### IPC Module

The IPC subsystem provides three transport mechanisms:

1. **Unix sockets** (primary): used for SBP binary protocol communication with the Volt Unifier and Kernel A
2. **Shared memory** (SHM ring buffer): used for high-frequency telemetry data exchange
3. **WebSocket** (port 9998): used for debug and development connectivity

The IPC module enforces message framing, byte ordering, and protocol versioning. All incoming messages are validated against the SBP header format before dispatch.

### Scheduler

A lightweight cooperative scheduler manages concurrent tasks within the Tesseract Engine. It supports task prioritisation, preemption, and resource budgeting. The scheduler is distinct from the Dynamic Worker Pool (DWP) — the Tesseract scheduler manages internal engine tasks, while the DWP manages system-wide background computation.

### Protocol Layer

The protocol subsystem implements the binary message framing for SBP messages. It provides flatbuffer serialization for complex structured data, opcode encoding and decoding, and ring buffer management for shared memory transport.

### Safety and Security

The safety subsystem implements:

- Emergency stop detection and propagation
- Per-task memory and command rate quotas
- Audit trail recording for security-relevant operations
- Thermal monitoring with configurable throttle and emergency thresholds

## 4.4 Configuration

The Tesseract Engine loads its configuration from `/opt/volt/kernel-b/config.toml`. Key parameters include:

- **Socket path**: Unix socket location for IPC (`/run/samaris/volt-kernel-b.sock`)
- **WebSocket port**: development debug port (9998)
- **Worker pool**: maximum concurrent tasks, scheduling tick interval
- **Thermal thresholds**: throttle (85°C), emergency (95°C), critical (100°C)
- **Memory limits**: maximum total memory, per-quota defaults for commands and tasks
- **Orbit reservation**: dedicated workers, memory, and priority for AI inference tasks

## 4.5 Relationship to Other Modules

The Tesseract Engine occupies a privileged position in the VOLT architecture:

- **Kernel A** connects to Tesseract as a client over Unix socket SBP
- **Volt Unifier** monitors Tesseract health via heartbeat messages
- **ASC** configuration influences Tesseract's worker count and memory limits
- **DWP** and **VRM** operate independently but coordinate through the Unifier

The Tesseract Engine's `BootSequence` is the canonical entry point for the VOLT runtime; its successful completion is a prerequisite for full desktop functionality.
