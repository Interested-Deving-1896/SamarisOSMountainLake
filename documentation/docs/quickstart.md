# Quickstart — 5 Minutes with Samaris OS

Get the ISO running in QEMU and explore the desktop in under 5 minutes.

---

## 1. Download the ISO

Get the latest release from [samaris.tech](https://samaris.tech).

```bash
# Or on macOS with Homebrew
brew install qemu
```

## 2. Boot in QEMU

```bash
qemu-system-x86_64 \
  -m 4096 \
  -smp 4 \
  -cdrom Samaris-OS-Alpha-One-RC.iso \
  -vga virtio \
  -display cocoa
```

**Wait ~60 seconds** for the desktop to appear.

## 3. Log In

| Field | Value |
|-------|-------|
| Username | `user` |
| Password | `user` |

## 4. Explore

| Action | How |
|--------|-----|
| Launch apps | Click icons in the Dock |
| Open Finder | Browse filesystem, drag files to desktop |
| Open Settings | Change theme, appearance, security |
| Launch Orbit AI | `Cmd/Ctrl + Space` |
| Open Browser | Peregrine icon in the Dock |
| Open Terminal | Terminal icon in the Dock |
| Search files | `Cmd/Ctrl + Shift + Space` |

## 5. Shut Down

Click the power icon in the AirBar (top-right) → Shut Down.

---

## Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Cmd/Ctrl + Space` | Launch Orbit AI |
| `Cmd/Ctrl + Shift + Space` | Spotlight search |
| `Cmd + Q` (macOS) / `Alt + F4` (Linux) | Close focused window |

## Next Steps

- [Full getting started guide](guides/getting-started.md)
- [Installing to USB](guides/installing-iso.md)
- [First boot detailed walkthrough](guides/first-boot.md)
- [Complete documentation index](index.md)
