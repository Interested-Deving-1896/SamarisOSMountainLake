#!/usr/bin/env bash

require_squashfs_path() {
  local sq="$1" path="$2"
  unsquashfs -lls "$sq" "$path" 2>/dev/null | grep -q "squashfs-root/$path" \
    || die "Missing $path in $sq"
}

warn_squashfs_path() {
  local sq="$1" path="$2"
  if ! unsquashfs -lls "$sq" "$path" 2>/dev/null | grep -q "squashfs-root/$path"; then
    warn "Missing optional artifact $path in $sq"
  fi
}

require_squashfs_elf_machine() {
  local sq="$1" path="$2" machine="$3"
  local tmp
  tmp=$(mktemp)
  if ! unsquashfs -cat "$sq" "$path" > "$tmp" 2>/dev/null; then
    rm -f "$tmp"
    die "Missing ELF $path in $sq"
  fi
  if ! readelf -h "$tmp" 2>/dev/null | grep -q "Machine:.*$machine"; then
    rm -f "$tmp"
    die "ELF $path in $sq is not built for $machine"
  fi
  rm -f "$tmp"
}

require_squashfs_text() {
  local sq="$1" path="$2" pattern="$3"
  unsquashfs -cat "$sq" "$path" 2>/dev/null | grep -q "$pattern" \
    || die "Missing expected pattern '$pattern' in $path from $sq"
}

validate_squashfs_contents() {
  local arch sq
  for arch in $SAMARIS_ARCHES; do
    sq="$ISO_TREE/live/$arch/filesystem.squashfs"
    local electron_machine node_machine
    if [ "$arch" = "aarch64" ]; then
      electron_machine="AArch64"
      node_machine="AArch64"
    else
      electron_machine="Advanced Micro Devices X86-64"
      node_machine="Advanced Micro Devices X86-64"
    fi

    # Core binaries (always present if module is enabled)
    local has_bench=0
    build_config_is_selected VOLT_BENCH 2>/dev/null && has_bench=1 || has_bench="${BUILD_CONFIG_VOLT_BENCH:-0}"

    if [ "$has_bench" = "1" ]; then
      warn_squashfs_path "$sq" opt/volt/bin/bench
      require_squashfs_elf_machine "$sq" opt/volt/bin/bench-x86_64 "Advanced Micro Devices X86-64"
      if [ "$arch" = "aarch64" ]; then
        require_squashfs_elf_machine "$sq" opt/volt/bin/bench-aarch64 "AArch64"
      fi
    fi

    require_squashfs_elf_machine "$sq" opt/volt/bin/tesseract-engine-x86_64 "Advanced Micro Devices X86-64"
    [ "$arch" = "aarch64" ] && require_squashfs_elf_machine "$sq" opt/volt/bin/tesseract-engine-aarch64 "AArch64"
    require_squashfs_path "$sq" opt/volt/bin/kernel-b-x86_64
    [ "$arch" = "aarch64" ] && require_squashfs_path "$sq" opt/volt/bin/kernel-b-aarch64
    require_squashfs_elf_machine "$sq" opt/volt/bin/volt-asc-x86_64 "Advanced Micro Devices X86-64"
    [ "$arch" = "aarch64" ] && require_squashfs_elf_machine "$sq" opt/volt/bin/volt-asc-aarch64 "AArch64"
    require_squashfs_elf_machine "$sq" opt/volt/bin/volt-dynamic-worker-pool-x86_64 "Advanced Micro Devices X86-64"
    [ "$arch" = "aarch64" ] && require_squashfs_elf_machine "$sq" opt/volt/bin/volt-dynamic-worker-pool-aarch64 "AArch64"

    # Display manager (only if desktop UI selected)
    local has_display=1
    build_config_is_selected DESKTOP_UI 2>/dev/null && has_display=1 || has_display="${BUILD_CONFIG_DESKTOP_UI:-1}"
    if [ "$has_display" = "1" ]; then
      warn_squashfs_path "$sq" opt/volt/bin/volt-display-manager-x86_64
      [ "$arch" = "aarch64" ] && warn_squashfs_path "$sq" opt/volt/bin/volt-display-manager-aarch64
      warn_squashfs_path "$sq" usr/local/bin/start-display-manager.sh
    fi

    require_squashfs_elf_machine "$sq" opt/volt/bin/volt-ram-manager-x86_64 "Advanced Micro Devices X86-64"
    [ "$arch" = "aarch64" ] && require_squashfs_elf_machine "$sq" opt/volt/bin/volt-ram-manager-aarch64 "AArch64"
    require_squashfs_elf_machine "$sq" opt/volt/bin/volt-usb-manager-x86_64 "Advanced Micro Devices X86-64"
    [ "$arch" = "aarch64" ] && require_squashfs_elf_machine "$sq" opt/volt/bin/volt-usb-manager-aarch64 "AArch64"
    require_squashfs_elf_machine "$sq" opt/volt/bin/volt-gpu-manager-x86_64 "Advanced Micro Devices X86-64"
    [ "$arch" = "aarch64" ] && require_squashfs_elf_machine "$sq" opt/volt/bin/volt-gpu-manager-aarch64 "AArch64"

    # Desktop UI
    local has_ui=1
    build_config_is_selected DESKTOP_UI 2>/dev/null && has_ui=1 || has_ui="${BUILD_CONFIG_DESKTOP_UI:-1}"
    if [ "$has_ui" = "1" ]; then
      warn_squashfs_path "$sq" opt/volt/desktop/app/index.html
    fi

    # Electron / Browser
    local has_browser=1
    build_config_is_selected DESKTOP_BROWSER 2>/dev/null && has_browser=1 || has_browser="${BUILD_CONFIG_DESKTOP_BROWSER:-1}"
    if [ "$has_browser" = "1" ]; then
      warn_squashfs_path "$sq" opt/volt/electron/main.js
      warn_squashfs_path "$sq" opt/volt/electron/node_modules/electron/path.txt
      warn_squashfs_path "$sq" opt/volt/electron/node_modules/electron/dist/electron
      warn_squashfs_path "$sq" opt/volt/electron/node_modules/node-pty/build/Release/pty.node
    fi

    require_squashfs_path "$sq" opt/volt/kernel/server.js

    # AI models
    local has_llm=1 has_stt=1 has_tts=1 post=0
    build_config_is_selected AI_LLM 2>/dev/null && has_llm=1 || has_llm="${BUILD_CONFIG_AI_LLM:-1}"
    build_config_is_selected AI_STT 2>/dev/null && has_stt=1 || has_stt="${BUILD_CONFIG_AI_STT:-1}"
    build_config_is_selected AI_TTS 2>/dev/null && has_tts=1 || has_tts="${BUILD_CONFIG_AI_TTS:-1}"
    build_config_is_selected AI_POSTINSTALL 2>/dev/null && post=1 || post="${BUILD_AI_POSTINSTALL:-0}"

    if [ "$has_llm" = "1" ] && [ "$post" != "1" ]; then
      warn_squashfs_path "$sq" opt/volt/ai-models/Qwen3-1.7B-Q8_0.gguf
    fi
    if [ "$has_tts" = "1" ] && [ "$post" != "1" ]; then
      warn_squashfs_path "$sq" opt/volt/ai-models/outetts/OuteTTS-0.2-500M-Q8_0.gguf
      warn_squashfs_path "$sq" opt/volt/ai-models/outetts/WavTokenizer-Large-75-F16.gguf
    fi
    if [ "$has_stt" = "1" ] && [ "$post" != "1" ]; then
      warn_squashfs_path "$sq" opt/volt/ai-models/whisper/ggml-small.bin
      warn_squashfs_path "$sq" opt/volt/ai-models/bin/whisper
      warn_squashfs_path "$sq" opt/volt/ai-models/bin/whisper-x86_64
      [ "$arch" = "aarch64" ] && warn_squashfs_path "$sq" opt/volt/ai-models/bin/whisper-aarch64
    fi

    if [ "$has_display" = "1" ]; then
      require_squashfs_text "$sq" etc/systemd/system/volt-display-manager.service \
        'ExecStart=.*start-display-manager.sh .*--apply'
      require_squashfs_text "$sq" etc/systemd/system/volt-display-hotplug.service \
        'ExecStart=.*start-display-manager.sh .*--watch'
    fi
  done
}

step_main() {
  local iso="$OUTPUT_DIR/$OUTPUT_ISO"
  [ -s "$iso" ] || die "Generated ISO is empty or missing"
  local arch
  for arch in $SAMARIS_ARCHES; do
    [ -f "$ISO_TREE/live/$arch/vmlinuz" ] || die "Missing $arch kernel in ISO tree"
    [ -f "$ISO_TREE/live/$arch/filesystem.squashfs" ] || die "Missing $arch SquashFS"
  done
  if arch_enabled x86_64; then
    [ -f "$ISO_TREE/EFI/BOOT/BOOTX64.EFI" ] || die "Missing BOOTX64.EFI"
  fi
  if arch_enabled aarch64; then
    [ -f "$ISO_TREE/EFI/BOOT/BOOTAA64.EFI" ] || die "Missing BOOTAA64.EFI"
  fi
  xorriso -indev "$iso" -report_el_torito plain >/dev/null 2>&1 || die "xorriso could not inspect El Torito metadata"
  validate_squashfs_contents
  log "ISO validation passed"
}

step_validate() {
  step_main >/dev/null
}
