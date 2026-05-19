# 17. Watchdog and Recovery

## 17.1 Overview

The watchdog and recovery subsystem ensures that Samaris OS remains operational or fails gracefully under adverse conditions. Rather than leaving the user with a black screen or frozen interface, the system detects failures, implements proportionate responses, and provides recovery paths. The watchdog operates at multiple levels — from individual daemon health checks to system-wide recovery orchestration.

## 17.2 Watchdog Architecture

Watchdog functionality is distributed across three layers:

### Level 1: systemd Service Watchdog

Each VOLT systemd service has `Restart=on-failure` with appropriate restart delays. systemd handles:

- Process crash detection
- Automatic restart (with rate limiting)
- Dependency-aware restart ordering
- Service failure logging to journald

### Level 2: Volt Unifier Health Monitor

The Unifier's health monitor provides semantic-level health assessment beyond process liveness:

- Periodic SBP heartbeat messages to each Rust daemon
- Timeout detection (configurable, default: 10 s for requests)
- Degradation tracking: accumulated error counts, reconnect attempts
- Capability verification: each module's advertised capabilities are checked against actual behaviour

### Level 3: Desktop Crash Detection

The Electron shell monitors:

- Kernel A connectivity via HTTP health check (`/health` endpoint)
- Renderer process responsiveness
- Window event loop health
- IPC handler availability

## 17.3 Degraded Mode

When non-critical modules fail, the system enters degraded mode rather than halting. In degraded mode:

- Desktop remains functional with core features
- Failed module functionality is unavailable (e.g., USB mounts if VUM is down)
- AirBar displays a system health warning indicator
- The Unifier reports module-specific degradation reasons
- Background tasks from the failed module are suspended

## 17.4 Recovery Mode

Recovery mode is triggered when the desktop cannot start or a critical module fails repeatedly. The recovery subsystem:

1. Detects the failure condition (desktop crash loop, Kernel A unreachable, critical daemon repeatedly failing)
2. Disables non-essential animations and visual effects
3. Purges system caches
4. Ignores non-critical module initialisation
5. Launches a minimal UI with recovery options:
   - View system logs
   - Restart individual services
   - Full system restart
   - Launch recovery terminal
6. Logs the recovery event for post-mortem analysis

### Recovery Mode Entry Conditions

| Condition | Trigger |
|-----------|---------|
| Desktop crash loop | 3+ consecutive desktop service failures within 60 seconds |
| Kernel A unavailable | Health check fails for 30+ seconds after boot |
| Critical module failure | Tesseract Engine or VRM repeatedly crashes |
| Boot timeout | Desktop not ready within 120 seconds of boot start |

## 17.5 Heartbeat Protocol

The SBP HEARTBEAT message (opcode 0x02) is the canonical liveness signal:

- Sent by each Rust daemon at configurable intervals
- Received and tracked by the Volt Unifier's HeartbeatManager
- Missed heartbeats trigger health degradation assessment
- Heartbeat content includes: module status, last error, uptime, current load

## 17.6 Failure Response Matrix

| Failure | Detection | Immediate Response | Recovery |
|---------|-----------|-------------------|----------|
| Daemon crash | systemd, missing heartbeat | systemd restart (1s) | Persistent crashes → degraded mode |
| Memory exhaustion | VRM pressure red zone | Cache purge, background suspension | Recovery mode if critical |
| GPU hang | VGM thermal watchdog | CPU fallback, safe mode | Recovery mode if persistent |
| UI freeze | Frame budget overrun | Worker reduction, animation disable | User-triggered reload |
| Boot hang | Desktop timeout (120s) | Recovery mode activation | User selects recovery option |
| Kernel A crash | Electron health check | Electron starts replacement | systemd restart if detached |
