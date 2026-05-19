#!/usr/bin/env bash

debian_arch() {
  case "$1" in
    x86_64) printf 'amd64\n' ;;
    aarch64|arm64) printf 'arm64\n' ;;
    *) die "Unsupported architecture: $1" ;;
  esac
}

rust_target() {
  case "$1" in
    x86_64) printf 'x86_64-unknown-linux-gnu\n' ;;
    aarch64|arm64) printf 'aarch64-unknown-linux-gnu\n' ;;
    *) die "Unsupported Rust architecture: $1" ;;
  esac
}

rootfs_dir() {
  printf '%s/%s\n' "$ROOTFS_BASE" "$1"
}

iso_live_dir() {
  printf '%s/live/%s\n' "$ISO_TREE" "$1"
}

arch_enabled() {
  local requested="$1"
  local arch_name
  for arch_name in $SAMARIS_ARCHES; do
    case "$requested:$arch_name" in
      x86_64:x86_64|aarch64:aarch64|aarch64:arm64|arm64:aarch64|arm64:arm64)
        return 0
        ;;
    esac
  done
  return 1
}

require_rust_toolchain_for_arch() {
  local arch_name="$1"
  local target
  target="$(rust_target "$arch_name")"

  command -v cargo >/dev/null 2>&1 || die "cargo is required to build Rust artifacts"
  command -v rustup >/dev/null 2>&1 || die "rustup is required to validate Rust targets"
  rustup target list --installed | grep -qx "$target" \
    || die "Missing Rust target $target. Rebuild the Docker builder image."

  if [ "$arch_name" = "aarch64" ] || [ "$arch_name" = "arm64" ]; then
    command -v aarch64-linux-gnu-gcc >/dev/null 2>&1 \
      || die "Missing aarch64-linux-gnu-gcc. Rebuild the Docker builder image."
    command -v aarch64-linux-gnu-g++ >/dev/null 2>&1 \
      || die "Missing aarch64-linux-gnu-g++. Rebuild the Docker builder image."
    export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER="${CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER:-aarch64-linux-gnu-gcc}"
    export CC_aarch64_unknown_linux_gnu="${CC_aarch64_unknown_linux_gnu:-aarch64-linux-gnu-gcc}"
    export CXX_aarch64_unknown_linux_gnu="${CXX_aarch64_unknown_linux_gnu:-aarch64-linux-gnu-g++}"
    export AR_aarch64_unknown_linux_gnu="${AR_aarch64_unknown_linux_gnu:-aarch64-linux-gnu-ar}"
  fi
}
