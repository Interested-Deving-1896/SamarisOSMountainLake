# Boot Splash & Plymouth Theme

**Visual boot experience for Samaris OS**

Samaris OS uses Plymouth to provide a branded graphical boot splash that hides kernel boot messages and presents a clean, animated startup experience.

<br>

## Pipeline

```
GRUB (menu) → Linux kernel → initrd → Plymouth splash → systemd → desktop
                                    │
                                    ▼
                            volt-boot.service
                            (Plymouth client)
                                    │
                                    ▼
                            volt-kernel.service
                            (Kernel A + daemons)
                                    │
                                    ▼
                            Plymouth quit → Desktop UI
```

<br>

## Theme Location

The Plymouth theme lives at `builder/content/theme/plymouth/`. During the build, step `07-boot-theme` installs it into the overlay filesystem.

- Source: `builder/content/theme/plymouth/`
- Target in ISO: `/usr/share/plymouth/themes/samaris/`
- Theme name registered: `samaris`

The theme is configured as the default Plymouth theme in the initramfs via `/etc/plymouth/plymouthd.conf`.

<br>

## Theme Components

| File | Description |
|------|-------------|
| `logo.png` | Samaris OS logo watermark (centred or positioned) |
| `box.png` | Background box for status text |
| `bullet.png` | Progress indicator bullet |
| `entry.png` | Input field background (for LUKS password prompt) |
| `samaris.plymouth` | Theme descriptor: name, directories, scripts |
| `samaris.script` | Plymouth script — animation, transitions, progress bar |

<br>

## Script Animation

The `samaris.script` Plymouth script controls:

- **Logo fade-in**: Logo appears with a smooth alpha transition
- **Progress pulse**: Animated progress bar during boot stages
- **Status text**: "Starting Samaris OS..." with stage updates
- **Password prompt**: LUKS/encryption passphrase dialog styling
- **Error state**: Visual indicator if a boot service fails
- **Transition**: Smooth handoff from Plymouth to the desktop UI

<br>

## Build Integration

```bash
# The build step installs the theme:
# ISOGenerator/steps/07-boot-theme.sh

# It copies the theme directory to the overlay and regenerates
# the initramfs to include the Plymouth theme.
```

<br>

## Debugging

```bash
# Test Plymouth in a running system
sudo plymouthd --debug --debug-file=/tmp/plymouth-debug.log
sudo plymouth --show-splash
sudo plymouth --update=event:"Loading modules..."
sudo plymouth --quit

# List available themes
sudo plymouth-set-default-theme --list

# Set theme manually
sudo plymouth-set-default-theme samaris
sudo update-initramfs -u
```
