# Changelog

## Alpha One — Mountain Lake (2026-05-15)

**The first public release of Samaris OS — a bootable Native WebOS prototype.**

### Highlights
- First bootable ISO: boots into full Electron + React desktop
- 26 built-in applications (Finder, Terminal, Browser, Mail, Music, etc.)
- Orbit AI — fully local Qwen3 1.7B LLM + Whisper STT + OuteTTS TTS
- 9 Rust daemons managing RAM, GPU, USB, scheduling, display, adaptive config
- Dual-kernel architecture: Node.js (33 services) + Rust (Tesseract Engine)
- Volt Unifier: 8 bridges, 11 IPC clients connecting desktop to system
- Modular ISO build system: 26 build steps, 10 Debian module layers
- OverlayFS persistence across reboots
- Plymouth branded boot splash
- Benchmark suite with 0–10,000 scoring

### Known Issues
- `gpasswd: Permission denied` for groups `video` and `input` on first boot (non-critical)
- `dmesg` not accessible to unprivileged user
- Display manager: Xorg/xrandr only, no Wayland support
- AI models not pre-installed in ISO (requires `--ai-assets` build flag)
- WebSocket rate limit: 60 msg/s/client

### Documentation
- Complete 24-chapter VOLT specification
- Architecture: 12 documents covering full stack
- 26 app docs, 17 module/system docs, 5 API references, 7 guides
- 10 Mermaid architecture diagrams
- QEMU boot benchmark: 47.0s boot time, 0 panics
