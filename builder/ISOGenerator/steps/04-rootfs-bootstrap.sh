#!/usr/bin/env bash

debootstrap_wgetrc() {
  local wgetrc="$CACHE_DIR/debootstrap-wgetrc"
  mkdir -p "$CACHE_DIR"
  cat > "$wgetrc" <<'EOF'
timeout = 30
dns_timeout = 10
connect_timeout = 10
read_timeout = 30
tries = 3
waitretry = 2
retry_connrefused = on
EOF
  printf '%s\n' "$wgetrc"
}

run_debootstrap() {
  local wgetrc cache_dir timeout_value
  wgetrc="$(debootstrap_wgetrc)"
  cache_dir="$CACHE_DIR/debootstrap"
  timeout_value="${DEBOOTSTRAP_TIMEOUT:-45m}"
  mkdir -p "$cache_dir"
  if command -v timeout >/dev/null 2>&1; then
    WGETRC="$wgetrc" timeout --foreground "$timeout_value" debootstrap --verbose --cache-dir="$cache_dir" "$@"
  else
    WGETRC="$wgetrc" debootstrap --verbose --cache-dir="$cache_dir" "$@"
  fi
}

bootstrap_arch() {
  local arch_name="$1"
  local deb_arch
  deb_arch="$(debian_arch "$arch_name")"
  local rootfs
  rootfs="$(rootfs_dir "$arch_name")"

  safe_reset_dir "$rootfs"
  log "Bootstrapping $arch_name rootfs ($deb_arch)"

  if [ "$deb_arch" = "amd64" ]; then
    run_debootstrap --variant="${DEBOOTSTRAP_VARIANT:-minbase}" --include=libstdc++6,libtinfo6,libselinux1,libpcre2-8-0,libpam-modules,libpam-runtime --arch=amd64 "$DEBIAN_SUITE" "$rootfs" "$DEBIAN_MIRROR"
  else
    command -v qemu-aarch64-static >/dev/null 2>&1 || die "qemu-aarch64-static is required for ARM64 rootfs"
    run_debootstrap --variant="${DEBOOTSTRAP_VARIANT:-minbase}" --include=libstdc++6,libtinfo6,libselinux1,libpcre2-8-0,libpam-modules,libpam-runtime --arch=arm64 --foreign "$DEBIAN_SUITE" "$rootfs" "$DEBIAN_MIRROR"
    cp "$(command -v qemu-aarch64-static)" "$rootfs/usr/bin/"
    if command -v timeout >/dev/null 2>&1; then
      timeout --foreground "${DEBOOTSTRAP_SECOND_STAGE_TIMEOUT:-45m}" chroot "$rootfs" /debootstrap/debootstrap --second-stage
    else
      chroot "$rootfs" /debootstrap/debootstrap --second-stage
    fi
  fi

  printf 'samaris\n' > "$rootfs/etc/hostname"
  cat > "$rootfs/etc/hosts" <<'EOF'
127.0.0.1 localhost
127.0.1.1 samaris
EOF
}

step_main() {
  local arch_name
  for arch_name in $SAMARIS_ARCHES; do
    bootstrap_arch "$arch_name"
  done
}

step_validate() {
  local arch_name rootfs
  for arch_name in $SAMARIS_ARCHES; do
    rootfs="$(rootfs_dir "$arch_name")"
    [ -x "$rootfs/bin/bash" ] || return 1
    [ -f "$rootfs/etc/debian_version" ] || return 1
  done
}
