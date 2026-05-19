#!/usr/bin/env bash
set -euo pipefail

# Volt RAM Manager launcher — detects architecture
case "$(uname -m)" in
  x86_64|amd64) binary="/opt/volt/bin/volt-ram-manager-x86_64" ;;
  aarch64|arm64) binary="/opt/volt/bin/volt-ram-manager-aarch64" ;;
  *)
    echo "Unsupported architecture for Volt RAM Manager: $(uname -m)" >&2
    exit 0
    ;;
esac

if [ -x "$binary" ]; then
  exec "$binary" --config /opt/volt/ram-manager/config.toml "$@"
fi

echo "Volt RAM Manager not found at $binary — continuing without memory manager" >&2
exit 0
