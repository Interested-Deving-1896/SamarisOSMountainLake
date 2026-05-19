# Security

Multi-layered security architecture designed for the Alpha stage, establishing fundamental protections against common vulnerability classes. The architecture follows the principle of least privilege.

<br>

## Core Principles

1. **The renderer is untrusted**: React code never has direct access to Node.js, Electron, or system APIs
2. **All native access is bridged**: system calls pass through typed, validated IPC handlers
3. **Path access is whitelisted**: filesystem operations are restricted to authorised directories
4. **Protocol access is restricted**: external resource opening is limited to safe protocols
5. **Capabilities are verified**: IPC clients must be authorised for sensitive SBP opcodes
6. **Actions are audited**: security-relevant operations are logged with module ID, action, and decision

<br>

## Layer Isolation

```
React Renderer (untrusted)
    │
    ▼ contextBridge (preload.js)
    │   Only explicitly exposed functions
    ▼
Electron Main Process (trusted)
    │
    ▼ IPC Handlers with validation
    │   Path whitelist, protocol allowlist, permission checks
    ▼
Kernel A / Native Daemons (trusted)
```

<br>

## Preload Bridge Security

- Exposes a fixed, type-checked API surface via `contextBridge.exposeInMainWorld`
- Never passes raw Node.js objects, process references, or Electron internals
- Does not expose `ipcRenderer.send` or `ipcRenderer.on` directly

<br>

## IPC Handler Validation

| Handler | Validation |
|---------|------------|
| `shell:openPath` | Resolved path must be within `~/.volt` or `~/Downloads` |
| `shell:openExternal` | Protocol must be http:, https:, or mailto: |
| `shell:showItemInFolder` | Same path whitelist as openPath |
| All `browser:*` | URL validation, WebView creation blocked |
| All dialog handlers | Native Electron dialog with no programmatic path injection |

<br>

## Permission Categories

| Permission | Scope | Example Operation |
|------------|-------|-------------------|
| `fs.read` | User files | Read document content |
| `fs.write` | User files | Save file to Documents |
| `fs.delete` | User files | Delete selected file |
| `network.scan` | Wi-Fi | List available networks |
| `network.connect` | Wi-Fi | Connect to network |
| `terminal.run` | Shell | Open terminal session |
| `browser.download` | Web | Download file through browser |
| `system.power` | Power management | Shutdown, reboot |

At Alpha stage, permission enforcement is architectural but not fully gated. The infrastructure for capability-based access control is implemented in the Volt Unifier's capability guard.

<br>

## Vault Service

| Feature | Details |
|---------|---------|
| WiFi passwords | Encrypted via vault before storage |
| Methods | `vault.encryptForActiveUser()` / `decryptForActiveUser()` |
| Algorithm | AES-256-GCM |

<br>

## Disk Encryption

| Script | Purpose |
|--------|---------|
| `expand-and-encrypt.sh` | First-boot partition expansion + LUKS setup |
| `mount-user-storage.sh` | Mount user data with LUKS |

LUKS encryption architecture is implemented but temporarily disabled in Alpha One during VM-focused validation.

<br>

## Firewall

Stateful firewall with per-app outbound network controls. Architecture defined, nftables-based. Implementation on the roadmap.

<br>

## SBP Security

- CRC32 checksum verification on every message
- Opcode-based capability checking before routing
- Source validation: the router knows which module each connection belongs to
- Response correlation prevents message injection

<br>

## Path Protection

Protected paths never accessible through UI APIs: `/etc`, `/boot`, `/root`, `/sys`, `/proc`, `/dev`. Runtime paths (`/run`, `/var/log`) are read-only from the UI. User data paths (`/home/samaris/Desktop`, `Documents`) are accessible. Application data stored in `~/.volt/` with controlled access.

<br>

---

[← Back: Architecture Overview](overview.md)
