# VRM — Volt RAM Manager

**Deterministic Memory Manager for Samaris OS**

VRM is the memory management spine of Samaris OS. It manages physical RAM through a multi-tier architecture with adaptive compression, content-aware deduplication, per-application quotas, pressure-responsive allocation, and a coordinated garbage collection pipeline.

<br>

## Architecture

```
┌──────────────────────────────────────────────────────────────┐
│                      VRM — Engine Core                        │
│                                                               │
│  ┌─────────┐  ┌──────────┐  ┌──────────┐  ┌──────────────┐   │
│  │ VrmState │  │PageTable │  │ SbpRouter │  │MetricsRegistry│  │
│  └────┬────┘  └────┬─────┘  └────┬─────┘  └──────┬───────┘   │
│       │             │             │                │            │
│  ┌────▼─────────────▼─────────────▼────────────────▼────────┐  │
│  │                  VrmEngine (Orchestrator)                 │  │
│  └────────────────────────┬─────────────────────────────────┘  │
│                           │                                     │
│  ┌────────────────────────▼─────────────────────────────────┐  │
│  │                  VoltRamManager (Manager)                  │  │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────────┐│  │
│  │  │Allocator │ │Compressor│ │ Deduper  │ │GC Coordinator││  │
│  │  └──────────┘ └──────────┘ └──────────┘ └──────────────┘│  │
│  └──────────────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────────────┘
```

<br>

## Memory Tiers

VRM organises memory into three tiers with automatic promotion and demotion:

| Tier | Name | Compression | Access | Latency |
|------|------|------------|--------|---------|
| T1 | SHM (Hot) | None — direct mapping | Active app pages | ~0 µs |
| T2 | Direct (Warm) | LZ4 or ZSTD L1 | Recently accessed | <10 µs |
| T3 | Compressed (Cold) | ZSTD L3+ | Inactive >5s | <50 µs |

Transitions are managed by the tiers subsystem with residency tracking to prevent thrashing.

<br>

## Subsystems

### Allocator (`src/allocator/`)
- **small_alloc**: Fast-path allocations for pages ≤4 KB via slab pools
- **large_alloc**: Contiguous multi-page allocations with physical alignment
- **deallocator**: Delayed free with coalescing and quarantine
- **allocation_id / allocation_kind**: Typed allocation tracking with provenance
- **allocation**: Metadata structure per allocation (size, kind, tier, app_id)

### Pages (`src/pages/`)
- **page**: Core page structure with flags and metadata
- **page_id**: 64-bit unique page identifier
- **page_flags**: Dirty, pinned, compressed, dedup, write-protect bits
- **page_meta**: Access count, last touch timestamp, tier assignment
- **page_table**: Central page directory — maps virtual pages to physical tiers
- **access_tracker**: Per-page access frequency counter for tier promotion decisions
- **inactive_scanner**: Background scanner that detects cold pages for T3 demotion

### Compression (`src/compression/`)
- **algorithm**: Enum dispatch — ZSTD, LZ4, or none
- **levels**: Compression level presets per tier (T2: L1, T3: L3–L5)
- **lz4_backend**: Fast LZ4 compression for binary buffers
- **zstd_backend**: High-ratio ZSTD compression for cold pages
- **compressor**: Compression queue with async worker pool
- **decompression_queue**: Deferred decompression with LRU cache
- **compressed_page**: Storage format with header, algorithm tag, checksum
- **compression_queue**: Backpressure-aware compression pipeline
- **ratio_tracker**: Running compression ratio statistics per app

### Deduplication (`src/dedup/`)
- **fingerprint**: 64-bit truncated SHA-256 content fingerprint
- **hash**: Content hash computation with streaming support
- **dedup_table**: Global deduplication hash table (page → refcount)
- **ref_counter**: Atomic reference counting for shared pages
- **cow**: Copy-on-write mechanism for deduplicated pages
- **verifier**: Periodic integrity check — verifies dedup pages match fingerprints

### Pressure Management (`src/pressure/`)
- **level**: Four pressure zones — Green / Yellow / Orange / Red
- **thresholds**: Configurable usage thresholds per zone
- **monitor**: Periodic pressure sampling (default 1000ms)
- **actions**: Pressure-triggered actions per zone:
  - **Green** (<65%) : Normal operation
  - **Yellow** (≥70%) : Start inactive scanning, light compression
  - **Orange** (≥85%) : Aggressive compression, dedup activation, quota enforcement
  - **Red** (≥95%) : Emergency reclaim, T3 force-eviction, swap prevention
- **hysteresis**: Debounce logic to prevent oscillation between levels
- **cooldown**: Pressure cooldown timer after Red → Orange transition
- **emergency**: Emergency falling knife handler — immediate reclaim

### Quotas (`src/quotas/`)
- **app_quota**: Per-application memory budget (max bytes, page limit)
- **quota_table**: Global quota registry (app_id → AppQuota)
- **governor**: Quota enforcement engine — alloc/dealloc hooks, over-limit handling
- **policy**: Quota allocation policy (fixed, proportional, dynamic)
- **priority**: Priority levels mapped to quota classes
- **enforcement**: Runtime quota checking with soft/hard limits
- **errors**: Typed quota errors (OverLimit, Violation, NotFound)

### Garbage Collection (`src/gc/`)
- **coordinator**: Central GC orchestrator — coordinates all GC subsystems
- **v8_signal**: V8/JavaScript engine memory pressure signal bridge
- **animation_guard**: Prevents GC during UI animation frames
- **cooldown**: Minimum interval between GC cycles
- **report**: GC cycle report — freed bytes, duration, pages reclaimed

### Applications (`src/apps/`)
- **app_id**: Unique application identifier (UUID v4)
- **app_profile**: Memory profile per app (priority, tier preference, quota class)
- **app_memory**: Per-app memory usage tracking
- **app_state**: Application lifecycle state (active, suspended, terminated)
- **app_events**: Event bus for memory-related app events
- **registry**: Global application registry

### Safety (`src/safety/`)
- **SafetyGuard**: Pointer validation, range checking, zero-size detection
- **Canary**: Stack canary with 0xDEADBEEF_CAFEBABE pattern
- **MemoryBarrier**: Atomic fence operations (acquire, release, full)

### SBP-MEM Protocol (`src/sbp_mem/`)
- **opcode**: Memory-specific SBP opcodes (MEM_PRESSURE, MEM_QUOTA, etc.)
- **message**: Typed message structures with CRC32 checksums
- **handler**: Message handler dispatch
- **response**: Response message encoding
- **router**: Asynchronous message routing with timeouts

### Additional Subsystems
- **boot**: Boot-time initialisation sequence
- **cache**: Page cache with LRU eviction
- **config**: TOML-based configuration with validation
- **core**: VrmEngine, VoltRamManager, VrmState, error/result types
- **metrics**: Performance counters, latency histograms, event counters
- **platform**: Platform-specific memory operations (Linux / macOS / BSD)
- **policy**: Memory allocation and reclamation policies
- **pools**: Memory slab pools for small allocations
- **runtime**: Thread pool and async runtime for background operations
- **scheduler**: Memory task scheduler with priority queue
- **shm**: POSIX shared memory segment management

<br>

## Configuration

See [`vrm.toml`](../../config/vrm.toml.md) for the complete configuration reference.

Key settings:
- Memory tiers pool sizes
- Compression algorithm and levels per tier
- Deduplication block size and enabled/disabled
- Pressure thresholds and cooldown intervals
- Per-app quotas and priority levels
- GC coordination intervals

<br>

## SBP-MEM Protocol

VRM extends the standard SBP protocol with memory-specific opcodes:

| Opcode | Name | Purpose |
|--------|------|---------|
| 0x03 | MEM_PRESSURE | Memory pressure level notification |
| 0x04 | MEM_QUOTA | Per-application quota state |
| 0x?? | PAGE_STATUS | Tier assignment query |
| 0x?? | COMPRESSION_STATS | Compression ratio report |
| 0x?? | DEDUP_STATS | Deduplication ratio report |

Communication flows through the Volt Unifier's [`vrmClient`](../../architecture/volt-unifier.md) bridge.

<br>

## Build & Run

```bash
# Build
cd builder/content/volt-ram-manager
cargo build --release

# Run with default config
cargo run --release

# Run with custom config
cargo run --release -- --config /path/to/config.toml

# Run tests
cargo test --all-features

# Run benchmarks
cargo bench
```

<br>

## Benchmarks

| Benchmark | Description |
|-----------|-------------|
| alloc_bench | Small/large allocation throughput |
| compression_bench | ZSTD and LZ4 compression speed |
| decompression_bench | Decompression latency |
| dedup_bench | Deduplication throughput |
| pressure_bench | Pressure simulation and reclaim speed |
| sbp_mem_bench | SBP-MEM message round-trip latency |
