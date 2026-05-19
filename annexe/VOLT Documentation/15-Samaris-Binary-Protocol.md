# 15. Samaris Binary Protocol

## 15.1 Overview

The Samaris Binary Protocol (SBP) is the native inter-process communication protocol of Samaris OS. It is designed for low-latency, typed, checksummed message exchange between VOLT's native Rust daemons and the Node.js-based orchestration layer. SBP reduces the serialisation overhead of JSON-based communication for high-frequency system signals while maintaining structured message semantics.

The protocol is implemented in two locations:
- **Node.js side**: the Volt Unifier's SBP module (`volt-unifier/sbp/`) handles encoding, decoding, routing, and permission checking
- **Rust side**: each daemon implements SBP framing for its IPC transport

## 15.2 Message Format

Each SBP message consists of a 32-byte header followed by an optional payload:

| Offset | Size | Field | Description |
|--------|------|-------|-------------|
| 0 | 4 | Magic | Protocol identifier (`SBP_MAGIC`, constant) |
| 4 | 1 | Version | Protocol version number |
| 5 | 1 | Opcode | Message type identifier |
| 6 | 2 | Flags | Bitfield: REQUEST, RESPONSE, ERROR, EVENT |
| 8 | 8 | RequestId | Unique 64-bit request identifier |
| 16 | 8 | TimestampUs | Message creation timestamp (microseconds) |
| 24 | 4 | PayloadLen | Payload length in bytes (0–N) |
| 28 | 4 | Checksum | CRC32 of header prefix (bytes 0–27) + payload |
| 32 | N | Payload | Message body (opaque byte sequence) |

### Magic and Version

The magic number identifies the SBP protocol and is verified on every message. The version field allows future protocol evolution with backward compatibility checks.

### Opcodes

Each message type is identified by an opcode:

| Opcode | Name | Purpose |
|--------|------|---------|
| 0x01 | HELLO | Connection handshake |
| 0x02 | HEARTBEAT | Service liveness signal |
| 0x03 | MEM_PRESSURE | Memory pressure notification |
| 0x04 | MEM_QUOTA | Per-application quota state |
| 0x05 | GPU_PRESSURE | GPU/VRAM pressure signal |
| 0x06 | WORKER_STATE | Worker pool utilisation report |
| 0x07 | BOOT_READY | Service readiness signal |
| 0x08 | RECOVERY | Recovery mode activation |
| 0x09 | SHUTDOWN | Graceful shutdown request |

### Flags

The 16-bit flags field encodes message semantics:

- **REQUEST** (bit 0): message expects a response
- **RESPONSE** (bit 1): message is a response to a prior request
- **ERROR** (bit 2): message indicates an error condition
- **EVENT** (bit 3): asynchronous event notification

Multiple flags can be combined; for example, an error response would have RESPONSE | ERROR set.

## 15.3 Checksum

SBP uses CRC32 for integrity verification. The checksum is computed over bytes 0–27 of the header (all fields before checksum) concatenated with the payload. A mismatch between stored and computed checksum causes the message to be rejected with `InvalidSbpMessageError`.

## 15.4 SBP-MEM: Memory Extension

SBP-MEM is a protocol extension specifically for memory management communication between the VRM daemon and the Volt Unifier. It defines a structured message set for:

- Memory pressure levels and transitions
- Per-application quota utilisation
- Compression pool statistics
- Page migration events between tiers
- Recommended system actions based on pressure state

SBP-MEM messages use the base SBP framing with opcodes reserved for memory operations. The extension is implemented in the VRM's `sbp_mem/` module (Rust) and the Unifier's VrmClient (Node.js).

## 15.5 Transport

SBP messages are typically transmitted over Unix sockets between Kernel A and each Rust daemon. The transport layer handles:

- Message framing and reassembly
- Partial read buffering
- Ordered delivery guarantees (Unix socket semantics)
- Connection lifecycle management

## 15.6 Message Lifecycle

```
Sender:
  1. Create SbpMessage with opcode, flags, payload
  2. Compute CRC32 checksum over header + payload
  3. Serialize to 32-byte header + payload buffer
  4. Write to transport (Unix socket)

Receiver:
  1. Read bytes from transport
  2. Accumulate in per-module buffer
  3. Attempt to parse SbpMessage.fromBuffer()
  4. Verify magic, version, payload length bounds
  5. Verify CRC32 checksum
  6. Route to handler based on opcode and flags
  7. If response/error, correlate with pending request via RequestId
  8. If event, dispatch to registered subscribers
```
