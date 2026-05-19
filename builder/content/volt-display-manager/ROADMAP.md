# VOLT Display Manager — Roadmap

## Alpha (Current)

- [x] xrandr backend (Xorg)
- [x] Runtime TOML config (`/run/samaris/display.generated.toml`)
- [x] User preference TOML (`~/.config/samaris/display.toml`)
- [x] systemd user service (post-Xorg)
- [x] Hotplug detection (poll + udev trigger)
- [x] Event debouncing (500ms)
- [x] Rollback to last known good
- [x] Safe mode fallback (1280×720 @ 60Hz)
- [x] Single / Mirror / Extend layout modes
- [x] Primary display selection
- [x] Resolution detection (native preferred)
- [x] Refresh rate detection
- [x] Resolution-based HiDPI scaling
- [x] Rotation support (Normal/Left/Right/Inverted)
- [x] Kernel A / React UI notification (JSON file)
- [x] CLI diagnostics (--status, --detect, --dump, --apply, --safe, --watch)
- [x] Frontend TypeScript integration (useDisplay, displayProfile, display-tokens.css)

## Beta

- [ ] Wayland compositor backend (wlr-randr or compositor-specific)
- [ ] DRM/KMS direct probe (libdrm / `drm-rs`)
- [ ] Physical DPI detection via EDID `mm_width`/`mm_height`
- [ ] GUI display settings panel (integrated into Samaris Settings app)
- [ ] Per-screen color profile support
- [ ] Variable refresh rate (VRR/FreeSync/G-Sync) awareness
- [ ] HDR metadata passthrough
- [ ] Fractional scaling (1.25, 1.75)
- [ ] Display arrangement GUI (drag-to-reorder visual layout)
- [ ] SBP (Samaris Binary Protocol) bridge — replace JSON file with Unix socket

## Gamma

- [ ] Multi-GPU display routing (which GPU drives which screen)
- [ ] GPU hotplug (eGPU via Thunderbolt/USB4)
- [ ] Display stream compression (DSC) awareness
- [ ] Night light / blue light filter integration
- [ ] Power-aware display policy (dim unused screens, disable on battery)
- [ ] Touchscreen rotation sync
- [ ] Display calibration (ICC profiles)
- [ ] Virtual display support (for streaming/remote desktop)
- [ ] EDID override capability (for broken monitors)
