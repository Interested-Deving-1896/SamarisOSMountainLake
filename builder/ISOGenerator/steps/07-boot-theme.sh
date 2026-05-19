#!/usr/bin/env bash

step_main() {
  local arch_name rootfs
  for arch_name in $SAMARIS_ARCHES; do
    rootfs="$(rootfs_dir "$arch_name")"
    if [ -d "$CONTENT_ROOT/theme/plymouth" ]; then
      mkdir -p "$rootfs/usr/share/plymouth/themes/samaris"
      rsync -a "$CONTENT_ROOT/theme/plymouth"/ "$rootfs/usr/share/plymouth/themes/samaris"/
    fi
    if [ -x "$rootfs/usr/sbin/update-initramfs" ]; then
      mount_chroot_runtime "$rootfs"
      run_chroot "$rootfs" update-initramfs -u -k all || warn "update-initramfs failed for $arch_name"
      umount_chroot_runtime "$rootfs"
    fi
  done
}

step_validate() {
  local arch_name rootfs
  for arch_name in $SAMARIS_ARCHES; do
    rootfs="$(rootfs_dir "$arch_name")"
    [ -f "$rootfs/etc/plymouth/plymouthd.conf" ] || return 1
  done
}
