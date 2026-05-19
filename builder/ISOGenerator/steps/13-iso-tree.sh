#!/usr/bin/env bash

write_grub_cfg() {
  mkdir -p "$ISO_TREE/boot/grub"
  cat > "$ISO_TREE/boot/grub/grub.cfg" <<EOF
search --no-floppy --set=root --label $ISO_LABEL

set timeout=10
set default=0
EOF

  if arch_enabled x86_64 && arch_enabled aarch64; then
    cat >> "$ISO_TREE/boot/grub/grub.cfg" <<'EOF'

if [ "$grub_cpu" = "arm64" ]; then
  set samaris_arch="aarch64"
  set samaris_console="console=ttyAMA0,115200n8"
  set samaris_tty="ttyAMA0"
else
  set samaris_arch="x86_64"
  set samaris_console="console=ttyS0,115200n8"
  set samaris_tty="ttyS0"
fi

menuentry "Samaris OS (auto)" {
  linux /live/$samaris_arch/vmlinuz boot=live components ${samaris_console} loglevel=5 systemd.show_status=1 live-media-path=/live/$samaris_arch
  initrd /live/$samaris_arch/initrd.img
}
EOF
  elif arch_enabled x86_64; then
    cat >> "$ISO_TREE/boot/grub/grub.cfg" <<'EOF'

set samaris_arch="x86_64"
set samaris_console="console=ttyS0,115200n8"
set samaris_tty="ttyS0"

menuentry "Samaris OS (auto)" {
  linux /live/x86_64/vmlinuz boot=live components console=ttyS0,115200n8 loglevel=5 systemd.show_status=1 live-media-path=/live/x86_64
  initrd /live/x86_64/initrd.img
}
EOF
  elif arch_enabled aarch64; then
    cat >> "$ISO_TREE/boot/grub/grub.cfg" <<'EOF'

set samaris_arch="aarch64"
set samaris_console="console=ttyAMA0,115200n8"
set samaris_tty="ttyAMA0"

menuentry "Samaris OS (auto)" {
  linux /live/aarch64/vmlinuz boot=live components console=ttyAMA0,115200n8 loglevel=5 systemd.show_status=1 live-media-path=/live/aarch64
  initrd /live/aarch64/initrd.img
}
EOF
  fi

  if arch_enabled x86_64; then
    cat >> "$ISO_TREE/boot/grub/grub.cfg" <<'EOF'

menuentry "Samaris OS x86_64" {
  linux /live/x86_64/vmlinuz boot=live components console=ttyS0,115200n8 loglevel=5 systemd.show_status=1 live-media-path=/live/x86_64
  initrd /live/x86_64/initrd.img
}
EOF
  fi

  if arch_enabled aarch64; then
    cat >> "$ISO_TREE/boot/grub/grub.cfg" <<'EOF'

menuentry "Samaris OS ARM64" {
  linux /live/aarch64/vmlinuz boot=live components console=ttyAMA0,115200n8 loglevel=5 systemd.show_status=1 live-media-path=/live/aarch64
  initrd /live/aarch64/initrd.img
}

menuentry "Samaris OS ARM64 (debug - verbose, no plymouth)" {
  linux /live/aarch64/vmlinuz boot=live components console=ttyAMA0,115200n8 console=tty0 loglevel=7 systemd.show_status=1 systemd.log_level=debug systemd.debug-shell=1 plymouth.enable=0 nosplash live-media-path=/live/aarch64
  initrd /live/aarch64/initrd.img
}

menuentry "Samaris OS ARM64 (safe - nomodeset, single)" {
  linux /live/aarch64/vmlinuz boot=live components console=ttyAMA0,115200n8 loglevel=7 systemd.show_status=1 nomodeset plymouth.enable=0 single live-media-path=/live/aarch64
  initrd /live/aarch64/initrd.img
}
EOF
  fi
}

write_isolinux_cfg() {
  mkdir -p "$ISO_TREE/isolinux"
  cat > "$ISO_TREE/isolinux/isolinux.cfg" <<'EOF'
UI vesamenu.c32
PROMPT 0
TIMEOUT 50
DEFAULT samaris

LABEL samaris
  MENU LABEL Samaris OS Alpha One RC x86_64
  KERNEL /live/x86_64/vmlinuz
  APPEND initrd=/live/x86_64/initrd.img boot=live components quiet splash live-media-path=/live/x86_64
EOF
}

copy_isolinux_files() {
  local isolinux_bin ldlinux vesamenu libcom32 libutil
  isolinux_bin="$(find_first /usr/lib/ISOLINUX/isolinux.bin /usr/lib/syslinux/modules/bios/isolinux.bin)"
  ldlinux="$(find_first /usr/lib/syslinux/modules/bios/ldlinux.c32 /usr/lib/ISOLINUX/ldlinux.c32)"
  vesamenu="$(find_first /usr/lib/syslinux/modules/bios/vesamenu.c32 /usr/lib/ISOLINUX/vesamenu.c32)"
  libcom32="$(find_first /usr/lib/syslinux/modules/bios/libcom32.c32 /usr/lib/ISOLINUX/libcom32.c32)"
  libutil="$(find_first /usr/lib/syslinux/modules/bios/libutil.c32 /usr/lib/ISOLINUX/libutil.c32)"
  cp "$isolinux_bin" "$ISO_TREE/isolinux/isolinux.bin"
  cp "$ldlinux" "$ISO_TREE/isolinux/ldlinux.c32"
  cp "$vesamenu" "$ISO_TREE/isolinux/vesamenu.c32"
  cp "$libcom32" "$ISO_TREE/isolinux/libcom32.c32"
  cp "$libutil" "$ISO_TREE/isolinux/libutil.c32"
}

create_efi_image() {
  local efi_img="$ISO_TREE/boot/grub/efi.img"
  local bootx64="$WORK_DIR/BOOTX64.EFI"
  local bootaa64="$WORK_DIR/BOOTAA64.EFI"

  rm -f "$efi_img" "$bootx64" "$bootaa64"
  rm -f "$ISO_TREE/EFI/BOOT/BOOTX64.EFI" "$ISO_TREE/EFI/BOOT/BOOTAA64.EFI"

  if arch_enabled x86_64; then
    grub-mkstandalone -O x86_64-efi -o "$bootx64" \
      --modules="search search_label" \
      "boot/grub/grub.cfg=$ISO_TREE/boot/grub/grub.cfg"
  fi
  if arch_enabled aarch64; then
    grub-mkstandalone -O arm64-efi -o "$bootaa64" \
      --modules="search search_label" \
      "boot/grub/grub.cfg=$ISO_TREE/boot/grub/grub.cfg"
  fi

  dd if=/dev/zero of="$efi_img" bs=1M count=64 status=none
  mformat -i "$efi_img" -F ::
  mmd -i "$efi_img" ::/EFI
  mmd -i "$efi_img" ::/EFI/BOOT
  if [ -f "$bootx64" ]; then
    mcopy -i "$efi_img" "$bootx64" ::/EFI/BOOT/BOOTX64.EFI
  fi
  if [ -f "$bootaa64" ]; then
    mcopy -i "$efi_img" "$bootaa64" ::/EFI/BOOT/BOOTAA64.EFI
  fi

  mkdir -p "$ISO_TREE/EFI/BOOT"
  if [ -f "$bootx64" ]; then
    cp "$bootx64" "$ISO_TREE/EFI/BOOT/BOOTX64.EFI"
  fi
  if [ -f "$bootaa64" ]; then
    cp "$bootaa64" "$ISO_TREE/EFI/BOOT/BOOTAA64.EFI"
  fi
}

step_main() {
  local arch
  for arch in $SAMARIS_ARCHES; do
    [ -f "$ISO_TREE/live/$arch/filesystem.squashfs" ] || die "Missing $arch SquashFS"
  done
  write_grub_cfg
  if arch_enabled x86_64; then
    write_isolinux_cfg
    copy_isolinux_files
  fi
  create_efi_image
}

step_validate() {
  [ -f "$ISO_TREE/boot/grub/grub.cfg" ] || return 1
  [ -s "$ISO_TREE/boot/grub/efi.img" ] || return 1
  if arch_enabled x86_64; then
    [ -f "$ISO_TREE/isolinux/isolinux.cfg" ] || return 1
    [ -f "$ISO_TREE/isolinux/isolinux.bin" ] || return 1
    [ -s "$ISO_TREE/EFI/BOOT/BOOTX64.EFI" ] || return 1
  fi
  if arch_enabled aarch64; then
    [ -s "$ISO_TREE/EFI/BOOT/BOOTAA64.EFI" ] || return 1
  fi
  grep -q "search --no-floppy --set=root --label $ISO_LABEL" "$ISO_TREE/boot/grub/grub.cfg"
}
