#!/usr/bin/env bash

step_main() {
  clean_ds_store
  local arch_name rootfs
  for arch_name in $SAMARIS_ARCHES; do
    rootfs="$(rootfs_dir "$arch_name")"
    log "Applying project overlay to $arch_name"
    rsync -aHAX "$OVERLAY_DIR"/ "$rootfs"/
    find "$rootfs/usr/local/bin" "$rootfs/opt/volt/bin" -type f -name "*.sh" -exec chmod 0755 {} + 2>/dev/null || true
    if [ -x "$rootfs/usr/sbin/update-initramfs" ]; then
      mount_chroot_runtime "$rootfs"
      run_chroot "$rootfs" update-initramfs -u -k all || warn "update-initramfs failed after overlay for $arch_name"
      umount_chroot_runtime "$rootfs"
    fi
    fix_live_user_home "$rootfs"
    enable_boot_units "$rootfs"
  done
}

fix_live_user_home() {
  local rootfs="$1"
  local home_dir="$rootfs/home/$LIVE_USER"
  [ -d "$home_dir" ] || return 0

  local uid gid
  uid="$(awk -F: -v user="$LIVE_USER" '$1 == user { print $3; exit }' "$rootfs/etc/passwd" 2>/dev/null || true)"
  gid="$(awk -F: -v user="$LIVE_USER" '$1 == user { print $4; exit }' "$rootfs/etc/passwd" 2>/dev/null || true)"
  if [ -z "$uid" ] || [ -z "$gid" ]; then
    warn "Could not resolve $LIVE_USER uid/gid in $rootfs"
    return 0
  fi

  chown -R "$uid:$gid" "$home_dir" || warn "Could not chown $home_dir to $uid:$gid"
  chmod 0755 "$home_dir" || warn "Could not chmod $home_dir"
}

enable_boot_units() {
  local rootfs="$1"
  local unit
  for unit in \
    volt-asc.service \
    volt-kernel-b.service \
    volt-ram-manager.service \
    volt-usb-manager.service \
    volt-worker-pool.service \
    volt-gpu-manager.service \
    volt-display-manager.service \
    volt-display-hotplug.service \
    volt-unifier.service \
    volt-kernel.service \
    volt-fs.service \
    nodm.service \
    NetworkManager.service \
    bluetooth.service \
    ModemManager.service \
    cups.service \
    cups-browsed.service \
    avahi-daemon.service \
    upower.service \
    acpid.service; do
    if [ -f "$rootfs/etc/systemd/system/$unit" ] || [ -f "$rootfs/lib/systemd/system/$unit" ] || [ -f "$rootfs/usr/lib/systemd/system/$unit" ]; then
      systemctl --root="$rootfs" enable "$unit" >/dev/null 2>&1 || warn "Could not enable $unit in $rootfs"
    fi
  done

  mkdir -p "$rootfs/etc/systemd/system"
  ln -sfn /lib/systemd/system/graphical.target "$rootfs/etc/systemd/system/default.target"
}

step_validate() {
  local arch_name rootfs
  for arch_name in $SAMARIS_ARCHES; do
    rootfs="$(rootfs_dir "$arch_name")"
    [ -x "$rootfs/usr/local/bin/volt-desktop" ] || return 1
    [ -x "$rootfs/usr/local/bin/samaris-xsession" ] || return 1
    [ -f "$rootfs/etc/default/nodm" ] || return 1
    [ -f "$rootfs/etc/systemd/system/volt-kernel.service" ] || return 1
    [ -e "$rootfs/etc/systemd/system/default.target" ] || return 1
  done
}
