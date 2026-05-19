#!/usr/bin/env bash
set -euo pipefail

# Volt Dynamic Worker Pool launcher — detects architecture
case "$(uname -m)" in
  x86_64|amd64) binary="/opt/volt/bin/volt-dynamic-worker-pool-x86_64" ;;
  aarch64|arm64) binary="/opt/volt/bin/volt-dynamic-worker-pool-aarch64" ;;
  *)
    echo "Unsupported architecture for Volt Dynamic Worker Pool: $(uname -m)" >&2
    exit 0
    ;;
esac

if [ -x "$binary" ]; then
  exec "$binary" --config /opt/volt/worker-pool/config.toml run "$@"
fi

echo "Volt Dynamic Worker Pool not found at $binary — continuing without adaptive scheduler" >&2
exit 0
