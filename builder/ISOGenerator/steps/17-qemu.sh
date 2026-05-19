#!/usr/bin/env bash

step_main() {
  local iso="$OUTPUT_DIR/$OUTPUT_ISO"
  [ -f "$iso" ] || die "ISO not found: $iso"

  local qemu_bin="" arch="${1:-x86_64}"
  shift 2>/dev/null || true
  local ram="${QEMU_RAM:-4096}"
  local cpus="${QEMU_CPUS:-4}"
  local display="${QEMU_DISPLAY:-}"
  local network="${QEMU_NETWORK:-user}"
  local gpu_virtio="${QEMU_GPU_VIRTIO:-0}"
  local gpu_3d="${QEMU_GPU_3D:-0}"
  local disk_attach="${QEMU_DISK_ATTACH:-0}"
  local disk_size="${QEMU_DISK_SIZE:-8}"
  local qemu_args=()

  qemu_common_args() {
    qemu_args+=(-m "${ram}M" -smp "$cpus" -cdrom "$iso" -boot d)
    case "$display" in
      gtk|sdl) qemu_args+=(-display "$display") ;;
      vnc) qemu_args+=(-vnc :1) ;;
      headless) qemu_args+=(-display none -nographic) ;;
      "") ;;
      *) die "Unsupported QEMU display: $display" ;;
    esac
    [ "$gpu_virtio" = "1" ] && qemu_args+=(-vga virtio)
    [ "$gpu_3d" = "1" ] && qemu_args+=(-device virtio-gpu-pci)
    if [ "$disk_attach" = "1" ]; then
      qemu_args+=(-drive "file=samaris-disk-${disk_size}G.qcow2,format=qcow2")
    fi
    case "$network" in
      user) qemu_args+=(-netdev user,id=net0 -device virtio-net,netdev=net0) ;;
      tap) qemu_args+=(-netdev tap,id=net0 -device virtio-net,netdev=net0) ;;
      none|"") ;;
      *) die "Unsupported QEMU network: $network" ;;
    esac
    qemu_args+=("$@")
  }

  case "$arch" in
    x86_64|amd64)
      qemu_bin="qemu-system-x86_64"
      command -v "$qemu_bin" >/dev/null 2>&1 || die "qemu-system-x86_64 is required for x86_64"
      qemu_common_args "$@"
      "$qemu_bin" "${qemu_args[@]}"
      ;;
    aarch64|arm64)
      qemu_bin="qemu-system-aarch64"
      command -v "$qemu_bin" >/dev/null 2>&1 || die "qemu-system-aarch64 is required for aarch64"
      qemu_args=(-M virt -cpu max)
      qemu_common_args "$@"
      "$qemu_bin" "${qemu_args[@]}"
      ;;
    *)
      die "Unsupported QEMU arch: $arch (use x86_64 or aarch64)"
      ;;
  esac
}

step_validate() {
  command -v qemu-system-x86_64 >/dev/null 2>&1 || command -v qemu-system-aarch64 >/dev/null 2>&1
}
