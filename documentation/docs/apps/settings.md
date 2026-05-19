# Settings

System preferences and configuration app for the Samaris desktop environment.

<br>

## Sections

| Section | Features |
|---------|----------|
| **Appearance** | Theme (light/dark), wallpaper selection, accent color picker, font scaling |
| **Security** | Lock screen preferences, session lock timeout, auto-lock on sleep |
| **About** | System version, kernel build, hardware info, credits, licenses |

<br>

## Additional Panels

| Panel | Description |
|-------|-------------|
| **Desktop** | Icon arrangement, auto-arrange toggle, icon size |
| **Dock / AirBar** | Position, auto-hide, icon magnification |
| **Keyboard** | Key repeat rate, delay, modifier key customization |
| **Trackpad** | Scroll direction, tap-to-click, natural scrolling |
| **Language & Input** | System language, input sources, spell check |
| **Date & Time** | Timezone, 24h toggle, date format |
| **Accessibility** | Reduced motion, high contrast, cursor size |

Settings persist via `localStorage` and sync with the theme store. The `osStore` holds the active configuration and emits updates to all subscribed components.

<br>

## Related

- [Theme System](../modules/system/theme-system.md)
- [Styling Guide](../guides/styling-guide.md)
- [Onboarding Flow](../modules/system/onboarding.md)
- [System Configuration Reference](../config/system-config.md)

<br>

---

[← Back: Documentation Index](../index.md)
