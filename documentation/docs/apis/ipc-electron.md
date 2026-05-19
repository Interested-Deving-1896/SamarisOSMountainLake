# Electron IPC Reference

Communication channels between Electron's main and renderer processes via the preload bridge. The bridge exposes typed APIs through `window.*` objects registered in the preload script. Defined in the **VOLT specification (ch.12)**.

<br>

## Bridge APIs

### Window (4 methods)

| Method | Signature | Description |
|--------|-----------|-------------|
| `minimize` | `() => void` | Minimize window |
| `maximize` | `() => void` | Maximize window |
| `close` | `() => void` | Close window |
| `isMaximized` | `() => Promise<boolean>` | Check if maximized |

### Clipboard (2 methods)

| Method | Signature | Description |
|--------|-----------|-------------|
| `readText` | `() => Promise<string>` | Read clipboard text |
| `writeText` | `(text: string) => Promise<void>` | Write text to clipboard |

### System (4 methods)

| Method | Signature | Description |
|--------|-----------|-------------|
| `getPlatform` | `() => string` | Get OS platform |
| `getVersion` | `() => string` | Get app version |
| `getArch` | `() => string` | Get CPU architecture |
| `openExternal` | `(url: string) => void` | Open URL in system browser |

### Browser (25+ methods)

Peregrine browser bridge methods:

| Method | Signature | Description |
|--------|-----------|-------------|
| `createTab` | `(url?: string) => void` | Create new tab |
| `closeTab` | `(tabId: string) => void` | Close tab |
| `navigate` | `(tabId: string, url: string) => void` | Navigate tab |
| `goBack` | `(tabId: string) => void` | Navigate back |
| `goForward` | `(tabId: string) => void` | Navigate forward |
| `reload` | `(tabId: string) => void` | Reload page |
| `stop` | `(tabId: string) => void` | Stop loading |
| `getTabs` | `() => TabInfo[]` | List all tabs |
| `getActiveTab` | `() => string` | Active tab ID |
| `setActiveTab` | `(tabId: string) => void` | Switch tab |
| `zoomIn` | `(tabId: string) => void` | Zoom in |
| `zoomOut` | `(tabId: string) => void` | Zoom out |
| `zoomReset` | `(tabId: string) => void` | Reset zoom |
| `find` | `(text: string) => void` | Find in page |
| `findNext` | `() => void` | Next match |
| `print` | `(tabId: string) => void` | Print page |
| `screenshot` | `(tabId: string) => string` | Capture screenshot |
| `getBookmarks` | `() => Bookmark[]` | List bookmarks |
| `addBookmark` | `(url: string, title: string) => void` | Add bookmark |
| `removeBookmark` | `(id: string) => void` | Remove bookmark |
| `getHistory` | `() => HistoryEntry[]` | Browser history |
| `clearHistory` | `() => void` | Clear history |
| `setPrivateMode` | `(enabled: boolean) => void` | Toggle incognito |
| `downloadFile` | `(url: string) => void` | Download URL |
| `getDownloads` | `() => DownloadItem[]` | Active downloads |

### Terminal (6 methods)

| Method | Signature | Description |
|--------|-----------|-------------|
| `open` | `(cwd?: string) => number` | Open terminal session |
| `close` | `(id: number) => void` | Close session |
| `write` | `(id: number, data: string) => void` | Write to PTY |
| `resize` | `(id: number, cols: number, rows: number) => void` | Resize PTY |
| `list` | `() => number[]` | List active sessions |
| `onData` | `(id: number, cb: (data: string) => void) => void` | PTY output callback |

### Dialog (2 methods)

| Method | Signature | Description |
|--------|-----------|-------------|
| `openFile` | `(options?: OpenDialogOptions) => Promise<OpenDialogResult>` | Native file open |
| `saveFile` | `(options?: SaveDialogOptions) => Promise<SaveDialogResult>` | Native file save |

### Downloads (4 methods)

| Method | Signature | Description |
|--------|-----------|-------------|
| `getDownloads` | `() => DownloadItem[]` | List downloads |
| `cancelDownload` | `(id: string) => void` | Cancel download |
| `pauseDownload` | `(id: string) => void` | Pause download |
| `resumeDownload` | `(id: string) => void` | Resume download |

### Shell (3 methods)

| Method | Signature | Description |
|--------|-----------|-------------|
| `showItemInFolder` | `(path: string) => void` | Reveal in Finder |
| `openPath` | `(path: string) => void` | Open file/directory |
| `trashItem` | `(path: string) => void` | Move to trash |

### App (4 methods)

| Method | Signature | Description |
|--------|-----------|-------------|
| `quit` | `() => void` | Quit application |
| `restart` | `() => void` | Restart application |
| `getVersion` | `() => string` | App version |
| `getPath` | `(name: string) => string` | System path |

<br>

## Security

All IPC channels are validated in the preload script. Only whitelisted channels are exposed to the renderer via `contextBridge`. The renderer process has no direct access to Node.js or Electron APIs.

<br>

## Related

- [Kernel WebSocket Protocol](kernel-ws.md)
- [Kernel A — Node.js](../architecture/kernel-node.md)
- [Volt Client API](volt-client-api.md)

<br>

---

[← Back: Documentation Index](../index.md)
