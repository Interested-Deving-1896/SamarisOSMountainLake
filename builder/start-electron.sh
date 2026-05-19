#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

MODE="${1:-dev}"
ELECTRON_DIR="$SCRIPT_DIR/content/electron"
UI_DIR="$SCRIPT_DIR/content/ui"
KERNEL_DIR="$SCRIPT_DIR/content/volt-kernel-a"

cleanup() {
  echo ""
  echo "[start] Shutting down..."
  [ -n "${RUST_PID:-}" ] && kill "$RUST_PID" 2>/dev/null || true
  [ -n "${VITE_PID:-}" ] && kill "$VITE_PID" 2>/dev/null || true
  [ -n "${ELECTRON_PID:-}" ] && kill "$ELECTRON_PID" 2>/dev/null || true
  exit 0
}
trap cleanup INT TERM

echo "[start] Samaris OS — Electron Launcher"

if ! command -v node &>/dev/null; then
  echo "[start] ERROR: Node.js is required"
  exit 1
fi

# Kill any lingering processes from previous sessions
for port in 9999 5173 3170; do
  for pid in $(lsof -ti:$port 2>/dev/null || true); do
    kill -9 "$pid" 2>/dev/null || true
  done
done
sleep 1

# Ensure consistent socket dir for dev mode
export VOLT_SOCKET_DIR="${VOLT_SOCKET_DIR:-/tmp/samaris}"

# Point ASC client to the locally compiled binary
CONTENT_DIR="$SCRIPT_DIR/content"
export VOLT_ASC_BIN="${VOLT_ASC_BIN:-$CONTENT_DIR/volt-adaptive-system-config/target/release/volt-asc}"

# Point Orbit to the Qwen model in the overlay
OVERLAY_DIR="$SCRIPT_DIR/overlay"
export VOLT_ORBIT_MODEL_PATH="${VOLT_ORBIT_MODEL_PATH:-$OVERLAY_DIR/opt/volt/ai-models/Qwen3-1.7B-Q8_0.gguf}"

# Point AI models (STT/TTS) to the overlay
export OUTETTS_MODEL="${OUTETTS_MODEL:-$OVERLAY_DIR/opt/volt/ai-models/outetts/OuteTTS-0.2-500M-Q8_0.gguf}"
export WAVTOKENIZER_MODEL="${WAVTOKENIZER_MODEL:-$OVERLAY_DIR/opt/volt/ai-models/outetts/WavTokenizer-Large-75-F16.gguf}"
export WHISPER_BIN="${WHISPER_BIN:-$OVERLAY_DIR/opt/volt/ai-models/bin/whisper}"
export WHISPER_MODEL="${WHISPER_MODEL:-$OVERLAY_DIR/opt/volt/ai-models/whisper/ggml-small.bin}"
export FFMPEG_BIN="${FFMPEG_BIN:-ffmpeg}"

# Start Volt Rust modules (ASC, Kernel B, VRM)
echo "[start] Starting Rust modules..."
"$SCRIPT_DIR/start-rust-modules.sh" &
RUST_PID=$!
sleep 1

# Ensure Demo folder is copied BEFORE kernel starts
echo "[start] Copying Demo to desktop..."
DEMO_SRC="$SCRIPT_DIR/../Demo"
KERNEL_DESKTOP="$SCRIPT_DIR/content/.volt/user/Desktop/Demo"
if [ -d "$DEMO_SRC" ]; then
  mkdir -p "$KERNEL_DESKTOP"
  cp -R "$DEMO_SRC"/* "$KERNEL_DESKTOP"/
  echo "[start] Demo folder ready"
else
  echo "[start] WARNING: Demo folder not found at $DEMO_SRC"
fi

echo "[start] Installing deps..."
[ -d "$KERNEL_DIR/node_modules" ] || (cd "$KERNEL_DIR" && npm install --silent 2>/dev/null) &
[ -d "$ELECTRON_DIR/node_modules" ] || (cd "$ELECTRON_DIR" && npm install --silent 2>/dev/null) &
[ -d "$UI_DIR/node_modules" ] || (cd "$UI_DIR" && npm install --silent 2>/dev/null) &
sleep 0.5

ELECTRON_BIN="$ELECTRON_DIR/node_modules/.bin/electron"
if [ ! -x "$ELECTRON_BIN" ]; then
  echo "[start] ERROR: Electron not found at $ELECTRON_BIN"
  echo "[start] Try: cd $ELECTRON_DIR && npm install"
  exit 1
fi

if [ "$MODE" = "dev" ]; then
  echo "[start] Starting Vite dev server..."
  cd "$UI_DIR"
  npm run dev &>/dev/null &
  VITE_PID=$!

  echo "[start] Waiting for Vite..."
  for i in $(seq 1 30); do
    if curl -sS http://localhost:5173 &>/dev/null; then
      echo "[start] Vite ready on http://localhost:5173"
      break
    fi
    sleep 0.3
  done

  echo "[start] Launching Electron (dev)..."
  cd "$ELECTRON_DIR"
  export NODE_ENV=development
  "$ELECTRON_BIN" . --no-sandbox &
  ELECTRON_PID=$!

else
  echo "[start] Building UI..."
  cd "$UI_DIR" && npm run build --silent

  echo "[start] Launching Electron (production)..."
  cd "$ELECTRON_DIR"
  export NODE_ENV=production
  "$ELECTRON_BIN" . --no-sandbox &
  ELECTRON_PID=$!
fi

echo "[start] PID: Vite=$VITE_PID Electron=$ELECTRON_PID"
wait
