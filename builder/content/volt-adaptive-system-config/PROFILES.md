# Volt ASC Profiles

| Profile | Description |
|---------|-------------|
| `balanced` | Default. No modifications. |
| `performance` | +25% orbit quota, +25% max workers, +50% VUM cache |
| `powersave` | -25% orbit, -20% workers, shorter burst window |
| `safe` | -20% all quotas, -25% workers, strict VUM, safe_mode on |
| `debug` | Safe + verbose explain output |
| `vm` | Caps workers at 8, halves orbit quota, moderate VUM cache |
| `usb_boot` | 2x VUM cache, boot prefetch, WAL journal, 50ms flush |
| `low_ram` | Protects desktop, halves orbit/caches/workers |
