# AI Models

## Included Models

All models are stored at `/opt/volt/ai-models/` and are included in the ISO image.

| Model | File | Size | Format | License |
|-------|------|------|--------|---------|
| **Qwen3 1.7B** (Q8_0) | `ai-models/Qwen3-1.7B-Q8_0.gguf` | 1.7 GB | GGUF | Apache 2.0 |
| **Whisper Small** | `ai-models/whisper/ggml-small.bin` | 465 MB | GGML | MIT |
| **OuteTTS 0.2** (Q8_0) | `ai-models/outetts/OuteTTS-0.2-500M-Q8_0.gguf` | 512 MB | GGUF | Apache 2.0 |
| **WavTokenizer Large** | `ai-models/outetts/WavTokenizer-Large-75-F16.gguf` | 124 MB | GGUF | MIT |

**Total AI model footprint:** ~2.8 GB

<br>

## Model Directory Structure

```
/opt/volt/ai-models/
├── Qwen3-1.7B-Q8_0.gguf
├── outetts/
│   ├── OuteTTS-0.2-500M-Q8_0.gguf
│   └── WavTokenizer-Large-75-F16.gguf
├── whisper/
│   └── ggml-small.bin
└── bin/
    └── whisper
```

<br>

## Inference Runtimes

All models run via **llama.cpp**:
- **LLM**: `llama-server` with Qwen3 GGUF — conversation, reasoning, code
- **STT**: `whisper.cpp` with ggml-small — speech recognition
- **TTS**: `llama-tts` with OuteTTS GGUF + WavTokenizer — text-to-speech

<br>

## TTS Quality

OuteTTS 0.2 provides significantly better quality than the previous Piper TTS engine:
- 82M parameter StyleTTS2 architecture
- 24 kHz output sample rate
- Near-human speech quality
- WavTokenizer vocoder for audio decoding

<br>

## Resource Reservations

| Model | VRM Quota | DWP Workers | Priority |
|-------|-----------|-------------|----------|
| Qwen3 (Orbit) | 2048 MB | 4 dedicated | Critical |
| Whisper (STT) | 512 MB | Shared | High |
| OuteTTS (TTS) | 512 MB | Shared | High |

<br>

## Related

- [AI Stack Architecture](../architecture/ai-stack.md)
- [Orbit AI App](../components/apps/orbit.md)
- [Orbit Configuration](../config/orbit-config.md)

<br>

---

[← Back: Documentation Index](../index.md)
