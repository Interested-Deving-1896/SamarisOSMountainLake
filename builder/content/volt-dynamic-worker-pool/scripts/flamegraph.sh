#!/usr/bin/env bash
# Flamegraph Generation Script
# =============================
#
# Prerequisites:
#   cargo install flamegraph
#   perf (Linux) or dtrace/sample (macOS)
#
# Generate a flamegraph for the main binary:
#   cargo flamegraph --bin volt-dynamic-worker-pool -- --simulate-load
#
# Generate a flamegraph for a specific benchmark:
#   cargo flamegraph --bench scheduling_bench -- --profile-time 10
#
# Generate a flamegraph with custom output:
#   cargo flamegraph --bin volt-dynamic-worker-pool -o flamegraph.svg -- --simulate-load
#
# To use with perf on Linux (requires root):
#   sudo cargo flamegraph --bin volt-dynamic-worker-pool -- --simulate-load
#
# Example with frequency:
#   cargo flamegraph --bin volt-dynamic-worker-pool --freq 1000 -- --simulate-load
echo "✦ Usage: review the comments in this script for flamegraph commands."
echo ""
echo "    cargo flamegraph --bin volt-dynamic-worker-pool -- --simulate-load"
echo ""
