# Notes

Simple markdown text editor with live preview, file management, and auto-save.

<br>

## Features

- Plain text and markdown editing with syntax highlighting
- Live preview pane with rendered HTML (CommonMark compliant)
- Create, rename, delete notes from the sidebar
- Auto-save on content change (debounced at 500ms)
- Files stored in `/User/Documents/Notes/` as `.md` files
- Formatting toolbar with bold, italic, heading, list, link shortcuts

<br>

## Architecture

```
NotesApp (React)
├── NoteList (sidebar with file tree, sorted alphabetically)
├── NoteEditor (CodeMirror or textarea + markdown preview pane)
├── NoteToolbar (formatting shortcuts, file actions)
└── useAutoSave (debounced save on content change)
```

Uses the kernel filesystem API (`FsService`) for all read/write operations. Notes are persisted as individual `.md` files on the filesystem, not in a database.

<br>

## Related

- [Filesystem API](../apis/fs-api.md)
- [Filesystem Architecture](../architecture/filesystem.md)

<br>

---

[← Back: Documentation Index](../index.md)
