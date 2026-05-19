#!/usr/bin/env bash

validate_step_inventory() {
  local -A known_steps=()
  local step script name
  local missing=()

  for step in "${BUILD_STEPS[@]}" "17-qemu"; do
    known_steps["$step"]=1
  done

  while IFS= read -r script; do
    name="$(basename "$script" .sh)"
    if [ -z "${known_steps[$name]:-}" ]; then
      missing+=("$name")
    fi
  done < <(find "$STEPS_DIR" -maxdepth 1 -type f -name "*.sh" -print | sort)

  if [ "${#missing[@]}" -gt 0 ]; then
    die "Step scripts not registered in BUILD_STEPS or explicit exemptions: ${missing[*]}"
  fi
}

step_main() {
  mkdir -p "$WORK_DIR" "$CACHE_DIR" "$OUTPUT_DIR"

  if [ "${BASH_VERSINFO[0]:-0}" -lt 4 ]; then
    die "Bash >= 4.x is required. Use ./run.sh iso --docker on macOS."
  fi

  validate_step_inventory

  local required=(
    debootstrap xorriso mksquashfs rsync curl git grub-mkstandalone
    mformat mmd mcopy chroot mount umount dd tar unsquashfs
    unzip readelf
  )
  local tool
  for tool in "${required[@]}"; do
    command -v "$tool" >/dev/null 2>&1 || die "Missing required tool: $tool"
  done
  command -v sha256sum >/dev/null 2>&1 || command -v shasum >/dev/null 2>&1 || die "Missing required tool: sha256sum or shasum"

  if arch_enabled x86_64; then
    find_first \
      /usr/lib/ISOLINUX/isolinux.bin \
      /usr/lib/syslinux/modules/bios/isolinux.bin \
      >/dev/null || die "Missing isolinux.bin. Install isolinux/syslinux-common."

    find_first \
      /usr/lib/ISOLINUX/isohdpfx.bin \
      /usr/lib/syslinux/mbr/isohdpfx.bin \
      >/dev/null || die "Missing isohdpfx.bin. Install isolinux/syslinux-common."

    [ -f /usr/lib/grub/x86_64-efi/modinfo.sh ] || die "Missing GRUB x86_64 EFI modules."
  fi

  if arch_enabled aarch64; then
    [ -f /usr/lib/grub/arm64-efi/modinfo.sh ] || die "Missing GRUB ARM64 EFI modules. Install grub-efi-arm64-bin in the builder image."
  fi

  if arch_enabled aarch64; then
    require_rust_toolchain_for_arch aarch64
  fi

  if ! command -v qemu-system-x86_64 >/dev/null 2>&1; then
    warn "qemu-system-x86_64 is missing; ISO build can continue, VM test cannot."
  fi

  local free_kb
  free_kb="$(df -Pk "$WORK_DIR" | awk 'NR==2 {print $4}')"
  [ "${free_kb:-0}" -ge 10485760 ] || die "At least 10 GB free space is required on $WORK_DIR"

  if [ "$(id -u)" -ne 0 ] && [ "${SAMARIS_IN_DOCKER:-0}" != "1" ]; then
    sudo -n true >/dev/null 2>&1 || die "sudo is required for chroot/mount steps. Run with sudo or use ./run.sh iso --docker."
  fi

  log "Environment OK for universal ISO build"
}

step_validate() {
  validate_step_inventory
}
