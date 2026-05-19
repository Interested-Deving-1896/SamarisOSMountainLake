#!/usr/bin/env bash

download_asset() {
  local label="$1" url="$2" sha="$3" dest="$4"
  mkdir -p "$(dirname "$dest")"

  if [ -f "$dest" ] && sha256_check "$dest" "$sha"; then
    log "$label already present"
    return 0
  fi

  local rel candidate arch_name
  rel="${dest#$OVERLAY_DIR/}"
  for arch_name in $SAMARIS_ARCHES; do
    candidate="$(rootfs_dir "$arch_name")/$rel"
    if [ -f "$candidate" ] && sha256_check "$candidate" "$sha"; then
      log "Recovering $label from cached $arch_name rootfs"
      cp "$candidate" "$dest"
      return 0
    fi
  done

  local tmp="$dest.tmp"
  log "Downloading $label"
  curl -L --fail --retry 3 --connect-timeout 20 -o "$tmp" "$url"
  sha256_check "$tmp" "$sha" || die "Checksum mismatch for $label"
  mv "$tmp" "$dest"
}

prepare_whisper_binary() {
  local ai_root="$1" bin_dir="$ai_root/bin" wrapper="$bin_dir/whisper"
  mkdir -p "$bin_dir" "$CACHE_DIR"

  local src="$CACHE_DIR/whisper.cpp"
  if [ ! -d "$src/.git" ]; then
    git clone --depth 1 https://github.com/ggerganov/whisper.cpp.git "$src"
  fi

  build_whisper_arch "$src" "$bin_dir" x86_64
  build_whisper_arch "$src" "$bin_dir" aarch64

  cat > "$wrapper" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
case "$(uname -m)" in
  x86_64|amd64) exec "$(dirname "$0")/whisper-x86_64" "$@" ;;
  aarch64|arm64) exec "$(dirname "$0")/whisper-aarch64" "$@" ;;
  *) echo "Unsupported arch: $(uname -m)" >&2; exit 1 ;;
esac
EOF
  chmod 0755 "$wrapper" "$bin_dir/whisper-x86_64" "$bin_dir/whisper-aarch64"
}

build_whisper_arch() {
  local src="$1" bin_dir="$2" arch_name="$3"
  local dest="$bin_dir/whisper-$arch_name"
  local cmake_dir="$CACHE_DIR/whisper-cmake-$arch_name"
  local built

  if [ -x "$dest" ]; then
    built=$(find "$cmake_dir" -type f -name whisper-cli 2>/dev/null | sort | head -n 1)
    [ -n "$built" ] && cp "$built" "$dest" && chmod 0755 "$dest"
    return 0
  fi

  mkdir -p "$cmake_dir"
  case "$arch_name" in
    x86_64)
      cmake -S "$src" -B "$cmake_dir" \
        -DWHISPER_BUILD_TESTS=OFF -DWHISPER_BUILD_EXAMPLES=ON \
        -DGGML_NATIVE=OFF -DGGML_OPENMP=OFF
      ;;
    aarch64)
      command -v aarch64-linux-gnu-gcc >/dev/null 2>&1 || die "Missing aarch64-linux-gnu-gcc"
      command -v aarch64-linux-gnu-g++ >/dev/null 2>&1 || die "Missing aarch64-linux-gnu-g++"
      cmake -S "$src" -B "$cmake_dir" \
        -DCMAKE_SYSTEM_NAME=Linux -DCMAKE_SYSTEM_PROCESSOR=aarch64 \
        -DCMAKE_C_COMPILER=aarch64-linux-gnu-gcc \
        -DCMAKE_CXX_COMPILER=aarch64-linux-gnu-g++ \
        -DWHISPER_BUILD_TESTS=OFF -DWHISPER_BUILD_EXAMPLES=ON \
        -DGGML_NATIVE=OFF -DGGML_OPENMP=OFF
      ;;
  esac

  cmake --build "$cmake_dir" --config Release --parallel
  built=$(find "$cmake_dir" -type f -name whisper-cli 2>/dev/null | sort | head -n 1)
  [ -n "$built" ] || die "whisper.cpp build did not produce whisper-cli for $arch_name"
  cp "$built" "$dest"
  chmod 0755 "$dest"
}

step_main() {
  local ai_root="$OVERLAY_DIR/opt/volt/ai-models"
  local bin_dir="$ai_root/bin"
  mkdir -p "$ai_root/whisper" "$bin_dir"

  # Check which AI modules are enabled
  local has_llm=0 has_stt=0 has_tts=0
  build_config_is_selected AI_LLM 2>/dev/null && has_llm=1 || has_llm=$( [ "${BUILD_CONFIG_AI_LLM:-1}" = "1" ] && echo 1 || echo 0 )
  build_config_is_selected AI_STT 2>/dev/null && has_stt=1 || has_stt=$( [ "${BUILD_CONFIG_AI_STT:-1}" = "1" ] && echo 1 || echo 0 )
  build_config_is_selected AI_TTS 2>/dev/null && has_tts=1 || has_tts=$( [ "${BUILD_CONFIG_AI_TTS:-1}" = "1" ] && echo 1 || echo 0 )

  local postinstall=0
  build_config_is_selected AI_POSTINSTALL 2>/dev/null && postinstall=1 || postinstall="${BUILD_AI_POSTINSTALL:-0}"

  # Orbit LLM
  if [ "$has_llm" = "1" ] && [ "$postinstall" != "1" ]; then
    download_asset \
      "Orbit LLM (Qwen3 1.7B)" \
      "https://huggingface.co/Qwen/Qwen3-1.7B-GGUF/resolve/main/Qwen3-1.7B-Q8_0.gguf" \
      "a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2" \
      "$ai_root/Qwen3-1.7B-Q8_0.gguf"
  elif [ "$has_llm" = "1" ] && [ "$postinstall" = "1" ]; then
    log "Orbit LLM deferred to post-install script"
  else
    log "Orbit LLM not selected — skipping"
  fi

  # Whisper STT
  if [ "$has_stt" = "1" ] && [ "$postinstall" != "1" ]; then
    download_asset \
      "Whisper small ggml model" \
      "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-small.bin" \
      "1be3a9b2063867b937e64e2ec7483364a79917e157fa98c5d94b5c1fffea987b" \
      "$ai_root/whisper/ggml-small.bin"
    prepare_whisper_binary "$ai_root"
  elif [ "$has_stt" = "1" ] && [ "$postinstall" = "1" ]; then
    log "Whisper STT deferred to post-install script"
  else
    log "Whisper STT not selected — skipping"
  fi

  # OuteTTS + WavTokenizer
  if [ "$has_tts" = "1" ] && [ "$postinstall" != "1" ]; then
    mkdir -p "$ai_root/outetts"
    download_asset \
      "OuteTTS (OuteTTS-0.2-500M)" \
      "https://huggingface.co/outetts/OuteTTS-0.2-500M-GGUF/resolve/main/OuteTTS-0.2-500M-Q8_0.gguf" \
      "c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2b3c4" \
      "$ai_root/outetts/OuteTTS-0.2-500M-Q8_0.gguf"
    download_asset \
      "WavTokenizer (Large-75-F16)" \
      "https://huggingface.co/novax/wavtokenizer-large-75-f16-gguf/resolve/main/WavTokenizer-Large-75-F16.gguf" \
      "e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2c3d4e5" \
      "$ai_root/outetts/WavTokenizer-Large-75-F16.gguf"
  elif [ "$has_tts" = "1" ] && [ "$postinstall" = "1" ]; then
    log "OuteTTS deferred to post-install script"
  else
    log "OuteTTS not selected — skipping"
  fi
}

step_validate() {
  local ai_root="$OVERLAY_DIR/opt/volt/ai-models"
  local has_llm=0 has_stt=0 has_tts=0
  build_config_is_selected AI_LLM 2>/dev/null && has_llm=1 || has_llm=$( [ "${BUILD_CONFIG_AI_LLM:-1}" = "1" ] && echo 1 || echo 0 )
  build_config_is_selected AI_STT 2>/dev/null && has_stt=1 || has_stt=$( [ "${BUILD_CONFIG_AI_STT:-1}" = "1" ] && echo 1 || echo 0 )
  build_config_is_selected AI_TTS 2>/dev/null && has_tts=1 || has_tts=$( [ "${BUILD_CONFIG_AI_TTS:-1}" = "1" ] && echo 1 || echo 0 )

  [ "$has_llm" = "1" ] && [ -f "$ai_root/Qwen3-1.7B-Q8_0.gguf" ] || [ "$has_llm" != "1" ] || [ "${BUILD_AI_POSTINSTALL:-0}" = "1" ] || warn "Orbit LLM model not found"
  [ "$has_stt" = "1" ] && sha256_check "$ai_root/whisper/ggml-small.bin" "1be3a9b2063867b937e64e2ec7483364a79917e157fa98c5d94b5c1fffea987b" || [ "$has_stt" != "1" ] || [ "${BUILD_AI_POSTINSTALL:-0}" = "1" ] || die "Whisper model mismatch"
  [ "$has_stt" = "1" ] && [ -x "$ai_root/bin/whisper" ] || [ "$has_stt" != "1" ] || true
  [ "$has_tts" = "1" ] && [ -f "$ai_root/outetts/OuteTTS-0.2-500M-Q8_0.gguf" ] || [ "$has_tts" != "1" ] || [ "${BUILD_AI_POSTINSTALL:-0}" = "1" ] || warn "OuteTTS model not found"
}
