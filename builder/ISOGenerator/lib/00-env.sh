#!/usr/bin/env bash

SAMARIS_LIB_LOADED=1
ISO_GENERATOR_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BUILDER_ROOT="$(cd "$ISO_GENERATOR_ROOT/.." && pwd)"
PROJECT_ROOT="$(cd "$BUILDER_ROOT/.." && pwd)"
CONTENT_ROOT="$BUILDER_ROOT/content"
MODULES_DIR="$CONTENT_ROOT/modules"
OVERLAY_DIR="$BUILDER_ROOT/overlay"
CACHE_DIR="$BUILDER_ROOT/cache"
OUTPUT_DIR="$BUILDER_ROOT/output"
STEPS_DIR="$ISO_GENERATOR_ROOT/steps"
TEMPLATES_DIR="$ISO_GENERATOR_ROOT/templates"
CONFIG_DIR="$BUILDER_ROOT/configs"

samaris_env_error() {
  printf '[samaris] error: %s\n' "$*" >&2
  exit 1
}

samaris_trim() {
  local s="$1"
  s="${s#"${s%%[![:space:]]*}"}"
  s="${s%"${s##*[![:space:]]}"}"
  printf '%s' "$s"
}

samaris_unquote_value() {
  local v
  v="$(samaris_trim "$1")"
  case "$v" in
    \"*\")
      v="${v#\"}"
      v="${v%\"}"
      ;;
    \'*\')
      v="${v#\'}"
      v="${v%\'}"
      ;;
  esac
  printf '%s' "$v"
}

samaris_project_config_key() {
  case "$1" in
    PROJECT_NAME|PROJECT_ID|VERSION|DEBIAN_SUITE|DEBIAN_MIRROR|DEBOOTSTRAP_VARIANT|ARCH|LIVE_USER|ISO_LABEL|AUTO_VERSION_PATCH|OUTPUT_ISO|SAMARIS_ARCHES|ENABLED_MODULES|BUILD_AI_POSTINSTALL)
      return 0
      ;;
    *)
      return 1
      ;;
  esac
}

samaris_build_config_key() {
  case "$1" in
    BUILD_CONFIG_ARCH_X86_64|BUILD_CONFIG_ARCH_AARCH64|BUILD_CONFIG_AI_LLM|BUILD_CONFIG_AI_STT|BUILD_CONFIG_AI_TTS|BUILD_CONFIG_AI_POSTINSTALL|BUILD_CONFIG_VOLT_BENCH|BUILD_CONFIG_SVC_NETWORK|BUILD_CONFIG_SVC_CUPS|BUILD_CONFIG_SVC_POWER|BUILD_CONFIG_DESKTOP_UI|BUILD_CONFIG_DESKTOP_BROWSER|BUILD_CONFIG_DESKTOP_DEMO|BUILD_CONFIG_MODE|BUILD_CONFIG_USE_DOCKER|BUILD_CONFIG_FORCE|BUILD_CONFIG_DRY_RUN)
      return 0
      ;;
    *)
      return 1
      ;;
  esac
}

samaris_validate_config_value() {
  local key="$1" value="$2"
  case "$value" in
    *'$('*|*'`'*|*';'*|*'&&'*|*'||'*)
      return 1
      ;;
  esac
  case "$key" in
    BUILD_CONFIG_MODE)
      case "$value" in full|resume|from|single) return 0 ;; *) return 1 ;; esac
      ;;
    BUILD_CONFIG_*)
      case "$value" in 0|1) return 0 ;; *) return 1 ;; esac
      ;;
  esac
  return 0
}

samaris_assign_config_value() {
  local key="$1" value="$2" mode="${3:-project}"
  case "$key" in
    *[!A-Za-z0-9_]*|'')
      samaris_env_error "Invalid config key: $key"
      ;;
  esac
  if [ "$mode" = "build" ]; then
    samaris_build_config_key "$key" || samaris_env_error "Unsupported build config key: $key"
  else
    samaris_project_config_key "$key" || return 0
  fi
  samaris_validate_config_value "$key" "$value" || samaris_env_error "Unsafe or invalid value for $key"
  printf -v "$key" '%s' "$value"
}

samaris_load_kv_file() {
  local file="$1" mode="${2:-project}" line key value line_no=0
  [ -f "$file" ] || return 0
  while IFS= read -r line || [ -n "$line" ]; do
    line_no=$((line_no + 1))
    line="${line%%#*}"
    line="$(samaris_trim "$line")"
    [ -z "$line" ] && continue
    case "$line" in
      *=*) ;;
      *) samaris_env_error "$file:$line_no is not KEY=VALUE" ;;
    esac
    key="$(samaris_trim "${line%%=*}")"
    value="$(samaris_unquote_value "${line#*=}")"
    case "$value" in
      *'$'*)
        if [ "$mode" = "build" ]; then
          samaris_env_error "$file:$line_no contains shell expansion"
        fi
        continue
        ;;
    esac
    samaris_assign_config_value "$key" "$value" "$mode"
  done < "$file"
}

samaris_abs_existing_path() {
  local path="$1" dir base
  [ -n "$path" ] || return 1
  dir="$(cd "$(dirname "$path")" 2>/dev/null && pwd -P)" || return 1
  base="$(basename "$path")"
  printf '%s/%s\n' "$dir" "$base"
}

samaris_path_under() {
  local path="$1" base="$2"
  case "$path" in
    "$base"|"$base"/*)
      return 0
      ;;
    *)
      return 1
      ;;
  esac
}

samaris_assert_safe_layout() {
  local work="$WORK_DIR" cache="$CACHE_DIR" output="$OUTPUT_DIR"
  case "$work" in
    ''|/|/Users|/Users/*/Desktop|/private|/private/tmp|/tmp)
      samaris_env_error "Refusing unsafe WORK_DIR: $work"
      ;;
  esac
  if ! samaris_path_under "$work" "$BUILDER_ROOT/work" && ! samaris_path_under "$work" /samaris-work; then
    samaris_env_error "WORK_DIR must be under $BUILDER_ROOT/work or /samaris-work: $work"
  fi
  samaris_path_under "$cache" "$BUILDER_ROOT/cache" || samaris_env_error "CACHE_DIR must stay under $BUILDER_ROOT/cache"
  samaris_path_under "$output" "$BUILDER_ROOT/output" || samaris_env_error "OUTPUT_DIR must stay under $BUILDER_ROOT/output"
}

if [ -f "$BUILDER_ROOT/config.env" ]; then
  samaris_load_kv_file "$BUILDER_ROOT/config.env" project
fi

WORK_DIR="${SAMARIS_WORK_DIR:-$BUILDER_ROOT/work}"
ROOTFS_BASE="$WORK_DIR/rootfs"
ISO_TREE="$WORK_DIR/iso"
CONFIG_FILE="${CONFIG_FILE:-$WORK_DIR/.samaris-build-config}"
samaris_assert_safe_layout

# Load build config if available (overrides config.env)
if [ -f "$CONFIG_FILE" ]; then
  case "$CONFIG_FILE" in
    "$WORK_DIR"/*) samaris_load_kv_file "$CONFIG_FILE" build ;;
    *) samaris_env_error "CONFIG_FILE must stay under $WORK_DIR: $CONFIG_FILE" ;;
  esac
fi

DEBIAN_SUITE="${DEBIAN_SUITE:-trixie}"
DEBIAN_MIRROR="${DEBIAN_MIRROR:-https://deb.debian.org/debian}"
DEBOOTSTRAP_VARIANT="${DEBOOTSTRAP_VARIANT:-minbase}"
OUTPUT_ISO="${OUTPUT_ISO:-Samaris-OS-Alpha-One-RC.iso}"
ISO_LABEL="${ISO_LABEL:-SAMARIS_ALPHA_ONE_RC}"
LIVE_USER="${LIVE_USER:-user}"

# SAMARIS_ARCHES: build config wins over config.env when present.
if [ -n "${BUILD_CONFIG_ARCH_X86_64:-}" ] || [ -n "${BUILD_CONFIG_ARCH_AARCH64:-}" ]; then
  SAMARIS_ARCHES=""
  [ "${BUILD_CONFIG_ARCH_X86_64:-0}" = "1" ] && SAMARIS_ARCHES="$SAMARIS_ARCHES x86_64"
  [ "${BUILD_CONFIG_ARCH_AARCH64:-0}" = "1" ] && SAMARIS_ARCHES="$SAMARIS_ARCHES aarch64"
  SAMARIS_ARCHES="${SAMARIS_ARCHES# }"
else
  SAMARIS_ARCHES="${SAMARIS_ARCHES:-x86_64 aarch64}"
fi

# ENABLED_MODULES: build config wins over config.env when present.
if [ -n "${BUILD_CONFIG_DESKTOP_UI:-}" ] || [ -n "${BUILD_CONFIG_DESKTOP_BROWSER:-}" ] || [ -n "${BUILD_CONFIG_DESKTOP_DEMO:-}" ]; then
  ENABLED_MODULES="00-base 10-kernel 20-hardware 22-drivers 50-runtimes"
  [ "${BUILD_CONFIG_DESKTOP_UI:-0}" = "1" ] && ENABLED_MODULES="$ENABLED_MODULES 25-boot-splash 30-display-xorg 90-volt-shell"
  [ "${BUILD_CONFIG_DESKTOP_BROWSER:-0}" = "1" ] && ENABLED_MODULES="$ENABLED_MODULES 40-browser-chromium"
  [ "${BUILD_CONFIG_DESKTOP_DEMO:-0}" = "1" ] && ENABLED_MODULES="$ENABLED_MODULES 99-demo"
else
  ENABLED_MODULES="${ENABLED_MODULES:-00-base 10-kernel 20-hardware 22-drivers 25-boot-splash 30-display-xorg 40-browser-chromium 50-runtimes 90-volt-shell 99-demo}"
fi

true
