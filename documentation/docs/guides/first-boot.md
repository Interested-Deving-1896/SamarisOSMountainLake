# First Boot

## Boot Process (6-Stage Model)

The boot follows the VOLT 6-stage sequence defined in VOLT Architecture Chapter 3:

1. **GRUB** — Detects your CPU architecture (x86_64 or aarch64) and loads the matching Linux kernel
2. **Linux Kernel** — Boots Debian Trixie base with live filesystem
3. **Plymouth Splash** — Samaris-branded boot animation
4. **Systemd Services** — Volt daemons initialize in order:
   - **ASC** (Adaptive System Config) — detects hardware, generates `/run/samaris/adaptive.generated.toml`
   - **Kernel B** — backend kernel starts on `ws://localhost:9998`
   - **VRM / VUM / VGM / DWP** — RAM manager, update service, GPU monitor, worker pool
   - **Kernel A** — frontend kernel starts on `ws://localhost:9999`
   - **Unifier** — Volt Unifier bridges Kernel A and Kernel B
   - **Desktop** — Electron shell loads with React Samaris UI
5. **Onboarding** — First-boot welcome flow (theme, WiFi, tour)
6. **Desktop Ready** — Fully interactive desktop

<br>

## Boot Timeline

```
BIOS → GRUB → Kernel (10.7s) → Userspace (37.8s) → Desktop Ready
```

Total: **~48 seconds** in QEMU VM (4 GB RAM, 4 vCPU). Times may vary on bare metal.

<br>

## What Happens at First Boot

1. **Adaptive Config** — ASC detects hardware and generates `/run/samaris/adaptive.generated.toml`
2. **AI Models** — Qwen3, Whisper, and OuteTTS models load into VRM-managed memory from `/opt/volt/ai-models/`
3. **Desktop** — The onboarding flow welcomes you with language selection, theme selection, and WiFi setup
4. **Orbit AI** — The local AI assistant is ready after the first ~8s Metal shader compilation

<br>

## Verifying the System

```bash
# Check kernel health
curl ws://localhost:9999

# Check service status
systemctl status volt-kernel.service

# Verify daemons
ls -la /run/samaris/

# View boot logs
journalctl -b | grep volt
```

<br>

## Troubleshooting

If the desktop doesn't appear after boot:
- Wait up to 60 seconds for first-time shader compilation (Metal/Vulkan)
- Check `journalctl -u volt-desktop.service` for errors
- Verify display hardware in `lspci | grep VGA`
- Ensure VRM has sufficient memory allocated (at least 2048 MB for LLM)

<br>

## Related

- [Getting Started](getting-started.md)
- [Installing the ISO](installing-iso.md)
- [Debugging Guide](debugging.md)
- [VOLT Architecture — Chapter 3: Boot Sequence](../../architecture/volt-ch3-boot.md)

<br>

---

[← Back: Documentation Index](../index.md)
