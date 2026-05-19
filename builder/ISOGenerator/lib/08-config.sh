#!/usr/bin/env bash
# SAMARIS ISO GENERATOR - strict build configuration model

[ -n "${SAMARIS_CONFIG_LOADED:-}" ] && return 0
SAMARIS_CONFIG_LOADED=1

CONFIG_DIR="${CONFIG_DIR:-$BUILDER_ROOT/configs}"
CONFIG_FILE="${CONFIG_FILE:-$WORK_DIR/.samaris-build-config}"

build_config_defaults() {
  cat <<'CONF'
BUILD_CONFIG_ARCH_X86_64=1
BUILD_CONFIG_ARCH_AARCH64=1
BUILD_CONFIG_AI_LLM=0
BUILD_CONFIG_AI_STT=0
BUILD_CONFIG_AI_TTS=0
BUILD_CONFIG_AI_POSTINSTALL=0
BUILD_CONFIG_VOLT_BENCH=0
BUILD_CONFIG_SVC_NETWORK=1
BUILD_CONFIG_SVC_CUPS=1
BUILD_CONFIG_SVC_POWER=1
BUILD_CONFIG_DESKTOP_UI=0
BUILD_CONFIG_DESKTOP_BROWSER=0
BUILD_CONFIG_DESKTOP_DEMO=0
BUILD_CONFIG_MODE=full
BUILD_CONFIG_USE_DOCKER=1
BUILD_CONFIG_FORCE=0
BUILD_CONFIG_DRY_RUN=0
CONF
}

build_config_keys() {
  build_config_defaults | sed -n 's/=.*//p'
}

build_config_get() {
  local k="BUILD_CONFIG_$1"
  printf '%s' "${!k:-}"
}

build_config_set_var() {
  local key="$1" value="$2"
  samaris_build_config_key "$key" || die "Unsupported build config key: $key"
  samaris_validate_config_value "$key" "$value" || die "Invalid value for $key: $value"
  printf -v "$key" '%s' "$value"
}

build_config_write_current() {
  local key short value
  while IFS= read -r key; do
    short="${key#BUILD_CONFIG_}"
    value="${!key:-}"
    if [ -z "$value" ]; then
      value="$(build_config_defaults | sed -n "s/^${key}=//p")"
    fi
    printf '%s=%s\n' "$key" "$value"
  done < <(build_config_keys)
}

build_config_init() {
  case "$CONFIG_FILE" in
    "$WORK_DIR"/*) ;;
    *) die "CONFIG_FILE must stay under $WORK_DIR: $CONFIG_FILE" ;;
  esac
  mkdir -p "$(dirname "$CONFIG_FILE")" 2>/dev/null || true
  if [ ! -f "$CONFIG_FILE" ]; then
    build_config_defaults > "$CONFIG_FILE"
  fi
  samaris_load_kv_file "$CONFIG_FILE" build
}

build_config_set() {
  local short="$1" value="$2" key="BUILD_CONFIG_$1"
  build_config_init >/dev/null 2>&1 || true
  build_config_set_var "$key" "$value"
  local tmp="${CONFIG_FILE}.tmp.$$"
  build_config_write_current > "$tmp"
  mv "$tmp" "$CONFIG_FILE"
}

build_config_toggle() {
  local c
  c=$(build_config_get "$1")
  if [ "$c" = "1" ]; then
    build_config_set "$1" "0"
  else
    build_config_set "$1" "1"
  fi
}

build_config_is_selected() {
  [ "$(build_config_get "$1")" = "1" ]
}

build_config_skip() {
  local m="$1" v
  case "$m" in
    bench) v="VOLT_BENCH" ;;
    kernel_b|kernel-b) return 1 ;;
    display_manager|display-mgr) v="DESKTOP_UI" ;;
    electron) v="DESKTOP_BROWSER" ;;
    ui) v="DESKTOP_UI" ;;
    ai_llm) v="AI_LLM" ;;
    ai_stt|ai-stt) v="AI_STT" ;;
    ai_tts|ai-tts) v="AI_TTS" ;;
    *) return 1 ;;
  esac
  ! build_config_is_selected "$v"
}

build_config_step_needed() {
  local step="$1"
  case "$step" in
    03.11-bench) ! build_config_skip "bench" ;;
    03.10-display-manager) ! build_config_skip "display_manager" ;;
    11-electron) ! build_config_skip "electron" ;;
    09-ui) ! build_config_skip "ui" ;;
    02-ai-assets) build_config_is_selected AI_LLM || build_config_is_selected AI_STT || build_config_is_selected AI_TTS ;;
    *) return 0 ;;
  esac
}

build_config_arches() {
  local a=""
  build_config_is_selected ARCH_X86_64 && a="$a x86_64"
  build_config_is_selected ARCH_AARCH64 && a="$a aarch64"
  printf '%s\n' "${a# }"
}

build_config_enabled_modules() {
  local m="00-base 10-kernel 20-hardware 22-drivers 50-runtimes"
  build_config_is_selected DESKTOP_UI && m="$m 25-boot-splash 30-display-xorg 90-volt-shell"
  build_config_is_selected DESKTOP_BROWSER && m="$m 40-browser-chromium"
  build_config_is_selected DESKTOP_DEMO && m="$m 99-demo"
  printf '%s\n' "$m" | tr ' ' '\n' | sort -u | tr '\n' ' ' | sed 's/ $//'
}

build_config_assert_supported() {
  local arches="$1"
  [ -n "$arches" ] || die "At least one architecture must be selected"
  case " $arches " in
    *" x86_64 "*) ;;
    *) die "aarch64-only ISO builds are not supported yet because BIOS/ISOLINUX boot requires x86_64. Select x86_64 too." ;;
  esac
}

build_config_apply() {
  build_config_init >/dev/null 2>&1 || true
  export SAMARIS_ARCHES
  SAMARIS_ARCHES=$(build_config_arches)
  build_config_assert_supported "$SAMARIS_ARCHES"
  export ENABLED_MODULES
  ENABLED_MODULES=$(build_config_enabled_modules)
  export BUILD_AI_POSTINSTALL
  BUILD_AI_POSTINSTALL=$(build_config_get AI_POSTINSTALL)
  export BUILD_FORCE
  BUILD_FORCE=$(build_config_get FORCE)
  export BUILD_DRY_RUN
  if [ "${BUILD_DRY_RUN:-0}" = "1" ]; then
    BUILD_DRY_RUN=1
  else
    BUILD_DRY_RUN=$(build_config_get DRY_RUN)
  fi
  export BUILD_MODE
  BUILD_MODE=$(build_config_get MODE)
  export USE_DOCKER
  if [ "${USE_DOCKER:-}" = "1" ]; then
    USE_DOCKER=1
  else
    USE_DOCKER="$(build_config_get USE_DOCKER)"
  fi
  if [ "${RUN_FORCE:-0}" != "1" ]; then
    export RUN_FORCE="$BUILD_FORCE"
  fi
  log "Config applied: arches=$SAMARIS_ARCHES modules=$ENABLED_MODULES docker=${USE_DOCKER:-0} mode=$BUILD_MODE dry_run=$BUILD_DRY_RUN"
}

build_config_calc_size() {
  local s=900 ai=0
  build_config_is_selected ARCH_X86_64 && s=$((s + 3200))
  build_config_is_selected ARCH_AARCH64 && s=$((s + 3200))
  build_config_is_selected AI_LLM && s=$((s + 1740)) && ai=$((ai + 1740))
  build_config_is_selected AI_STT && s=$((s + 465)) && ai=$((ai + 465))
  build_config_is_selected AI_TTS && s=$((s + 636)) && ai=$((ai + 636))
  build_config_is_selected SVC_NETWORK && s=$((s + 40))
  build_config_is_selected SVC_CUPS && s=$((s + 30))
  build_config_is_selected SVC_POWER && s=$((s + 10))
  build_config_is_selected DESKTOP_UI && s=$((s + 25))
  build_config_is_selected DESKTOP_BROWSER && s=$((s + 350))
  build_config_is_selected DESKTOP_DEMO && s=$((s + 2))
  build_config_is_selected VOLT_BENCH && s=$((s + 10))
  build_config_is_selected AI_POSTINSTALL && s=$((s - ai))
  printf '%d\n' "$s"
}

build_config_name_to_path() {
  local name="$1" dir="${2:-$CONFIG_DIR}" clean
  clean="${name%.conf}"
  case "$clean" in
    ''|.|..|*/*|*'\'*|*'..'*|~*) die "Invalid config name: $name" ;;
  esac
  case "$clean" in
    *[!A-Za-z0-9._-]*) die "Invalid config name: $name" ;;
  esac
  printf '%s/%s.conf\n' "$dir" "$clean"
}

build_config_abs_input_path() {
  local input="$1" candidate abs
  case "$input" in
    /*) candidate="$input" ;;
    *)
      if [ -f "$input" ]; then
        candidate="$PWD/$input"
      else
        candidate="$PROJECT_ROOT/$input"
      fi
      ;;
  esac
  abs="$(samaris_abs_existing_path "$candidate")" || die "Config not found: $input"
  samaris_path_under "$abs" "$PROJECT_ROOT" || die "Refusing config outside project: $abs"
  printf '%s\n' "$abs"
}

build_config_save() {
  local name="$1" dir="${2:-$CONFIG_DIR}" f
  build_config_init
  mkdir -p "$dir" 2>/dev/null || true
  f="$(build_config_name_to_path "$name" "$dir")"
  build_config_write_current > "$f"
  log "Config saved: $f"
}

build_config_load() {
  local input="$1" f tmp
  f="$(build_config_abs_input_path "$input")"
  samaris_load_kv_file "$f" build
  mkdir -p "$(dirname "$CONFIG_FILE")" 2>/dev/null || true
  tmp="${CONFIG_FILE}.tmp.$$"
  build_config_write_current > "$tmp"
  mv "$tmp" "$CONFIG_FILE"
  log "Config loaded: $f"
}

build_config_load_by_name() {
  local f
  f="$(build_config_name_to_path "$1" "$CONFIG_DIR")"
  build_config_load "$f"
}

build_config_list() {
  local dir="${1:-$CONFIG_DIR}" f
  [ -d "$dir" ] || return 0
  for f in "$dir"/*.conf; do
    [ -f "$f" ] && basename "$f" .conf
  done
}

build_config_delete() {
  local f
  f="$(build_config_name_to_path "$1" "${2:-$CONFIG_DIR}")"
  rm -f "$f" 2>/dev/null
}

build_config_rename() {
  local oldf newf
  oldf="$(build_config_name_to_path "$1" "${3:-$CONFIG_DIR}")"
  newf="$(build_config_name_to_path "$2" "${3:-$CONFIG_DIR}")"
  [ -f "$oldf" ] || return 1
  mv "$oldf" "$newf"
}

build_config_read_field() {
  local f="$1" field="$2" line k v
  while IFS= read -r line || [ -n "$line" ]; do
    line="${line%%#*}"
    line="$(samaris_trim "$line")"
    [ -z "$line" ] && continue
    k="$(samaris_trim "${line%%=*}")"
    v="$(samaris_unquote_value "${line#*=}")"
    [ "$k" = "$field" ] && { printf '%s\n' "$v"; return 0; }
  done < "$f"
  return 1
}

build_config_calc_size_tmp() {
  local f="$1" key value
  (
    while IFS='=' read -r key value; do
      [ -n "$key" ] && printf -v "$key" '%s' "$value"
    done < <(build_config_defaults)
    samaris_load_kv_file "$f" build
    build_config_calc_size
  )
}

build_config_mod_time() {
  local f="$1"
  stat -f "%Sm" -t "%m-%d %H:%M" "$f" 2>/dev/null || stat -c "%y" "$f" 2>/dev/null | cut -c6-16 || printf '?'
}

build_config_describe() {
  local name="$1" dir="${2:-$CONFIG_DIR}" f
  f="$(build_config_name_to_path "$name" "$dir")"
  [ -f "$f" ] || return 1
  (samaris_load_kv_file "$f" build) || return 1

  local has_x86 has_aarch64 has_llm has_stt has_tts has_post has_bench
  has_x86="$(build_config_read_field "$f" BUILD_CONFIG_ARCH_X86_64 || echo 0)"
  has_aarch64="$(build_config_read_field "$f" BUILD_CONFIG_ARCH_AARCH64 || echo 0)"
  has_llm="$(build_config_read_field "$f" BUILD_CONFIG_AI_LLM || echo 0)"
  has_stt="$(build_config_read_field "$f" BUILD_CONFIG_AI_STT || echo 0)"
  has_tts="$(build_config_read_field "$f" BUILD_CONFIG_AI_TTS || echo 0)"
  has_post="$(build_config_read_field "$f" BUILD_CONFIG_AI_POSTINSTALL || echo 0)"
  has_bench="$(build_config_read_field "$f" BUILD_CONFIG_VOLT_BENCH || echo 0)"

  local arch ai volts size_mb size_str mod_time
  if [ "$has_x86" = "1" ] && [ "$has_aarch64" = "1" ]; then
    arch="x86_64+aarch64"
  elif [ "$has_x86" = "1" ]; then
    arch="x86_64 only"
  elif [ "$has_aarch64" = "1" ]; then
    arch="aarch64 only"
  else
    arch="none"
  fi

  ai=""
  [ "$has_llm" = "1" ] && ai="${ai}LLM "
  [ "$has_stt" = "1" ] && ai="${ai}STT "
  [ "$has_tts" = "1" ] && ai="${ai}TTS "
  [ -z "$ai" ] && ai="none" || ai="${ai% }"
  [ "$has_post" = "1" ] && ai="${ai}(post)"
  volts="all"
  [ "$has_bench" = "0" ] && volts="no bench"
  size_mb="$(build_config_calc_size_tmp "$f")"
  size_str="$(build_config_friendly_size "$size_mb")"
  mod_time="$(build_config_mod_time "$f")"
  printf '%s|%s|%s|%s|%s|%s\n' "$name" "$arch" "$ai" "$volts" "$size_str" "$mod_time"
}

build_config_friendly_size() {
  local mb=$1
  if [ "$mb" -ge 1000 ]; then
    printf '%.1f GB' "$(echo "scale=1; $mb / 1024" | bc -l 2>/dev/null || echo "$((mb / 1024))")"
  else
    printf '%d MB' "$mb"
  fi
}

build_config_usb_fit() {
  local mb=$1
  if [ "$mb" -le 7500 ]; then
    echo "fits_8gb"
  elif [ "$mb" -le 15000 ]; then
    echo "fits_16gb"
  else
    echo "fits_32gb"
  fi
}
