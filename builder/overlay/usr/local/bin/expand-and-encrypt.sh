#!/usr/bin/env bash
set -euo pipefail

KERNEL_ROOT="${SAMARIS_KERNEL_ROOT:-/opt/volt/kernel}"
CLI="$KERNEL_ROOT/scripts/storage-cli.js"

if [[ ! -f "$CLI" ]]; then
  echo "[samaris-storage] storage-cli.js is missing at $CLI" >&2
  exit 1
fi

if [[ "${SAMARIS_STORAGE_MODE:-dry-run}" != "live" ]]; then
  echo "[samaris-storage] Refusing destructive storage setup outside live mode." >&2
  echo "[samaris-storage] Set SAMARIS_STORAGE_MODE=live only on a verified Samaris USB boot." >&2
  exit 1
fi

exec node "$CLI" setup
