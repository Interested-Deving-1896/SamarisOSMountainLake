#!/usr/bin/env bash

step_main() {
  local iso="$OUTPUT_DIR/$OUTPUT_ISO"
  [ -f "$iso" ] || die "ISO not found: $iso"
  local size
  size="$(du -h "$iso" | awk '{print $1}')"
  printf '\nSamaris OS Alpha One RC generated\n'
  printf 'ISO: %s\n' "$iso"
  printf 'Size: %s\n' "$size"
  printf 'SHA256: %s\n' "$(cut -d ' ' -f 1 "$iso.sha256")"
}

step_validate() {
  [ -s "$OUTPUT_DIR/$OUTPUT_ISO" ] && [ -s "$OUTPUT_DIR/$OUTPUT_ISO.sha256" ]
}
