#!/usr/bin/env bash
set -euo pipefail
cargo build --release
sudo samply record ./target/release/volt-usb-manager --mount "$@"
