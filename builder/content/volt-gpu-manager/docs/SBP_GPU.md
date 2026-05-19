# SBP-GPU Protocol Specification

## Overview

SBP-GPU is the inter-process GPU control protocol for Samaris OS. It enables
applications (Kernel B processes) to communicate GPU commands to the Volt GPU
Manager daemon.

## Message Format

### Header (36 bytes)

| Offset | Size | Field | Description |
|--------|------|-------|-------------|
| 0 | 4 | Magic | `0x47505542` (`"GPUB"` in ASCII) |
| 4 | 1 | Opcode | Operation code (see below) |
| 5 | 2 | Flags | Bit flags (Request=1, Response=2, Error=4) |
| 7 | 8 | Request ID | Unique request identifier |
| 15 | 8 | Timestamp | Microseconds since Unix epoch |
| 23 | 4 | Payload Length | Length of payload in bytes |
| 27 | 4 | Checksum | CRC32 of header fields + payload |
| 31 | 5 | Reserved | Zero-padded to 36 bytes total |

Total header size: 36 bytes. Payload follows immediately after.

### Checksum

CRC32 computed over: magic (4) + opcode (1) + flags (2) + request_id (8) +
timestamp (8) + payload_len (4) + reserved (4) + payload (N).

## Opcodes (15 GPU opcodes)

| Code | Name | Permission | Description |
|------|------|------------|-------------|
| 0x40 | GpuStatus | CAP_READ_STATUS | Query GPU status |
| 0x41 | GpuAllocResource | CAP_GPU_ALLOC | Allocate GPU resource |
| 0x42 | GpuFreeResource | CAP_GPU_ALLOC | Free GPU resource |
| 0x43 | GpuExecCompute | CAP_GPU_COMPUTE | Execute compute job |
| 0x44 | GpuRenderFrame | CAP_GPU_RENDER | Render a frame |
| 0x45 | GpuThermalStatus | CAP_READ_STATUS | Query thermal status |
| 0x46 | GpuSwitchDevice | CAP_ADMIN_GPU | Switch active GPU device |
| 0x47 | GpuShaderCompile | CAP_GPU_COMPUTE | Compile a shader |
| 0x48 | GpuVramStatus | CAP_READ_STATUS | Query VRAM status |
| 0x49 | GpuBatchSubmit | CAP_GPU_COMPUTE | Submit batch of commands |
| 0x4A | GpuPrefetchShaders | CAP_GPU_COMPUTE | Prefetch shaders to cache |
| 0x4B | GpuCompressResource | CAP_GPU_ALLOC | Compress a VRAM resource |
| 0x4C | GpuRestoreResource | CAP_GPU_ALLOC | Restore a compressed resource |
| 0x4D | GpuEvictResource | CAP_GPU_ALLOC | Evict resource to T3 |
| 0x4E | GpuMetricsSnapshot | CAP_READ_STATUS | Get metrics snapshot |

Legacy management opcodes (non-GPU): Init=0, Shutdown=1, Submit=2, Query=3, Reset=4.

## Permissions (5 levels)

| Permission | Value | Description |
|------------|-------|-------------|
| CAP_READ_STATUS | 0x01 | Read GPU status and metrics |
| CAP_GPU_ALLOC | 0x02 | Allocate/free GPU resources |
| CAP_GPU_COMPUTE | 0x04 | Execute compute workloads |
| CAP_GPU_RENDER | 0x08 | Execute render workloads |
| CAP_ADMIN_GPU | 0x10 | Administrative GPU operations |
| INTERNAL | 0x20 | Internal daemon operations |

## Error Codes

Errors are returned as response messages with the ERROR flag set and a
human-readable error string in the payload. All `VgmError` variants may be
returned (see `core::error::VgmError` for full list — 30+ error types).

## Response Format

Responses use the same header format with:
- Flags set to `RESPONSE` (and optionally `ERROR`)
- Same Request ID as the originating request
- Payload contains response data or error message

## Usage Example

```rust
use volt_gpu_manager::sbp_gpu::{SbpGpuMessage, SbpGpuOpcode};

// Client sends
let msg = SbpGpuMessage::new(SbpGpuOpcode::GpuStatus, vec![]);
let bytes = msg.to_bytes();
// ... send via IPC channel ...

// Server receives and responds
let decoded = SbpGpuMessage::from_bytes(&bytes)?;
let response = manager.handle_sbp(decoded);
let response_bytes = response.to_bytes();
```
