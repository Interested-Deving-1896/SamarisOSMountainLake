#!/bin/bash
set -euo pipefail
cargo clippy --all-targets --all-features && cargo fmt --check
