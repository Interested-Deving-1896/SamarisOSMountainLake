# VUM — Volt USB Manager

**Journaled RAM-first removable storage manager for Samaris OS.**

VUM provides a high-performance, safe, FUSE-based filesystem for USB removable storage. It uses a RAM-first architecture with a read cache, writeback buffer, journal (WAL), configurable compression, and I/O scheduling — entirely in user space.

<br>

## Architecture

```
User Apps → FUSE Mount (/mnt/volt_usb) → VumFilesystem
                                            │
              ┌─────────────────────────────┼─────────────────────────────┐
              │                             │                             │
         ┌────▼────┐                 ┌──────▼──────┐           ┌─────────▼─────────┐
         │  Read   │                 │   Write     │           │     Journal       │
         │  Cache  │                 │   Buffer    │           │  (WAL, CRC32)     │
         │ (RAM)   │                 │  (RAM)      │           │  (Disk)           │
         └────┬────┘                 └──────┬──────┘           └─────────┬─────────┘
              │                             │                            │
              └──────────────┬──────────────┘                            │
                             │                                           │
                        ┌────▼────┐                                      │
                        │   I/O   │◄─────────────────────────────────────┘
                        │Scheduler│
                        └────┬────┘
                             │
                        ┌────▼────────┐
                        │ USB Device  │
                        │(Backing Store)│
                        └─────────────┘
```

<br>

## Core Components

### FUSE Layer
Implements standard POSIX filesystem operations: `lookup`, `getattr`, `read`, `write`, `create`, `mkdir`, `unlink`, `rename`, `readdir`, `flush`, `fsync`, `setattr`. All operations are journaled except reads and attribute queries.

### Read Cache
- **RAM-first**: Cache misses return empty (real impl reads from USB)
- **Capacity**: Configurable (default 256 MB)
- **Eviction**: LRU, triggers at 90% full
- **Compression**: Zstd for files > 1 MB, LZ4 for files ≤ 1 MB
- **Pinning**: Boot/desktop assets can be pinned (never evicted)
- **Concurrency**: DashMap with lock-free reads

### Write Buffer
- **Capacity**: Configurable (default 64 MB)
- **ACK semantics**: `ACK_BUFFERED` on enqueue, `ACK_DURABLE` after fsync
- **Flush triggers**: At 80% full or every 5000ms
- **Metadata priority**: Metadata writes flush before data writes
- **Durability modes**: balanced (default), performance, maximum

### Journal (WAL)
- **Write-ahead log**: All mutations recorded before application
- **CRC32 checksums**: Every record has integrity verification
- **Lifecycle**: begin → commit/abort for writes, deletes, renames
- **Checkpoints**: Compact the journal periodically
- **Clean shutdown**: Writes marker, recovery skipped on next mount
- **Recovery**: On unclean shutdown, replays WAL → committed writes applied, incomplete writes ignored → read-only fallback on failure

### I/O Scheduler
Five priority levels:
| Priority | Max Concurrent | Example |
|----------|---------------|---------|
| CriticalMetadata | 8 | Directory operations |
| Desktop | 6 | UI file access |
| UserVisible | 4 | User file operations |
| Background | 2 | Cache flush |
| Cache | 1 | Prefetch, demotion |

Features: adjacent I/O batching, fairness window (100ms), background throttle (50%), NAND-aware block alignment (128 KB).

### Compression
| Algorithm | Use Case | Level |
|-----------|----------|-------|
| Zstd | Cache (large files > 1 MB) | Level 3 |
| LZ4 | Cache (small files ≤ 1 MB) | Default |
| Zstd | Write buffer | Level 3 |

Already-compressed formats (zip, png, jpg, mp4, etc.) detected by extension and stored as-is. Files < 64 KB skip compression.

### Device Management
- **Detection**: Polling at configurable interval (default 2000ms)
- **Eject**: Clean journal required, force flush on eject, configurable timeout
- **Surprise removal**: Handled gracefully, read-only fallback
- **Health checks**: Periodic device health verification

<br>

## CLI Commands

```bash
volt-usb-manager --mount          # Mount FUSE and run service
volt-usb-manager --unmount        # Unmount device
volt-usb-manager --status         # Print status and exit
volt-usb-manager --flush          # Flush write buffer
volt-usb-manager --eject          # Flush buffers and eject
volt-usb-manager --recover        # Recover journal and replay WAL
```

<br>

## Data Flow

### Read Path
1. Application `read()` on FUSE file
2. Check inode table for metadata
3. Lookup content in Read Cache by SHA-256(path + mtime + size)
4. **Cache hit**: Return data (decompress if compressed)
5. **Cache miss**: Return empty (real impl reads from USB)

### Write Path
1. Application `write()` on FUSE file
2. Update inode size, enqueue PendingWrite
3. WriteBuffer sends `ACK_BUFFERED`
4. Journal records `BeginWrite` + `CommitWrite`
5. Flush daemon batches writes → USB device
6. On `fsync`: journal checkpoint + `ACK_DURABLE`

### Recovery Path
1. Mount → check for `CleanShutdown` marker
2. No marker → RecoveryEngine replays WAL
3. Committed writes → applied to backing store
4. Incomplete writes → discarded
5. Recovery fails → read-only mode

<br>

## Configuration

See [`vum.toml`](../../config/vum.toml.md) for the complete configuration reference.

Key settings:
- Mount point, backing path
- Read cache and write buffer sizes
- Journal path and flush intervals
- Compression algorithms and levels
- I/O scheduler priorities and concurrency
- Device detection and eject behaviour

<br>

## Safety Invariants

| Invariant | Enforced By |
|-----------|------------|
| No dirty writes after clean shutdown | Safety check on mount |
| No ACK_DURABLE before journal commit | Writeback safety |
| No mount with unrecoverable journal | Recovery validation |
| No path escaping the backing store | Path traversal prevention |
