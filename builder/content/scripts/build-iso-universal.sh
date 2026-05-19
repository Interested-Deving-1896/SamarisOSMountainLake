#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
GENERATOR_ROOT="$(cd "$SCRIPT_DIR/../../ISOGenerator" && pwd)"
exec "$GENERATOR_ROOT/generator.sh" iso "$@"
