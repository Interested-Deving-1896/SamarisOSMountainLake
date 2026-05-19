#!/usr/bin/env bash

BUILD_STEPS=(
  "00-check-env"
  "01-clean"
  "02-ai-assets"
  "03-rust-kernel"
  "03.5-ram-manager"
  "03.6-usb-manager"
  "03.7-gpu-manager"
  "03.8-worker-pool"
  "03.9-adaptive-config"
  "03.10-display-manager"
  "03.11-bench"
  "04-rootfs-bootstrap"
  "05-packages"
  "06-modules"
  "07-boot-theme"
  "08-overlay"
  "09-ui"
  "10-kernel-a"
  "11-electron"
  "12-squashfs"
  "13-iso-tree"
  "14-iso-image"
  "15-checksums"
  "16-validate"
  "18-report"
)

normalize_step() {
  local requested="$1"
  local step matches=()
  for step in "${BUILD_STEPS[@]}" "17-qemu"; do
    if [ "$requested" = "$step" ] || [[ "$step" == "$requested"-* ]]; then
      matches+=("$step")
    fi
  done
  if [ "${#matches[@]}" -eq 1 ]; then
    printf '%s\n' "${matches[0]}"
    return 0
  fi
  if [ "${#matches[@]}" -gt 1 ]; then
    die "Ambiguous step '$requested': ${matches[*]}"
  fi
  die "Unknown step: $requested"
}

step_index() {
  local requested
  requested="$(normalize_step "$1")"
  local i
  for i in "${!BUILD_STEPS[@]}"; do
    if [ "${BUILD_STEPS[$i]}" = "$requested" ]; then
      printf '%s\n' "$i"
      return 0
    fi
  done
  return 1
}

list_steps() {
  local step
  for step in "${BUILD_STEPS[@]}"; do
    printf '%s\n' "$step"
  done
}

checkpoint_valid() {
  local name="$1"
  local file="$STEPS_DIR/$name.sh"
  [ -f "$file" ] || return 1
  (
    set -euo pipefail
    # shellcheck source=/dev/null
    source "$file"
    if declare -F step_validate >/dev/null 2>&1; then
      step_validate
    fi
  )
}

run_step() {
  local name
  name="$(normalize_step "$1")"
  shift || true
  local file="$STEPS_DIR/$name.sh"
  local log_file
  log_file="$(state_log_path "$name")"
  [ -f "$file" ] || die "Missing step: $name"

  # Check if this step is needed per build config
  if ! build_config_step_needed "$name" 2>/dev/null; then
    local reason
    case "$name" in
      03.11-bench)         reason="Bench not selected" ;;
      03.10-display-manager) reason="Display manager not selected" ;;
      11-electron)         reason="Browser not selected" ;;
      09-ui)               reason="Desktop UI not selected" ;;
      02-ai-assets)        reason="No AI modules selected" ;;
      *)                   reason="Module not in config" ;;
    esac
    log "skip [$name] $reason"
    state_mark_skipped "$name" "$reason"
    return 0
  fi

  if [ "${BUILD_DRY_RUN:-0}" = "1" ]; then
    log "dry-run [$name] would execute"
    return 0
  fi

  local start_time end_time duration
  start_time=$(date +%s)

  if [ "${RUN_FORCE:-0}" != "1" ] && state_done "$name"; then
    if checkpoint_valid "$name" >/dev/null 2>&1; then
      log "cache HIT  $name"
      echo "cache_hit" >> "$STATE_DIR/.build_session_cache" 2>/dev/null || true
      return 0
    fi
    warn "checkpoint invalid, rerunning: $name"
    state_clear_step "$name"
  fi

  banner "$name"
  state_mark_running "$name"
  local statuses=()
  set +e
  (
    set -euo pipefail
    # shellcheck source=/dev/null
    source "$file"
    step_main "$@"
    if declare -F step_validate >/dev/null 2>&1; then
      step_validate
    fi
  ) 2>&1 | tee "$log_file"
  statuses=("${PIPESTATUS[@]}")
  set -e

  end_time=$(date +%s)
  duration=$((end_time - start_time))

  if [ "${statuses[0]:-1}" -eq 0 ] && [ "${statuses[1]:-1}" -eq 0 ]; then
    state_finish_running "$name"
    state_mark "$name" "$duration" "miss"
    echo "cache_miss" >> "$STATE_DIR/.build_session_cache" 2>/dev/null || true
    log "done  [$name] ${duration}s"
  else
    state_mark_failed "$name"
    log "FAIL  [$name] ${duration}s"
    return "${statuses[0]:-1}"
  fi
}

run_steps() {
  local step
  for step in "$@"; do
    run_step "$step"
  done
}

run_from_step() {
  local from="$1"
  local start
  start="$(step_index "$from")"
  local i
  for ((i = start; i < ${#BUILD_STEPS[@]}; i++)); do
    state_clear_step "${BUILD_STEPS[$i]}"
  done
  for ((i = start; i < ${#BUILD_STEPS[@]}; i++)); do
    run_step "${BUILD_STEPS[$i]}"
  done
}

run_next_step() {
  local step
  for step in "${BUILD_STEPS[@]}"; do
    if ! state_complete "$step"; then
      run_step "$step"
      return 0
    fi
  done
  log "all checkpoints completed"
}

run_remaining_steps() {
  local step
  for step in "${BUILD_STEPS[@]}"; do
    if ! state_complete "$step"; then
      run_from_step "$step"
      return 0
    fi
  done
  log "all checkpoints completed"
}

status_steps() {
  local validate="${1:-0}"
  local step status
  state_init
  for step in "${BUILD_STEPS[@]}"; do
    status="pending"
    if state_status_file "$step" running >/dev/null 2>&1; then
      status="running"
    elif state_status_file "$step" failed >/dev/null 2>&1; then
      status="failed"
    elif state_status_file "$step" skipped >/dev/null 2>&1; then
      status="skipped"
    elif state_done "$step"; then
      status="done"
      if [ "$validate" = "1" ]; then
        if checkpoint_valid "$step" >/dev/null 2>&1; then
          status="done:valid"
        else
          status="done:invalid"
        fi
      fi
    fi
    printf '%-22s %s\n' "$step" "$status"
  done
  printf '\nstate: %s\nlogs:  %s\n' "$STATE_DIR" "$STATE_LOG_DIR"
}

# ─── Background Build ─────────────────────────────────────

BUILD_BG_PID=""
BUILD_BG_ABORT=0

run_steps_bg() {
  local build_log="$STATE_LOG_DIR/build.log"
  mkdir -p "$STATE_LOG_DIR" 2>/dev/null || true
  rm -f "$STATE_DIR/.build_session_cache" 2>/dev/null || true
  (
    run_steps "$@"
  ) > "$build_log" 2>&1 &
  BUILD_BG_PID=$!
  printf '%d' "$BUILD_BG_PID"
}

build_bg_is_running() {
  local pid="${1:-$BUILD_BG_PID}"
  [ -z "$pid" ] || [ "$pid" -le 0 ] 2>/dev/null && return 1
  kill -0 "$pid" 2>/dev/null
}

build_bg_abort() {
  local pid="${1:-$BUILD_BG_PID}"
  [ -z "$pid" ] || [ "$pid" -le 0 ] 2>/dev/null && return
  BUILD_BG_ABORT=1
  kill -TERM -"$pid" 2>/dev/null || kill -TERM "$pid" 2>/dev/null || true
  sleep 1
  kill -KILL -"$pid" 2>/dev/null || kill -KILL "$pid" 2>/dev/null || true
}

build_bg_toggle_pause() {
  local pid="${1:-$BUILD_BG_PID}"
  [ -z "$pid" ] || [ "$pid" -le 0 ] 2>/dev/null && return
  if kill -STOP "$pid" 2>/dev/null; then
    # send SIGCONT after a while (manual unpause)
    BUILD_BG_PAUSED=1
  else
    kill -CONT "$pid" 2>/dev/null || true
    BUILD_BG_PAUSED=0
  fi
}

build_bg_session_cache_stats() {
  local session_file="$STATE_DIR/.build_session_cache"
  local hits=0 misses=0
  if [ -f "$session_file" ]; then
    hits=$(grep -c 'cache_hit' "$session_file" 2>/dev/null || echo 0)
    misses=$(grep -c 'cache_miss' "$session_file" 2>/dev/null || echo 0)
  fi
  printf '%d %d' "$hits" "$misses"
}
