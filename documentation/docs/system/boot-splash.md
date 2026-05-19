# Boot Splash

A custom **Plymouth** boot splash theme with a 36-frame animation sequence, providing visual feedback while system services initialise.

<br>

## Theme Files

```
/usr/share/plymouth/themes/
├── samaris/
│   ├── animation-0001.png  through  animation-0036.png
│   ├── background.png
│   ├── logo.png
│   ├── samaris.plymouth
│   └── samaris.script
└── volt/
    ├── volt.plymouth
    └── volt.script
```

<br>

## Display

The theme is displayed from **GRUB handoff** until the desktop session starts, spanning the 6-stage VOLT boot sequence:
- Stage 0: Linux kernel boot (10.7s)
- Stage 1: ASC — hardware detection and policy generation (0.5s)
- Stage 2: Tesseract Engine initialisation (12.8s)
- Stage 3: Volt subsystem initialisation — VRM, VGM, VUM, DWP
- Stage 4: Kernel A orchestrator — WebSocket server on port 9999
- Stage 5: Desktop shell — Electron launch

<br>

## Related

- [Systemd Services](systemd-services.md)
- [Desktop Session](desktop-session.md)

<br>

---

[← Back: Documentation Index](../index.md)
