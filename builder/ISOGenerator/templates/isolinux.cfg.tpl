UI vesamenu.c32
PROMPT 0
TIMEOUT 50
DEFAULT samaris

LABEL samaris
  MENU LABEL Samaris OS Alpha One RC x86_64
  KERNEL /live/x86_64/vmlinuz
  APPEND initrd=/live/x86_64/initrd.img boot=live components quiet splash live-media-path=/live/x86_64
