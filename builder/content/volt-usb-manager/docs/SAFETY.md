# Volt USB Manager — Safety Document

## ACK_BUFFERED vs ACK_DURABLE

| Status      | Meaning                                           | When Sent                          |
|-------------|---------------------------------------------------|------------------------------------|
| BUFFERED    | Data is in RAM write buffer, NOT on device        | Immediately on `enqueue()`         |
| DURABLE     | Data is on the USB device (fsynced + journaled)   | After `fsync()` + checkpoint       |

**Safety rule**: Applications MUST NOT consider data persistent until `ACK_DURABLE` is received.
Reordering or accepting `ACK_BUFFERED` as durable can cause data loss on power failure
or surprise removal.

## Journal

- All mutations (write, delete, rename) are recorded in the WAL before being applied
- Every record has a CRC32 checksum covering the full record
- `BeginWrite` + `CommitWrite` pairs define an atomic write transaction
- Incomplete transactions (begin without commit) are NOT applied during recovery
- On clean shutdown, a `CleanShutdown` marker is written
- If the marker is absent on next mount, recovery is required

## Fsync

- `fsync` forces: (1) write buffer flush to USB, (2) journal checkpoint
- After `fsync` completes, all prior writes are durable
- Flush does NOT guarantee durability — only fsync+checkpoint does

## Clean Eject

1. Flush write buffer to USB device
2. Write journal checkpoint
3. Write `CleanShutdown` marker
4. Unmount the FUSE filesystem
5. Eject the USB device

## Surprise Removal

- Device disappears without clean eject
- Pending writes in buffer are lost
- Journal stays dirty — recovery will replay committed writes on next mount
- Cache is cleared (not a source of truth)
- Device transitions to `DeviceRemoved` or `ReadOnlyFallback` state

## Corruption Handling

| Scenario                           | Detection                         | Action                             |
|------------------------------------|-----------------------------------|------------------------------------|
| Corrupt WAL record (bad checksum)  | `from_bytes()` checksum verify    | Record skipped, recovery aborts    |
| Corrupt cache entry                | Compression checksum              | Entry evicted, re-read from device |
| Corrupt inode table                | In-memory only — lost on restart  | Rebuilt from backing store         |
| Invalid SBP message                | Magic + checksum verification     | Message rejected with error        |

## Path Validation

- All paths are checked against the backing store root
- `PathTraversalRejected` error if `..` or symlink escape is detected
- `..` in any path component is rejected
- Backing store canonicalization prevents symlink-based escapes

## Invariants Enforced

1. **No dirty writes after clean shutdown**: After `clean_shutdown()`, the journal must
   not be dirty. Violation indicates a bug or hardware fault.
2. **No ACK_DURABLE before commit**: An `ACK_DURABLE` must never be sent before the
   journal records the commit. This is enforced by the writeback ack ordering.
3. **No mount if unrecoverable journal**: If the journal is corrupted or in a
   `RecoveryRequired` state, FUSE mount is blocked.
4. **Cache is never the sole source**: The cache is always a performance optimization.
   All user data must be recoverable from the backing store + journal.
5. **No path escapes backing store**: All file paths are validated to remain within
   the configured backing storage path.
