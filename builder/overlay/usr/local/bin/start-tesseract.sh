#!/usr/bin/env bash
set -euo pipefail

# Tesseract Engine launcher — detects architecture and starts Kernel B
case "$(uname -m)" in
  x86_64|amd64) binary="/opt/volt/bin/tesseract-engine-x86_64" ;;
  aarch64|arm64) binary="/opt/volt/bin/tesseract-engine-aarch64" ;;
  *)
    echo "Unsupported architecture for Tesseract Engine: $(uname -m)" >&2
    exit 0
    ;;
esac

if [ -x "$binary" ]; then
  mkdir -p /run/samaris
  exec "$binary" --boot-mode --socket /run/samaris/volt-kernel-b.sock "$@"
fi

echo "Tesseract Engine not found at $binary — Kernel A fallback remains active" >&2
exit 0
