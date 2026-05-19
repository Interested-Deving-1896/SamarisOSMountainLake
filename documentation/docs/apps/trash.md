# Trash

**File deletion management with restore, secure empty, and drag-to-trash support.**

Manages deleted files and folders with the ability to restore items or permanently remove them.

## Features

- **Browsing**: Navigate trashed items grouped by deletion date
- **Restore**: Restore selected items back to their original locations
- **Empty trash**: Permanently delete all trashed items
- **Secure empty**: Overwrite deleted files before removal for added privacy
- **Search**: Filter trashed items by filename
- **Drag-to-trash**: Drag files from Finder or desktop directly into the Trash window
- **Selection**: Multi-select items for batch restore
- **Size display**: Shows total size of trashed items

## Date Grouping

Items are automatically grouped by deletion date:
- **Today**: Items deleted in the current session or today
- **Yesterday**: Items deleted the previous day
- **Older**: Items deleted before yesterday

## Interface

```
┌─────────────────────────────────────────────┐
│ Trash  (12 items, 45.2 MB)                  │
│ [Restore Selected] [Empty] [Secure Empty]   │
│ [🔍 Search trashed files...]                │
├─────────────────────────────────────────────┤
│ ── Today ──                                 │
│  report.txt  2 KB  Just now                 │
│  photo.webp  1.2 MB  2 hours ago            │
│ ── Yesterday ──                              │
│  notes.md    4 KB  Yesterday                │
└─────────────────────────────────────────────┘
```

## Integration

- Uses `useTrashController` hook for state management
- Supports drag-and-drop via `useFileDrop` with `allowedChoices: ["trash"]`
- Communicates with the filesystem service for restore and delete operations
- Real-time search filtering through `searchQuery` state
