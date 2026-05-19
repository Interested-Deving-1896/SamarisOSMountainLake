#!/usr/bin/env bash

log() {
  local ts
  ts="$(date +%H:%M:%S)"
  local msg="[samaris:$ts] $*"
  if command -v tput >/dev/null 2>&1 && [ "$(tput colors 2>/dev/null || echo 0)" -ge 8 ]; then
    tput setaf 2 2>/dev/null || true
    printf '%s\n' "$msg"
    tput sgr0 2>/dev/null || true
  else
    printf '%s\n' "$msg"
  fi
}

warn() {
  local ts
  ts="$(date +%H:%M:%S)"
  local msg="[samaris:$ts] warning: $*"
  if command -v tput >/dev/null 2>&1 && [ "$(tput colors 2>/dev/null || echo 0)" -ge 8 ]; then
    tput setaf 3 2>/dev/null || true
    printf '%s\n' "$msg" >&2
    tput sgr0 2>/dev/null || true
  else
    printf '%s\n' "$msg" >&2
  fi
}

die() {
  local ts
  ts="$(date +%H:%M:%S)"
  local msg="[samaris:$ts] error: $*"
  if command -v tput >/dev/null 2>&1 && [ "$(tput colors 2>/dev/null || echo 0)" -ge 8 ]; then
    tput setaf 1 2>/dev/null || true
    printf '%s\n' "$msg" >&2
    tput sgr0 2>/dev/null || true
  else
    printf '%s\n' "$msg" >&2
  fi
  exit 1
}

banner() {
  printf '\n==> %s\n' "$*"
}
