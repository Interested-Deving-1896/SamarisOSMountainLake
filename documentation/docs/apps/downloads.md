# Downloads

**Download manager with progress tracking and file organisation.**

Monitors and manages all file downloads initiated from Peregrine browser or system apps.

## Features

- **Real-time progress**: Speed, percentage, ETA per active download
- **State management**: Downloading, completed, failed, cancelled
- **Filtering**: All / Active / Completed tabs
- **File preview**: Auto-detects file type and shows relevant icon (image, video, audio, archive, document)
- **Drag support**: Drag completed files from the list to the desktop or Finder
- **History**: Persistent download history with clear option
- **Native dialogs**: "Save As" dialog via Electron's download manager
- **Path control**: All files saved under `~/.volt/user/Downloads/`

## Interface

- List view with filename, size, progress bar, and status
- Active downloads show percentage and bytes transferred
- Completed items show save path and allow dragging
- Failed items display the error reason
- Clear button to reset history

## Integration

Uses `downloadStore` (Zustand) for state management. Communicates with Electron's `DownloadManager` via IPC for native save dialogs and progress events.
