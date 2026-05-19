# AI Stack

Local, offline, private AI running entirely on-device — **zero cloud dependency**. Total model footprint: **~2.8 GB**, stored at `/opt/volt/ai-models/`.

<br>

## Pipeline

```
Text → Qwen3 1.7B (llama-server) → Response text

Audio → Whisper (STT) → Text → Qwen3 → OuteTTS (TTS) → Audio
```

<br>

## Models

| Model | Format | Size | Role |
|-------|--------|------|------|
| **Qwen3-1.7B-Q8_0.gguf** | GGUF (Q8_0) | 1.7 GB | LLM — conversation, reasoning, code |
| **ggml-small.bin** | GGML (f16) | 465 MB | Speech recognition (Whisper) |
| **OuteTTS-0.2-500M-Q8_0.gguf** | GGUF (Q8_0) | 512 MB | Text-to-speech generation |
| **WavTokenizer-Large-75-F16.gguf** | GGUF (F16) | 124 MB | Audio vocoder |

<br>

## Resource Management

Orbit receives dedicated resource reservations to ensure interactive performance:

| Resource | Value |
|----------|-------|
| DWP workers | 4 dedicated inference workers |
| Tesseract reservation | 512 MiB reserved memory |
| VRM memory quota | 2048 MiB (critical priority, T1 tier) |
| Burst window | 100ms with 75% worker allocation during inference |
| Max consecutive bursts | 3 |

<br>

## Qwen3 1.7B Parameters

| Parameter | Fast | Smart | Code | Data-science |
|-----------|------|-------|------|-------------|
| Temperature | 0.7 | 0.6 | 0.5 | 0.5 |
| top_p | 0.8 | 0.95 | 0.95 | 0.95 |
| top_k | 20 | 20 | 20 | 20 |
| min_p | 0 | 0 | 0 | 0 |
| presence_penalty | 1.5 | 1.5 | 1.5 | 1.5 |
| num_ctx | 8192 | 8192 | 8192 | 8192 |
| n_predict | 1024 | 4096 | 4096 | 4096 |

<br>

## Inference Details

- All inference runs locally via **llama.cpp** (`llama-server` for LLM, `llama-tts` for TTS)
- GPU acceleration via **Metal** (macOS) or **CUDA / Vulkan** (Linux)
- Models idle-shutdown after **45 seconds** of inactivity (configurable)
- First call incurs ~8s Metal shader init (subsequent calls use cached pipeline)
- Lazy loading: model loaded on first request (or at Orbit startup)

<br>

## Integration

Orbit runs as a service within Kernel A:

- LLM inference managed by `orbitRuntime.js` interfacing with llama.cpp
- STT handled by `sttService.js` using the Whisper binary
- TTS handled by `ttsService.js` using OuteTTS
- The Orbit bridge (`orbitBridge.js`) in the Volt Unifier handles desktop communication

<br>

## Related

- [Orbit AI App](../components/apps/orbit.md)
- [AI Models Reference](../system/ai-models.md)
- [Orbit AI Configuration](../config/orbit-config.md)

<br>

---

[← Back: Architecture Overview](overview.md)
