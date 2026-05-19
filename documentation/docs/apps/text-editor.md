# Text Editor

**Lightweight document editor with file system integration and unsaved-changes protection.**

A simple text/code editor for creating and editing plain text files on the Samaris filesystem.

## Features

- **File loading**: Opens any text file from the filesystem via `useFs` hook
- **Save**: Writes changes back to the filesystem with overwrite confirmation
- **Unsaved changes guard**: Prevents accidental window close when content has been modified
- **Line counting**: Displays total line count in the status bar
- **Minimal toolbar**: File name display, save button, status indicator
- **New file creation**: Opening a non-existent path starts a new blank document

## Interface

```
┌─────────────────────────────────────────────┐
│ [Filename]                     [Save] [Close]│
├─────────────────────────────────────────────┤
│                                             │
│  Content area — plain text editing          │
│                                             │
├─────────────────────────────────────────────┤
│  Ready  |  Lines: 42                        │
└─────────────────────────────────────────────┘
```

- Toolbar with file path indicator
- Editing area with monospace font
- Status bar showing "Ready", "Loaded", "New file", or "Saved"
- Line count tracker

## Integration

- Filesystem reads/writes through `useFs()` service hook
- Close guard via `windowCloseGuards.register()` — confirms on unsaved changes
- Window parameters: `path` — the file path to open (`/User/Desktop/untitled.txt` default)
