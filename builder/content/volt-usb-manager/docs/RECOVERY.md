# Volt USB Manager — Recovery Document

## Journal Format

The journal is a Write-Ahead Log (WAL) stored in the `journal.path` directory as `wal.dat`.
Each record is binary-encoded with the following layout:

```
Offset  Size  Field
────────────────────────────────────────
0       4     Magic (0x4A524E4C = "JRNL")
4       1     Version (1)
5       1     Record Type (see below)
6       8     Record ID
14      8     Timestamp (µs since epoch)
22      4     Path Length (bytes)
26      N     Path (UTF-8)
26+N    4     Data Length (bytes)
30+N    M     Data
30+N+M  32    Data Hash (SHA-256)
62+N+M  4     Checksum (CRC32 of all above)
────────────────────────────────────────
```

### Record Types

| Byte | Type            | Description                         |
|------|-----------------|-------------------------------------|
| 1    | BeginWrite      | Start of a write transaction        |
| 2    | CommitWrite     | Commit a write transaction          |
| 3    | AbortWrite      | Abort a write transaction           |
| 4    | BeginDelete     | Start of a delete transaction       |
| 5    | CommitDelete    | Commit a delete transaction         |
| 6    | BeginRename     | Start of a rename transaction       |
| 7    | CommitRename    | Commit a rename transaction         |
| 8    | Checkpoint      | Journal checkpoint marker           |
| 9    | CleanShutdown   | Clean shutdown marker               |

## Replay Algorithm

The `RecoveryEngine::run()` function implements the following algorithm:

```
1. Open the WAL file for reading
2. Read all records into memory (verify checksums on each)
3. If no records, return immediately (nothing to replay)
4. Find "incomplete" transactions (Begin without Commit)
5. For each record:
   a. Skip incomplete transactions
   b. For CommitWrite: write BeginWrite's data to backing store
   c. For CommitDelete: remove the file from backing store
   d. For AbortWrite: remove any partial file from backing store
6. Return JournalReplay statistics
```

## Corruption Handling

| Corruption Type            | Detection                   | Behavior                       |
|----------------------------|-----------------------------|--------------------------------|
| Bad magic number           | `from_bytes()` magic check  | Record skipped, replay stops   |
| CRC32 checksum mismatch    | `verify_checksum()`         | Returns `JournalChecksumFailed`|
| Truncated record           | Length check                | Record skipped, replay stops   |
| Invalid record type byte   | `from_byte()`               | Returns `JournalCorrupt`       |
| Invalid UTF-8 in path      | `String::from_utf8`         | Returns `JournalCorrupt`       |

If any corruption is detected during replay, the recovery fails and the system
falls back to read-only mode.

## Dirty Journal

A journal is "dirty" when:
- It contains any records since the last checkpoint
- The most recent record is NOT a CleanShutdown marker
- The WAL file exists and is non-empty

Dirty journal detection:
- `Journal::is_dirty()` — true if records have been written since last checkpoint
- `RecoveryEngine::check_needed()` — true if WAL exists, is non-empty, and does not
  end with a CleanShutdown record

## Read-Only Fallback

If recovery fails, the system can enter `ReadOnlyFallback` state:
1. Recovery returns an error
2. FUSE mount is blocked if journal is unrecoverable
3. If `device.read_only_fallback` is true, the device is mounted read-only
4. No writes are permitted until recovery is manually resolved

## Manual Recovery

```bash
# Check if recovery is needed
volt-usb-manager --recover --config /etc/volt/usb-manager.toml

# Simulate recovery (no side effects)
volt-usb-manager --simulate-recovery

# Force recovery (replays WAL to backing store)
volt-usb-manager --recover
```
