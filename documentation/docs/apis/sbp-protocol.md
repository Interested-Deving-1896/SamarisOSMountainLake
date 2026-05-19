# SBP — Secure Binary Protocol

The binary protocol for communication between **Kernel A** (Node.js) and the **Rust daemons** over Unix domain sockets.

<br>

## Wire Format

32-byte header + optional payload:

```
Offset  Size  Field
─────────────────────────────────
0       4     Magic: SBP_MAGIC (0x53425005 "SBP\5")
4       1     Version: 0x05
5       1     Opcode
6       2     Flags
8       8     Request ID (uint64)
16      8     Timestamp (microseconds, uint64)
24      4     Payload length (uint32)
28      4     CRC32 checksum
32      N     Payload (JSON-encoded)
```

CRC32 is computed over header bytes **0–27** concatenated with the payload.

<br>

## Flags

| Bit  | Constant | Description |
|------|----------|-------------|
| 0    | `REQUEST` | Message is a request |
| 1    | `RESPONSE` | Message is a response |
| 2    | `ERROR` | Message indicates an error |
| 3    | `EVENT` | Message is an event notification |

<br>

## Opcodes

| Opcode | Name | Description |
|--------|------|-------------|
| 0x01 | `HELLO` | Client handshake |
| 0x02 | `HEARTBEAT` | Keep-alive ping/pong |
| 0x03 | `MEM_PRESSURE` | Memory pressure notification |
| 0x04 | `MEM_QUOTA` | Memory quota enforcement |
| 0x05 | `GPU_PRESSURE` | GPU pressure notification |
| 0x06 | `WORKER_STATE` | Worker pool state update |
| 0x07 | `BOOT_READY` | Boot sequence complete |
| 0x08 | `RECOVERY` | Recovery mode signal |
| 0x09 | `SHUTDOWN` | Daemon shutdown request |

<br>

## SBP-MEM Extension

The SBP-MEM extension adds dedicated opcodes for memory management:

| Opcode | Name | Direction |
|--------|------|-----------|
| 0x15 | RAM_STATUS | C → S |
| 0x16 | RAM_FLUSH | C → S |
| 0x17 | RAM_GC_SIGNAL | C → S |
| 0x18 | RAM_REGISTER_APP | C → S |
| 0x19 | RAM_UNREGISTER_APP | C → S |
| 0x1A | RAM_SET_QUOTA | C → S |
| 0x1B | RAM_APP_STATUS | C → S |
| 0x23 | RAM_SNAPSHOT | C → S |

`C = Kernel A (Unifier)`, `S = VRM Daemon`.

<br>

## Clients (Volt Unifier)

Registered SBP client types from Kernel A:

| Client | Daemon |
|--------|--------|
| `KernelBClient` | Kernel B — Tesseract Engine |
| `VrmClient` | VRM — RAM Manager |
| `VumClient` | VUM — USB Manager |
| `VgmClient` | VGM — GPU Manager |
| `DwpClient` | DWP — Worker Pool |
| `AscClient` | ASC — Adaptive System Config |

<br>

## Bridges (Electron)

Bridges connect Electron renderer processes to Kernel A via the preload layer:

| Bridge | Renderer Scope |
|--------|----------------|
| `DesktopBridge` | Desktop shell |
| `FinderBridge` | File manager |
| `SettingsBridge` | System settings |
| `OrbitBridge` | Orbit AI assistant |
| `DevToolsBridge` | Developer tools |
| `AirBarBridge` | AirBar app launcher |

<br>

## Limits

| Parameter | Value |
|-----------|-------|
| Max payload size | 65,536 bytes (64 KB) |
| Transport | Unix domain sockets |
| CRC32 | IEEE 802.3 polynomial |

<br>

## Related

- [Kernel WebSocket Protocol](kernel-ws.md)
- [Kernel B Configuration](../config/kernel-b.toml.md)
- [Volt Client API](volt-client-api.md)

<br>

---

[← Back: Documentation Index](../index.md)
