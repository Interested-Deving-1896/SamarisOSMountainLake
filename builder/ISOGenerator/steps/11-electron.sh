#!/usr/bin/env bash

electron_zip_arch() {
  case "$1" in
    x86_64) printf 'x64\n' ;;
    aarch64|arm64) printf 'arm64\n' ;;
    *) die "Unsupported Electron architecture: $1" ;;
  esac
}

electron_machine() {
  case "$1" in
    x86_64) printf 'Advanced Micro Devices X86-64\n' ;;
    aarch64|arm64) printf 'AArch64\n' ;;
    *) die "Unsupported Electron architecture: $1" ;;
  esac
}

find_cached_electron_zip() {
  local rootfs="$1" arch_name="$2"
  local zip_arch cache_dir found
  zip_arch="$(electron_zip_arch "$arch_name")"
  for cache_dir in "$rootfs/root/.cache/electron" "$CACHE_DIR/electron"; do
    [ -d "$cache_dir" ] || continue
    found=$(find "$cache_dir" -type f -name "electron-v*-linux-$zip_arch.zip" 2>/dev/null | sort -V | tail -n 1)
    [ -n "$found" ] && { printf '%s\n' "$found"; return 0; }
  done
  return 1
}

install_electron_dist_from_cache() {
  local rootfs="$1" arch_name="$2"
  local electron_dir="$rootfs/opt/volt/electron/node_modules/electron"
  local dist_dir="$electron_dir/dist"
  local zip machine

  zip=$(find_cached_electron_zip "$rootfs" "$arch_name") \
    || die "Missing cached Electron zip for $arch_name. Place under $CACHE_DIR/electron."

  log "Installing cached Electron runtime for $arch_name from $zip"
  rm -rf "$dist_dir"
  mkdir -p "$dist_dir"
  unzip -q "$zip" -d "$dist_dir"
  printf 'electron\n' > "$electron_dir/path.txt"
  chmod 0755 "$dist_dir/electron" 2>/dev/null || true
  chmod 4755 "$dist_dir/chrome-sandbox" 2>/dev/null || true

  machine=$(electron_machine "$arch_name")
  readelf -h "$dist_dir/electron" | grep -q "Machine:.*$machine" \
    || die "Cached Electron runtime for $arch_name is not built for $machine"
}

step_main() {
  build_config_skip "electron" 2>/dev/null && { log "Browser not selected — skipping"; return 0; }

  local src="$CONTENT_ROOT/electron"
  [ -f "$src/package.json" ] || die "Electron package.json not found at $src"

  mkdir -p "$OVERLAY_DIR/opt/volt/electron"
  rsync -a --delete \
    --exclude='.git' --exclude='dist' --exclude='node_modules' \
    "$src"/ "$OVERLAY_DIR/opt/volt/electron"/

  local arch_name rootfs
  for arch_name in $SAMARIS_ARCHES; do
    rootfs="$(rootfs_dir "$arch_name")"
    log "Installing Electron for $arch_name inside target rootfs"
    mkdir -p "$rootfs/opt/volt/electron"
    rsync -a --delete \
      --exclude='node_modules' --exclude='dist' \
      "$src"/ "$rootfs/opt/volt/electron"/

    SAMARIS_ELECTRON_ROOTFS="$rootfs"
    trap 'umount_chroot_runtime "$SAMARIS_ELECTRON_ROOTFS" || true' EXIT
    mount_chroot_runtime "$rootfs"
    run_chroot "$rootfs" bash -lc 'cd /opt/volt/electron && npm install --include=dev --unsafe-perm --no-audit --fund=false'
    install_electron_dist_from_cache "$rootfs" "$arch_name"
    run_chroot "$rootfs" bash -lc 'cd /opt/volt/electron && electron_version="$(sed "s/^v//" node_modules/electron/dist/version)" && npx electron-rebuild -f -v "$electron_version"'

    # Remove electron install.js to prevent it from re-downloading the binary at boot
    rm -f "$rootfs/opt/volt/electron/node_modules/electron/install.js" 2>/dev/null || true
    # Also neuter the electron index.js entry point — make it a no-op binary path resolver
    echo "module.exports = require('path').join(__dirname, 'dist', 'electron');" \
      > "$rootfs/opt/volt/electron/node_modules/electron/index.js" 2>/dev/null || true

    run_chroot "$rootfs" bash -lc 'npm cache clean --force >/dev/null 2>&1 || true'
    umount_chroot_runtime "$rootfs"
    trap - EXIT
    unset SAMARIS_ELECTRON_ROOTFS
  done
}

step_validate() {
  build_config_skip "electron" 2>/dev/null && return 0
  local arch_name rootfs
  for arch_name in $SAMARIS_ARCHES; do
    rootfs="$(rootfs_dir "$arch_name")"
    [ -f "$rootfs/opt/volt/electron/main.js" ] || return 1
    [ -x "$rootfs/opt/volt/electron/node_modules/.bin/electron" ] || return 1
    [ -f "$rootfs/opt/volt/electron/node_modules/electron/path.txt" ] || return 1
    [ -x "$rootfs/opt/volt/electron/node_modules/electron/dist/electron" ] || return 1
    readelf -h "$rootfs/opt/volt/electron/node_modules/electron/dist/electron" \
      | grep -q "Machine:.*$(electron_machine "$arch_name")" || return 1
    readelf -h "$rootfs/opt/volt/electron/node_modules/node-pty/build/Release/pty.node" \
      | grep -q "Machine:.*$(electron_machine "$arch_name")" || return 1
    [ -d "$rootfs/opt/volt/electron/node_modules/node-pty" ] || return 1
  done
}
