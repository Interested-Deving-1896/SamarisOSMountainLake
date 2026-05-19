#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SOCKET_DIR="${VOLT_SOCKET_DIR:-/tmp/samaris}"
CONTENT_DIR="$SCRIPT_DIR/content"
OVERLAY_DIR="$SCRIPT_DIR/overlay"

# Binary paths
KERNEL_B_BIN="$CONTENT_DIR/volt-kernel-b/target/release/tesseract-engine"
VRM_BIN="$CONTENT_DIR/volt-ram-manager/target/release/volt-ram-manager"
ASC_BIN="$CONTENT_DIR/volt-adaptive-system-config/target/release/volt-asc"
DWP_BIN="$CONTENT_DIR/volt-dynamic-worker-pool/target/release/volt-dynamic-worker-pool"
VGM_BIN="$CONTENT_DIR/volt-gpu-manager/target/release/volt-gpu-manager"
VUM_BIN="$CONTENT_DIR/volt-usb-manager/target/release/volt-usb-manager"
VDM_BIN="$CONTENT_DIR/volt-display-manager/target/release/volt-display-manager"

PIDS=""

cleanup() {
  echo ""; echo "[rust] Shutting down modules..."
  for pid in $PIDS; do kill "$pid" 2>/dev/null || true; done
  rm -rf "$SOCKET_DIR" 2>/dev/null || true
  exit 0
}
trap cleanup INT TERM

echo "[rust] Starting Volt Rust modules (socket dir: $SOCKET_DIR)"
mkdir -p "$SOCKET_DIR"
export VOLT_SOCKET_DIR="$SOCKET_DIR"

# ASC — generate config (runs once, exits)
ASC_OUTPUT="$SOCKET_DIR/adaptive.generated.toml"
if [ -x "$ASC_BIN" ]; then
  echo "[rust] ASC: generating config..."
  "$ASC_BIN" --config "$OVERLAY_DIR/opt/volt/asc/config.toml" --write "$ASC_OUTPUT" write &
  sleep 0.5
  if [ -f "$ASC_OUTPUT" ]; then
    echo "[rust] ASC: config written"
  fi
else
  echo "[rust] ASC: not found — skipping"
fi

# Kernel B — Tesseract Engine
if [ -x "$KERNEL_B_BIN" ]; then
  echo "[rust] Kernel B: starting..."
  "$KERNEL_B_BIN" --socket "$SOCKET_DIR/volt-kernel-b.sock" --config "$OVERLAY_DIR/opt/volt/kernel-b/config.toml" &
  PIDS="$PIDS $!"
  for i in $(seq 1 15); do
    if [ -S "$SOCKET_DIR/volt-kernel-b.sock" ]; then
      echo "[rust] Kernel B: ready"
      break
    fi
    sleep 0.3
  done
else
  echo "[rust] Kernel B: not found — skipping"
fi

# VRM — RAM Manager
if [ -x "$VRM_BIN" ]; then
  echo "[rust] VRM: starting..."
  "$VRM_BIN" --config "$OVERLAY_DIR/opt/volt/ram-manager/config.toml" &
  PIDS="$PIDS $!"
  echo "[rust] VRM: started"
else
  echo "[rust] VRM: not found — skipping"
fi

# DWP — Dynamic Worker Pool
if [ -x "$DWP_BIN" ]; then
  echo "[rust] DWP: starting..."
  "$DWP_BIN" --config "$OVERLAY_DIR/opt/volt/worker-pool/config.toml" &
  PIDS="$PIDS $!"
  echo "[rust] DWP: started"
else
  echo "[rust] DWP: not found — skipping"
fi

# VGM — GPU Manager
if [ -x "$VGM_BIN" ]; then
  echo "[rust] VGM: starting..."
  "$VGM_BIN" --config "$OVERLAY_DIR/opt/volt/gpu-manager/config.toml" &
  PIDS="$PIDS $!"
  echo "[rust] VGM: started"
else
  echo "[rust] VGM: not found — skipping"
fi

# VUM — USB Manager (without fuse feature)
if [ -x "$VUM_BIN" ]; then
  echo "[rust] VUM: starting..."
  "$VUM_BIN" --config "$OVERLAY_DIR/opt/volt/usb-manager/config.toml" --mount &
  PIDS="$PIDS $!"
  echo "[rust] VUM: started"
else
  echo "[rust] VUM: not found — skipping"
fi

# VDM — Display Manager (needs DISPLAY set)
if [ -x "$VDM_BIN" ]; then
  echo "[rust] VDM: detecting display..."
  if [ -n "${DISPLAY:-}" ]; then
    "$VDM_BIN" --apply --config-path "$SOCKET_DIR/display.generated.toml" --event-path "$SOCKET_DIR/display.event.json" &
    PIDS="$PIDS $!"
    echo "[rust] VDM: applied"
  else
    echo "[rust] VDM: no DISPLAY set — skipping display detection"
  fi
else
  echo "[rust] VDM: not found — skipping"
fi

echo "[rust] All modules running. PIDs: $PIDS"
wait
