#!/usr/bin/env bash

STATE_DIR="$WORK_DIR/state"
PUBLIC_STATE_DIR="$OUTPUT_DIR/checkpoints"
STATE_LOG_DIR="$PUBLIC_STATE_DIR/logs"

state_init() {
  mkdir -p "$STATE_DIR" 2>/dev/null || true
  mkdir -p "$PUBLIC_STATE_DIR" "$STATE_LOG_DIR" 2>/dev/null || true
}

state_mark() {
  local name="$1" duration="${2:-}" cache="${3:-}"
  state_init
  local stamp
  stamp="$(date -u +"%Y-%m-%dT%H:%M:%SZ")"
  {
    printf 'step=%s\n' "$name"
    printf 'status=done\n'
    printf 'completed_at=%s\n' "$stamp"
    [ -n "$duration" ] && printf 'duration=%s\n' "$duration"
    [ -n "$cache" ] && printf 'cache=%s\n' "$cache"
    printf 'work_dir=%s\n' "$WORK_DIR"
  } > "$STATE_DIR/$name.done"
  cp "$STATE_DIR/$name.done" "$PUBLIC_STATE_DIR/$name.done" 2>/dev/null || true
  rm -f "$STATE_DIR/$name.failed" "$STATE_DIR/$name.skipped" "$STATE_DIR/$name.running"
  rm -f "$PUBLIC_STATE_DIR/$name.failed" "$PUBLIC_STATE_DIR/$name.skipped" "$PUBLIC_STATE_DIR/$name.running"
}

state_mark_running() {
  local name="$1"
  state_init
  printf 'step=%s\nstatus=running\nstarted_at=%s\n' "$name" "$(date -u +"%Y-%m-%dT%H:%M:%SZ")" > "$STATE_DIR/$name.running"
  cp "$STATE_DIR/$name.running" "$PUBLIC_STATE_DIR/$name.running" 2>/dev/null || true
  rm -f "$STATE_DIR/$name.failed" "$STATE_DIR/$name.skipped"
  rm -f "$PUBLIC_STATE_DIR/$name.failed" "$PUBLIC_STATE_DIR/$name.skipped"
}

state_mark_failed() {
  local name="$1"
  state_init
  local stamp
  stamp="$(date -u +"%Y-%m-%dT%H:%M:%SZ")"
  {
    printf 'step=%s\n' "$name"
    printf 'status=failed\n'
    printf 'failed_at=%s\n' "$stamp"
    printf 'log=%s\n' "$STATE_LOG_DIR/$name.log"
  } > "$STATE_DIR/$name.failed"
  cp "$STATE_DIR/$name.failed" "$PUBLIC_STATE_DIR/$name.failed" 2>/dev/null || true
  rm -f "$STATE_DIR/$name.running" "$STATE_DIR/$name.skipped"
  rm -f "$PUBLIC_STATE_DIR/$name.running" "$PUBLIC_STATE_DIR/$name.skipped"
}

state_mark_skipped() {
  local name="$1" reason="${2:-skipped by config}"
  state_init
  local stamp
  stamp="$(date -u +"%Y-%m-%dT%H:%M:%SZ")"
  {
    printf 'step=%s\n' "$name"
    printf 'status=skipped\n'
    printf 'skipped_at=%s\n' "$stamp"
    printf 'reason=%s\n' "$reason"
    printf 'work_dir=%s\n' "$WORK_DIR"
  } > "$STATE_DIR/$name.skipped"
  cp "$STATE_DIR/$name.skipped" "$PUBLIC_STATE_DIR/$name.skipped" 2>/dev/null || true
  rm -f "$STATE_DIR/$name.done" "$STATE_DIR/$name.failed" "$STATE_DIR/$name.running"
  rm -f "$PUBLIC_STATE_DIR/$name.done" "$PUBLIC_STATE_DIR/$name.failed" "$PUBLIC_STATE_DIR/$name.running"
}

state_finish_running() {
  local name="$1"
  rm -f "$STATE_DIR/$name.running" "$PUBLIC_STATE_DIR/$name.running"
}

state_done() {
  [ -f "$STATE_DIR/$1.done" ] || [ -f "$PUBLIC_STATE_DIR/$1.done" ]
}

state_skipped() {
  [ -f "$STATE_DIR/$1.skipped" ] || [ -f "$PUBLIC_STATE_DIR/$1.skipped" ]
}

state_complete() {
  state_done "$1" || state_skipped "$1"
}

state_status_file() {
  local name="$1" ext="$2"
  if [ -f "$STATE_DIR/$name.$ext" ]; then
    printf '%s/%s.%s\n' "$STATE_DIR" "$name" "$ext"
  elif [ -f "$PUBLIC_STATE_DIR/$name.$ext" ]; then
    printf '%s/%s.%s\n' "$PUBLIC_STATE_DIR" "$name" "$ext"
  else
    return 1
  fi
}

state_clear_step() {
  local name="$1"
  rm -f "$STATE_DIR/$name.done" "$STATE_DIR/$name.failed" "$STATE_DIR/$name.running" "$STATE_DIR/$name.skipped"
  rm -f "$PUBLIC_STATE_DIR/$name.done" "$PUBLIC_STATE_DIR/$name.failed" "$PUBLIC_STATE_DIR/$name.running" "$PUBLIC_STATE_DIR/$name.skipped"
}

state_clear_all() {
  samaris_assert_safe_layout
  rm -rf "$STATE_DIR" "$PUBLIC_STATE_DIR"
  state_init
}

state_log_path() {
  state_init
  printf '%s/%s.log\n' "$STATE_LOG_DIR" "$1"
}

state_count_done() {
  local count=0 step
  for step in "${BUILD_STEPS[@]}"; do
    state_complete "$step" && count=$((count + 1))
  done
  printf '%d' "$count"
}

state_total_steps() {
  printf '%d' "${#BUILD_STEPS[@]}"
}

state_all_done() {
  local step
  for step in "${BUILD_STEPS[@]}"; do
    state_complete "$step" || return 1
  done
  return 0
}

# Extract field from key=value state file (macOS grep -P compat)
state_get_field() {
  local field="$1" file="${2:-}"
  [ -f "$file" ] || return 1
  sed -n "s/.*${field}=//p" "$file"
}
