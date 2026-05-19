#!/usr/bin/env bash
set -euo pipefail

ISO_GENERATOR_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

for lib in "$ISO_GENERATOR_ROOT"/lib/*.sh; do
  # shellcheck source=/dev/null
  source "$lib"
done

main "$@"
