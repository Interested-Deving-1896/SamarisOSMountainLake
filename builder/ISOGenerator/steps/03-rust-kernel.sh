#!/usr/bin/env bash

step_main() {
  local src="$CONTENT_ROOT/volt-kernel-b"
  local bin_dir="$OVERLAY_DIR/opt/volt/bin"
  mkdir -p "$bin_dir"

  if [ ! -f "$src/Cargo.toml" ]; then
    warn "Tesseract Engine (Kernel B) Cargo.toml not found; continuing with Node fallback only"
    return 0
  fi

  (
    cd "$src"
    require_rust_toolchain_for_arch x86_64
    cargo build --release --target "$(rust_target x86_64)"
    cp "target/$(rust_target x86_64)/release/tesseract-engine" "$bin_dir/tesseract-engine-x86_64"

    if arch_enabled aarch64; then
      require_rust_toolchain_for_arch aarch64
      cargo build --release --target "$(rust_target aarch64)"
      cp "target/$(rust_target aarch64)/release/tesseract-engine" "$bin_dir/tesseract-engine-aarch64"
    fi
  )

  chmod 0755 "$bin_dir"/tesseract-engine-* 2>/dev/null || true

  # Keep legacy compatibility symlink for older scripts
  if [ -f "$bin_dir/tesseract-engine-x86_64" ]; then
    ln -sf tesseract-engine-x86_64 "$bin_dir/kernel-b-x86_64" 2>/dev/null || true
  fi
  if [ -f "$bin_dir/tesseract-engine-aarch64" ]; then
    ln -sf tesseract-engine-aarch64 "$bin_dir/kernel-b-aarch64" 2>/dev/null || true
  fi
}

step_validate() {
  [ -x "$OVERLAY_DIR/opt/volt/bin/tesseract-engine-x86_64" ]
  if arch_enabled aarch64; then
    [ -x "$OVERLAY_DIR/opt/volt/bin/tesseract-engine-aarch64" ]
    [ -e "$OVERLAY_DIR/opt/volt/bin/kernel-b-aarch64" ]
  fi
}
