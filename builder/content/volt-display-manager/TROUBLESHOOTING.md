# VOLT Display Manager — Troubleshooting

## xrandr not found

**Symptom:** `VDM ERROR: xrandr binary not found in PATH`

**Cause:** The `xrandr` package is not installed.

**Fix:**
```bash
sudo apt install x11-xserver-utils
# or
sudo apt install xrandr
```

## DISPLAY missing

**Symptom:** `VDM ERROR: X11 session unavailable: DISPLAY environment variable not set`

**Cause:** VDM was run outside an Xorg session (e.g., from SSH, from a TTY, or before the graphical session started).

**Fix:**
```bash
# Check if DISPLAY is set
echo $DISPLAY

# If blank, ensure you're inside an Xorg session
# VDM must be run as a user service, not as root or from a TTY

# Correct: 
systemctl --user start volt-display-manager.service

# Incorrect:
sudo volt-display-manager --apply
```

## XAUTHORITY missing

**Symptom:** `VDM ERROR: Permission denied accessing display`

**Cause:** The user running VDM does not have permission to access the X server. This usually happens when running as root or with `sudo`.

**Fix:**
- Run VDM as the session user, not as root.
- Ensure `XAUTHORITY` is set: `export XAUTHORITY=$HOME/.Xauthority`
- If using `sudo`, use `sudo -E` to preserve environment variables (not recommended).

## Service launched too early

**Symptom:** systemd journal shows VDM failing with `SessionUnavailable` on boot.

**Cause:** The service unit ran before the Xorg session was fully initialized.

**Fix:**
```bash
# Verify the unit has the correct ordering
systemctl --user cat volt-display-manager.service

# Should contain:
# After=graphical-session.target

# If the issue persists, add a small delay:
# ExecStartPre=/bin/sleep 2
```

## HDMI not detected after hotplug

**Symptom:** Plugging in HDMI does not trigger reconfiguration.

**Troubleshooting steps:**
```bash
# 1. Check if xrandr sees the screen
xrandr --query | grep HDMI

# 2. Manually trigger detection
volt-display-manager --detect

# 3. Check hotplug watcher is running
systemctl --user status volt-display-hotplug.service

# 4. Check udev is emitting DRM events
udevadm monitor --subsystem=drm
# Then plug/unplug HDMI. Events should appear.

# 5. If no udev events, start the watcher manually
volt-display-manager --watch
```

## Wrong scaling on HiDPI screens

**Symptom:** UI elements are too small or too large on a 4K / Retina display.

**Fix:**
```bash
# Check what VDM detected
volt-display-manager --detect | grep -E "name|scale|dpi|native"

# If scale_factor is wrong, set a user override:
mkdir -p ~/.config/samaris
cat > ~/.config/samaris/display.toml << 'EOF'
[preferences]
primary = "HDMI-1"

[preferences.screens.HDMI-1]
scale_factor = 2.0
EOF

# Re-apply
volt-display-manager --apply
```

## Black screen recovery

**Symptom:** After applying a display config, all screens go black.

**Recovery steps:**
```bash
# 1. Switch to a TTY: Ctrl+Alt+F2
# 2. Log in as the session user
# 3. Run safe mode:
volt-display-manager --safe

# 4. Switch back to Xorg: Ctrl+Alt+F7
# The display should be at 1280×720.

# 5. Diagnose:
volt-display-manager --dump
volt-display-manager --status

# 6. Re-apply with corrections:
# (after fixing the user config or resolution issue)
volt-display-manager --apply
```

## Force safe mode at boot

**Symptom:** Need to boot with minimal display configuration for recovery.

**Fix:**
```bash
# Create a flag file that VDM checks (optional enhancement)
touch /run/samaris/display.force-safe

# Or manually run safe mode before the desktop starts:
volt-display-manager --safe
```

## Check current runtime state

```bash
# View runtime config
cat /run/samaris/display.generated.toml

# View last event
cat /run/samaris/display.event.json

# View user preferences
cat ~/.config/samaris/display.toml
```

## Enable debug logging

```bash
RUST_LOG=debug volt-display-manager --apply
RUST_LOG=trace volt-display-manager --watch
```

## Common Error Codes

| Error | Meaning | Action |
|-------|---------|--------|
| `SessionUnavailable` | No X11/DISPLAY | Run inside graphical session |
| `XrandrNotFound` | xrandr not installed | `apt install xrandr` |
| `PermissionDenied` | XAUTHORITY missing | Run as session user, not root |
| `ParseError` | Corrupt xrandr output | Run `xrandr --query` manually to check |
| `NoScreensDetected` | No connected displays | Check cables, GPU drivers |
| `ValidationFailed` | Apply didn't stick | Check for GPU driver issues |
| `RollbackFailed` | Couldn't restore | Run `--safe` to force fallback |
