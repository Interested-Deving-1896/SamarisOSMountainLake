# Finder

File explorer with **sidebar navigation**, **grid/list views**, **thumbnails**, **context menus**, and **desktop icon integration**.

<br>

## Features

- Browse filesystem with path navigation and breadcrumb trail
- Grid view with thumbnail previews (images, video, PDF)
- List view with columns (name, size, type, date modified)
- File context menu (open, rename, delete, move, copy, compress)
- Click to open files in appropriate registered app
- Desktop icons overlay for quick access
- Drag-and-drop between directories
- Search within current directory

<br>

## Architecture

```
Finder (React)
├── Sidebar (directory tree with favorites)
├── FileGrid (thumbnail + name grid)
├── FileList (list view with sortable columns)
├── FileContextMenu (actions)
├── PathBar (breadcrumb navigation)
├── PreviewPane (file preview for images, text, PDF)
└── SearchBar (inline directory search)
```

Uses the virtual filesystem API through `useFs()` which routes through the kernel. Thumbnails are generated via `fileThumbnails.ts` with an LRU cache (max 200 entries).

<br>

## VOLT Integration

| Kernel Service | Channel |
|----------------|---------|
| FsService | `fs.list`, `fs.read`, `fs.write`, `fs.delete` |
| FsWatch | Real-time file change notifications via WebSocket |
| ThumbnailService | On-demand thumbnail generation |

<br>

## Related

- [Filesystem API](../apis/fs-api.md)
- [Filesystem Architecture](../architecture/filesystem.md)
- [Desktop Icons Module](../modules/system/desktop-icons.md)
- [VOLT Architecture — Kernel A Chapter](../architecture/volt-kernel-a.md)

<br>

---

[← Back: Documentation Index](../index.md)
