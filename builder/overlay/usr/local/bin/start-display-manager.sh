#!/usr/bin/env bash
set -euo pipefail

LOG="${VOLT_DISPLAY_MANAGER_LOG:-/tmp/volt-display-manager.log}"

log() { printf '[%s] %s\n' "$(date +%H:%M:%S)" "$*" >> "$LOG" 2>/dev/null || true; }
log "Starting Volt Display Manager: $*"

case "$(uname -m)" in
  x86_64|amd64) binary="/opt/volt/bin/volt-display-manager-x86_64" ;;
  aarch64|arm64) binary="/opt/volt/bin/volt-display-manager-aarch64" ;;
  *)
    log "Unsupported architecture: $(uname -m)"
    exit 0
    ;;
esac

if [ ! -x "$binary" ]; then
  log "Binary not found at $binary — skipping"
  exit 0
fi

if [ -z "${DISPLAY:-}" ]; then
  log "ERROR: DISPLAY not set — cannot proceed"
  exit 1
fi

log "Running: $binary $*"
exec "$binary" "$@" 2>> "$LOG"
