#!/usr/bin/env bash
set -euo pipefail

echo "✦ Building release binary..."
cargo build --release 2>&1
echo ""
echo "✦ Profiling (time + simulate-load)..."
time ./target/release/volt-dynamic-worker-pool --simulate-load
echo ""
echo "✦ Done."
