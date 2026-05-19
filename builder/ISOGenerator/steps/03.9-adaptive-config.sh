#!/usr/bin/env bash

step_main() {
  local src="$CONTENT_ROOT/volt-adaptive-system-config"
  local bin_dir="$OVERLAY_DIR/opt/volt/bin"
  mkdir -p "$bin_dir"

  if [ ! -f "$src/Cargo.toml" ]; then
    warn "Volt Adaptive System Config Cargo.toml not found; skipping"
    return 0
  fi

  (
    cd "$src"
    require_rust_toolchain_for_arch x86_64
    cargo build --release --target "$(rust_target x86_64)"
    cp "target/$(rust_target x86_64)/release/volt-asc" "$bin_dir/volt-asc-x86_64"

    if arch_enabled aarch64; then
      require_rust_toolchain_for_arch aarch64
      cargo build --release --target "$(rust_target aarch64)"
      cp "target/$(rust_target aarch64)/release/volt-asc" "$bin_dir/volt-asc-aarch64"
    fi
  )

  chmod 0755 "$bin_dir"/volt-asc-* 2>/dev/null || true
}

step_validate() {
  [ -x "$OVERLAY_DIR/usr/local/bin/start-volt-asc.sh" ]
  [ -x "$OVERLAY_DIR/opt/volt/bin/volt-asc-x86_64" ]
  if arch_enabled aarch64; then
    [ -x "$OVERLAY_DIR/opt/volt/bin/volt-asc-aarch64" ]
  fi
}
