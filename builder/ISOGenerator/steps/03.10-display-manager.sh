#!/usr/bin/env bash

step_main() {
  build_config_skip "display_manager" 2>/dev/null && { log "Display manager not selected — skipping"; return 0; }

  local src="$CONTENT_ROOT/volt-display-manager"
  local bin_dir="$OVERLAY_DIR/opt/volt/bin"
  local launcher="$OVERLAY_DIR/usr/local/bin/start-display-manager.sh"
  mkdir -p "$bin_dir"

  if [ ! -f "$src/Cargo.toml" ]; then
    warn "Volt Display Manager Cargo.toml not found; skipping"
    return 0
  fi

  (
    cd "$src"
    require_rust_toolchain_for_arch x86_64
    cargo build --release --target "$(rust_target x86_64)"
    cp "target/$(rust_target x86_64)/release/volt-display-manager" "$bin_dir/volt-display-manager-x86_64"

    if arch_enabled aarch64; then
      require_rust_toolchain_for_arch aarch64
      cargo build --release --target "$(rust_target aarch64)"
      cp "target/$(rust_target aarch64)/release/volt-display-manager" "$bin_dir/volt-display-manager-aarch64"
    fi
  )

  chmod 0755 "$bin_dir"/volt-display-manager-* 2>/dev/null || true
  chmod 0755 "$launcher" 2>/dev/null || true

  if [ -f "$src/udev/99-volt-display.rules" ]; then
    mkdir -p "$OVERLAY_DIR/etc/udev/rules.d"
    cp "$src/udev/99-volt-display.rules" "$OVERLAY_DIR/etc/udev/rules.d/99-volt-display.rules"
  fi

  mkdir -p "$OVERLAY_DIR/opt/volt/display-manager"
}

step_validate() {
  build_config_skip "display_manager" 2>/dev/null && return 0
  [ -x "$OVERLAY_DIR/opt/volt/bin/volt-display-manager-x86_64" ]
  if arch_enabled aarch64; then
    [ -x "$OVERLAY_DIR/opt/volt/bin/volt-display-manager-aarch64" ]
  fi
  [ -x "$OVERLAY_DIR/usr/local/bin/start-display-manager.sh" ]
  [ -f "$OVERLAY_DIR/etc/udev/rules.d/99-volt-display.rules" ]
  [ -f "$OVERLAY_DIR/etc/systemd/system/volt-display-manager.service" ]
  [ -f "$OVERLAY_DIR/etc/systemd/system/volt-display-hotplug.service" ]
  grep -q 'ExecStart=.*start-display-manager.sh.*--apply' "$OVERLAY_DIR/etc/systemd/system/volt-display-manager.service"
  grep -q 'ExecStart=.*start-display-manager.sh.*--watch' "$OVERLAY_DIR/etc/systemd/system/volt-display-hotplug.service"
  grep -q 'TimeoutStartSec=30' "$OVERLAY_DIR/etc/systemd/system/volt-display-manager.service"
  grep -q 'Restart=on-failure' "$OVERLAY_DIR/etc/systemd/system/volt-display-manager.service"
  grep -q 'Wants=volt-display-hotplug.service' "$OVERLAY_DIR/etc/systemd/system/volt-display-manager.service"
  grep -q 'ExecStartPre=.*xrandr' "$OVERLAY_DIR/etc/systemd/system/volt-display-manager.service"
}
