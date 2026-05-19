#!/usr/bin/env bash

write_bench_launcher() {
  local dest="$1"
  cat > "$dest" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
case "$(uname -m)" in
  x86_64|amd64) binary="/opt/volt/bin/bench-x86_64" ;;
  aarch64|arm64) binary="/opt/volt/bin/bench-aarch64" ;;
  *)
    echo "Unsupported architecture: $(uname -m)" >&2
    exit 0
    ;;
esac
if [ -x "$binary" ]; then
  exec "$binary" "$@"
fi
echo "Samaris Bench not found at $binary" >&2
exit 0
EOF
  chmod 0755 "$dest"
}

step_main() {
  build_config_skip "bench" 2>/dev/null && { log "Bench not selected — skipping"; return 0; }

  local src="$CONTENT_ROOT/volt-bench"
  local bin_dir="$OVERLAY_DIR/opt/volt/bin"
  local bench_dir="$OVERLAY_DIR/opt/volt/bench"
  mkdir -p "$bin_dir" "$bench_dir"

  if [ ! -f "$src/Cargo.toml" ]; then
    warn "Samaris Bench Cargo.toml not found; skipping"
    return 0
  fi

  (
    cd "$src"
    require_rust_toolchain_for_arch x86_64
    cargo build --release --target "$(rust_target x86_64)"
    cp "target/$(rust_target x86_64)/release/bench" "$bin_dir/bench-x86_64"

    if arch_enabled aarch64; then
      require_rust_toolchain_for_arch aarch64
      cargo build --release --target "$(rust_target aarch64)"
      cp "target/$(rust_target aarch64)/release/bench" "$bin_dir/bench-aarch64"
    fi
  )

  chmod 0755 "$bin_dir"/bench-* 2>/dev/null || true
  write_bench_launcher "$bin_dir/bench"

  if [ ! -f "$bench_dir/config.toml" ] && [ -f "$src/config.toml" ]; then
    cp "$src/config.toml" "$bench_dir/config.toml"
  fi
}

step_validate() {
  build_config_skip "bench" 2>/dev/null && return 0
  [ -x "$OVERLAY_DIR/opt/volt/bin/bench" ]
  [ -x "$OVERLAY_DIR/opt/volt/bin/bench-x86_64" ]
  if arch_enabled aarch64; then
    [ -x "$OVERLAY_DIR/opt/volt/bin/bench-aarch64" ]
  fi
  [ -f "$OVERLAY_DIR/opt/volt/bench/config.toml" ]
  [ -f "$OVERLAY_DIR/etc/systemd/system/volt-bench.service" ]
  [ -f "$OVERLAY_DIR/etc/systemd/system/volt-bench-watch.service" ]
}
