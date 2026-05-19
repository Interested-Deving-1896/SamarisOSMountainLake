# Volt USB Manager — Performance Document

## Honest Explanations

### Cache Hit vs USB Read

Reading from the in-RAM cache is **orders of magnitude faster** than reading from a USB device.
Typical USB 3.0 sequential read: ~200 MB/s. RAM read: ~20 GB/s. However, the cache is limited
to `read_cache_max_mb` (default 256 MB). Large working sets that exceed the cache will
experience frequent misses and evictions.

**Trade-off**: A larger cache improves hit rate but consumes RAM that other processes need.
The 256 MB default is a reasonable balance for a desktop system.

### Flush Limits

Flushing to USB is the bottleneck. Key constraints:
- **USB write speed**: Typically 30-100 MB/s for consumer USB 3.0 drives
- **Batch size**: `batch_size_kb` (default 512 KB) — larger batches improve throughput
  but increase latency for any single write
- **Flush interval**: `flush_interval_ms` (default 5000 ms) — longer intervals batch
  more writes but risk data loss on surprise removal
- **Background throttle**: 50% default means background I/O gets at most half the
  available USB bandwidth

**Expected throughput**: With default settings, expect ~40 MB/s sustained write throughput.
Metadata-heavy workloads will be slower due to fsync overhead.

### Batching Effect

Batching small writes into larger USB transfers dramatically improves throughput.
A 4 KB random write pattern might achieve only 1 MB/s without batching.
The same workload batched into 512 KB chunks can reach 30-50 MB/s.

### How to Run Benchmarks

```bash
# All benchmarks
cargo bench

# Specific benchmark
cargo bench --bench read_cache_bench
cargo bench --bench write_buffer_bench
cargo bench --bench flush_bench
cargo bench --bench journal_bench
cargo bench --bench compression_bench
cargo bench --bench sbp_usb_bench

# With all features
cargo bench --all-features
```

## Exposed Metrics

All metrics are available via the `MetricsSnapshot` struct, accessible through:
- SBP-USB `UsbStatus`/`UsbMetricsSnapshot` opcodes (JSON payload)
- `VoltUsbManager::snapshot()` method
- `volt-usb-manager --status` CLI command

### Key Performance Metrics

| Metric                     | Description                                  | Unit   |
|----------------------------|----------------------------------------------|--------|
| `cache_hit_ratio`          | Fraction of reads served from cache          | 0.0-1.0 |
| `cache_hit_count`          | Total cache hits                             | count  |
| `cache_miss_count`         | Total cache misses                           | count  |
| `cache_eviction_count`     | Number of entries evicted                    | count  |
| `pending_write_count`      | Writes in buffer not yet flushed             | count  |
| `dirty_bytes`              | Bytes in buffer not yet on device            | bytes  |
| `flush_count`              | Successful flush operations                  | count  |
| `flush_error_count`        | Failed flush operations                      | count  |
| `last_flush_duration_us`   | Duration of most recent flush                | µs     |
| `avg_read_latency_us`      | Average read latency (from FUSE perspective) | µs     |
| `avg_write_latency_us`     | Average write latency                        | µs     |
| `avg_flush_latency_us`     | Average flush latency                        | µs     |
| `compression_count`        | Compressions performed                       | count  |
| `decompression_count`      | Decompressions performed                     | count  |
| `compression_saved_bytes`  | Bytes saved by compression                   | bytes  |
| `journal_records`          | Total journal records written                | count  |
| `journal_bytes`            | Total journal bytes written                  | bytes  |
| `ack_buffered_count`       | ACK_BUFFERED events sent                     | count  |
| `ack_durable_count`        | ACK_DURABLE events sent                      | count  |
| `eject_count`              | Successful eject operations                   | count  |

### Optimizing Performance

1. **Increase cache size** if working set exceeds 256 MB and RAM is available
2. **Reduce flush interval** for lower latency at the cost of more USB writes
3. **Increase batch size** for better throughput on large sequential writes
4. **Disable compression** on fast machines with slow CPUs (unlikely — zstd is very fast)
5. **Tune background throttle** higher (e.g., 80%) if foreground I/O is not latency-sensitive
