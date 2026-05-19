# 14. Native Terminal

## 14.1 Overview

The Samaris native terminal provides a genuine Linux shell experience within the desktop environment. Built on `node-pty` in the Electron backend and `xterm.js` in the React UI, it supports real command execution, streaming output, session management, and resize handling. The terminal is a first-class desktop application with the same security and integration patterns as other Samaris components.

## 14.2 Architecture

### Backend (Electron — TerminalManager)

The `TerminalManager` class manages node-pty pseudoterminal processes:

- Each terminal session creates a new PTY process (bash/zsh/sh depending on system configuration)
- PTY output is streamed to the UI via IPC (`terminal:data` events)
- Input from the UI is written to the PTY's stdin
- Terminal resize events update the PTY's dimensions (columns × rows)
- Sessions are tracked by unique ID for multi-tab support
- All sessions are terminated on application quit (`before-quit` event)

### IPC Bridge

The preload bridge exposes terminal functions:

| Function | Purpose |
|----------|---------|
| `create(id, options)` | Create new PTY session |
| `write(id, data)` | Write input to PTY stdin |
| `resize(id, cols, rows)` | Resize PTY dimensions |
| `kill(id)` | Terminate PTY session |
| `onData(callback)` | Subscribe to PTY output stream |
| `onExit(callback)` | Subscribe to session exit events |

### Frontend (React — xterm.js)

The frontend uses the xterm.js library to render the terminal emulator in the browser. It:

- Connects to the Electron backend via preload bridge
- Renders PTY output with full ANSI escape sequence support
- Captures keyboard input and forwards to the PTY
- Detects container resize and propagates new dimensions
- Applies the Samaris OS colour theme

## 14.3 Lifecycle

```
User opens terminal
  → UI calls electronAPI.terminal.create(id, options)
  → Electron: TerminalManager spawns node-pty with shell
  → PTY output streams via terminal:data IPC event
  → UI renders output in xterm.js instance

User types command
  → xterm.js captures keystrokes
  → UI calls electronAPI.terminal.write(id, data)
  → Electron writes to PTY stdin
  → Shell processes command, produces output
  → Output streams back to UI

User closes terminal
  → UI calls electronAPI.terminal.kill(id)
  → Electron sends SIGHUP to PTY process
  → Session cleaned up
```

## 14.4 Security

The terminal runs as the desktop user with their natural permissions. Command execution is not sandboxed at the terminal level — users have shell access equivalent to a standard Linux desktop terminal. The terminal respects:

- Filesystem permissions of the desktop user
- Process ownership boundaries
- Standard Linux security model (no privilege escalation)

A future enhancement may introduce optional command filtering or risk warnings for dangerous operations, but the Alpha terminal provides unrestricted shell access.
