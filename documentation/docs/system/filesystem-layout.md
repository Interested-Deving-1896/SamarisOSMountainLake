# Filesystem Layout

The ISO's filesystem structure at runtime.

<br>

## Root Filesystem (`/`)

```
/ (squashfs root)
├── etc/
│   ├── systemd/system/         (Volt service units)
│   ├── default/nodm            (auto-login config)
│   └── chromium/policies/      (browser policies)
├── opt/volt/
│   ├── ai-models/              (LLM, STT, TTS models)
│   ├── bin/                    (Rust daemon binaries per arch)
│   ├── asc/config.toml
│   ├── kernel-b/config.toml
│   ├── ram-manager/config.toml
│   ├── worker-pool/config.toml
│   ├── gpu-manager/config.toml
│   ├── usb-manager/config.toml
│   ├── boot.html
│   ├── desktop/
│   │   ├── index.html          (UI entry)
│   │   └── app/                (built React bundle)
│   ├── kernel/                 (Node.js kernel + services)
│   └── electron/               (Electron app + preload)
├── home/user/Desktop/Demo/     (demo assets)
├── run/samaris/                (runtime sockets + state)
└── usr/local/bin/              (startup scripts)
```

<br>

## Runtime Sockets (`/run/samaris/`)

```
/run/samaris/
├── volt-kernel-b.sock          (Kernel B JSON-RPC)
├── volt-ram-manager.sock       (VRM SBP-MEM)
├── volt-worker-pool.sock       (DWP SBP)
├── volt-gpu-manager.sock       (VGM SBP-GPU)
├── volt-usb-manager.sock       (VUM SBP-USB)
└── adaptive.generated.toml     (ASC output)
```

<br>

## Persistent Data (`/var/lib/samaris/`)

```
/var/lib/samaris/
├── asc/
│   ├── last-hardware-profile.json
│   ├── last-generated-config.toml
│   └── last-explain-report.md
└── volt-usb-manager/
    └── journal.wal             (VUM write-ahead log)
```

<br>

---

[← Back: Documentation Index](../index.md)
