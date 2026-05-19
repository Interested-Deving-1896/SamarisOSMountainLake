#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
GENERATOR_ROOT="$(cd "$SCRIPT_DIR/../../ISOGenerator" && pwd)"
source "$GENERATOR_ROOT/lib/00-env.sh"
source "$GENERATOR_ROOT/lib/01-log.sh"
source "$GENERATOR_ROOT/lib/02-fs.sh"
source "$GENERATOR_ROOT/steps/02-ai-assets.sh"
step_main "$@"
