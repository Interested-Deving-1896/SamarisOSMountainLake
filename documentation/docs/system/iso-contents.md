# ISO Contents

The Samaris OS ISO is a **bootable live image** based on Debian Trixie, containing two architecture variants in a single file.

<br>

## What's Included

| Category | Contents |
|----------|----------|
| **OS Base** | Debian Trixie (minbase), Linux kernel 6.x, systemd 256 |
| **Display** | Xorg, nodm (auto-login), custom Xsession |
| **Audio** | PipeWire, ALSA, PulseAudio compatibility |
| **Desktop Shell** | Electron-based Samaris desktop, Node.js kernel, React UI |
| **Rust Daemons** | VRM, DWP, VGM, VUM, Tesseract Engine (Kernel B), ASC |
| **AI Models** | Qwen3 1.7B (LLM), Whisper small (STT), OuteTTS 0.2 + WavTokenizer (TTS) |
| **Connectivity** | NetworkManager, bluez, blueman, iwd, wpasupplicant |
| **Drivers** | Broad WiFi/BT/GPU firmware coverage |

<br>

## What's NOT Included

- **Build toolchains** (CMake, Rust/Cargo, Node.js) — ISO is runtime-only
- **System browsers** (Firefox/Chromium) — use Peregrine built-in
- **Office suites** — Samaris is designed for AI-native workflows

<br>

## Dual Architecture

A single ISO contains both **x86_64** and **aarch64** kernels, initrds, and squashfs filesystems. GRUB detects your CPU architecture at boot and loads the right path.

```
/live/
├── x86_64/
│   ├── vmlinuz
│   ├── initrd.img
│   └── filesystem.squashfs
└── aarch64/
    ├── vmlinuz
    ├── initrd.img
    └── filesystem.squashfs
```

<br>

## ISO Size Breakdown

| Component | Size |
|-----------|------|
| AI Models | ~2.8 GB |
| Desktop + Kernel | ~0.6 GB |
| Base OS + Drivers | ~0.8 GB |
| **Total ISO** | **~4.2 GB** |

<br>

---

[← Back: Documentation Index](../index.md)
