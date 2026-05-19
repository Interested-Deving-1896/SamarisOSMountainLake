# Samaris Icons

Custom icon set used throughout the Samaris desktop environment — designed for clarity, consistency, and theme adaptability.

<br>

## Design

- **Style**: Minimal, rounded, two-tone with transparent backgrounds
- **Format**: SVG for crisp rendering at any resolution
- **Color**: Adapts to theme (dark/light) via `currentColor` and CSS custom properties
- **Sizes**: 16px, 24px, 32px, 48px, 64px, 128px (SVG scales cleanly to any size)
- **Grid**: 24×24 viewBox with 2px consistent padding

<br>

## Categories

| Category | Icons |
|----------|-------|
| **Apps** | Finder, Settings, Terminal, Orbit, Peregrine, Photos, Music, Videos, Mail, App Store, Notes, Trash, Doom |
| **System** | AirBar WiFi, Bluetooth, Sound, Battery, Volume, Brightness, Power, Network, Lock |
| **Actions** | Open, Close, Minimize, Maximize, Resize, Search, Share, Delete, Rename, Copy, Paste, Cut |
| **Status** | Checkmark, Warning, Error, Info, Loading, Progress, Success |
| **Navigation** | Back, Forward, Up, Down, Chevron, Hamburger, Grid, List |

<br>

## Usage

```tsx
import { Icon } from "../components/ui/Icon";

<Icon name="finder" size={32} />
<Icon name="wifi" size={24} color="var(--accent)" />
<Icon name="trash" size={48} className="desktop-icon" />
```

The `Icon` component resolves SVG paths from the icon registry and applies the requested size and color. Icons can be themed via CSS or inline styles.

<br>

## Related

- [Theme System](theme-system.md)
- [Styling Guide](../../guides/styling-guide.md)
- [Icon Component API](../../apis/icon-component.md)

<br>

---

[← Back: Documentation Index](../../index.md)
