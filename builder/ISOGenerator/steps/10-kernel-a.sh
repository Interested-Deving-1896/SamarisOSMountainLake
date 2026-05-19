#!/usr/bin/env bash

step_main() {
  local src="$CONTENT_ROOT/volt-kernel-a"
  [ -f "$src/package.json" ] || die "Kernel A package.json not found at $src"

  (
    cd "$src"
    npm install
  )

  mkdir -p "$OVERLAY_DIR/opt/volt/kernel"
  rsync -a --delete \
    --exclude='.git' \
    --exclude='logs' \
    "$src"/ "$OVERLAY_DIR/opt/volt/kernel"/

  local arch_name rootfs
  for arch_name in $SAMARIS_ARCHES; do
    rootfs="$(rootfs_dir "$arch_name")"
    mkdir -p "$rootfs/opt/volt/kernel"
    rsync -a --delete "$OVERLAY_DIR/opt/volt/kernel"/ "$rootfs/opt/volt/kernel"/
  done
}

step_validate() {
  local arch_name rootfs
  [ -f "$OVERLAY_DIR/opt/volt/kernel/server.js" ] || return 1
  for arch_name in $SAMARIS_ARCHES; do
    rootfs="$(rootfs_dir "$arch_name")"
    [ -f "$rootfs/opt/volt/kernel/server.js" ] || return 1
    [ -d "$rootfs/opt/volt/kernel/node_modules" ] || return 1
  done
}
