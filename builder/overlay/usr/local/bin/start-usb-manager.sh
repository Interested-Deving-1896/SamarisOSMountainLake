#!/usr/bin/env bash
set -euo pipefail

case "$(uname -m)" in
  x86_64|amd64) binary="/opt/volt/bin/volt-usb-manager-x86_64" ;;
  aarch64|arm64) binary="/opt/volt/bin/volt-usb-manager-aarch64" ;;
  *)
    echo "Unsupported architecture for Volt USB Manager: $(uname -m)" >&2
    exit 0
    ;;
esac

if [ -x "$binary" ]; then
  mkdir -p /run/samaris /mnt/samaris-usb /run/media/samaris/USB /var/lib/samaris/volt-usb-manager
  exec "$binary" --config /opt/volt/usb-manager/config.toml --mount "$@"
fi

echo "Volt USB Manager not found at $binary" >&2
exit 0
