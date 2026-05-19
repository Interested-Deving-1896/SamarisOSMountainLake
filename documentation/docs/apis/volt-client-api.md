# Volt Client API

Client-side library for interacting with the Volt daemon infrastructure through Kernel A. Provides a high-level TypeScript interface for apps to interact with system resources managed by the Volt daemons.

<br>

## Daemon Clients

The Volt Client connects to each daemon through Kernel A:

| Client | Daemon | Function |
|--------|--------|----------|
| `KernelBClient` | Kernel B — Tesseract Engine | Worker scheduling, thermal management |
| `VrmClient` | VRM — RAM Manager | Memory pressure, quotas, compression |
| `VumClient` | VUM — USB Manager | USB device detection, mounting |
| `VgmClient` | VGM — GPU Manager | GPU resources, VRAM, shaders |
| `DwpClient` | DWP — Worker Pool | Worker allocation, burst, priorities |
| `AscClient` | ASC — Adaptive System Config | Hardware detection, profile selection |

<br>

## Usage

```typescript
import { voltClient } from "../services/volt/voltClient";

// Check daemon health
const health = await voltClient.health();
// → { vrm: "online", dwp: "online", vgm: "online", vum: "online" }

// Get system metrics
const metrics = await voltClient.metrics();
// → { cpu: 23.4, memory: { used: 4096, total: 8192 }, gpu: { ... } }

// Request Orbit burst (DWP)
await voltClient.requestOrbitBurst();
```

<br>

## API Reference

### `voltClient.health()`

Returns the health status of all Volt daemons.

### `voltClient.metrics()`

Returns current system metrics (CPU, memory, GPU).

### `voltClient.requestOrbitBurst()`

Requests a DWP burst allocation for Orbit AI inference (critical priority). Uses Orbit's configured burst parameters: 100 ms window, 75% of workers, max 3 consecutive bursts.

### `voltClient.registerApp(config)`

Registers an app with VRM for memory quota enforcement.

### `voltClient.getAdaptiveConfig()`

Returns the ASC-generated hardware configuration from `/run/samaris/adaptive.generated.toml`.

<br>

## Architecture

```
App → voltClient → Kernel A (WebSocket) → SBP → Rust Daemon
                       ↓
                 HTTP REST (Electron → Kernel A)
```

<br>

## Related

- [SBP Binary Protocol](sbp-protocol.md)
- [Kernel WebSocket Protocol](kernel-ws.md)
- [Kernel B Configuration](../config/kernel-b.toml.md)

<br>

---

[← Back: Documentation Index](../index.md)
