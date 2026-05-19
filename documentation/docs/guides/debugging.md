# Debugging

## Kernel Logs

```bash
# Follow kernel logs
journalctl -u volt-kernel.service -f

# View desktop service logs
journalctl -u volt-desktop.service

# All Volt daemon logs
journalctl -t volt-kernel-b
journalctl -t volt-ram-manager
journalctl -t volt-worker-pool
```

<br>

## All Volt Service Logs

```bash
# Kernel A (frontend, port 9999)
journalctl -u volt-kernel.service

# Kernel B (backend, port 9998)
journalctl -u volt-kernel-b.service

# VRM — Video/RRAM Manager
journalctl -u volt-vrm.service

# VUM — Update Manager
journalctl -u volt-vum.service

# VGM — GPU Monitor
journalctl -u volt-vgm.service

# DWP — Dedicated Worker Pool
journalctl -u volt-dwp.service

# ASC — Adaptive System Config
journalctl -u volt-asc.service

# Unifier — bridges Kernel A and Kernel B
journalctl -u volt-unifier.service

# Desktop — Electron shell
journalctl -u volt-desktop.service
```

<br>

## Developer Tools

The Electron renderer supports standard Chrome DevTools:

- **Open DevTools:** `Ctrl+Shift+I` or `Cmd+Option+I`
- **Console:** Kernel messages are gated to dev mode in `kernelClient.ts`
- **Network:** Inspect WebSocket communication on `ws://localhost:9999` and `ws://localhost:9998`

<br>

## Enabling Debug Mode

Set the `DEBUG` environment variable before launching:

```bash
DEBUG=1 volt-desktop
```

<br>

## Observability Endpoints

Refer to **VOLT Architecture Chapter 19** for the full observability specification, including:

- `/metrics` — Prometheus metrics for all Volt services
- `/health` — Health check endpoints per daemon
- `/debug/vars` — Exposed Go runtime vars for Kernel A and B

```bash
# Example: health check the unifier
curl http://localhost:9100/health

# Example: VRM memory usage metrics
curl http://localhost:9101/metrics | grep vrm
```

<br>

## Common Issues

| Issue | Check |
|-------|-------|
| Desktop not starting | `journalctl -u volt-desktop.service` |
| Kernel connection refused | `systemctl is-active volt-kernel.service` |
| AI models not loading | `ls -la /opt/volt/ai-models/` |
| WiFi not working | `iwconfig` or `nmcli device status` |
| GPU acceleration | Verify Metal/Vulkan driver presence |
| VRM allocation failure | `journalctl -u volt-vrm.service` |
| Unifier bridge down | `journalctl -u volt-unifier.service` |

<br>

## Related

- [First Boot Guide](first-boot.md)
- [VOLT Architecture — Chapter 19: Observability](../../architecture/volt-ch19-observability.md)
- [System Configuration Reference](../config/system-config.md)
- [Hardware Matrix](../system/hardware-matrix.md)

<br>

---

[← Back: Documentation Index](../index.md)
