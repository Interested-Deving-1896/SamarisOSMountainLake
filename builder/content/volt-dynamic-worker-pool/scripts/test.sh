#!/usr/bin/env bash
set -euo pipefail

echo "✦ Running all tests (all features)..."
cargo test --all-features --color always 2>&1
echo ""
echo "✦ Done."
