#!/usr/bin/env bash
set -euo pipefail
cargo build --release
sudo perf record -g ./target/release/volt-usb-manager --mount "$@"
sudo perf script | stackcollapse-perf | flamegraph > flamegraph.svg
