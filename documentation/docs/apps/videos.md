# Videos

Video player with library browsing, thumbnail grid, and full playback controls.

<br>

## Features

- Open video files from Finder or file dialog
- Playback controls (play/pause, seek bar with preview thumbnail, volume, fullscreen)
- Thumbnail grid for directory browsing (video file previews)
- Supports common formats via HTML5 video (MP4, WebM, OGV, MOV)
- Keyboard shortcuts (Space = play/pause, F = fullscreen, arrows = seek)
- Subtitle support via WebVTT
- Playback speed control (0.5x – 2x)
- Picture-in-picture mode

<br>

## Architecture

```
VideosApp (React)
├── VideoGrid (thumbnail gallery of video files)
├── VideoPlayer (HTML5 video element with HLS/DASH support)
├── PlaybackControls (seek bar, time display, volume, speed, PiP)
├── SubtitleOverlay (WebVTT subtitle rendering)
└── FullscreenToggle
```

Uses `readDataUrl` from the kernel filesystem to load video files into the player. For streaming formats, the kernel provides `fs.stream` for progressive loading.

<br>

## Related

- [Filesystem API](../apis/fs-api.md)
- [VOLT Architecture — Media Pipeline](../architecture/volt-media.md)

<br>

---

[← Back: Documentation Index](../index.md)
