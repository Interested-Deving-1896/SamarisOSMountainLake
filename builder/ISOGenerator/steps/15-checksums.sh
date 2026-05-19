#!/usr/bin/env bash

step_main() {
  local iso="$OUTPUT_DIR/$OUTPUT_ISO"
  [ -f "$iso" ] || die "ISO not found: $iso"
  if command -v sha256sum >/dev/null 2>&1; then
    (cd "$OUTPUT_DIR" && sha256sum "$OUTPUT_ISO" > "$OUTPUT_ISO.sha256")
  else
    (cd "$OUTPUT_DIR" && shasum -a 256 "$OUTPUT_ISO" > "$OUTPUT_ISO.sha256")
  fi
  log "Checksum written to $iso.sha256"
}

step_validate() {
  local iso="$OUTPUT_DIR/$OUTPUT_ISO"
  [ -s "$iso" ] || return 1
  [ -s "$iso.sha256" ] || return 1
  if command -v sha256sum >/dev/null 2>&1; then
    (cd "$OUTPUT_DIR" && sha256sum -c "$OUTPUT_ISO.sha256" >/dev/null 2>&1)
  else
    local expected actual
    expected="$(awk '{print $1}' "$iso.sha256")"
    actual="$(shasum -a 256 "$iso" | awk '{print $1}')"
    [ "$expected" = "$actual" ]
  fi
}
