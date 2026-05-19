# Terminal

Full terminal emulator using **xterm.js** with local shell access via **node-pty**, supporting multiple tabs and full PTY lifecycle management.

<br>

## Features

- Multiple terminal tabs with rename and reorder
- xterm.js with `fit` addon for automatic resize
- Web Links addon for clickable URLs
- Local system shell via PTY
- Copy/paste support
- Configurable font size and theme

<br>

## Lifecycle

```
User opens tab
  → createTerminal()
    → node-pty spawns shell (forkpty)
      → xterm.js attaches to PTY fd
        → bidirectional I/O via terminal:data IPC
  → User closes tab
    → pty.kill() → SIGTERM → SIGKILL (after grace period)
```

<br>

## Architecture

```
Terminal (React)
├── xterm.js instance (with fit + web-links addons)
├── pty-helper.js (Node.js child process)
│   └── node-pty → local shell
├── TabBar (multi-tab management)
├── TerminalThemeProvider (light/dark theme sync)
└── IPC bridge via Electron
    └── terminal:data (bidirectional I/O)
```

**Write limit:** 100 KB per data transmission to prevent renderer flooding.

<br>

## Security Note

The terminal runs as the desktop user and provides **unrestricted shell access** to the underlying system. Users have full access to all files and commands their user account can execute. No sandboxing is applied to the PTY process.

<br>

## Related

- [VOLT Architecture — Chapter 14: Terminal Spec](../architecture/volt-ch14-terminal.md)
- [VOLT Architecture — Security Model](../architecture/volt-security.md)
- [Kernel WebSocket — Terminal Channels](../apis/kernel-ws.md#terminal)

<br>

---

[← Back: Documentation Index](../index.md)
