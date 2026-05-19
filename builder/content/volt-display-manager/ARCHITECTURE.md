# VOLT Display Manager — Architecture

## Module Graph

```
main.rs (CLI)
  └─ lib.rs (detect_and_apply)
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

## Detector

Scans connected displays via the active backend. For Alpha (Xorg), this means parsing `xrandr --query` output. Returns a `Vec<Screen>`.

## Planner

Computes the optimal layout given:
- Connected screens (from detector)
- User preferences (from `~/.config/samaris/display.toml`)
- Layout mode (auto/single/mirror/extend)

Selects primary display (user preference > HDMI > DP > eDP > first connected).
Assigns positions (extend: left-to-right; mirror: all at origin).
Computes scale factors (resolution-based DPI classification).

## Applier

Executes the planned configuration through the active backend.
For xrandr: runs a sequence of `xrandr --output X --mode WxH --rate R --pos XxY` commands.
Waits 500ms for X11 stabilization before validation.

## Validator

Re-queries the display state and compares against the planned config.
Returns `Ok(())` if all screens match, or `ValidationFailed(mismatches)` with per-screen details.

## Rollback

Before applying a new config, the current state is snapshotted to `/run/samaris/display.lastgood.toml`.
If apply/validate fails, this snapshot is restored.
If rollback also fails, safe mode is activated.

## Persister

Two separate config files:
- **Runtime** (`/run/samaris/display.generated.toml`): TOML, regenerated on every apply.
- **User** (`~/.config/samaris/display.toml`): TOML, written by user or settings app.

## Hotplug

Two strategies:
1. **Primary**: udev DRM events → systemd user service trigger → debounce → full cycle.
2. **Fallback**: 2-second xrandr poll loop detecting screen count changes.

The hotplug watcher preserves the primary display when possible and falls back to the best available connected screen if the primary disappears.

## Debouncer

300–800ms event coalescing. Rapid udev events during cable insertion/removal are collapsed into a single stable trigger.

## Notifier

Writes JSON events to `/run/samaris/display.event.json`:
- `display.ready` — initial config applied at session start
- `display.changed` — hotplug or user-triggered reconfiguration
- `display.failed` — apply/validate failed
- `display.rollback` — rollback executed
- `display.safe_mode` — safe mode activated

Kernel A / React UI poll or watch this file for changes.

## Xorg Timing Warning

VDM must NOT run before Xorg is available. It requires:
- `DISPLAY` environment variable (e.g., `:0`)
- `XAUTHORITY` access to the session user's cookie
- xrandr binary in PATH

For this reason, the systemd unit is a **user** service with `After=graphical-session.target`, NOT an early-boot system service.

## Wayland Limitations

The Wayland backend is a stub for Alpha. Wayland detection is implemented (`WAYLAND_DISPLAY` env var), but the apply/plan cycle cannot use xrandr. In Wayland, VDM falls back to a virtual safe-mode screen. Full Wayland support (wlr-randr backend) is planned for Beta.
