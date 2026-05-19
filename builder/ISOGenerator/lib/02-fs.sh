#!/usr/bin/env bash

safe_reset_dir() {
  local target="$1"
  samaris_assert_safe_layout
  case "$target" in
    "$WORK_DIR")
      mkdir -p "$target"
      find "$target" -mindepth 1 -maxdepth 1 ! -name state -exec rm -rf {} +
      mkdir -p "$target"
      ;;
    "$WORK_DIR"/*|"$OUTPUT_DIR"/*|"$CACHE_DIR"/*)
      rm -rf "$target"
      mkdir -p "$target"
      ;;
    *)
      die "Refusing to reset unsafe path: $target"
      ;;
  esac
}

safe_remove_dir() {
  local target="$1"
  samaris_assert_safe_layout
  case "$target" in
    "$WORK_DIR"|"$WORK_DIR"/*|"$CACHE_DIR"|"$CACHE_DIR"/*)
      rm -rf "$target"
      ;;
    *)
      die "Refusing to remove unsafe path: $target"
      ;;
  esac
}

safe_clean_output_files() {
  samaris_assert_safe_layout
  mkdir -p "$OUTPUT_DIR" 2>/dev/null || true
  rm -f "$OUTPUT_DIR"/*.iso "$OUTPUT_DIR"/*.sha256 2>/dev/null || true
}

find_first() {
  local candidate
  for candidate in "$@"; do
    if [ -e "$candidate" ]; then
      printf '%s\n' "$candidate"
      return 0
    fi
  done
  return 1
}

sha256_file() {
  local file="$1"
  if command -v sha256sum >/dev/null 2>&1; then
    sha256sum "$file" | awk '{print $1}'
  else
    shasum -a 256 "$file" | awk '{print $1}'
  fi
}

sha256_check() {
  local file="$1"
  local expected="$2"
  local actual
  actual="$(sha256_file "$file")"
  [ "$actual" = "$expected" ]
}

clean_ds_store() {
  find "$PROJECT_ROOT" -name ".DS_Store" -delete 2>/dev/null || true
}

is_mounted() {
  mount | awk '{print $3}' | grep -Fxq "$1"
}

mount_if_needed() {
  local source="$1"
  local target="$2"
  local type="${3:-}"
  mkdir -p "$target"
  if is_mounted "$target"; then
    return 0
  fi
  if [ -n "$type" ]; then
    mount -t "$type" "$source" "$target"
  else
    mount "$source" "$target"
  fi
}

bind_mount_if_needed() {
  local source="$1"
  local target="$2"
  mkdir -p "$target"
  if ! is_mounted "$target"; then
    mount --bind "$source" "$target"
  fi
}

umount_if_mounted() {
  local target="$1"
  if is_mounted "$target"; then
    umount "$target"
  fi
}
