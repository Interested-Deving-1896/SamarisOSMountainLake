#!/usr/bin/env bash
set -euo pipefail

case "$(uname -m)" in
  x86_64|amd64) binary="/opt/volt/bin/volt-gpu-manager-x86_64" ;;
  aarch64|arm64) binary="/opt/volt/bin/volt-gpu-manager-aarch64" ;;
  *)
    echo "Unsupported architecture for Volt GPU Manager: $(uname -m)" >&2
    exit 0
    ;;
esac

if [ -x "$binary" ]; then
  exec "$binary" --config /opt/volt/gpu-manager/config.toml "$@"
fi

echo "Volt GPU Manager not found at $binary" >&2
exit 0
