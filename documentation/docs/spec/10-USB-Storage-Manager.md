# 10. USB Storage Manager

## 10.1 Overview

The Volt USB Manager (VUM) is the journaled storage management subsystem of Samaris OS. Written in Rust as an independent daemon (`volt-usb-manager`), it provides write caching, journaling, device hotplug detection, and FUSE-based filesystem integration for removable storage. The VUM ensures safe, performant access to USB storage devices with crash recovery guarantees.

## 10.2 Architecture

### Device Detection

The device subsystem continuously monitors for removable storage using udev events and periodic polling (configurable interval, default 1000 ms). It:

- Detects device insertion and removal events
- Verifies removable flag before auto-mounting
- Handles surprise removal with pending writeback
- Enumerates device metadata (vendor, model, capacity, filesystem type)

### Write Cache

A read-write cache layer provides:

- Read cache: up to 256 MiB with LRU eviction at 90% utilisation
- Writeback buffer: up to 128 MiB with configurable flush interval (default 5000 ms)
- Compression of cached data using ZSTD level 3
- Boot asset pinning: critical boot files are retained in cache
- Cache is flushed before device removal

### Journal (WAL)

The journal implements a Write-Ahead Log (WAL) for crash-consistent write operations:

- Stored at `/var/lib/samaris/volt-usb-manager/journal.wal`
- CRC32 checksums on every record
- fsync on each record write for durability
- Automatic replay on daemon startup
- Corrupt record rejection with logged error
- Write coalescing and batching (default 128 KiB batch size)

### IO Scheduler

Manages IO operations with configurable parameters:

- Maximum concurrent flushes (default: 2)
- NAND-aware block alignment (128 KiB)
- Latency metrics collection for performance analysis

### FUSE Integration

Optional FUSE filesystem layer provides:

- User-space filesystem mounting at `/mnt/samaris-usb`
- Read-only or read-write modes
- Auto-unmount on device removal
- Metadata caching for directory listings

## 10.3 Write Durability Modes

| Mode | Behaviour | Use Case |
|------|-----------|----------|
| journaled | Full WAL, metadata fsync, checksums | Maximum data safety |
| buffered | In-memory buffer, periodic flush | Performance-sensitive operations |

## 10.4 Lifecycle

```
Device Inserted
  → udev event detected
  → Device verified (removable flag, filesystem)
  → Cache initialised
  → Journal replayed (if recovering from crash)
  → FUSE mount established (if enabled)
  → Ready for IO operations

Device Removed (normal)
  → All pending writes flushed
  → Journal finalised
  → Cache evicted
  → FUSE unmounted

Device Removed (surprise)
  → Dirty cache detected
  → Emergency flush attempted
  → Journal retained for replay
  → Consistency markers checked on next mount
```

## 10.5 Configuration

The VUM is configured via `/opt/volt/usb-manager/config.toml`. Key sections include:

- Mount point, backing path, safe mode
- FUSE enable/disable, read-only mode
- Device detection interval, removable requirement
- Cache size, eviction threshold, compression settings
- Writeback buffer size, flush interval, batch size, durability mode
- Journal path, checksum enable, fsync policy
- IO scheduler parameters
- Metrics enablement
