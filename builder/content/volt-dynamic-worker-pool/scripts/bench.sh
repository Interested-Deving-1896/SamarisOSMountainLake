#!/usr/bin/env bash
set -euo pipefail

echo "✦ Running benchmarks..."
cargo bench --color always 2>&1
echo ""
echo "✦ Done."
