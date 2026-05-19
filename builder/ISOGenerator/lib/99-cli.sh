#!/usr/bin/env bash
# SAMARIS ISO GENERATOR - CLI dispatcher

usage() {
  cat <<'USAGE'
Samaris OS ISO Generator

Usage:
  generator.sh
  generator.sh help
  generator.sh iso [--docker] [--config FILE] [--from STEP|--only STEP] [--force] [--dry-run]
  generator.sh status [--docker] [--validate] [--config FILE]
  generator.sh steps
  generator.sh next [--docker] [--config FILE] [--force] [--dry-run]
  generator.sh run STEP [--docker] [--config FILE] [--force] [--dry-run]
  generator.sh clean [--docker] [--dry-run]
  generator.sh qemu [ARCH] [-- qemu args]
  generator.sh tui
  generator.sh build [--config FILE] [--docker]

Options:
  --docker      Run inside Docker container
  --config FILE Load a strict build config
  --from STEP   Resume build from STEP
  --only STEP   Run only STEP
  --force       Ignore checkpoints
  --dry-run     Print steps without executing them
  --validate    Validate checkpoint integrity
USAGE
}

docker_volume_name() {
  printf '%s\n' "${SAMARIS_DOCKER_WORK_VOLUME:-samaris-os-work}"
}

docker_project_path() {
  local path="$1"
  case "$path" in
    "$PROJECT_ROOT") printf '/work\n' ;;
    "$PROJECT_ROOT"/*) printf '/work/%s\n' "${path#"$PROJECT_ROOT"/}" ;;
    *) return 1 ;;
  esac
}

run_in_docker() {
  command -v docker >/dev/null 2>&1 || die "Docker is required for --docker"
  local volume
  volume="$(docker_volume_name)"
  if [ "${SAMARIS_REBUILD_BUILDER:-0}" = "1" ] || ! docker image inspect samaris-os-builder:trixie >/dev/null 2>&1; then
    log "Building Docker builder image"
    docker build --platform=linux/amd64 -t samaris-os-builder:trixie -f "$BUILDER_ROOT/docker/Dockerfile.builder" "$BUILDER_ROOT"
  else
    log "Using cached Docker builder image"
  fi
  docker volume create "$volume" >/dev/null
  log "Running inside Docker with persistent work volume: $volume"
  docker run --rm --privileged --platform=linux/amd64 \
    -e SAMARIS_IN_DOCKER=1 \
    -e SAMARIS_WORK_DIR=/samaris-work \
    -e SAMARIS_DOCKER_WORK_VOLUME="$volume" \
    -v "$volume":/samaris-work \
    -v "$PROJECT_ROOT":/work \
    -w /work \
    samaris-os-builder:trixie \
    bash -lc 'exec ./builder/ISOGenerator/generator.sh "$@"' bash "$@"
}

parse_docker_flag() {
  USE_DOCKER="${USE_DOCKER:-}"
  PARSED_ARGS=()
  PARSED_COUNT=0
  local arg
  for arg in "$@"; do
    if [ "$arg" = "--docker" ]; then
      USE_DOCKER=1
    else
      PARSED_ARGS+=("$arg")
      PARSED_COUNT=$((PARSED_COUNT + 1))
    fi
  done
}

apply_cli_config() {
  local config_file="$1"
  if [ -n "$config_file" ]; then
    build_config_load "$config_file"
  else
    build_config_init
  fi
  build_config_apply
}

run_iso_local() {
  local from="" only="" config_file=""
  local cli_dry_run=0
  RUN_FORCE=0
  BUILD_DRY_RUN="${BUILD_DRY_RUN:-0}"
  while [ "$#" -gt 0 ]; do
    case "$1" in
      --config) config_file="${2:?--config requires a file}"; shift 2 ;;
      --from) from="${2:?--from requires a step}"; shift 2 ;;
      --only) only="${2:?--only requires a step}"; shift 2 ;;
      --force) RUN_FORCE=1; shift ;;
      --dry-run) cli_dry_run=1; shift ;;
      *) die "Unknown iso option: $1" ;;
    esac
  done
  [ "$cli_dry_run" = "1" ] && BUILD_DRY_RUN=1
  apply_cli_config "$config_file"
  [ "$cli_dry_run" = "1" ] && BUILD_DRY_RUN=1
  export BUILD_DRY_RUN RUN_FORCE
  if [ -n "$only" ]; then
    run_step "$only"
  elif [ -n "$from" ]; then
    run_from_step "$from"
  elif [ "${BUILD_MODE:-full}" = "resume" ]; then
    run_remaining_steps
  else
    run_steps "${BUILD_STEPS[@]}"
  fi
}

run_step_local_cli() {
  local step="${1:-}" config_file=""
  local cli_dry_run=0
  [ -n "$step" ] || die "run requires a step"
  shift || true
  RUN_FORCE=0
  BUILD_DRY_RUN="${BUILD_DRY_RUN:-0}"
  while [ "$#" -gt 0 ]; do
    case "$1" in
      --config) config_file="${2:?--config requires a file}"; shift 2 ;;
      --force) RUN_FORCE=1; shift ;;
      --dry-run) cli_dry_run=1; shift ;;
      *) die "Unknown run option: $1" ;;
    esac
  done
  [ "$cli_dry_run" = "1" ] && BUILD_DRY_RUN=1
  apply_cli_config "$config_file"
  [ "$cli_dry_run" = "1" ] && BUILD_DRY_RUN=1
  export RUN_FORCE BUILD_DRY_RUN
  [ "$RUN_FORCE" = "1" ] && state_clear_step "$step"
  run_step "$step"
}

run_next_local() {
  local config_file=""
  local cli_dry_run=0
  RUN_FORCE=0
  BUILD_DRY_RUN="${BUILD_DRY_RUN:-0}"
  while [ "$#" -gt 0 ]; do
    case "$1" in
      --config) config_file="${2:?--config requires a file}"; shift 2 ;;
      --force) RUN_FORCE=1; shift ;;
      --dry-run) cli_dry_run=1; shift ;;
      *) die "Unknown next option: $1" ;;
    esac
  done
  [ "$cli_dry_run" = "1" ] && BUILD_DRY_RUN=1
  apply_cli_config "$config_file"
  [ "$cli_dry_run" = "1" ] && BUILD_DRY_RUN=1
  export RUN_FORCE BUILD_DRY_RUN
  run_next_step
}

run_clean_local() {
  local dry_run=0
  while [ "$#" -gt 0 ]; do
    case "$1" in
      --dry-run) dry_run=1; shift ;;
      *) die "Unknown clean option: $1" ;;
    esac
  done
  if [ "$dry_run" = "1" ]; then
    printf 'would clean state:  %s\n' "$STATE_DIR"
    printf 'would clean public: %s\n' "$PUBLIC_STATE_DIR"
    printf 'would reset work:   %s\n' "$WORK_DIR"
    printf 'would keep cache:   %s\n' "$CACHE_DIR"
    printf 'would keep output:  %s\n' "$OUTPUT_DIR"
    return 0
  fi
  RUN_FORCE=1
  state_clear_all
  run_step "01-clean"
}

run_build_local() {
  local config_file=""
  local cli_dry_run=0
  RUN_FORCE=0
  BUILD_DRY_RUN="${BUILD_DRY_RUN:-0}"
  while [ "$#" -gt 0 ]; do
    case "$1" in
      --config) config_file="${2:?--config requires a file}"; shift 2 ;;
      --force) RUN_FORCE=1; shift ;;
      --dry-run) cli_dry_run=1; shift ;;
      *) die "Unknown build option: $1" ;;
    esac
  done
  [ "$cli_dry_run" = "1" ] && BUILD_DRY_RUN=1
  apply_cli_config "$config_file"
  [ "$cli_dry_run" = "1" ] && BUILD_DRY_RUN=1
  export RUN_FORCE BUILD_DRY_RUN
  run_steps "${BUILD_STEPS[@]}"
}

status_local() {
  local validate=0 config_file=""
  while [ "$#" -gt 0 ]; do
    case "$1" in
      --validate) validate=1; shift ;;
      --config) config_file="${2:?--config requires a file}"; shift 2 ;;
      *) die "Unknown status option: $1" ;;
    esac
  done
  [ -n "$config_file" ] && apply_cli_config "$config_file" >/dev/null
  status_steps "$validate"
}

main() {
  local cmd="${1:-}"
  shift 2>/dev/null || true

  if [ -z "$cmd" ] && [ -t 0 ] && [ -t 1 ]; then
    tui_main
    return 0
  fi

  parse_docker_flag "$@"
  if [ "${PARSED_COUNT:-0}" -gt 0 ]; then
    set -- "${PARSED_ARGS[@]}"
  else
    set --
  fi

  if [ "${USE_DOCKER:-0}" = "1" ] && [ "$cmd" != "help" ] && [ "${SAMARIS_IN_DOCKER:-0}" != "1" ]; then
    run_in_docker "$cmd" "$@"
    return $?
  fi

  case "$cmd" in
    ""|tui)
      tui_main
      ;;
    help|-h|--help)
      usage
      ;;
    build)
      run_build_local "$@"
      ;;
    check)
      apply_cli_config ""
      run_step "00-check-env"
      ;;
    iso)
      run_iso_local "$@"
      ;;
    status)
      status_local "$@"
      ;;
    steps)
      list_steps
      ;;
    next)
      run_next_local "$@"
      ;;
    run)
      run_step_local_cli "$@"
      ;;
    clean)
      run_clean_local "$@"
      ;;
    qemu)
      apply_cli_config ""
      run_step "17-qemu" "$@"
      ;;
    *)
      usage >&2
      die "Unknown command: $cmd"
      ;;
  esac
}
