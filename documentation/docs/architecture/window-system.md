# Window System

A modern desktop experience with **glass-design** windows, snap-to-edge, and smooth window animations.

<br>

## Window Lifecycle

| Phase | Animation | Duration |
|-------|-----------|----------|
| **Open** | `width: calc(100% - 40px)` entrance | 210ms ease-out |
| **Focus** | Raise z-index +10 | Instant |
| **Drag** | `requestAnimationFrame` throttled | Real-time |
| **Resize** | 8 edge handles, minimum size enforced | Real-time |
| **Snap** | Left/right half, top maximize | 150ms |
| **Minimize** | WebGL animation → dock (Genie effect) | 720–860ms |
| **Restore** | Reverse animation ← dock | 580–640ms |
| **Close** | `scale(0.972)` + `opacity(0)` | 180ms |

<br>

## Performance Optimizations

| Technique | Benefit |
|-----------|---------|
| `backdrop-filter: blur(18px)` | Reduced GPU fill cost |
| `will-change` only during drag | Minimized compositor layers |
| `contain: layout paint` | Layout isolation |
| requestAnimationFrame throttle | Smooth animation at display refresh rate |

<br>

## Related

- [Electron Shell](electron-shell.md)
- [Styling Guide](../guides/styling-guide.md)
- [Theme System](../components/modules/theme-system.md)

<br>

---

[← Back: Architecture Overview](overview.md)
