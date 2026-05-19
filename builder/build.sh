#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
case "${1:-}" in
  help|-h|--help) exec "$SCRIPT_DIR/ISOGenerator/generator.sh" help ;;
  check|iso|status|steps|next|run|clean|qemu|tui|build) exec "$SCRIPT_DIR/ISOGenerator/generator.sh" "$@" ;;
  *) exec "$SCRIPT_DIR/ISOGenerator/generator.sh" "$@" ;;
esac
