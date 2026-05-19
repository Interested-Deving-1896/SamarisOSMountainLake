#!/usr/bin/env bash

mount_chroot_runtime() {
  local rootfs="$1"
  mount_if_needed proc "$rootfs/proc" proc
  bind_mount_if_needed /sys "$rootfs/sys"
  bind_mount_if_needed /dev "$rootfs/dev"
  bind_mount_if_needed /dev/pts "$rootfs/dev/pts"
  if [ -e /etc/resolv.conf ]; then
    cp /etc/resolv.conf "$rootfs/etc/resolv.conf"
  fi
  # Provide CA certificates for SSL/TLS verification (apt-get update needs these)
  if [ -d /etc/ssl/certs ] && [ -d "$rootfs/etc/ssl/certs" ]; then
    if [ -z "$(ls -A "$rootfs/etc/ssl/certs" 2>/dev/null)" ]; then
      cp -r /etc/ssl/certs/* "$rootfs/etc/ssl/certs/" 2>/dev/null || true
    fi
  fi
}

umount_chroot_runtime() {
  local rootfs="$1"
  umount_if_mounted "$rootfs/dev/pts" || true
  umount_if_mounted "$rootfs/dev" || true
  umount_if_mounted "$rootfs/sys" || true
  umount_if_mounted "$rootfs/proc" || true
}

run_chroot() {
  local rootfs="$1"
  shift
  chroot "$rootfs" /usr/bin/env DEBIAN_FRONTEND=noninteractive "$@"
}
