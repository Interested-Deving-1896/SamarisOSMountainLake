#!/bin/bash
set -euo pipefail
echo "=== Generating example config ==="
cargo run --release -- --dry-run 2>&1
echo "=== Done ==="
