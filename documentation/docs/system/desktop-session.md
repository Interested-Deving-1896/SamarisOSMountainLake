# Desktop Session

The desktop session starts via a custom **Xsession script** that launches the Electron shell after all VOLT services are initialised.

<br>

## Verified Process Tree (QEMU)

```
nodm (943)
└─ Xorg :0 (944)
   └─ samaris-xsession
      └─ openbox
         └─ node /opt/volt/electron/node_modules/.bin/electron
            ├─ Electron main
            ├─ GPU process
            └─ Renderer
```

<br>

## Service Verification

| Endpoint | Status | Verified |
|----------|--------|----------|
| Kernel A WebSocket | `ws://localhost:9999` | ✅ |
| Kernel B JSON-RPC | `/run/samaris/volt-kernel-b.sock` | ✅ health OK |
| Unifier health | `/api/unifier/health` | ✅ "online" |
| FS service | `:3000/health` | ✅ |

<br>

## Boot Flow

```
lightdm (display manager)
    └── samaris-xsession
            ├── Xorg setup
            ├── start Rust daemons (ASC, Kernel B, VRM, DWP, VGM, VUM)
            ├── start Node.js kernel (server.js)
            └── launch Electron desktop
```

<br>

## Startup Scripts

| Script | Purpose |
|--------|---------|
| `samaris-xsession` | Xsession entry point |
| `volt-desktop` | Launches Electron app window |
| `start-tesseract.sh` | Kernel B daemon |
| `start-ram-manager.sh` | VRM daemon |
| `start-gpu-manager.sh` | VGM daemon |
| `start-worker-pool.sh` | DWP daemon |
| `start-usb-manager.sh` | VUM daemon |
| `start-volt-asc.sh` | ASC config generation |
| `volt-fs-service` | Filesystem HTTP bridge |
| `volt-unifier-watchdog` | Health monitor |

<br>

## Related

- [Systemd Services](systemd-services.md)
- [First Boot Guide](../guides/first-boot.md)
- [ISO Boot Benchmark](../../assets/benchmarks/iso-boot-qemu.md)

<br>

---

[← Back: Documentation Index](../index.md)
