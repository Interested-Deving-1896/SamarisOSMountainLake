# Kernel WebSocket Protocol

Communication between the **Desktop UI** (TypeScript/React) and **Kernel A** (Node.js) over WebSocket JSON at `ws://localhost:9999`.

<br>

## Architecture — 4-Layer Structure

Kernel A follows a 4-layer architecture:

| Layer | Role |
|-------|------|
| **Core** | Connection management, authentication, rate limiting |
| **Router** | Message dispatch by namespace, request-response correlation |
| **Handler** | 26 domain-specific handlers (see below) |
| **Service** | Business logic coordinating daemon communication via SBP |

<br>

## Connection

```
ws://localhost:9999
```

Rate limit: **60 messages per second per client**.

<br>

## Message Format

### Request

```json
{
  "type": "namespace.action",
  "data": { ... },
  "requestId": "req-12345",
  "appId": "volt.desktop"
}
```

### Response

```json
{
  "type": "namespace.action.result",
  "data": { ... },
  "requestId": "req-12345"
}
```

<br>

## Handler Domains (26)

The following handler domains are registered:

| Domain | Description |
|--------|-------------|
| `app-lifecycle` | Application lifecycle management |
| `archive` | Archive extraction and compression |
| `audio` | Audio playback and recording |
| `battery` | Battery status and power events |
| `devices` | Hardware device enumeration |
| `disk` | Disk usage and partition info |
| `encryption` | File/system encryption |
| `filesystem` | File read, write, watch |
| `firewall` | Network firewall rules |
| `mail` | Email services |
| `media-metadata` | Media file metadata extraction |
| `network` | Network interfaces and WiFi |
| `onboarding` | First-time setup wizard |
| `orbit-ai` | Orbit LLM inference |
| `power` | Power management (sleep, hibernate) |
| `printing` | Printer discovery and jobs |
| `processes` | Process listing and management |
| `runtime` | System runtime info |
| `search` | File and content search |
| `session` | Session save/restore |
| `storage` | Storage device management |
| `stt` | Speech-to-text |
| `system-info` | System information queries |
| `tts` | Text-to-speech |
| `user-management` | User account management |
| `wine` | Wine/Windows compatibility |

<br>

## REST Endpoints

Kernel A also exposes HTTP REST endpoints:

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/health` | GET | Kernel A health check |
| `/api/unifier/health` | GET | Volt Unifier daemon health |
| `/api/unifier/snapshot` | GET | Full system state snapshot |
| `/api/unifier/modules` | GET | Loaded daemon module list |
| `/api/peregrine/open` | POST | Open URL in Peregrine browser |
| `/api/peregrine/status` | GET | Peregrine browser status |
| `/api/fs/read-file` | GET | Read file contents |

<br>

## Communication Flows

| Direction | Transport | Details |
|-----------|-----------|---------|
| **UI → Kernel A** | WebSocket JSON | `ws://localhost:9999` |
| **Kernel A → Rust Daemons** | SBP over Unix socket | Binary protocol |
| **Electron → Kernel A** | HTTP REST | Localhost HTTP |

<br>

## Namespace Catalog

### Orbit

| Message | Data | Returns |
|---------|------|---------|
| `orbit.status` | `{}` | `{name, sizeLabel, runtimeStatus, runtimeLabel}` |
| `orbit.generate` | `{prompt, modeId, strategy}` | `{finalAnswer, modelId}` |
| `orbit.shutdown` | `{}` | `{ok, stopped}` |

### STT

| Message | Data | Returns |
|---------|------|---------|
| `stt.transcribe` | `{audioBase64, mimeType, language}` | `{text, language, durationMs}` |

### TTS

| Message | Data | Returns |
|---------|------|---------|
| `tts.speak` | `{text, voice}` | `{audioBase64, mimeType, durationMs}` |

### WiFi

| Message | Data | Returns |
|---------|------|---------|
| `device.connectivity.status` | `{}` | Full connectivity state |
| `device.wifi.toggle` | `{enabled}` | Updated state |
| `device.wifi.connect` | `{ssid, password}` | Updated state |
| `device.wifi.disconnect` | `{}` | Updated state |
| `device.wifi.forget` | `{ssid}` | `{ok}` |
| `device.wifi.saved` | `{}` | `[{ssid}]` |

### Network

| Message | Data | Returns |
|---------|------|---------|
| `network.list` | `{}` | `[{id, name, type, mac, address, mode, connected}]` |
| `network.setConfig` | `{interfaceId, mode, address, ...}` | `{applied, note, interfaces}` |

### Filesystem

| Message | Data | Returns |
|---------|------|---------|
| `fs.list` | `{path}` | `{path, nodes[]}` |
| `fs.read` | `{path}` | `{path, content}` |
| `fs.readDataUrl` | `{path}` | `{path, dataUrl}` |
| `fs.write` | `{path, content}` | `{ok}` |
| `fs.writeBase64` | `{path, base64}` | `{ok}` |
| `fs.mkdir` | `{path}` | `{ok}` |
| `fs.rename` | `{from, to}` | `{ok}` |
| `fs.copy` | `{from, to}` | `{ok}` |
| `fs.delete` | `{path, recursive}` | `{ok}` |
| `fs.watch` | `{path}` | `{ok}` |
| `fs.unwatch` | `{path}`, `{subscriptionId}` | `{ok}` |

### System

| Message | Data | Returns |
|---------|------|---------|
| `system.ping` | `{}` | `{ok}` |
| `system.state` | `{}` | Full state snapshot |
| `session.save` | snapshot | `{ok}` |
| `session.security.get` | `{}` | Security state |

<br>

## Related

- [SBP Binary Protocol](sbp-protocol.md)
- [Volt Client API](volt-client-api.md)
- [Kernel B Configuration](../config/kernel-b.toml.md)

<br>

---

[← Back: Documentation Index](../index.md)
