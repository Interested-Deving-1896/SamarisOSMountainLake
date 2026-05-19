# Theme System

CSS custom property-based theming with full **dark/light mode** support, glass effects, and animation tokens.

<br>

## Design Tokens

```css
:root {
  --text: rgba(10, 26, 46, 0.94);
  --text-soft: rgba(10, 26, 46, 0.62);
  --text-faint: rgba(10, 26, 46, 0.42);
  --blur-window: 32px;
  --blur-ui: 24px;
  --fast: 140ms;
  --normal: 220ms;
  --slow: 360ms;
  --ease: cubic-bezier(.25, .46, .45, .94);
  --glass-a: rgba(255, 255, 255, 0.32);
  --glass-c: rgba(255, 255, 255, 0.16);
  --panel: rgba(255, 255, 255, 0.08);
}
```

<br>

## Dark Mode

Toggle via `data-theme="dark"` on the document element:

```css
[data-theme="dark"] {
  --text: rgba(235, 245, 255, 0.94);
  --text-soft: rgba(235, 245, 255, 0.62);
  --text-faint: rgba(235, 245, 255, 0.42);
  --bg: rgba(15, 25, 40, 0.85);
  --glass-a: rgba(0, 0, 0, 0.32);
  --glass-c: rgba(0, 0, 0, 0.16);
  --panel: rgba(0, 0, 0, 0.12);
}
```

<br>

## Glass Effect

```css
.glass-panel {
  background: var(--glass-a);
  backdrop-filter: blur(var(--blur-ui));
  border: 1px solid rgba(255, 255, 255, 0.08);
  border-radius: 12px;
}
```

<br>

## AirBar Adaptive Glass

The AirBar features an **adaptive glass effect** that samples desktop wallpaper colors and adjusts the glass tint accordingly, creating a translucent frosted-glass appearance. The sampling runs at 1 FPS and uses canvas-based color extraction from the wallpaper.

<br>

## Animation Tokens

| Token | Duration | Easing |
|-------|----------|--------|
| `--fast` | 140ms | `var(--ease)` |
| `--normal` | 220ms | `var(--ease)` |
| `--slow` | 360ms | `var(--ease)` |

All UI transitions use `var(--ease)` (`cubic-bezier(.25, .46, .45, .94)`) for a consistent feel.

<br>

## Accent Colors

The system supports accent color customization via `data-accent` attribute:

```css
[data-accent="blue"] { --accent: #007aff; }
[data-accent="green"] { --accent: #34c759; }
[data-accent="purple"] { --accent: #af52de; }
[data-accent="orange"] { --accent: #ff9500; }
[data-accent="red"] { --accent: #ff3b30; }
```

<br>

## Related

- [Styling Guide](../../guides/styling-guide.md)
- [Settings App](../../apps/settings.md)
- [AirBar System Panel](../../architecture/airbar.md)
- [Samaris Icons](samaris-icons.md)

<br>

---

[← Back: Documentation Index](../../index.md)
