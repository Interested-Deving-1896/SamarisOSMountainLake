#!/usr/bin/env bash

apply_modules_for_arch() {
  local arch_name="$1"
  local rootfs
  rootfs="$(rootfs_dir "$arch_name")"
  local module overlay

  for module in $ENABLED_MODULES; do
    overlay="$MODULES_DIR/$module/overlay"
    if [ -d "$overlay" ]; then
      log "Applying module overlay $module to $arch_name"
      rsync -aHAX "$overlay"/ "$rootfs"/
    fi
  done
}

step_main() {
  local arch_name
  for arch_name in $SAMARIS_ARCHES; do
    apply_modules_for_arch "$arch_name"
  done
}

step_validate() {
  local arch_name rootfs
  for arch_name in $SAMARIS_ARCHES; do
    rootfs="$(rootfs_dir "$arch_name")"
    [ -d "$rootfs" ] || return 1
  done
}
