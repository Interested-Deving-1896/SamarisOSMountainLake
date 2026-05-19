# ISO Size Benchmark

## x86-64 FULL (5.1 GB)

**ISO:** Samaris-OS-Alpha-One-RC - x86-64 FULL.iso  

| Component | Size | % of total |
|-----------|------|-----------|
| filesystem.squashfs | 4.88 GB | ~95.6% |
| initrd.img | 99 MB | ~1.9% |
| vmlinuz (kernel) | 11.6 MB | ~0.2% |
| ISO overhead + bootloaders | ~100 MB | ~2.3% |
| **Total ISO** | **5.1 GB** | **100%** |

### Filesystem Breakdown

| Category | Estimated Size |
|----------|---------------|
| AI Models (Qwen3 + Whisper + OuteTTS) | ~2.8 GB |
| Desktop + Kernel (Node.js, Electron, React) | ~0.6 GB |
| Base OS + Drivers (Debian Trixie minbase, firmware) | ~0.8 GB |
| VOLT Rust daemons | ~0.1 GB |
| Additional packages + apps | ~0.5 GB |

---

## Universal (10.68 GB)

**ISO:** Samaris-OS-Alpha-One-RC-Universal.iso  

Dual-architecture ISO containing both x86_64 and aarch64 root filesystems.

| Component | Size |
|-----------|------|
| x86_64 squashfs + kernel + initrd | ~5.0 GB |
| aarch64 squashfs + kernel + initrd | ~5.0 GB |
| Shared bootloaders + ISO overhead | ~0.68 GB |
| **Total ISO** | **10.68 GB** |

---

## Comparisons

| Metric | x86-64 FULL | Universal |
|--------|-------------|-----------|
| Size | 5.1 GB | 10.68 GB |
| Architectures | x86_64 | x86_64 + aarch64 |
| Boot time (QEMU) | 47.0s | TBD |
| AI models | full | full |
| Use case | Intel/AMD desktops, laptops | Multi-arch deployment |
