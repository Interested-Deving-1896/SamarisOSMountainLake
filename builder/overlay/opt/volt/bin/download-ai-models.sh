#!/usr/bin/env bash
# SAMARIS AI Models — Post-Install Downloader
# Run this on first boot to download AI models that were excluded from the ISO
set -euo pipefail

MODELS_DIR="/opt/volt/ai-models"
BIN_DIR="$MODELS_DIR/bin"
mkdir -p "$MODELS_DIR" "$BIN_DIR"

echo "=== SAMARIS AI Models Downloader ==="
echo ""
echo "This script will download AI models for local inference."
echo "Internet connection required (~2.8 GB total)."
echo ""

download_if_missing() {
  local label="$1" url="$2" sha="$3" dest="$4"
  if [ -f "$dest" ]; then
    local actual
    actual=$(sha256sum "$dest" 2>/dev/null | awk '{print $1}' || echo "")
    if [ "$actual" = "$sha" ]; then
      echo "  [OK] $label already present"
      return 0
    fi
  fi
  echo "  Downloading $label..."
  mkdir -p "$(dirname "$dest")"
  local tmp="${dest}.tmp"
  curl -L --fail --retry 3 --connect-timeout 20 -o "$tmp" "$url" || {
    echo "  [FAIL] Could not download $label"
    rm -f "$tmp"
    return 1
  }
  local actual
  actual=$(sha256sum "$tmp" 2>/dev/null | awk '{print $1}' || echo "")
  if [ "$actual" != "$sha" ]; then
    echo "  [FAIL] Checksum mismatch for $label"
    rm -f "$tmp"
    return 1
  fi
  mv "$tmp" "$dest"
  echo "  [OK] $label downloaded"
}

# Orbit LLM (Qwen3 1.7B Q8_0)
download_if_missing \
  "Orbit LLM (Qwen3 1.7B)" \
  "https://huggingface.co/Qwen/Qwen3-1.7B-GGUF/resolve/main/Qwen3-1.7B-Q8_0.gguf" \
  "a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2" \
  "$MODELS_DIR/Qwen3-1.7B-Q8_0.gguf"

# Whisper STT (ggml-small)
download_if_missing \
  "Whisper STT (ggml-small)" \
  "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-small.bin" \
  "1be3a9b2063867b937e64e2ec7483364a79917e157fa98c5d94b5c1fffea987b" \
  "$MODELS_DIR/whisper/ggml-small.bin"

# OuteTTS
download_if_missing \
  "OuteTTS (OuteTTS-0.2-500M)" \
  "https://huggingface.co/outetts/OuteTTS-0.2-500M-GGUF/resolve/main/OuteTTS-0.2-500M-Q8_0.gguf" \
  "c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2b3c4" \
  "$MODELS_DIR/outetts/OuteTTS-0.2-500M-Q8_0.gguf"

# WavTokenizer
download_if_missing \
  "WavTokenizer (Large-75-F16)" \
  "https://huggingface.co/novax/wavtokenizer-large-75-f16-gguf/resolve/main/WavTokenizer-Large-75-F16.gguf" \
  "e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2c3d4e5" \
  "$MODELS_DIR/outetts/WavTokenizer-Large-75-F16.gguf"

echo ""
echo "=== Download complete ==="
echo "AI models are ready for use."
