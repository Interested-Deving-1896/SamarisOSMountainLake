# DOOM

Classic **DOOM** running via **js-dos** in the browser — a showcase of the Samaris desktop's ability to run retro games as first-class applications.

<br>

## Features

- Full game playable in a resizable window
- Uses js-dos library (DOSBox compiled to WebAssembly via Emscripten)
- Keyboard controls mapped to DOS key bindings
- Audio via Web Audio API (OPL3 emulation)
- No additional setup or configuration required
- Save state persistence to filesystem

<br>

## Assets

The game assets (DOOM.WAD) are bundled in the `public/games/doom/` directory. The js-dos kernel and BIOS files are served from the same location.

<br>

## Architecture

```
DoomApp (React)
├── DosInstance (js-dos WebAssembly loader)
├── DosCanvas (canvas element for rendered output)
├── KeyboardMapper (DOS key bindings)
└── SaveStateManager (persist/restore game state via FsService)
```

<br>

## Related

- [VOLT Architecture — App Lifecycle](../architecture/volt-app-lifecycle.md)
- [Kernel Filesystem API](../apis/fs-api.md)

<br>

---

[← Back: Documentation Index](../index.md)
