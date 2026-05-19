# 20. System States

## 20.1 Overview

Samaris OS operates as a deterministic state machine. The system state governs desktop behaviour, available features, module activation, and user interface presentation. States are managed centrally by the Volt Unifier and propagated to all subsystems.

## 20.2 State Definitions

| State | Description | UI Presentation |
|-------|-------------|-----------------|
| BOOTING | System initialising, services starting | Boot splash screen |
| READY | Desktop fully operational | Normal desktop interface |
| DEGRADED | System usable with reduced functionality | Warning indicator in AirBar |
| PRESSURE | Resources under significant load | Performance adjustments active |
| RECOVERY | Recovery mode active | Minimal recovery UI |
| SHUTTING_DOWN | Graceful shutdown in progress | Shutdown screen |
| FAILED | Critical error, system inoperable | Error display with options |

## 20.3 State Transitions

```
BOOTING ──────→ READY
BOOTING ──────→ DEGRADED
DEGRADED ─────→ READY
READY ─────────→ PRESSURE
PRESSURE ──────→ READY
PRESSURE ──────→ RECOVERY
RECOVERY ──────→ READY
READY ─────────→ SHUTTING_DOWN
ANY ───────────→ FAILED
ANY ───────────→ SHUTTING_DOWN
```

### BOOTING

Entered at system power-on. The boot splash screen (`/opt/volt/boot.html`) is displayed. Systemd services start in dependency order. The Volt Unifier enters "starting" state. Exit condition: desktop ready signal received.

### READY

Normal operating state. All modules are online (or gracefully degraded). Desktop UI is fully interactive. The Unifier reports "running" state. Transitions to PRESSURE when resource thresholds are crossed.

### DEGRADED

One or more non-critical modules are unavailable. Desktop remains functional:

- Missing module functionality is hidden or shows "unavailable"
- AirBar displays a health indicator (amber)
- Unifier reports which modules are degraded and why
- System attempts automatic recovery via module restarts

### PRESSURE

System resources (memory, GPU, or CPU) are under significant load. Transition is triggered by:

- VRM pressure entering orange or red zone
- VGM thermal backoff activation
- DWP desktop frame budget overrun

In PRESSURE state:

- Background tasks are throttled or suspended
- Cache purging and aggressive compression are activated
- Animations may be reduced
- Orbit AI bursts are restricted

### RECOVERY

Critical failure prevents normal operation. Recovery mode:

- Launches minimal UI with recovery options
- Disables all non-essential features
- Provides log viewing and diagnostic tools
- Awaits user action (restart, retry, or shutdown)

### SHUTTING_DOWN

Graceful system shutdown. The Unifier:

- Notifies all modules of shutdown
- Awaits module acknowledgement
- Flushes pending writes (VRM, VUM)
- Terminates processes in reverse dependency order
- Signals systemd for final power-off

### FAILED

Catastrophic failure. The system cannot recover. Displays:

- Error diagnostic screen
- Last known logs
- Options: restart, power off, recovery mode
