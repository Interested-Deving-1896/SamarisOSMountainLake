# 8. RAM Manager

## 8.1 Overview

The Volt RAM Manager (VRM) is the deterministic memory management subsystem of Samaris OS. Written in Rust as an independent daemon (`volt-ram-manager`), it provides adaptive memory compression, content-aware deduplication, per-application memory quotas, and pressure-responsive allocation policies. The VRM does not create physical RAM — it increases effective memory capacity through intelligent data management, enabling Samaris OS to remain responsive on memory-constrained hardware while fully exploiting abundant memory on capable systems.

## 8.2 Architecture

The VRM is structured around a pipeline:

```
Observer → Classifier → Compressor → Deduplicator → Quota Engine → Pressure Controller → GC Coordinator
```

### Observer

Monitors memory allocation patterns across the system. Tracks per-application allocations, deallocations, and access frequency. Operates on a configurable monitoring interval (default: 1000 ms).

### Classifier

Assigns memory pages to tiers based on access frequency and criticality:

| Tier | Name | Content | Management |
|------|------|---------|------------|
| T1 | Hot | Active application memory, visible UI | No compression, immediate access |
| T2 | Warm | Background application memory | Compressed with fast algorithm (LZ4 for binary, ZSTD level 1 for UI) |
| T3 | Cold | Inactive, wallpaper, caches | Compressed with ZSTD level 3+; candidates for swap |

Tier assignment is dynamic — pages migrate between tiers based on access patterns, with a configurable inactivity timeout (default: 5000 ms) before demotion.

### Compressor

Supports two compression algorithms selected per allocation context:

- **ZSTD**: general-purpose compression with configurable levels (1–5). Default level 3 with level 5 for archival data.
- **LZ4**: fast compression for binary buffers and latency-sensitive data.

Compression is optional and can be disabled per application quota. The compression subsystem runs on dedicated background threads.

### Deduplicator

Identifies identical memory pages using SHA-256 truncated to 64-bit hashes. When a match is found, the duplicate page is replaced with a reference to the canonical copy. Verification on match is configurable. Deduplication is most effective for:

- Repeated UI component data
- Common library text segments
- Identical file cache pages
- Application configuration blobs

### Quota Engine

Enforces per-application memory limits based on application type:

| Application | Quota (MiB) | Priority | Compression | Preferred Tier |
|-------------|-------------|----------|-------------|----------------|
| Desktop UI | 256 | Critical | No | T1 |
| Orbit AI | 2048 | Critical | No | T1 |
| Peregrine Browser | 512 | High | Yes | T2 |
| Finder (File Manager) | 256 | High | Yes | T2 |
| Photos | 256 | Normal | Yes | T2 |
| Music | 128 | Normal | Yes | T2 |
| Settings | 64 | Low | Yes | T2 |
| Wallpaper | 32 | Idle | Yes | T3 |
| Default (other apps) | 128 | Normal | Yes | T2 |

### Pressure Controller

The pressure controller implements a multi-level response system with configurable thresholds:

| Zone | Occupied RAM | System Response |
|------|-------------|-----------------|
| Green | ≤65% | Normal operation, no intervention |
| Yellow | 65–70% (entry), 65% exit | Thumbnail deferral, cache reduction |
| Orange | 70–85% (entry) / 80% exit | Compression of T2 pages, worker reduction |
| Red | ≥85% (entry) / 80% exit | Cache purge, background job suspension, aggressive compression |

The pressure monitor operates with configurable cooldown intervals to prevent thrashing.

### GC Coordinator

Coordinates garbage collection across subsystems. Respects animation guard to avoid triggering GC during frame rendering. Can be configured to operate only in orange pressure zones to minimise performance impact.

## 8.3 Shared Memory Integration

The VRM can allocate a shared memory (SHM) ring buffer for high-frequency telemetry exchange. The SHM size is configurable (default: 64 MiB). When available, the VRM publishes memory pressure events, quota utilisation, and compression statistics via SBP-MEM messages over the SHM channel.

## 8.4 Memory Tiers: Theoretical Gain

The effective memory multiplier depends on data compressibility, workload, and CPU capacity. Under typical desktop workloads with ZSTD level 3, a compression ratio of approximately 65% can be expected for compressible pages (background applications, caches, text data). This yields a theoretical effective memory of:

- 2 GB physical → ~5.7 GB effective
- 4 GB physical → ~11.4 GB effective
- 8 GB physical → ~22.8 GB effective

These figures are theoretical upper bounds. Real-world gains are workload-dependent. The VRM is designed to improve responsiveness under pressure rather than to provide equivalent capacity to additional physical RAM.

## 8.5 Configuration

The VRM is configured via `/opt/volt/ram-manager/config.toml`. Key configuration sections include:

- Pressure thresholds (green/yellow/orange/red boundaries)
- Compression algorithm selection and levels per data type
- Deduplication settings (enabled/disabled, hash algorithm)
- Per-application quota definitions
- Pool size classes for slab allocation
- GC behaviour (cooldown, animation guard, pressure-only mode)
