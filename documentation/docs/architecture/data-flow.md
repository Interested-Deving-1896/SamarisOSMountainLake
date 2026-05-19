# Data Flow

The complete lifecycle of a WebSocket message from the React UI to backend services and back.

<br>

## Message Lifecycle

```
[React UI] → kernelClient.request({ type, data })
    │
    ▼
WebSocket :9999
    │
    ▼
[server.js] handleClientMessage()
    │
    ▼
parsed JSON → router.route(message)
    │
    ▼
namespace → handler (e.g., "orbit" → orbit.js)
    │
    ▼
handler.handle() → service method (e.g., orbitRuntime.generate())
    │
    ▼
[sbp/unifier] → Rust daemon via SBP (optional)
    │
    ▼
result → { type: ".result", data, requestId }
    │
    ▼
WebSocket ← JSON envelope
    │
    ▼
[kernelClient.ts] handleIncoming()
    │
    ▼
resolve pending request → UI receives data
```

<br>

## Message Pattern

All messages follow the same naming convention:

| Direction | Pattern | Example |
|-----------|---------|---------|
| Request | `namespace.action` | `orbit.generate` |
| Response | `namespace.action.result` | `orbit.generate.result` |
| Error | `namespace.action.error` | `orbit.generate.error` |

<br>

## Examples

| Request | Response | Purpose |
|---------|----------|---------|
| `orbit.generate` | `orbit.generate.result` | LLM text generation |
| `stt.transcribe` | `stt.transcribe.result` | Speech-to-text |
| `tts.speak` | `tts.speak.result` | Text-to-speech |
| `fs.list` | `fs.list.result` | Directory listing |
| `device.wifi.connect` | `device.wifi.connect.result` | WiFi connection |

<br>

## Event Broadcast (System → UI)

```
Rust Daemon (SBP event) → Volt Unifier → Event Bus →
  Kernel A → WebSocket → React state update
```

<br>

## Rate Limiting

Kernel A enforces a rate limit of **60 messages/second/client** to prevent abuse and ensure fair resource allocation.

<br>

---

[← Back: Architecture Overview](overview.md)
