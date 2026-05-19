# VOLT Display Manager

**One OS. Any screen. Zero manual tuning.**

The user should never manually configure the screen.
Samaris OS must detect, understand, apply, persist, and notify display configuration automatically.

---

## What VDM Is

VOLT Display Manager is the display adaptation layer of Samaris OS. It:

- Detects connected screens (HDMI, DisplayPort, eDP, VGA)
- Computes optimal resolution, refresh rate, position, and scaling
- Applies configuration through the active display backend (xrandr for Xorg)
- Persists runtime state and user preferences
- Watches for hotplug events and reconfigures automatically
- Notifies Kernel A / React UI of display state changes
- Recovers gracefully from bad configurations via rollback and safe mode

## Why It Exists

Before VDM, Samaris OS relied entirely on manual xrandr invocations, Electron's `screen.getAllDisplays()`, and a hardcoded 1920×1080 baseline in the UI scaling engine. There was no:

- Screen enumeration at the OS layer
- Multi-monitor layout management
- Hotplug detection
- HiDPI awareness
- Safe fallback from bad configurations
- Persistent user preferences

VDM fills every one of these gaps.

## Quick Start

```bash
# Build
cd builder/content/volt-display-manager
cargo build --release
sudo cp target/release/volt-display-manager /usr/local/bin/

# Check status
volt-display-manager --status

# Detect screens (JSON output)
volt-display-manager --detect

# Full cycle: detect -> plan -> apply -> validate -> persist
volt-display-manager --apply

# Force safe mode
volt-display-manager --safe

# Debug dump
volt-display-manager --dump

# Start hotplug watcher
volt-display-manager --watch
```

## Generated Files

| Path | Purpose |
|------|---------|
| `/run/samaris/display.generated.toml` | Runtime display state, regenerated every apply |
| `/run/samaris/display.lastgood.toml` | Last known good config for rollback |
| `/run/samaris/display.event.json` | Latest display event for Kernel A / React UI |
| `~/.config/samaris/display.toml` | Persistent user preferences |

## Enabling systemd Services

```bash
systemctl --user enable volt-display-manager.service
systemctl --user enable volt-display-hotplug.service
systemctl --user start volt-display-manager.service
systemctl --user start volt-display-hotplug.service
```

## Frontend Integration

```typescript
import { useDisplay } from "./useDisplay";

function App() {
  const { profile, isSafeMode, loading } = useDisplay();
  // profile.scale_factor → CSS --scale
  // profile.dpi_class → UI density tokens
}
```

## Known Limitations

- **Alpha only: Xorg / xrandr.** Wayland and pure DRM/KMS are Beta.
- **Physical DPI** relies on resolution heuristics (native width ≥ 3840 → Retina). True EDID-based physical DPI detection requires `xrandr --prop` parsing, which is a Beta feature.
- **Touchscreen rotation** not supported.
- **Variable refresh rate (VRR/FreeSync/G-Sync)** not supported.
- **GPU vendor-specific features** (NVIDIA nvidia-settings, AMD Catalyst) not integrated.

## Troubleshooting

See [TROUBLESHOOTING.md](TROUBLESHOOTING.md).
