#!/usr/bin/env bash

step_main() {
  build_config_skip "ui" 2>/dev/null && { log "Desktop UI not selected — skipping"; return 0; }

  local src="$CONTENT_ROOT/ui"
  [ -f "$src/package.json" ] || die "UI package.json not found at $src"

  (
    cd "$src"
    npm install
    npm run build
  )

  mkdir -p "$OVERLAY_DIR/opt/volt/desktop/app"
  rsync -a --delete "$src/dist"/ "$OVERLAY_DIR/opt/volt/desktop/app"/

  local arch_name rootfs
  for arch_name in $SAMARIS_ARCHES; do
    rootfs="$(rootfs_dir "$arch_name")"
    mkdir -p "$rootfs/opt/volt/desktop/app"
    rsync -a --delete "$src/dist"/ "$rootfs/opt/volt/desktop/app"/
  done
}

step_validate() {
  build_config_skip "ui" 2>/dev/null && return 0
  local arch_name rootfs
  [ -f "$OVERLAY_DIR/opt/volt/desktop/app/index.html" ] || return 1
  for arch_name in $SAMARIS_ARCHES; do
    rootfs="$(rootfs_dir "$arch_name")"
    [ -f "$rootfs/opt/volt/desktop/app/index.html" ] || return 1
  done
}
