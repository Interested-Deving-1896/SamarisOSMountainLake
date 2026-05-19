# Audio System

Volume control, output device management, and audio routing via the kernel's `audioService.js`.

<br>

## Audio Store

The `audioStore.ts` manages local audio state:

| State | Type | Description |
|-------|------|-------------|
| Volume | 0–100 | Current system volume level |
| Mute | boolean | Mute state |
| Output device | string | Active audio output device ID |
| Input device | string | Active microphone device ID |
| Debounce | 30ms | Debounce interval before committing to kernel |

<br>

## Kernel Service

| Feature | Detail |
|---------|--------|
| Volume get/set | System audio via platform APIs (OSD on macOS, ALSA/PulseAudio/WirePlumber on Linux) |
| Output devices | List available devices and switch active |
| Input devices | List available microphones |
| Status polling | Every 12 seconds for device changes |

<br>

## Sound Panel (AirBar)

- Volume slider with gradient track (CSS native range input)
- Mute toggle button
- Output device selector with active highlight
- Input device selector
- Audio visualization (optional animated waveform)

<br>

## Audio Routing

```
App (Web Audio API)
  → Hardware output device (selected in Sound Panel)
    → Kernel audioService controls system mixer
```

Per-app audio control is managed through the kernel's `audioKernel` module, which maps each app's AudioContext to a system audio stream.

<br>

## Related

- [AirBar System Panel](../../architecture/airbar.md)
- [Music App](../../apps/music.md)
- [VOLT Architecture — Audio Pipeline](../../architecture/volt-audio.md)

<br>

---

[← Back: Documentation Index](../../index.md)
