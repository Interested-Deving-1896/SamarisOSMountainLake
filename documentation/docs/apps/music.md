# Music

Audio player with library management, playlist support, and hardware volume integration.

<br>

## Features

- Audio file playback from filesystem (MP3, FLAC, WAV, OGG, AAC)
- Library browsing by artist, album, genre
- Playlist creation and management
- **Web Audio API** for playback with waveform visualization
- Volume control with hardware sync
- Media metadata extraction (cover art, duration, bitrate)
- Seek, shuffle, repeat modes

<br>

## Architecture

```
MusicApp (React)
├── LibraryView (artist/album/genre browser)
├── PlaylistPanel (playlist queue management)
├── PlayerBar (play/pause, next/prev, seek, volume, time display)
├── NowPlayingView (cover art, metadata, waveform)
└── useAudioPlayback (Web Audio API hook with AudioContext)
```

**Backend:** Kernel's `audioService.js` for hardware volume control and `MediaService` for file metadata extraction (via `music-metadata`).

<br>

## Related

- [Audio System Module](../modules/system/audio-system.md)
- [Filesystem API](../apis/fs-api.md)
- [VOLT Architecture — Audio Pipeline](../architecture/volt-audio.md)

<br>

---

[← Back: Documentation Index](../index.md)
