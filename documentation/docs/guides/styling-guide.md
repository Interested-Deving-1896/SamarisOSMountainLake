# Styling Guide

## Design Philosophy

Samaris uses a **glass design** aesthetic — translucent panels, frosted backgrounds, subtle glow effects, and a cohesive dark/light theme system. All components follow the VOLT Design System.

<br>

## Theme Variables

All styling is driven by CSS custom properties defined in the theme:

```css
:root {
  --text: rgba(10, 26, 46, 0.94);
  --text-soft: rgba(10, 26, 46, 0.62);
  --text-faint: rgba(10, 26, 46, 0.42);
  --bg: rgba(255, 255, 255, 0.72);
  --glass-a: rgba(255, 255, 255, 0.32);
  --glass-c: rgba(255, 255, 255, 0.16);
  --panel: rgba(255, 255, 255, 0.08);
  --blur-window: 32px;
  --blur-ui: 24px;
  --fast: 140ms;
  --normal: 220ms;
  --slow: 360ms;
  --ease: cubic-bezier(.25, .46, .45, .94);
}
```

<br>

## Dark Mode

Toggle via `data-theme="dark"` on the document element:

```css
[data-theme="dark"] {
  --text: rgba(235, 245, 255, 0.94);
  --text-soft: rgba(235, 245, 255, 0.62);
  --bg: rgba(15, 25, 40, 0.85);
  --glass-a: rgba(0, 0, 0, 0.32);
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

## Animation Tokens

| Token | Duration | Easing |
|-------|----------|--------|
| `--fast` | 140ms | `var(--ease)` |
| `--normal` | 220ms | `var(--ease)` |
| `--slow` | 360ms | `var(--ease)` |

<br>

## Component Patterns

### Window

```css
.window {
  background: var(--bg);
  backdrop-filter: blur(var(--blur-window));
  border-radius: 12px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.12);
}
```

### Button

```css
.btn {
  background: var(--glass-a);
  border: 1px solid var(--glass-c);
  border-radius: 8px;
  padding: 8px 16px;
  transition: all var(--fast) var(--ease);
}
.btn:hover {
  background: var(--panel);
}
```

<br>

## Related

- [Theme System](../components/modules/theme-system.md)
- [Samaris Icons](../components/modules/samaris-icons.md)
- [VOLT Design System Reference](../architecture/volt-design-system.md)

<br>

---

[← Back: Documentation Index](../index.md)
