#!/usr/bin/env bash

fix_rootfs_libraries() {
  local rootfs="$1" deb_arch="$2"
  local cache_dir="$CACHE_DIR/debootstrap" count=0

  if [ -d "$cache_dir" ]; then
    for deb in "$cache_dir"/*_"$deb_arch".deb "$cache_dir"/*_all.deb; do
      [ -f "$deb" ] || continue
      local tmpdir="/tmp/deb-fix-$$"
      mkdir -p "$tmpdir"
      cp "$deb" "$tmpdir/pkg.deb" 2>/dev/null || continue
      (cd "$tmpdir" && ar x pkg.deb 2>/dev/null)
      for tarf in "$tmpdir"/data.tar.*; do
        [ -f "$tarf" ] && tar xf "$tarf" -C "$rootfs" 2>/dev/null
      done
      rm -rf "$tmpdir"
      count=$((count + 1))
    done
  fi

  if [ -d "$rootfs/var/cache/apt/archives" ]; then
    for deb in "$rootfs/var/cache/apt/archives/"*.deb; do
      [ -f "$deb" ] || continue
      local tmpdir="/tmp/deb-fix-apt-$$"
      mkdir -p "$tmpdir"
      cp "$deb" "$tmpdir/pkg.deb" 2>/dev/null || continue
      (cd "$tmpdir" && ar x pkg.deb 2>/dev/null)
      for tarf in "$tmpdir"/data.tar.*; do
        [ -f "$tarf" ] && tar xf "$tarf" -C "$rootfs" 2>/dev/null
      done
      rm -rf "$tmpdir"
      count=$((count + 1))
    done
  fi

  log "fix_rootfs_libraries: extracted $count .deb packages for $deb_arch"
}

collect_packages() {
  local arch_name="$1"
  local deb_arch
  deb_arch="$(debian_arch "$arch_name")"
  local module file

  for module in $ENABLED_MODULES; do
    for file in \
      "$MODULES_DIR/$module/packages.list" \
      "$MODULES_DIR/$module/packages.$deb_arch.list" \
      "$MODULES_DIR/$module/packages.$arch_name.list"; do
      if [ -f "$file" ]; then
        sed -e 's/#.*$//' -e '/^[[:space:]]*$/d' "$file"
      fi
    done
  done | filter_service_packages
}

filter_service_packages() {
  local pkg
  while IFS= read -r pkg; do
    case "$pkg" in
      network-manager|network-manager-*|bluetooth|bluez|bluez-*|modemmanager|libspa-0.2-bluetooth)
        [ "${BUILD_CONFIG_SVC_NETWORK:-1}" = "1" ] || continue
        ;;
      cups|cups-*|avahi-daemon|printer-driver-*|libcups2)
        [ "${BUILD_CONFIG_SVC_CUPS:-1}" = "1" ] || continue
        ;;
      upower|acpid)
        [ "${BUILD_CONFIG_SVC_POWER:-1}" = "1" ] || continue
        ;;
    esac
    printf '%s\n' "$pkg"
  done
}

install_packages_for_arch() {
  local arch_name="$1"
  local deb_arch
  deb_arch="$(debian_arch "$arch_name")"
  local rootfs
  rootfs="$(rootfs_dir "$arch_name")"
  local pkg_file="$WORK_DIR/packages-$arch_name.list"

  collect_packages "$arch_name" | sort -u > "$pkg_file"
  log "Installing $(wc -l < "$pkg_file" | tr -d ' ') packages for $arch_name"

  local apt_cache_dir="$CACHE_DIR/apt-archives"
  local apt_lists_dir="$CACHE_DIR/apt-lists"
  mkdir -p "$apt_cache_dir" "$apt_lists_dir"
  mkdir -p "$rootfs/var/cache/apt/archives/partial" "$rootfs/var/lib/apt/lists/partial"

  fix_rootfs_libraries "$rootfs" "$deb_arch"

  cat > "$rootfs/etc/apt/sources.list" <<EOF
deb $DEBIAN_MIRROR $DEBIAN_SUITE main contrib non-free non-free-firmware
deb $DEBIAN_MIRROR $DEBIAN_SUITE-updates main contrib non-free non-free-firmware
deb http://security.debian.org/debian-security $DEBIAN_SUITE-security main contrib non-free non-free-firmware
EOF

  SAMARIS_PACKAGE_ROOTFS="$rootfs"
  cleanup() {
    umount_if_mounted "$rootfs/var/cache/apt/archives"
    umount_if_mounted "$rootfs/var/lib/apt/lists"
    umount_chroot_runtime "$SAMARIS_PACKAGE_ROOTFS" || true
  }
  trap cleanup EXIT

  mount_chroot_runtime "$rootfs"
  bind_mount_if_needed "$apt_cache_dir" "$rootfs/var/cache/apt/archives"
  bind_mount_if_needed "$apt_lists_dir" "$rootfs/var/lib/apt/lists"

  run_chroot "$rootfs" apt-get update
  if [ -s "$pkg_file" ]; then
    local packages=()
    mapfile -t packages < "$pkg_file"
    run_chroot "$rootfs" apt-get install -y --no-install-recommends "${packages[@]}"
  fi

  run_chroot "$rootfs" bash -lc "useradd -m -s /bin/bash '$LIVE_USER' || true"
  run_chroot "$rootfs" bash -lc "passwd -d '$LIVE_USER' || true"
  run_chroot "$rootfs" install -d -m 0755 /etc/sudoers.d
  run_chroot "$rootfs" bash -lc "echo '$LIVE_USER ALL=(ALL) NOPASSWD:ALL' > /etc/sudoers.d/90-samaris-live"
  run_chroot "$rootfs" chmod 0440 /etc/sudoers.d/90-samaris-live

  cleanup
  trap - EXIT
  unset SAMARIS_PACKAGE_ROOTFS
}

step_main() {
  local arch_name
  for arch_name in $SAMARIS_ARCHES; do
    install_packages_for_arch "$arch_name"
  done
}

step_validate() {
  local arch_name rootfs
  for arch_name in $SAMARIS_ARCHES; do
    rootfs="$(rootfs_dir "$arch_name")"
    [ -x "$rootfs/usr/bin/node" ] || return 1
    [ -x "$rootfs/usr/bin/npm" ] || return 1
    [ -d "$rootfs/lib/modules" ] || return 1
    if build_config_is_selected DESKTOP_UI; then
      [ -x "$rootfs/usr/bin/xinit" ] || return 1
    fi
    if build_config_is_selected SVC_NETWORK; then
      [ -x "$rootfs/usr/sbin/NetworkManager" ] || return 1
    fi
  done
}
