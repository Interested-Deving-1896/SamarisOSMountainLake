# Volt USB Manager

**Samaris OS — Journaled RAM-First Removable Storage Manager**

VUM provides a high-performance, safe, FUSE-based filesystem for USB removable storage.
It uses a RAM-first architecture with a read cache, writeback buffer, journal (WAL),
configurable compression, and I/O scheduling — all in user space.

## Features

- **RAM-first**: Reads served from in-memory cache; writes buffered in RAM
- **Journaled**: Write-ahead log (WAL) with CRC32 checksums for crash recovery
- **FUSE filesystem**: Standard POSIX operations with no kernel modifications
- **Compression**: Zstd (large files) and LZ4 (small files) in the cache
- **I/O Scheduling**: Multi-level priority queue with fairness and batching
- **Safety**: `ACK_BUFFERED` / `ACK_DURABLE` semantics, path traversal prevention,
  clean eject protocol, surprise removal handling
- **SBP-USB protocol**: Binary management protocol with 16 opcodes
- **Metrics**: Comprehensive latency, throughput, and hit-ratio counters

## Build

```bash
# Build with default features (fuse, compression, journal, writeback)
cargo build --release

# Build with all features
cargo build --release --all-features

# Build without FUSE (headless/management mode)
cargo build --release --no-default-features
```

## CLI Commands

| Command              | Description                                    |
|----------------------|------------------------------------------------|
| `--mount`            | Mount the FUSE filesystem and run the service  |
| `--unmount`          | Unmount the device                             |
| `--status`           | Print current status and exit                  |
| `--flush`            | Flush write buffer to device                   |
| `--eject`            | Flush buffers and eject device                 |
| `--recover`          | Recover journal and replay records             |
| `--check-config`     | Load and validate configuration, then exit     |
| `--simulate-write`   | Simulate write operations in temporary dir     |
| `--simulate-recovery`| Simulate journal recovery in temporary dir     |
| `--version`          | Print version information                      |
| `--config <FILE>`    | Path to configuration file                     |

### Example Usage

```bash
# Start the service
volt-usb-manager --mount --config /etc/volt/usb-manager.toml

# Check status
volt-usb-manager --status

# Safe eject
volt-usb-manager --eject
```

## Configuration

See `config.example.toml` for a complete configuration file with all sections. Key paths:

```toml
[manager]
mount_point = "/mnt/volt_usb"
backing_path = "/var/volt/usb_backing"

[cache]
read_cache_max_mb = 256

[writeback]
buffer_max_mb = 64

[journal]
path = "/var/volt/journal"
```

## Architecture Overview

```
User Apps → FUSE Mount → VumFilesystem
                           ├── ReadCache (RAM)
                           ├── WriteBuffer (RAM)
                           ├── Journal (WAL, CRC32)
                           └── I/O Scheduler
                                   └── USB Device (Backing Store)
```

- **Reads**: Check cache → miss returns empty (real impl reads from USB via cache)
- **Writes**: Buffer in RAM → journal (WAL) → flush to USB → fsync → durable
- **Recovery**: On next mount, replay WAL to apply committed writes

## Example Mount Workflow

```bash
# 1. Create directories
sudo mkdir -p /mnt/volt_usb /var/volt/usb_backing /var/volt/journal

# 2. Copy example config
sudo cp config.example.toml /etc/volt/usb-manager.toml

# 3. Validate config
volt-usb-manager --check-config /etc/volt/usb-manager.toml

# 4. Mount
volt-usb-manager --mount

# 5. Use the filesystem
echo "hello" > /mnt/volt_usb/test.txt
cat /mnt/volt_usb/test.txt

# 6. Check metrics
volt-usb-manager --status

# 7. Eject safely
volt-usb-manager --eject
```

## Run Tests

```bash
# All tests
cargo test --all-features

# Specific test suite
cargo test --test config_tests
cargo test --test sbp_usb_tests
cargo test --test cache_tests
cargo test --test writeback_tests
cargo test --test journal_tests
cargo test --test recovery_tests
cargo test --test fuse_tests
cargo test --test scheduler_tests
cargo test --test compression_tests
cargo test --test eject_tests
cargo test --test invariants_tests

# Unit tests
cargo test --all-features --lib
```

## Run Benchmarks

```bash
cargo bench
cargo bench --bench read_cache_bench
cargo bench --bench write_buffer_bench
cargo bench --bench flush_bench
cargo bench --bench journal_bench
cargo bench --bench compression_bench
cargo bench --bench sbp_usb_bench
```

## Documentation

| Document              | Description                                  |
|-----------------------|----------------------------------------------|
| `docs/ARCHITECTURE.md`| System architecture and data flow diagram    |
| `docs/SPEC.md`        | RC 1.0 feature specification                 |
| `docs/SAFETY.md`      | Safety guarantees and invariants             |
| `docs/PERFORMANCE.md` | Performance characteristics and metrics      |
| `docs/RECOVERY.md`    | Journal format and recovery algorithm        |
| `docs/SBP_USB.md`     | SBP-USB binary protocol specification        |
| `docs/RECOVERY_PROCEDURE.md` | Admin recovery step-by-step procedure |
| `config.example.toml` | Complete example configuration file          |

## License

Samaris OS — Volt USB Manager. Internal use.
