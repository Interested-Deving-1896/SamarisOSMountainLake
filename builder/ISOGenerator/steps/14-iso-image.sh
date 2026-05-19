#!/usr/bin/env bash

step_main() {
  mkdir -p "$OUTPUT_DIR"
  local iso="$OUTPUT_DIR/$OUTPUT_ISO"
  local isohdpfx
  isohdpfx="$(find_first /usr/lib/ISOLINUX/isohdpfx.bin /usr/lib/syslinux/mbr/isohdpfx.bin)"

  rm -f "$iso"
  xorriso -as mkisofs \
    -iso-level 3 \
    -full-iso9660-filenames \
    -volid "$ISO_LABEL" \
    -eltorito-boot isolinux/isolinux.bin \
    -eltorito-catalog isolinux/boot.cat \
    -no-emul-boot \
    -boot-load-size 4 \
    -boot-info-table \
    -isohybrid-mbr "$isohdpfx" \
    -eltorito-alt-boot \
    -e boot/grub/efi.img \
    -no-emul-boot \
    -isohybrid-gpt-basdat \
    -isohybrid-apm-hfsplus \
    -output "$iso" \
    "$ISO_TREE"
}

step_validate() {
  [ -s "$OUTPUT_DIR/$OUTPUT_ISO" ]
}
