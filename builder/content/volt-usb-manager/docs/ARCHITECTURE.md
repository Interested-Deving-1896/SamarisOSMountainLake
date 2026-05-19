# Volt USB Manager — Architecture

## Overview

Volt USB Manager (VUM) is a **RAM-first, journaled** removable storage manager for Samaris OS. It
sits between user-space applications and a physical USB device, providing a FUSE filesystem backed
by a read cache, a write-back buffer, a write-ahead journal, and an I/O scheduler.

## High-Level Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                      User Applications                          │
├─────────────────────────────────────────────────────────────────┤
│                     FUSE Mount Point                            │
│                     /mnt/volt_usb                               │
├─────────────────────────────────────────────────────────────────┤
│                        VumFilesystem                            │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌────────────────┐  │
│  │  Lookup  │  │   Read   │  │  Write   │  │  Fsync/Flush   │  │
│  │  Getattr │  │  (Cache) │  │ (Buffer) │  │  (Journal)     │  │
│  └──────────┘  └────┬─────┘  └────┬─────┘  └───────┬────────┘  │
├──────────────────────┼─────────────┼────────────────┼───────────┤
│                      │             │                │           │
│                  ┌───▼──┐    ┌────▼────┐     ┌─────▼──────┐    │
│                  │Read  │    │ Write   │     │  Journal   │    │
│                  │Cache │    │ Buffer  │     │  (WAL)     │    │
│                  │(RAM) │    │ (RAM)   │     │  (Disk)    │    │
│                  └──┬───┘    └────┬────┘     └─────┬──────┘    │
│                     │             │                │           │
│                     └──────┬──────┘                │           │
│                            │                       │           │
│                       ┌────▼────┐                  │           │
│                       │  I/O    │◄─────────────────┘           │
│                       │Scheduler│                              │
│                       └────┬────┘                              │
│                            │                                   │
├────────────────────────────┼───────────────────────────────────┤
│                    ┌───────▼────────┐                          │
│                    │   USB Device   │                          │
│                    │  (Backing Store)│                          │
│                    └────────────────┘                          │
└─────────────────────────────────────────────────────────────────┘
```

## Components

### FUSE Layer (`src/fuse/`)
Implements the FUSE protocol operations: `lookup`, `getattr`, `read`, `write`, `create`, `mkdir`,
`unlink`, `rename`, `readdir`, `flush`, `fsync`, `setattr`. The `VumFilesystem` struct holds
references to the cache, write buffer, journal, and inode table.

### Read Cache (`src/cache/`)
RAM-first cache using a DashMap for concurrent access with LRU eviction. Supports pinning
(for boot/desktop assets) and optional compression (zstd for large files, lz4 for small).
Cache entries track access count and last-access time for eviction policy.

### Write Buffer (`src/writeback/`)
Buffers writes in RAM before flushing to the USB device. Returns `ACK_BUFFERED` immediately
on enqueue. Flushes produce batches that respect a configurable max batch size. Metadata
writes are prioritized. A flush daemon triggers based on percentage-full or interval.

### Journal (`src/journal/`)
Write-ahead log (WAL) that records all mutations before they are applied. Supports
begin/commit/abort lifecycle for writes, deletes, and renames. Uses CRC32 checksums on
every record. Checkpoints compact the journal. Clean shutdown writes a marker so
recovery is not needed on next mount.

### I/O Scheduler (`src/scheduler/`)
Multi-level priority queue (CriticalMetadata > Desktop > UserVisible > Background > Cache).
Batches adjacent I/O to the same file. Fairness policy limits how many operations per
priority class can execute in a time window. Throttling prevents cache-flush
(background) I/O from starving user-visible operations.

### Compression (`src/compression/`)
Zstd (for files > 1MB) and LZ4 (for smaller files) backends. Already-compressed formats
(zip, png, jpg, mp4, etc.) are detected by extension and stored uncompressed. The ratio
tracker keeps running savings statistics.

### Device Manager (`src/device/`)
Handles device detection, mount info, health checks, and eject logic. On macOS, eject
uses `diskutil eject`. On Linux, it uses `umount -f`. Surprise removal is handled by
polling at the configured detect interval.

### Safety (`src/safety/`)
Invariant checker enforces: no dirty writes after clean shutdown, no ACK_DURABLE before
commit, no mount with unrecoverable journal, no path escaping the backing store.

## Read Path

1. Application calls `read()` on a FUSE file
2. `VumFilesystem::read()` checks the inode table for metadata
3. Looks up content in the Read Cache by CacheKey (SHA-256 of path+mtime+size)
4. **Cache hit**: return data directly (decompress if compressed)
5. **Cache miss**: return empty (real implementation would read from USB)

## Write Path

1. Application calls `write()` on a FUSE file
2. `VumFilesystem::write()` updates inode size
3. Enqueues a PendingWrite in the WriteBuffer
4. WriteBuffer sends `ACK_BUFFERED` to the ack channel
5. Journal records `BeginWrite` + `CommitWrite` (if journaling enabled)
6. Flush daemon or explicit flush batches pending writes and sends them to USB
7. On fsync, journal checkpoint is written and `ACK_DURABLE` is sent

## Journal & Recovery

1. Every mutation goes through `begin_*` / `commit_*` in the Journal
2. Records are appended to the WAL file with CRC32 checksums
3. On clean shutdown, a `CleanShutdown` marker is written
4. On next mount: if no `CleanShutdown` marker, RecoveryEngine replays the WAL
5. Committed writes are applied to the backing store; incomplete writes are ignored
6. If recovery fails, the system falls back to read-only mode

## User-Space Design Philosophy

VUM operates entirely in user space with no kernel module dependencies beyond FUSE.
All RAM-first buffering, caching, scheduling, and journaling happens in user space.
The backing USB device is accessed through standard file I/O.
This design prioritizes safety (isolated crashes), portability, and ease of debugging.
The trade-off is higher memory usage and potential latency under memory pressure.
