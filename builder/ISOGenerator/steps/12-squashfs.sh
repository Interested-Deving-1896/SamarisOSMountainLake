#!/usr/bin/env bash

copy_boot_files() {
  local arch_name="$1"
  local rootfs
  rootfs="$(rootfs_dir "$arch_name")"
  local live_dir
  live_dir="$(iso_live_dir "$arch_name")"
  mkdir -p "$live_dir"

  local kernel initrd
  kernel="$(find "$rootfs/boot" -maxdepth 1 -type f -name 'vmlinuz-*' | sort | tail -n 1)"
  initrd="$(find "$rootfs/boot" -maxdepth 1 -type f -name 'initrd.img-*' | sort | tail -n 1)"
  [ -n "$kernel" ] || die "No kernel found for $arch_name"
  [ -n "$initrd" ] || die "No initrd found for $arch_name"

  cp "$kernel" "$live_dir/vmlinuz"
  cp "$initrd" "$live_dir/initrd.img"
}

step_main() {
  rm -rf "$ISO_TREE/live"
  mkdir -p "$ISO_TREE/live"

  local arch_name rootfs live_dir
  for arch_name in $SAMARIS_ARCHES; do
    rootfs="$(rootfs_dir "$arch_name")"
    live_dir="$(iso_live_dir "$arch_name")"
    copy_boot_files "$arch_name"
    log "Creating SquashFS for $arch_name"
    mksquashfs "$rootfs" "$live_dir/filesystem.squashfs" -e boot -noappend -comp xz -b 1M
  done
}

step_validate() {
  local arch_name live_dir
  for arch_name in $SAMARIS_ARCHES; do
    live_dir="$(iso_live_dir "$arch_name")"
    [ -s "$live_dir/vmlinuz" ] || return 1
    [ -s "$live_dir/initrd.img" ] || return 1
    [ -s "$live_dir/filesystem.squashfs" ] || return 1
  done
}
