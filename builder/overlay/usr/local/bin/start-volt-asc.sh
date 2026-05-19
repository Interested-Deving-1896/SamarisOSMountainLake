#!/usr/bin/env bash
set -euo pipefail

# Volt Adaptive System Config launcher — detects architecture, generates module configs
case "$(uname -m)" in
  x86_64|amd64) binary="/opt/volt/bin/volt-asc-x86_64" ;;
  aarch64|arm64) binary="/opt/volt/bin/volt-asc-aarch64" ;;
  *)
    echo "Unsupported architecture for Volt ASC: $(uname -m)" >&2
    exit 0
    ;;
esac

if [ -x "$binary" ]; then
  mkdir -p /run/samaris /var/lib/samaris/asc
  exec "$binary" --config /opt/volt/asc/config.toml write
fi

echo "Volt ASC not found at $binary — generating without adaptive configuration" >&2
exit 0
