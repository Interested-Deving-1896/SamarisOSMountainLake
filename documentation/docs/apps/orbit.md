# Orbit AI

The local AI assistant — featuring LLM, speech-to-text, and text-to-speech, all running **locally with zero cloud dependency** — powered by the VOLT AI pipeline.

<br>

## Modes

| Mode | Strategy | Use Case |
|------|----------|----------|
| **Fast** | Self-consistency | Quick answers, general chat |
| **Smart** | Chain-of-thought | Complex reasoning, deep analysis |
| **Code** | Chain-of-thought | Programming, debugging |
| **Data-science** | Chain-of-thought | Analytics, metrics, experiments |

<br>

## Voice Mode

Full voice interaction loop with **interruption support**:

```
1. VAD detects speech (Web Audio API)
2. Whisper STT transcribes to text
3. Qwen3 LLM generates response (runs under VRM-managed quota)
4. OuteTTS reads response aloud
5. Loop back to listening
```

Speaking while Orbit is responding **cancels** current TTS playback and starts listening again.

<br>

## AI Model Locations

All models are stored under `/opt/volt/ai-models/`:

| Model | Path | Type | VRAM |
|-------|------|------|------|
| Qwen3 1.8B | `/opt/volt/ai-models/qwen3-1.8b-q4.gguf` | LLM | 2048 MB |
| Whisper Small | `/opt/volt/ai-models/whisper-small.gguf` | STT | 512 MB |
| OuteTTS | `/opt/volt/ai-models/outetts-q4.gguf` | TTS | 512 MB |
| Tesseract | `/opt/volt/ai-models/tesseract.traineddata` | OCR | 512 MB |

<br>

## Architecture

```
OrbitApp (React)
├── useOrbitChat (hook)
├── useVoiceMode (hook)
├── ChatInput (text + voice mic button)
├── MessageBubble (chat messages + TTS read-aloud)
├── ThreadSidebar (chat history)
├── ModeSelector (Fast / Smart toggle)
└── ModelStatusCard (model status indicator)
```

<br>

## VOLT Integration

| Component | Integration |
|-----------|-------------|
| **VRM** | Registers LLM (2048 MB), STT (512 MB), TTS (512 MB) quotas — all **critical priority T1** |
| **DWP** | 4 dedicated workers; burst config: 100ms window, 75% worker pool, 3 max consecutive bursts |
| **VGM** | Logs GPU metrics after each generation |
| **Unifier** | `OrbitBridge` — dedicated unification channel for AI requests routed via the Volt Unifier |

<br>

## Resource Quotas

| Resource | Allocation |
|----------|------------|
| RRAM (LLM) | 2048 MB — critical T1 |
| VRAM (STT) | 512 MB |
| VRAM (TTS) | 512 MB |
| VRAM (Tesseract OCR) | 512 MB |
| DWP Workers | 4 dedicated |
| Burst window | 100 ms |
| Burst worker cap | 75% of pool |
| Max consecutive bursts | 3 |

<br>

## Related

- [AI Stack Architecture](../architecture/ai-stack.md)
- [AI Models Reference](../system/ai-models.md)
- [Orbit AI Configuration](../config/orbit-config.md)
- [VOLT Architecture — VRM Chapter](../architecture/volt-vrm.md)
- [VOLT Architecture — DWP Chapter](../architecture/volt-dwp.md)

<br>

---

[← Back: Documentation Index](../index.md)
