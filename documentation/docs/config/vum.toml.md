# VUM — USB Manager Configuration

**Path:** `/opt/volt/usb-manager/config.toml`

This configuration is defined in the **VOLT specification (ch.10 — VUM)**.

```toml
[usb]
enabled = true
auto_mount = true
poll_interval_ms = 2000

[cache]
read_cache_mb = 256
write_buffer_mb = 128
flush_interval_ms = 5000

[journal]
enabled = true
fsync_on_record = true
path = "/var/lib/samaris/volt-usb-manager/journal.wal"
```

<br>

## USB Settings

| Parameter | Default | Description |
|-----------|---------|-------------|
| `enabled` | `true` | USB manager active |
| `auto_mount` | `true` | Automatically mount inserted devices |
| `poll_interval_ms` | `2000` | USB device polling interval |

<br>

## Cache

| Parameter | Default | Description |
|-----------|---------|-------------|
| `read_cache_mb` | 256 | Read-ahead cache for USB storage |
| `write_buffer_mb` | 128 | Write buffer before flush to device |
| `flush_interval_ms` | `5000` | How often write buffer is flushed |

<br>

## Journal

| Parameter | Default | Description |
|-----------|---------|-------------|
| `enabled` | `true` | Journal active |
| `fsync_on_record` | `true` | Fsync after each record |
| `path` | `/var/lib/samaris/volt-usb-manager/journal.wal` | WAL journal path |

<br>

## Write Durability Modes

| Mode | Behavior |
|------|----------|
| **Journaled** | Records go through WAL journal with fsync |
| **Buffered** | Writes buffered in memory, flushed periodically |

<br>

## Related

- [VRM — RAM Manager Configuration](vrm.toml.md)
- [ASC — Adaptive System Config](asc.toml.md)
- [Filesystem Layout](../system/filesystem-layout.md)

<br>

---

[← Back: Documentation Index](../index.md)
