# Permissions Manager

**Review and manage permissions granted to every Samaris system app.**

Centralised permissions dashboard for controlling what system resources and actions each application can access.

## Features

- **Per-app listing**: All registered apps displayed with their permission sets
- **Granular toggle**: Individual permission toggles per action
- **Real-time apply**: Changes take effect immediately through Kernel A
- **Read-only view**: Current state loaded on open, reflects all granted permissions

## Permission Actions

Permissions shown depend on the app. Examples include:
- **filesystem:read** — Read files and directories
- **filesystem:write** — Create, modify, delete files
- **network:connect** — Establish network connections
- **audio:playback** — Play audio
- **microphone:access** — Access microphone input
- **camera:access** — Access camera
- **notifications:send** — Send system notifications

## Interface

```
┌─────────────────────────────────────────┐
│ Permissions Manager                     │
│ Review permissions for system apps      │
├─────────────────────────────────────────┤
│ ┌─ com.samaris.filesystem ────────────┐ │
│ │ ☑ filesystem:read                   │ │
│ │ ☑ filesystem:write                  │ │
│ │ ☐ microphone:access                 │ │
│ └─────────────────────────────────────┘ │
│ ┌─ com.samaris.browser ──────────────┐  │
│ │ ☑ network:connect                  │ │
│ │ ☐ camera:access                    │ │
│ └─────────────────────────────────────┘ │
└─────────────────────────────────────────┘
```

## Integration

Communicates with Kernel A's permission manager service via `permissionsKernel` client. Supports `listAll()` and `set(appId, action, allowed)` operations.
