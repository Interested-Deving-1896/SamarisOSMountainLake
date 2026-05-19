# Utilities

**Central hub for system tools — System Monitor, Task Manager, Disk Utility.**

Provides access to various system monitoring and administration tools in a single interface.

## Available Tools

### System Monitor
Real-time system resource monitoring:
- CPU utilisation and per-core load
- Memory usage (RAM, swap)
- Disk I/O activity
- Network throughput
- Process list with resource consumption

### Task Manager
Process and application lifecycle management:
- Running processes and services
- CPU and memory usage per process
- Force-terminate unresponsive applications
- Service restart controls

### Disk Utility
Storage device and partition management:
- Connected block devices overview
- Partition layout and filesystem type
- Disk usage analytics
- Mount/unmount controls

## Interface

Tabbed layout with tool selector at the top. Each tool occupies a separate tab with its own controls and data displays.

## Integration

- System Monitor reads metrics from Kernel A's `systemMetricsService`
- Task Manager communicates via `processManager` service
- Disk Utility uses `diskService` and `storageService` via Kernel A WebSocket
