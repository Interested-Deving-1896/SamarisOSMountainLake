#!/usr/bin/env bash

step_main() {
  clean_ds_store
  safe_reset_dir "$WORK_DIR"
  mkdir -p "$ROOTFS_BASE" "$ISO_TREE" "$CACHE_DIR" "$OUTPUT_DIR"
  log "Workspace reset at $WORK_DIR"
}

step_validate() {
  [ -d "$ROOTFS_BASE" ] && [ -d "$ISO_TREE" ] && [ -d "$CACHE_DIR" ] && [ -d "$OUTPUT_DIR" ]
}
