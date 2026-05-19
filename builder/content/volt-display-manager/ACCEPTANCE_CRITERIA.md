# VOLT Display Manager — Acceptance Criteria

## Test Hardware Matrix

| Hardware | Output | Resolution | Refresh | Scaling | Primary | Status |
|----------|--------|------------|----------|---------|---------|--------|
| Mac Mini 2012 | HDMI | 1080p | 60Hz | 1.0x | HDMI-1 | ✅ |
| Raspberry Pi 5 | HDMI-0 | 4K | 60Hz | 2.0x | HDMI-0 | ✅ |
| QEMU VM | virtio | 1024×768 | 60Hz | 1.0x | Virtual-1 | ✅ (safe mode) |
| Dell U2415 | DP | 1920×1200 | 60Hz | 1.0x | DP-1 | ✅ |
| Samsung U28E590 | HDMI | 3840×2160 | 60Hz | 2.0x | HDMI-1 | ✅ |

## Criterion 1: Mac Mini 2012 — 1080p @ 60Hz

- [ ] `volt-display-manager --detect` returns 1 connected screen
- [ ] Screen name is `HDMI-1` or similar
- [ ] Native resolution 1920×1080
- [ ] Refresh rate 60Hz
- [ ] DPI class `standard`
- [ ] Scale factor 1.0
- [ ] `--apply` succeeds
- [ ] `/run/samaris/display.generated.toml` written
- [ ] `/run/samaris/display.event.json` contains `display.ready`

## Criterion 2: Raspberry Pi 5 — 4K @ 60Hz with Scaling

- [ ] Native resolution 3840×2160
- [ ] DPI class `retina`
- [ ] Scale factor 2.0
- [ ] `primary_logical_width` = 1920
- [ ] `primary_logical_height` = 1080
- [ ] `recommended_ui_density` ≥ 1.1
- [ ] `recommended_font_scale` ≥ 1.1
- [ ] UI renders correctly at 2x

## Criterion 3: QEMU Fallback

- [ ] When xrandr fails (no display), VDM returns virtual fallback screen
- [ ] Screen name is `FALLBACK`
- [ ] Resolution 1280×720
- [ ] Safe mode flag in profile
- [ ] UI renders correctly

## Criterion 4: HDMI Hotplug

- [ ] Unplugging HDMI triggers `display.changed` event within 5 seconds
- [ ] Primary screen preserved or re-selected
- [ ] No crash or panic
- [ ] Plugging HDMI back restores the original configuration
- [ ] Debouncer prevents duplicate apply cycles

## Criterion 5: Primary Screen Preservation

- [ ] With 2 screens (HDMI + DP), setting primary via user config persists across hotplug cycles
- [ ] If primary is physically disconnected, VDM falls back to the best available connected screen
- [ ] No primary = no crash

## Criterion 6: Rollback

- [ ] Intentionally apply an invalid config
- [ ] VDM detects validation failure
- [ ] VDM restores last known good config
- [ ] `display.rollback` event emitted
- [ ] UI remains functional

## Criterion 7: Safe Mode

- [ ] Force safe mode via `--safe` flag
- [ ] Config has `safe_mode: true`
- [ ] Resolution 1280×720 or 1024×768
- [ ] Single screen layout
- [ ] Scale 1.0
- [ ] `display.safe_mode` event emitted
- [ ] UI shows safe mode indicator

## Criterion 8: Runtime TOML

- [ ] `/run/samaris/display.generated.toml` exists after `--apply`
- [ ] Valid TOML syntax
- [ ] Contains all Screen fields
- [ ] Contains DisplayProfile
- [ ] Timestamp updated on re-apply

## Criterion 9: React Profile

- [ ] `useDisplay()` hook loads profile within 2 seconds
- [ ] `--vdm-scale` CSS custom property set on `<html>`
- [ ] `html[data-vdm-active="true"]` attribute present
- [ ] Safe mode indicator visible in safe mode

## Criterion 10: Zero Panic

- [ ] No `.unwrap()` panics in production paths
- [ ] All errors returned as `DisplayError` variants
- [ ] CLI exits with code 1 on error, never panics
- [ ] `RUST_BACKTRACE=1 volt-display-manager --detect` shows no unwrap panics
