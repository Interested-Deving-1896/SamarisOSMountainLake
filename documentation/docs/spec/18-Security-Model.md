# 18. Security Model

## 18.1 Overview

The Samaris OS security model is designed for the Alpha stage: it does not implement a military-grade permission system but establishes fundamental protections against common vulnerability classes. The architecture follows the principle of least privilege, ensuring that each layer has access only to the resources it explicitly requires.

## 18.2 Core Principles

1. **The renderer is untrusted**: React code never has direct access to Node.js, Electron, or system APIs
2. **All native access is bridged**: system calls pass through typed, validated IPC handlers
3. **Path access is whitelisted**: filesystem operations are restricted to authorised directories
4. **Protocol access is restricted**: external resource opening is limited to safe protocols
5. **Capabilities are verified**: IPC clients must be authorised for sensitive SBP opcodes
6. **Actions are audited**: security-relevant operations are logged with module ID, action, and decision

## 18.3 Layer Isolation

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

## 18.4 Preload Bridge Security

The preload bridge (`preload.js`) is the only communication channel between the renderer and Electron. It:

- Exposes a fixed, type-checked API surface via `contextBridge.exposeInMainWorld`
- Never passes raw Node.js objects, process references, or Electron internals
- Translates all renderer requests into structured IPC invocations
- Does not expose `ipcRenderer.send` or `ipcRenderer.on` directly

## 18.5 IPC Handler Validation

Every IPC handler in the main process applies input validation:

| Handler | Validation |
|---------|------------|
| `shell:openPath` | Resolved path must be within `~/.volt` or `~/Downloads` |
| `shell:openExternal` | Protocol must be http:, https:, or mailto: |
| `shell:showItemInFolder` | Same path whitelist as openPath |
| All `browser:*` | URL validation, WebView creation blocked |
| All dialog handlers | Native Electron dialog with no programmatic path injection |

## 18.6 Permission Categories

The system defines the following permission categories for future enforcement:

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

## 18.7 Audit Logging

The Volt Unifier's audit log records:

- Module initialisation and shutdown events
- Configuration generation and loading
- Security-relevant IPC operations
- Permission grants and denials
- Degradation and recovery events
- Error conditions with module attribution

Each audit entry includes: action type, module identifier, allowed/denied status, reason, and timestamp.

## 18.8 SBP Security

SBP communication implements:

- Checksum verification on every message
- Opcode-based capability checking before routing
- Source validation: the router knows which module each connection belongs to
- Response correlation prevents message injection

## 18.9 Path Protection

The system protects critical Linux paths from UI-level access:

- `/etc`, `/boot`, `/root`, `/sys`, `/proc`, `/dev` are never accessible through UI APIs
- Runtime paths (`/run`, `/var/log`) are read-only from the UI
- User data paths (`/home/samaris/Desktop`, `Documents`, etc.) are accessible
- Application data is stored in `~/.volt/` with controlled access
