# 22. Orbit AI Assistant

## 22.1 Overview

Orbit is the local AI assistant integrated into Samaris OS. Unlike cloud-dependent assistants, Orbit runs entirely on-device using a quantised large language model (LLM) with local speech-to-text and text-to-speech capabilities. Orbit is orchestrated by Kernel A and has dedicated resource reservations in the Tesseract Engine and Dynamic Worker Pool to ensure responsive interaction without degrading desktop performance.

## 22.2 Architecture

### Model Stack

| Component | Technology | Model |
|-----------|------------|-------|
| Language model | llama.cpp | Qwen3 1.7B Q8_0 (GGUF format) |
| Speech-to-text | whisper.cpp | ggml-small.bin |
| Text-to-speech | OuteTTS + WavTokenizer | OuteTTS v1 |

All models are stored in `/opt/volt/ai-models/` and are included in the ISO image.

### Integration

Orbit runs as a service within Kernel A:

- LLM inference is managed by `orbitRuntime.js` which interfaces with llama.cpp
- STT is handled by `sttService.js` using the Whisper binary
- TTS is handled by `ttsService.js` using OuteTTS
- The Orbit bridge (`orbitBridge.js`) in the Volt Unifier handles desktop communication

### Resource Management

Orbit receives dedicated resource reservations to ensure interactive performance:

- **Workers**: 4 dedicated inference workers in the DWP
- **Memory**: 512 MiB reserved in the Tesseract Engine's `orbit_reservation` config
- **Quota**: 2048 MiB memory quota in the VRM (critical priority, T1 tier)
- **Burst**: 100 ms burst window with 75% worker allocation during active inference

The resource reservation system ensures that Orbit can run inference without starving the desktop UI of workers or memory.

### Communication Flow

```
User speaks → STT (Whisper) → Text → LLM (Qwen3) → Response text
                                                          │
                                                    TTS (OuteTTS) → Audio
                                                          │
                                              Desktop displays both text + audio
```

## 22.3 Inference Management

The Orbit runtime manages the LLM lifecycle:

- **Model loading**: Qwen3 GGUF model loaded at Orbit startup (or on first request for lazy loading)
- **Inference**: streaming SSE responses for real-time text generation
- **Context management**: conversation history maintained for session continuity
- **Idle shutdown**: model unloaded after configurable idle period to free memory

## 22.4 Desktop Integration

Orbit is accessed through the desktop UI as a conversational interface:

- Chat-style interaction in a dedicated window or sidebar
- Voice input via microphone capture → Whisper transcription
- Voice output via TTS synthesis
- System actions: Orbit can trigger Kernel A handlers (file search, app launch, system info queries)

## 22.5 Limitations

As a local 1.7B parameter model running on consumer hardware:

- Response quality is comparable to small on-device models, not large cloud LLMs
- Inference speed depends on CPU/GPU capability and available workers
- Multi-turn conversation context is limited by available memory
- TTS and STT accuracy depend on microphone quality and background noise
- Orbit is designed for assistant tasks (information lookup, system control, simple Q&A), not for complex reasoning or code generation
