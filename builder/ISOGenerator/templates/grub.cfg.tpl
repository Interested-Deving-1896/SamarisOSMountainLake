search --no-floppy --set=root --label SAMARIS_ML_ALPHA_ONE

set timeout=5
set default=0

if [ "$grub_cpu" = "arm64" ]; then
  set samaris_arch="aarch64"
else
  set samaris_arch="x86_64"
fi

menuentry "Samaris OS Alpha One RC (auto architecture)" {
  linux /live/$samaris_arch/vmlinuz boot=live components quiet splash live-media-path=/live/$samaris_arch
  initrd /live/$samaris_arch/initrd.img
}

menuentry "Samaris OS Alpha One RC x86_64" {
  linux /live/x86_64/vmlinuz boot=live components quiet splash live-media-path=/live/x86_64
  initrd /live/x86_64/initrd.img
}

menuentry "Samaris OS Alpha One RC ARM64" {
  linux /live/aarch64/vmlinuz boot=live components quiet splash live-media-path=/live/aarch64
  initrd /live/aarch64/initrd.img
}
