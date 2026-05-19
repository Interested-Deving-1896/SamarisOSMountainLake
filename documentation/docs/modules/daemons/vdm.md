# VDM — VOLT Display Manager

**Automatic display detection, configuration, and adaptation for Samaris OS**

The user should never manually configure the screen. Samaris OS must detect, understand, apply, persist, and notify display configuration automatically.

VDM is the display adaptation layer — it detects connected screens, computes optimal layouts, applies configuration through xrandr, watches for hotplug events, and recovers gracefully from bad configurations.

<br>

## Architecture

```
main.rs (CLI)
  └─ detect_and_apply()
       ├─ detector.rs    → detects screens via backends
       ├─ planner.rs     → plans layout from screens + user prefs
       ├─ applier.rs     → executes plan via backend
       ├─ validator.rs   → verifies applied state matches plan
       ├─ rollback.rs    → stores/restores last_known_good, safe mode
       ├─ persist.rs     → reads/writes runtime TOML + user TOML
       ├─ notifier.rs    → emits display events to Kernel A
       ├─ hotplug.rs     → watches for display changes
       ├─ debounce.rs    → coalesces rapid events
       └─ backends/
            └─ xrandr.rs → xrandr --query parser + command builder
```

<br>

## Data Flow

```
[Hardware] → detect() → Screen[]
    │
    ▼
[User Config] → plan() → DisplayConfig
    │
    ▼
[Backend] → apply() → writes xrandr commands
    │
    ▼
[Hardware] → detect() again → validate()
    │
    ├── OK → persist_runtime() → notify_kernel_a()
    │
    └── FAIL → rollback() → safe_mode() if rollback fails
```

<br>

## Components

### Detector
Scans connected displays via the active backend. For Alpha (Xorg), this means parsing `xrandr --query` output. Returns `Vec<Screen>` with:
- Screen name (e.g. `HDMI-1`, `eDP-1`)
- Native resolution and refresh rate
- Current mode, position, rotation
- Connected/disconnected status

### Planner
Computes the optimal layout given connected screens, user preferences, and layout mode:
- **Modes**: auto, single, mirror, extend
- Primary display selection: user preference > HDMI > DP > eDP > first connected
- Position assignment: extend left-to-right, mirror at origin
- Scale factor computation: resolution-based DPI classification
  - Native width ≥ 3840 → Retina (2.0x)
  - Native width ≥ 2560 → HiDPI (1.5x)
  - Otherwise → Standard (1.0x)

### Applier
Executes the planned configuration through the active backend. For xrandr: runs a sequence of `xrandr --output <name> --mode <WxH> --rate <R> --pos <XxY>` commands. Waits 500ms for X11 stabilisation before validation.

### Validator
Re-queries the display state and compares against the planned config. Returns mismatches per screen if validation fails.

### Rollback
Before applying a new config, the current state is snapshotted to `/run/samaris/display.lastgood.toml`. If apply/validate fails, this snapshot is restored. If rollback also fails, safe mode is activated (1280×720, 1.0x scale, single screen).

### Persister
Two separate config files:
- **Runtime** (`/run/samaris/display.generated.toml`): Regenerated on every apply
- **User** (`~/.config/samaris/display.toml`): Written by user or Settings app

### Hotplug
Two strategies:
1. **Primary**: udev DRM events → systemd user service trigger → debounce → full cycle
2. **Fallback**: 2-second xrandr poll loop detecting screen count changes

Preserves the primary display when possible; falls back to best available if primary disappears.

### Debouncer
300–800ms event coalescing. Rapid udev events during cable insertion/removal are collapsed into a single stable trigger.

### Notifier
Writes JSON events to `/run/samaris/display.event.json`:

| Event | Trigger |
|-------|---------|
| `display.ready` | Initial config applied at session start |
| `display.changed` | Hotplug or user-triggered reconfiguration |
| `display.failed` | Apply/validate failed |
| `display.rollback` | Rollback executed |
| `display.safe_mode` | Safe mode activated |

Kernel A / React UI poll or watch this file for changes.

<br>

## Generated Files

| Path | Purpose |
|------|---------|
| `/run/samaris/display.generated.toml` | Runtime display state, regenerated every apply |
| `/run/samaris/display.lastgood.toml` | Last known good config for rollback |
| `/run/samaris/display.event.json` | Latest display event for Kernel A / React UI |
| `~/.config/samaris/display.toml` | Persistent user preferences |

<br>

## CLI Commands

```bash
volt-display-manager --status          # Check current status
volt-display-manager --detect          # Detect screens (JSON output)
volt-display-manager --apply           # Full cycle: detect → plan → apply → validate → persist
volt-display-manager --safe            # Force safe mode (1280×720)
volt-display-manager --watch           # Start hotplug watcher
volt-display-manager --dump            # Debug dump
```

<br>

## Systemd Integration

```bash
systemctl --user enable volt-display-manager.service
systemctl --user enable volt-display-hotplug.service
systemctl --user start volt-display-manager.service
```

**Warning**: VDM must NOT run before Xorg is available. The systemd units are **user** services with `After=graphical-session.target`.

<br>

## Frontend Integration

```typescript
import { useDisplay } from "./useDisplay";

function App() {
  const { profile, isSafeMode, loading } = useDisplay();
  // profile.scale_factor → CSS --scale
  // profile.dpi_class → UI density tokens
}
```

<br>

## Known Limitations (Alpha)

- **Xorg / xrandr only** — Wayland and pure DRM/KMS are Beta
- **Physical DPI relies on resolution heuristics** — True EDID parsing is Beta
- **Touchscreen rotation** not supported
- **VRR/FreeSync/G-Sync** not supported
- **GPU vendor-specific features** (NVIDIA nvidia-settings, AMD) not integrated

<br>

## Troubleshooting

See `TROUBLESHOOTING.md` in `builder/content/volt-display-manager/` for complete troubleshooting guide covering:
- xrandr not found
- DISPLAY / XAUTHORITY missing
- Service launched too early
- HDMI not detected after hotplug
- Wrong scaling on HiDPI screens
- Black screen recovery
- Common error codes

<br>

## Acceptance Criteria

VDM is tested against a hardware matrix:

| Hardware | Best Resolution | Scaling |
|----------|----------------|---------|
| Mac Mini 2012 | 1080p @ 60Hz | 1.0x |
| Raspberry Pi 5 | 4K @ 60Hz | 2.0x |
| QEMU VM | 1024×768 | 1.0x (safe mode) |
| Dell U2415 | 1920×1200 | 1.0x |
| Samsung U28E590 | 3840×2160 | 2.0x |

See `ACCEPTANCE_CRITERIA.md` for full test criteria.
