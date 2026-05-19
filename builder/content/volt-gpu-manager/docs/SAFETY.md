# Safety

## Desktop Protection

Desktop frame resources (`GpuResourceUsage::DesktopFrame`) can never be
compressed. This invariant is enforced at two levels:

1. `GpuResourceMeta::compression_allowed` is always `false` for DesktopFrame usage
2. `InvariantChecker::check_no_desktop_compression()` validates at runtime

Desktop frames are considered critical for user experience and must remain
immediately bindable at all times.

## Scratch Budget Guarantees

Before any T2→T1 restore operation, `VramResidencyManager::restore_to_t1()`
checks the scratch budget:

```rust
fn can_restore(&self, current_free_vram: u64, resource_size: u64) -> bool {
    let needed = self.reserved_bytes + resource_size + self.min_free_bytes;
    current_free_vram >= needed
}
```

If insufficient scratch space is available, the restore is rejected with
`VgmError::ScratchBudgetInsufficient`. This guarantees that restore operations
will never cause VRAM exhaustion.

## No-Impossible-Restore

A resource cannot be restored if:
- It is not in T2 tier
- It does not exist in the compressed pool
- The scratch budget is insufficient
- T1 pool allocation fails

These checks prevent silent data loss or system instability from failed
restore operations.

## Permission Checks (SBP-GPU)

Every SBP-GPU opcode requires a specific permission level:

| Opcode Range | Required Permission |
|-------------|-------------------|
| GpuStatus, GpuThermalStatus, GpuVramStatus, GpuMetricsSnapshot | CAP_READ_STATUS |
| GpuAllocResource, GpuFreeResource, GpuCompressResource, GpuRestoreResource, GpuEvictResource | CAP_GPU_ALLOC |
| GpuExecCompute, GpuShaderCompile, GpuBatchSubmit, GpuPrefetchShaders | CAP_GPU_COMPUTE |
| GpuRenderFrame | CAP_GPU_RENDER |
| GpuSwitchDevice, Init, Shutdown, Reset | CAP_ADMIN_GPU / INTERNAL |

All permission checks return `VgmError::PermissionDenied` on failure.

## Backend Fallback Chain

When a GPU backend is unavailable:
1. If the configured backend fails, the system falls back to the next available
2. If no real GPU backend succeeds, `NullBackend` is used
3. `NullBackend` never fails — it is always available
4. The fallback is transparent to the application

## Thermal Backoff Cascade

Thermal backoff follows a progressive cascade:
1. **Hot**: Pause Idle priority jobs
2. **Throttle**: Pause Idle, reduce burst compute
3. **Critical**: Only Critical priority jobs execute
4. **Emergency**: All jobs blocked, CPU fallback activated
5. **Fatal**: Complete shutdown

Each level is strictly gated — a lower level never skips to a higher level's
behavior without passing through intermediate levels.

## Pointer Safety

The `verify_pointer` function in `safety::invariants` validates raw pointers
before use:
- Rejects null pointers
- Rejects suspiciously large sizes (>1 GB)
- Used throughout FFI boundary code
