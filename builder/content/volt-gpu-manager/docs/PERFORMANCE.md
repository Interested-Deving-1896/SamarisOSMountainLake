# Performance

## Measured Metrics

All metrics are tracked through `GpuMetricsSnapshot`:

### Compression

- `compression_count`: Number of compression operations performed
- `decompression_count`: Number of decompression operations
- `compression_saved_bytes`: Total bytes saved by compression
- `average_compression_ratio`: Average ratio across all compressions
- `compression_latency_avg_us`: Average latency of compression
- `restore_latency_avg_us`: Average latency of decompression

### Shader Cache

- `shader_cache_entries`: Number of cached shaders
- `shader_cache_hit_count`: Cache hit count
- `shader_cache_miss_count`: Cache miss count
- `shader_compile_count`: Number of compilations
- `shader_compile_error_count`: Number of compilation failures

### Frame Timing

- `frame_count`: Total frames processed
- `average_frame_time_ms`: Rolling average frame time
- `frame_budget_miss_count`: Frames exceeding budget

### Fallback

- `fallback_count`: Count of CPU fallback activations

## Honest Claims

VGM makes no fake guarantees about performance. All claims are based on
measured data from the actual compression backend in use:

1. **Compression ratios** are tracked per-operation and reported as averages
2. **Frame budgets** are measured against wall-clock time
3. **Shader cache hit rates** are real counts from actual lookups
4. **Latency figures** are measured in microseconds from real operations
5. **No "synthetic" or "estimated" metrics** are ever reported

## Typical Performance Characteristics

Measured from CPU fallback backend (zstd, single thread):
- 64 KB buffer compress: ~50-100 µs
- 64 KB buffer decompress: ~20-40 µs
- 1 MB buffer compress: ~500-1500 µs
- 1 MB buffer decompress: ~150-400 µs
- Compression ratio (zstd, level 3): typically 2:1 to 4:1 for textures

GPU-accelerated compression (when available via Vulkan/Metal) is expected to be
5-10x faster but ratios may be similar.

## Running Benchmarks

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench --bench shader_cache_bench
cargo bench --bench scheduler_bench
cargo bench --bench resource_table_bench
cargo bench --bench compression_bench
cargo bench --bench restore_bench
cargo bench --bench sbp_gpu_bench
```

Benchmarks use Criterion.rs for statistical measurement.
Run with `-- --verbose` for detailed per-iteration timing.
