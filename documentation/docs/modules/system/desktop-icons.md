# Desktop Icons

Icon grid overlay on the desktop background providing quick access to files, folders, and applications. Part of the VOLT Desktop Module.

<br>

## Features

- File and folder icons rendered on desktop background
- Double-click to open in default registered app
- Drag to rearrange (freeform positioning)
- Right-click context menu (open, rename, delete, properties)
- Auto-arrange on grid (configurable spacing)
- Icon size control (small, medium, large)
- Multi-select with `Cmd/Ctrl+click` or rubber-band selection
- Desktop refresh on filesystem changes

<br>

## Architecture

```
DesktopIcons (React)
├── IconGrid (positioned icon elements with drag support)
├── IconContextMenu (right-click actions)
├── IconSelectionOverlay (rubber-band selection rectangle)
└── useDesktopIcons (drag state, positions, persistence, fs watch)
```

<br>

## Persistence

Icon positions persist via the session store (`osStore.desktopIcons`). The store is backed by a JSON file at `/User/.config/samaris/desktop-icons.json`.

<br>

## VOLT Integration

| Service | Channel |
|---------|---------|
| FsWatch | Real-time updates when files are added/removed on the desktop |
| AppRegistry | Resolves default app for file types on double-click |
| ThemeStore | Adapts icon label colors to current theme |

<br>

## Related

- [Finder App](../../apps/finder.md)
- [Theme System](theme-system.md)
- [VOLT Architecture — Desktop Module](../../architecture/volt-desktop.md)
- [Filesystem API](../../apis/fs-api.md)

<br>

---

[← Back: Documentation Index](../../index.md)
