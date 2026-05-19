# Spotlight Search

System-wide search accessible via keyboard shortcut, providing instant access to apps, files, settings, contacts, and quick calculations.

<br>

## Features

- Global search overlay (hotkey: `Cmd/Ctrl + Space`)
- Search across apps, files, settings, contacts, and bookmarks
- Fuzzy matching with ranked results (TF-IDF scoring)
- Keyboard navigation through results (arrow keys, Enter to select)
- Quick actions (open app, launch file, calculate expression, define word)
- Recent searches history
- Search results grouped by category with section headers

<br>

## Architecture

```
SpotlightSearch (React)
├── SearchInput (auto-focused text field with debounce)
├── SearchResults (categorized result list with infinite scroll)
├── ResultItem (icon, title, subtitle, action button)
├── SearchIndex (in-memory inverted index for fast lookup)
└── useSpotlight (query state, keyboard navigation, result selection)
```

<br>

## Search Index

The search index is built from kernel-provided metadata on boot:
- **Apps**: Registered apps from `AppRegistry` (name, description, keywords)
- **Files**: Filesystem index from `FsService` (filename, path, type)
- **Settings**: Settings panel entries (section names, descriptions)
- **Contacts**: Address book entries (name, email, phone)

The index is updated incrementally when apps are installed/uninstalled or files are created/deleted.

<br>

## Related

- [Orbit AI App](../../apps/orbit.md) — natural language queries via Spotlight integration
- [Kernel App Registry API](../../apis/app-registry.md)
- [Filesystem API](../../apis/fs-api.md)
- [VOLT Architecture — Spotlight Module](../../architecture/volt-spotlight.md)

<br>

---

[← Back: Documentation Index](../../index.md)
