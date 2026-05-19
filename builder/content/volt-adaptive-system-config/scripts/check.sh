#!/bin/bash
set -euo pipefail
echo "=== Checking ==="
cargo fmt --check && echo "fmt OK" || echo "fmt FAIL"
cargo clippy --all-targets --all-features && echo "clippy OK" || echo "clippy FAIL"
cargo test --all-features && echo "test OK" || echo "test FAIL"
cargo build --release && echo "build OK" || echo "build FAIL"
echo "=== All checks done ==="
