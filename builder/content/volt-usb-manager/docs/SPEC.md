# Volt USB Manager — RC 1.0 Feature Specification

## 1. FUSE Operations

| Operation   | Description                            | Journaled | Cache Interaction |
|-------------|----------------------------------------|-----------|-------------------|
| lookup      | Resolve filename to inode              | No        | No                |
| getattr     | Get file/directory attributes          | No        | No                |
| read        | Read file contents                     | No        | Cache hit/miss    |
| write       | Write data to file                     | Yes       | Invalidates cache |
| create      | Create a new regular file              | Yes       | No                |
| mkdir       | Create a new directory                 | Yes       | No                |
| unlink      | Remove a file                          | Yes       | Evicts from cache |
| rename      | Move/rename a file or directory        | Yes       | Evicts from cache |
| readdir     | List directory entries                 | No        | No                |
| flush       | Close file descriptor                  | No        | Triggers flush    |
| fsync       | Synchronize file to backing store      | Yes       | Forces checkpoint |
| setattr     | Change file attributes (mode, size)    | No        | No                |

## 2. Cache

- **Capacity**: Configurable via `cache.read_cache_max_mb` (default 256 MB)
- **Eviction**: LRU, triggers at `cache.evict_at_percent` (default 90% full)
- **Compression**: Zstd for files > 1 MB, LZ4 for files ≤ 1 MB; skip already-compressed
  formats (png, jpg, mp4, zip, gz, etc.)
- **Pinning**: Boot assets and desktop assets can be pinned (never evicted)
- **Keying**: SHA-256(path + mtime + size) — deterministic, no collision risk
- **Concurrency**: DashMap with lock-free reads

## 3. Writeback

- **Buffer**: Configurable via `writeback.buffer_max_mb` (default 64 MB)
- **ACK semantics**: `ACK_BUFFERED` on enqueue, `ACK_DURABLE` after fsync/checkpoint
- **Flush trigger**: At `writeback.flush_at_percent` (default 80%) or every
  `writeback.flush_interval_ms` (default 5000 ms)
- **Batch size**: `writeback.batch_size_kb` (default 512 KB) per flush
- **Durability modes**: "balanced" (default), "performance", "maximum"
- **Metadata priority**: Metadata writes are always flushed before data writes

## 4. Scheduler

- **Priority levels**: CriticalMetadata (0), Desktop (1), UserVisible (2), Background (3), Cache (4)
- **Concurrency limits**: Metadata=8, Desktop=6, UserVisible=4, Background=2, Cache=1
- **Fairness window**: Configurable `scheduler.fairness_window_ms` (default 100 ms)
- **Batching**: Adjacent I/O to same file + same priority merged into single operation
- **Background throttle**: `scheduler.background_throttle` (default 50%) limits background I/O
- **NAND-aware**: Block-aligned writes via `scheduler.nand_block_kb` (default 128 KB)

## 5. Compression

| Algorithm | Use Case              | Config Key                      | Default |
|-----------|-----------------------|----------------------------------|---------|
| Zstd      | Cache (large files)   | `compression.read_cache_algorithm` | "zstd" |
| LZ4       | Cache (small files)   | `compression.write_cache_algorithm` | "lz4"  |
| Zstd      | Write buffer          | `compression.zstd_level`         | 3       |

- Files < `compression.small_file_threshold_kb` (default 64 KB) skip compression
- Already-compressed formats are detected by extension and stored as-is

## 6. Device Management

- **Detection**: Polls at `device.detect_interval_ms` (default 2000 ms)
- **Removable check**: `device.require_removable` (default true)
- **Surprise removal**: `device.handle_surprise_removal` (default true)
- **Read-only fallback**: `device.read_only_fallback` (default true)
- **Eject**: Requires clean journal (`eject.require_clean_journal`), force flushes
  (`eject.force_flush`), configurable timeout (`eject.timeout_ms`, default 10000 ms)
