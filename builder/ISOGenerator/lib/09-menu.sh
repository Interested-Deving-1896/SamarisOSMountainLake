#!/usr/bin/env bash
# SAMARIS ISO GENERATOR — Screen Handlers

[ -n "${SAMARIS_MENU_LOADED:-}" ] && return 0
SAMARIS_MENU_LOADED=1

tui_state_file() {
  state_status_file "$1" "$2" 2>/dev/null || true
}

tui_step_status() {
  local step="$1"
  if state_status_file "$step" running >/dev/null 2>&1; then
    printf 'running'
  elif state_status_file "$step" failed >/dev/null 2>&1; then
    printf 'failed'
  elif state_status_file "$step" skipped >/dev/null 2>&1; then
    printf 'skipped'
  elif state_status_file "$step" done >/dev/null 2>&1; then
    printf 'done'
  else
    printf 'pending'
  fi
}

tui_state_get() {
  local step="$1" ext="$2" field="$3" file
  file="$(tui_state_file "$step" "$ext")"
  [ -n "$file" ] || return 1
  state_get_field "$field" "$file"
}

tui_latest_done_file() {
  local step file latest_file=""
  for step in "${BUILD_STEPS[@]}"; do
    file="$(tui_state_file "$step" done)"
    [ -n "$file" ] && [ "$file" -nt "$latest_file" ] 2>/dev/null && latest_file="$file"
  done
  [ -n "$latest_file" ] && printf '%s\n' "$latest_file"
}

tui_first_incomplete_step() {
  local step
  for step in "${BUILD_STEPS[@]}"; do
    state_complete "$step" || { printf '%s\n' "$step"; return 0; }
  done
  return 1
}

# ─── Main Menu (Screen 2) ─────────────────────────────────

tui_main_menu() {
  local choice
  local w=$TUI_COLS
  local total=${#BUILD_STEPS[@]} done_count=0
  for step in "${BUILD_STEPS[@]}"; do state_complete "$step" && done_count=$((done_count+1)); done

  local last_build="Aucun"
  local latest_file
  latest_file="$(tui_latest_done_file)"
  if [ -n "$latest_file" ]; then
    local ts; ts=$(state_get_field completed_at "$latest_file" 2>/dev/null | sed 's/T/ /; s/Z//' || echo "?")
    ts="${ts%:*}"; ts="${ts%:*}"; [ -z "$ts" ] && ts="?"
    last_build="$ts"
  fi
  local arches="${SAMARIS_ARCHES:-x86_64 aarch64}"
  local arch1="${arches%% *}" arch2="${arches##* }"
  [ "$arch1" = "$arch2" ] && arch2=""

  while true; do
    tui_clear; tui_hide_cursor
    tui_box_top $w
    tui_center "$w" "$(tui_c bold "$(tui_icon config) SAMARIS ISO GENERATOR $(tui_icon config)")"; echo
    echo

    f() { printf '%s  %s\n' "$TUI_BOX_V" "$*"; }
    local ml=40
    local mr=$((w - ml - 6))
    f ""
    f "  $(tui_c bold "1")  $(tui_icon build) BUILDER SON ISO                $(tui_c bold "$(tui_h "[1]")")"
    f "     Configuration « à la carte »"
    f ""
    f "  $(tui_c bold "2")  $(tui_icon chart) TABLEAU DE BORD                $(tui_c bold "$(tui_h "[2]")")"
    f "     Checkpoints, validation, stockage"
    f ""
    f "  $(tui_c bold "3")  $(tui_icon play) EXÉCUTER UNE ÉTAPE             $(tui_c bold "$(tui_h "[3]")")"
    f "     Choisir et lancer une étape du pipeline"
    f ""
    f "  $(tui_c bold "4")  $(tui_icon search) VÉRIFIER ENVIRONNEMENT        $(tui_c bold "$(tui_h "[4]")")"
    f "     Valider les dépendances de build"
    f ""
    f "  $(tui_c bold "5")  $(tui_icon clean) NETTOYER                       $(tui_c bold "$(tui_h "[5]")")"
    f "     Workdir, caches, checkpoints"
    f ""
    f "  $(tui_c bold "6")  $(tui_icon save) CONFIGURATIONS                 $(tui_c bold "$(tui_h "[6]")")"
    f "     Sauvegarder / restaurer"
    f ""
    f "  $(tui_c bold "7")  $(tui_icon qemu) DÉMARRER DANS QEMU             $(tui_c bold "$(tui_h "[7]")")"
    f "     Booter l'ISO dans un émulateur"
    f ""
    f "  $(tui_c bold "q")  $(tui_icon quit) QUITTER                        $(tui_c bold "$(tui_h "[q]")")"
    f ""

    tui_box_sep $w
    f "  $(tui_h "[1-7] menu  [r] refresh  [q] quit")"
    f "  Dernière build : $(tui_h "$last_build")  |  $arch1: $(tui_c $([ "$done_count" -eq "$total" ] && echo green || echo yellow) "$done_count/$total")${arch2:+  |  ${arch2}: $(tui_c $([ "$done_count" -eq "$total" ] && echo green || echo yellow) "$done_count/$total")}"

    tui_box_bot $w
    printf '\n  %s  ' "$(tui_c bold 'Votre choix [1-7, q] :')"
    choice=$(tui_read_key)
    case "$choice" in
      1|b|B) echo "build";  return 0 ;;
      2|s|S) echo "status"; return 0 ;;
      3) echo "run";    return 0 ;;
      4|c|C) echo "check";  return 0 ;;
      5|l|L) echo "clean";  return 0 ;;
      6|o|O) echo "config"; return 0 ;;
      7|qemu|QEMU) echo "qemu";   return 0 ;;
      r|R)   continue ;;
      q|Q)   echo "quit";  return 0 ;;
      *)     printf '\r%s\r' "$(tui_c yellow "Choix invalide — appuyez sur 1-7 ou q")"; sleep 0.5; continue ;;
    esac
  done
}

# ─── Build Config ──────────────────────────────────────────

tui_build_config() {
  build_config_init
  local focus=0 items=() vals=() labels=() sections=() rows=()
  local section_names=("ARCHITECTURES" "AI MODULES" "VOLT DAEMONS" "SERVICES" "DESKTOP & UI" "BUILD MODE" "BUILD OPTIONS")
  local section_starts=(0 2 7 13 16 19 22)

  build_sections() {
    items=(); vals=(); labels=(); sections=(); rows=()
    local r=0
    add_static_row() {
      local section="$1"
      items[r]=""; vals[r]=""; labels[r]=""; sections[r]="$section"; rows[r]=0
      r=$((r+1))
    }
    add_item() {
      local item="$1" val="$2" label="$3" section="$4" row="$5"
      items[r]="$item"; vals[r]="$val"; labels[r]="$label"; sections[r]="$section"; rows[r]="$row"
      r=$((r+1))
    }
    # Row 0: title
    add_static_row "header"
    add_static_row "arch_header"
    add_item "ARCH_X86_64" "$(build_config_get ARCH_X86_64)" "x86_64 (Intel/AMD)                    ~3.2 GB" "arch" 1
    add_item "ARCH_AARCH64" "$(build_config_get ARCH_AARCH64)" "aarch64 (ARM64 / Apple Silicon)       ~3.2 GB" "arch" 2
    add_static_row "ai_header"
    add_item "AI_LLM" "$(build_config_get AI_LLM)" "Orbit LLM (Qwen3 1.7B Q8_0)         ~1.7 GB" "ai" 1
    add_item "AI_STT" "$(build_config_get AI_STT)" "Whisper STT (ggml-small)             ~465 MB" "ai" 2
    add_item "AI_TTS" "$(build_config_get AI_TTS)" "OuteTTS + WavTokenizer               ~636 MB" "ai" 3
    add_item "AI_POSTINSTALL" "$(build_config_get AI_POSTINSTALL)" "Download models after install (smaller ISO)" "ai" 4
    add_static_row "volt_header"
    # Volt daemons are always included (no toggle)
    add_static_row "volt_info"
    add_item "VOLT_BENCH" "$(build_config_get VOLT_BENCH)" "Bench (performance testing)          ~10 MB" "volt" 1
    add_static_row "svc_header"
    add_item "SVC_NETWORK" "$(build_config_get SVC_NETWORK)" "NetworkManager + Bluetooth           ~40 MB" "svc" 1
    add_item "SVC_CUPS" "$(build_config_get SVC_CUPS)" "CUPS + Avahi                         ~30 MB" "svc" 2
    add_item "SVC_POWER" "$(build_config_get SVC_POWER)" "upower + acpid                       ~10 MB" "svc" 3
    add_static_row "desktop_header"
    add_item "DESKTOP_UI" "$(build_config_get DESKTOP_UI)" "Volt Desktop UI (React)              ~5 MB" "desktop" 1
    add_item "DESKTOP_BROWSER" "$(build_config_get DESKTOP_BROWSER)" "Browser Chromium (Electron)          ~350 MB" "desktop" 2
    add_item "DESKTOP_DEMO" "$(build_config_get DESKTOP_DEMO)" "Demo module                         ~2 MB" "desktop" 3
    add_static_row "mode_header"
    local mode_val; mode_val=$(build_config_get MODE)
    add_item "MODE_FULL" "$( [ "$mode_val" = "full" ] && echo 1 || echo 0 )" "Full build (all steps)" "mode" 1
    add_item "MODE_RESUME" "$( [ "$mode_val" = "resume" ] && echo 1 || echo 0 )" "Resume from last checkpoint" "mode" 2
    add_static_row "opt_header"
    add_item "USE_DOCKER" "$(build_config_get USE_DOCKER)" "Use Docker for cross-compilation" "opt" 1
    add_item "FORCE" "$(build_config_get FORCE)" "Force rebuild (ignore checkpoints)" "opt" 2
    add_item "DRY_RUN" "$(build_config_get DRY_RUN)" "Dry run (show what would execute)" "opt" 3
  }

  build_sections
  local total_items=${#items[@]}
  local scroll_offset=0
  local max_visible=$((TUI_LINES - 12))

  first_config_item() {
    local i
    for ((i = 0; i < total_items; i++)); do
      [ -n "${items[$i]:-}" ] && { printf '%d\n' "$i"; return; }
    done
    printf '0\n'
  }

  move_config_focus() {
    local dir="$1" next="$focus"
    while true; do
      next=$((next + dir))
      [ "$next" -lt 0 ] && return
      [ "$next" -ge "$total_items" ] && return
      if [ -n "${items[$next]:-}" ]; then
        focus="$next"
        return
      fi
    done
  }

  focus="$(first_config_item)"

  render() {
    tui_clear
    tui_hide_cursor
    local w=$TUI_COLS
    local title="BUILD CONFIGURATION — A LA CARTE"
    local pad=$(((w - ${#title}) / 2))
    [ "$pad" -lt 0 ] && pad=0
    printf '%*s' "$pad" ''; tui_c bold "  $title  "; echo
    tui_h "  $(tui_hr '~' $((w-4)))"; echo
    echo

    local line=0 disp=0 i=$scroll_offset
    while [ "$disp" -lt "$max_visible" ] && [ "$i" -lt "$total_items" ]; do
      local sec="${sections[$i]:-}"
      local row="${rows[$i]:-0}"

      if [ "$row" -eq 0 ] && [[ "$sec" == *_header ]]; then
        local hdr="${sec%_header}"
        printf '  '; tui_c bold "${hdr^^}"; echo
        echo
        line=$((line+2)); disp=$((disp+2))
        i=$((i+1)); continue
      fi

      if [ "$row" -eq 0 ] && [ "$sec" = "volt_info" ]; then
        tui_h "  Kernel B, VRM, VGM, VUM, DWP, ASC are always included (core system)"; echo
        echo
        line=$((line+2)); disp=$((disp+2))
        i=$((i+1)); continue
      fi

      if [ "$row" -eq 0 ] && [ "$sec" = "header" ]; then
        i=$((i+1)); continue
      fi

      local prefix="  "
      [ "$i" = "$focus" ] && prefix=" >"

      local item="${items[$i]}"
      local val="${vals[$i]}"
      local lbl="${labels[$i]}"

      if [ "${item#MODE_}" != "$item" ]; then
        if [ "$val" = "1" ]; then printf '%s ' "$prefix$(tui_c rad_on)"; else printf '%s ' "$prefix$(tui_c rad_off)"; fi
        printf '%s\n' "$(tui_c bold "$lbl")"
      elif [ "$item" = "AI_POSTINSTALL" ]; then
        if [ "$val" = "1" ]; then printf '%s ' "$prefix$(tui_c check_on)"; else printf '%s ' "$prefix$(tui_c check_off)"; fi
        printf '%s\n' "$lbl"
      else
        if [ "$val" = "1" ]; then printf '%s ' "$prefix$(tui_c check_on)"; else printf '%s ' "$prefix$(tui_c check_off)"; fi
        printf '%s\n' "$lbl"
      fi

      line=$((line+1)); disp=$((disp+1))
      i=$((i+1))
    done

    # Size summary
    local total_size; total_size=$(build_config_calc_size)
    local size_str; size_str=$(build_config_friendly_size "$total_size")
    local usb; usb=$(build_config_usb_fit "$total_size")
    local usb_msg
    case "$usb" in
      fits_8gb)  usb_msg="$(tui_c green "Fits 8 GB USB")" ;;
      fits_16gb) usb_msg="$(tui_c yellow "Fits 16 GB USB")" ;;
      *)         usb_msg="$(tui_c red "Needs 32+ GB USB")" ;;
    esac

    tui_h "  $(tui_hr '-' $((w-4)))"; echo
    printf '  Total: %-10s  %s\n' "$size_str" "$usb_msg"
    echo
    tui_h "  [↑↓] Navigate  [Space] Toggle  [Enter] Build  [F1] Save  [F2] Load  [Esc] Menu"; echo
  }

  while true; do
    render
    local key
    key=$(tui_read_key)
    case "$key" in
      UP)
        move_config_focus -1
        if [ "$focus" -lt "$scroll_offset" ]; then scroll_offset=$focus; fi
        ;;
      DOWN)
        move_config_focus 1
        if [ "$focus" -ge $((scroll_offset + max_visible)) ]; then scroll_offset=$((focus - max_visible + 1)); fi
        ;;
      SPACE)
        if [ "$focus" -ge 0 ] && [ "$focus" -lt "$total_items" ] && [ -n "${items[$focus]:-}" ]; then
          local fi=${items[$focus]}
          case "$fi" in
            MODE_FULL|MODE_RESUME)
              local mode_name
              mode_name="$(printf '%s' "${fi#MODE_}" | tr '[:upper:]' '[:lower:]')"
              build_config_set MODE "$mode_name"
              ;;
            *)
              build_config_toggle "$fi"
              ;;
          esac
          build_sections
          total_items=${#items[@]}
        fi
        ;;
      ENTER)
        build_config_apply
        return 0
        ;;
      F1)
        printf '\n  Config name: '
        read -r cname
        [ -n "$cname" ] && build_config_save "$cname"
        ;;
      F2)
        local configs; configs=$(build_config_list)
        if [ -n "$configs" ]; then
          echo; echo "  Available configs:"
          echo "$configs" | nl -w2 -s') '
          printf '  Load config #: '
          read -r cnum
          local cname; cname=$(echo "$configs" | sed -n "${cnum}p")
          [ -n "$cname" ] && build_config_load_by_name "$cname" && build_sections
        fi
        ;;
      ESC)
        return 1
        ;;
    esac
  done
}

# ─── Live Dashboard ─────────────────────────────────────────

tui_render_live_dashboard() {
  local start_time=$1 total=${2:-29} done_count=${3:-0} \
        current_step="${4:-}" scroll_offset=${5:-0} paused=${6:-0}
  tui_clear
  tui_hide_cursor

  local w=$TUI_COLS l=$TUI_LINES
  local elapsed elapsed_str elapsed_m elapsed_s
  elapsed=$(($(date +%s) - start_time))
  elapsed_m=$((elapsed / 60))
  elapsed_s=$((elapsed % 60))
  printf -v elapsed_str '%02d:%02d' "$elapsed_m" "$elapsed_s"

  # ── Header ──────────────────────────────────────────────
  tui_box_top $w
  local header="BUILD EN COURS"
  local info="$(tui_c bold "$header")  $(tui_h "${elapsed_str}  |  ${done_count}/${total} steps")"
  [ "$paused" = "1" ] && info="$info  $(tui_c yellow "PAUSED")"
  tui_center $w "$info"
  echo
  tui_box_sep $w

  # ── Arch panels (side by side) ──────────────────────────
  local pct=$((done_count * 100 / (total > 0 ? total : 1)))
  local arch_cols=$(( (w - 6) / 2 ))
  local pbar_cols=$((arch_cols - 12))
  [ "$pbar_cols" -lt 5 ] && pbar_cols=5

  local arches="${SAMARIS_ARCHES:-x86_64 aarch64}"
  local arch1="${arches%% *}" arch2="${arches##* }"
  [ "$arch1" = "$arch2" ] && arch2=""

  printf '%s ' "$TUI_BOX_V"
  printf '╔═ %-*s ═════════╗' $((arch_cols - 7)) "$arch1"
  if [ -n "$arch2" ]; then
    printf '  ╔═ %-*s ═════════╗' $((arch_cols - 7)) "$arch2"
  fi
  printf ' %s\n' "$TUI_BOX_V"

  printf '%s ' "$TUI_BOX_V"
  printf '║ %s │ %3d%%%% %s' "$(tui_progress "$pct" "$pbar_cols")" "$pct" "$(printf '%*s' $((arch_cols - pbar_cols - 12)) '')"
  if [ -n "$arch2" ]; then
    printf ' ║  ║ %s │ %3d%%%% %s' "$(tui_progress "$pct" "$pbar_cols")" "$pct" "$(printf '%*s' $((arch_cols - pbar_cols - 12)) '')"
  fi
  printf ' ║ %s\n' "$TUI_BOX_V"

  printf '%s ' "$TUI_BOX_V"
  local eta_str="--"
  if [ "$done_count" -gt 0 ] && [ "$done_count" -le "$total" ]; then
    local eta=$((elapsed * (total - done_count) / done_count))
    local eta_m=$((eta / 60)) eta_s=$((eta % 60))
    printf -v eta_str '%02d:%02d' "$eta_m" "$eta_s"
  fi
  printf '║ Steps: %d/%-3d ETA: %s %s' "$done_count" "$total" "$eta_str" "$(printf '%*s' $((arch_cols - 22)) '')"
  if [ -n "$arch2" ]; then
    printf ' ║  ║ Steps: %d/%-3d ETA: %s %s' "$done_count" "$total" "$eta_str" "$(printf '%*s' $((arch_cols - 22)) '')"
  fi
  printf ' ║ %s\n' "$TUI_BOX_V"

  printf '%s ' "$TUI_BOX_V"
  printf '╚%s╝%s' "$(tui_hr_to "$TUI_BOX_H" $((arch_cols + 1)))" "$(printf '%*s' $((arch_cols - 6)) '')"
  if [ -n "$arch2" ]; then
    printf ' ╚%s╝%s' "$(tui_hr_to "$TUI_BOX_H" $((arch_cols + 1)))" "$(printf '%*s' $((arch_cols - 6)) '')"
  fi
  printf ' %s\n' "$TUI_BOX_V"

  # ── Pipeline section ────────────────────────────────────
  local pipe_rows=$((l - 20))
  [ "$pipe_rows" -lt 3 ] && pipe_rows=3
  local max_visible=$((pipe_rows - 2))
  [ "$max_visible" -lt 1 ] && max_visible=1

  tui_box_sep $w
  local show_scroll=""
  [ "$scroll_offset" -gt 0 ] && show_scroll="↑"
  [ $((scroll_offset + max_visible)) -lt ${#BUILD_STEPS[@]} ] && show_scroll="${show_scroll}↓"
  printf '%s %s%s%s\n' "$TUI_BOX_V" "$(tui_c bold "PIPELINE")" "$(printf '%*s' $((w - 16)) '')" "${show_scroll:+$(tui_h " ($show_scroll)")}"

  local i=0 visible=0
  for step in "${BUILD_STEPS[@]}"; do
    [ "$visible" -ge "$max_visible" ] && break
    if [ "$i" -lt "$scroll_offset" ]; then
      i=$((i + 1)); continue
    fi
    local s
    s="$(tui_step_status "$step")"

    local status_tag step_dur="--" cache_tag=""
    case "$s" in
      done)    status_tag="$(tui_c done "[OK]")"
                step_dur=$(tui_state_get "$step" done duration 2>/dev/null || echo "--")
                cache_tag=$(tui_state_get "$step" done cache 2>/dev/null || echo "")
               [ "$step_dur" != "--" ] && step_dur="$((step_dur / 60))m$((step_dur % 60))s" ;;
      failed)  status_tag="$(tui_c fail "[!!]")" ;;
      running) status_tag="$(tui_c running "[>>]")"
               step_dur=$(ps -o etimes= -p "$BUILD_BG_PID" 2>/dev/null | tr -d ' ' || echo "")
               [ -n "$step_dur" ] && step_dur="${step_dur}s" || step_dur="--" ;;
      skipped) status_tag="$(tui_c skip "[--]")" ;;
      *)       status_tag="$(tui_c pending "[..]")" ;;
    esac

    local line=""
    if [ "$s" = done ]; then
      local pstep=100 pbar_col=10
      line="$(tui_progress 100 $pbar_col)  ${step_dur}"
    elif [ "$s" = running ]; then
      line="$(tui_c yellow "$(tui_progress 50 10)")  ${step_dur}"
    elif [ "$s" = skipped ]; then
      line="$(tui_h "$(tui_progress 100 10)")  skip"
    else
      line="$(tui_h "$(tui_progress 0 10)")  --"
    fi
    local step_name=$(printf '%-22s' "$step")
    printf '%s  %s %s %s %s\n' "$TUI_BOX_V" "$status_tag" "$step_name" "$line"
    visible=$((visible + 1))
    i=$((i + 1))
  done
  for ((; visible < max_visible; visible++)); do
    printf '%s%*s\n' "$TUI_BOX_V" $((w - 1)) "$TUI_BOX_V"
  done

  # ── Live console section ─────────────────────────────────
  tui_box_sep $w
  local console_rows=3
  local log_content=""
  local current_name="${current_step:-}"
  if [ -z "$current_name" ]; then
    for s in "${BUILD_STEPS[@]}"; do
      state_status_file "$s" running >/dev/null 2>&1 && { current_name="$s"; break; }
    done
  fi
  printf '%s %s\n' "$TUI_BOX_V" "$(tui_c bold "CONSOLE${current_name:+: ${current_name}}")"
  if [ -n "$current_name" ]; then
    local lf; lf=$(state_log_path "$current_name")
    log_content=$(tail -$console_rows "$lf" 2>/dev/null | sed 's/^/[logs] /' || true)
  fi
  if [ -n "$log_content" ]; then
    while IFS= read -r lline; do
      local ll=${#lline}
      local rem=$((w - ll - 4))
      [ "$rem" -lt 1 ] && rem=1
      printf '%s  %s%*s\n' "$TUI_BOX_V" "$(tui_h "$lline")" "$rem" "$TUI_BOX_V"
    done <<< "$log_content"
  else
    printf '%s  %s%*s\n' "$TUI_BOX_V" "$(tui_h "Waiting for step output...")" $((w - 32)) "$TUI_BOX_V"
  fi

  # ── Statistics ──────────────────────────────────────────
  tui_box_sep $w
  local ch=0 cm=0
  local cache_stats; cache_stats=$(build_bg_session_cache_stats 2>/dev/null || echo "0 0")
  ch=${cache_stats%% *}; cm=${cache_stats##* }
  local skipped=0
  for step in "${BUILD_STEPS[@]}"; do
    if ! build_config_step_needed "$step" 2>/dev/null; then
      skipped=$((skipped + 1))
    fi
  done
  local failures=0
  for step in "${BUILD_STEPS[@]}"; do
    state_status_file "$step" failed >/dev/null 2>&1 && failures=$((failures + 1))
  done
  printf '%s  Cache hits: %s  |  Cache misses: %s  |  Skipped: %s  |  Failures: %s%*s\n' \
    "$TUI_BOX_V" \
    "$(tui_c green "$ch")" \
    "$(tui_c cyan "$cm")" \
    "$(tui_c dim "$skipped")" \
    "$(tui_c red "$failures")" \
    $((w - 70)) "$TUI_BOX_V"

  # ── Key bindings footer ─────────────────────────────────
  tui_box_sep $w
  printf '%s  %s%*s\n' "$TUI_BOX_V" \
    "$(tui_h "[Ctrl+C] Stop    [P] Pause    [L] Logs    [Q] Back to menu")" \
    $((w - 64)) "$TUI_BOX_V"
  tui_box_bot $w
}

tui_start_build_bg() {
  local build_log="$STATE_LOG_DIR/build.log"
  local mode="${BUILD_MODE:-full}"
  mkdir -p "$STATE_LOG_DIR" 2>/dev/null || true
  rm -f "$STATE_DIR/.build_session_cache" 2>/dev/null || true

  if [ "${USE_DOCKER:-0}" = "1" ] && [ "${SAMARIS_IN_DOCKER:-0}" != "1" ]; then
    (
      local args=(iso)
      build_config_save "tui-current" >/dev/null 2>&1 || true
      args+=(--config builder/configs/tui-current.conf)
      [ "${RUN_FORCE:-0}" = "1" ] && args+=(--force)
      [ "${BUILD_DRY_RUN:-0}" = "1" ] && args+=(--dry-run)
      if [ "$mode" = "resume" ]; then
        local first
        first="$(tui_first_incomplete_step || true)"
        [ -n "$first" ] && args+=(--from "$first")
      fi
      run_in_docker "${args[@]}"
    ) > "$build_log" 2>&1 &
  else
    (
      if [ "$mode" = "resume" ]; then
        run_remaining_steps
      else
        run_steps "${BUILD_STEPS[@]}"
      fi
    ) > "$build_log" 2>&1 &
  fi
  BUILD_BG_PID=$!
  printf '%d' "$BUILD_BG_PID"
}

tui_run_build_with_dashboard() {
  local start_time
  start_time=$(date +%s)
  local paused=0 scroll_offset=0 current_step="" last_step=""
  local build_pid

  tui_clear
  tui_hide_cursor

  # Initialize state
  state_init
  local total=${#BUILD_STEPS[@]}

  # Fork the build
  build_config_apply
  build_pid=$(tui_start_build_bg)
  echo "Build started (PID: $build_pid)" >/dev/null

  local aborted=0
  while true; do
    # Check if build process is still running
    if ! build_bg_is_running "$build_pid"; then
      # Process may have exited
      local finished_pending=0
      for step in "${BUILD_STEPS[@]}"; do
        state_status_file "$step" running >/dev/null 2>&1 && finished_pending=1
        state_status_file "$step" done >/dev/null 2>&1 || \
          state_status_file "$step" skipped >/dev/null 2>&1 || \
          state_status_file "$step" failed >/dev/null 2>&1 || finished_pending=$((finished_pending+1))
      done
      sleep 1
      break
    fi

    local done_count=0
    for step in "${BUILD_STEPS[@]}"; do
      state_complete "$step" && done_count=$((done_count + 1))
      if state_status_file "$step" running >/dev/null 2>&1; then
        current_step="$step"
      fi
    done

    tui_render_live_dashboard "$start_time" "$total" "$done_count" "$current_step" "$scroll_offset" "$paused"

    local key
    key=$(tui_read_key_timeout 0.5)
    case "$key" in
      q|Q)
        build_bg_abort "$build_pid"
        aborted=1
        break
        ;;
      p|P)
        if [ "$paused" = "1" ]; then
          kill -CONT -"$build_pid" 2>/dev/null || kill -CONT "$build_pid" 2>/dev/null || true
          paused=0
        else
          kill -STOP -"$build_pid" 2>/dev/null || kill -STOP "$build_pid" 2>/dev/null || true
          paused=1
        fi
        ;;
      l|L)
        if [ -n "$current_step" ]; then
          local lf; lf=$(state_log_path "$current_step")
          if [ -f "$lf" ]; then
            tui_clear
            tui_show_cursor
            tui_restore=$(stty -g 2>/dev/null || true)
            less "$lf" 2>/dev/null || cat "$lf"
            stty "$tui_restore" 2>/dev/null || true
          fi
        fi
        ;;
      UP)
        [ "$scroll_offset" -gt 0 ] && scroll_offset=$((scroll_offset - 1))
        ;;
      DOWN)
        local max_visible=$((TUI_LINES - 22))
        [ "$max_visible" -lt 1 ] && max_visible=1
        [ $((scroll_offset + max_visible)) -lt $total ] && scroll_offset=$((scroll_offset + 1))
        ;;
    esac
  done

  # Final state render
  local final_done=0
  for step in "${BUILD_STEPS[@]}"; do
    state_complete "$step" && final_done=$((final_done + 1))
  done
  tui_render_live_dashboard "$start_time" "$total" "$final_done" "" "$scroll_offset" "0"
  echo
  if [ "$aborted" = "1" ]; then
    tui_center $w "$(tui_c yellow "Build aborted by user")"
  else
    tui_center $w "$(tui_c green "Build finished — $final_done/$total steps completed")"
  fi
  echo
  tui_center $w "$(tui_h "Press ENTER to continue...")"
  tui_read_key >/dev/null
}

# ─── Build Summary ──────────────────────────────────────────

tui_build_summary() {
  tui_clear
  tui_hide_cursor
  local w=$TUI_COLS l=$TUI_LINES
  local total=${#BUILD_STEPS[@]}
  local done_count=0 failed_count=0
  local step status_s

  for step in "${BUILD_STEPS[@]}"; do
    state_complete "$step" && done_count=$((done_count + 1))
    state_status_file "$step" failed >/dev/null 2>&1 && failed_count=$((failed_count + 1))
  done

  # Compute total duration and collect step data
  local total_duration=0 step_dur
  declare -A step_durations step_cache step_valid
  for step in "${BUILD_STEPS[@]}"; do
    local done_file
    done_file="$(tui_state_file "$step" done)"
    if [ -n "$done_file" ]; then
      step_dur=$(state_get_field duration "$done_file" 2>/dev/null || echo 0)
      step_durations["$step"]=$step_dur
      total_duration=$((total_duration + step_dur))
      local cache_s; cache_s=$(state_get_field cache "$done_file" 2>/dev/null || echo "")
      step_cache["$step"]="$cache_s"
      if checkpoint_valid "$step" >/dev/null 2>&1; then
        step_valid["$step"]="1"
      else
        step_valid["$step"]="0"
      fi
    fi
  done

  local total_min=$((total_duration / 60)) total_sec=$((total_duration % 60))
  local total_hr=$((total_min / 60))
  total_min=$((total_min % 60))
  local duration_str
  printf -v duration_str '%02d:%02d:%02d' "$total_hr" "$total_min" "$total_sec"

  # ── Header with summary panels ──────────────────────────
  tui_box_top $w
  tui_center $w "$(tui_c green_bold "BUILD TERMINE")"
  echo
  tui_box_sep $w

  local panel_w=$(( (w - 6) / 2 ))
  printf '%s ' "$TUI_BOX_V"
  printf '╔═ %-*s ═══╗' $((panel_w - 9)) "RESULTATS"
  printf '  ╔═ %-*s ═══╗' $((panel_w - 10)) "AVERTISSEMENTS"
  printf ' %s\n' "$TUI_BOX_V"

  local success_all=0
  [ "$done_count" -eq "$total" ] && success_all=1
  local pct=$((done_count * 100 / (total > 0 ? total : 1)))
  printf '%s ' "$TUI_BOX_V"
  printf '║ %s ' "$(tui_c green "${done_count}/${total}")"
  [ "$success_all" = "1" ] && printf 'SUCCES' || printf 'partiel'
  printf ' %s' "$(printf '%*s' $((panel_w - 20)) '')"
  printf ' ║  ║ '
  local warn_count=0
  for step in "${BUILD_STEPS[@]}"; do
    local lf; lf=$(state_log_path "$step")
    [ -f "$lf" ] && warn_count=$((warn_count + $(grep -ci 'warning\|warn' "$lf" 2>/dev/null || echo 0)))
  done
  [ "$warn_count" -gt 0 ] && printf '%s ' "$(tui_c yellow "$warn_count")" || printf '0 '
  printf 'warnings%*s' $((panel_w - 16)) ''
  printf ' ║ %s\n' "$TUI_BOX_V"

  printf '%s ' "$TUI_BOX_V"
  printf '║ Duree: %s %s' "$(tui_c bold "$duration_str")" "$(printf '%*s' $((panel_w - 14)) '')"
  printf ' ║  ║ '
  printf '%s failed%*s' "$failed_count" $((panel_w - 14)) ''
  printf ' ║ %s\n' "$TUI_BOX_V"

  printf '%s ' "$TUI_BOX_V"
  local output_size="?"
  local out_iso; out_iso=$(ls -lh "$OUTPUT_DIR"/*.iso 2>/dev/null | head -1 | awk '{print $5}')
  [ -n "$out_iso" ] && output_size="$out_iso"
  printf '║ ISO: %s %s' "$(tui_c bold "$output_size")" "$(printf '%*s' $((panel_w - 11)) '')"
  printf ' ║  ║ '
  local ch=0 cm=0
  for step in "${BUILD_STEPS[@]}"; do
    [ "${step_cache[$step]:-}" = "hit" ] && ch=$((ch + 1))
    [ "${step_cache[$step]:-}" = "miss" ] && cm=$((cm + 1))
  done
  printf '%s cache hits%*s' "$ch" $((panel_w - 18)) ''
  printf ' ║ %s\n' "$TUI_BOX_V"

  printf '%s ' "$TUI_BOX_V"
  printf '╚%s╝%s' "$(tui_hr_to "$TUI_BOX_H" $((panel_w + 1)))" "$(printf '%*s' $((panel_w + 2 - 6)) '')"
  printf ' ╚%s╝' "$(tui_hr_to "$TUI_BOX_H" $((panel_w + 1)))"
  printf ' %s\n' "$TUI_BOX_V"

  # ── Per-step results table ────────────────────────────────
  local table_rows=$((l - 20))
  [ "$table_rows" -lt 3 ] && table_rows=3
  tui_box_sep $w
  printf '%s %s\n' "$TUI_BOX_V" "$(tui_c bold "RESULTATS PAR ETAPE")"

  local step_idx=0 visible=0
  local disp_rows=$((table_rows - 2))
  [ "$disp_rows" -lt 1 ] && disp_rows=1
  for step in "${BUILD_STEPS[@]}"; do
    [ "$visible" -ge "$disp_rows" ] && break
    local s status_str
    s="$(tui_step_status "$step")"
    case "$s" in
      done)
        local dur_s=${step_durations["$step"]:-0}
        local d_min=$((dur_s / 60)) d_sec=$((dur_s % 60))
        local d_str; printf -v d_str '%02d:%02d' "$d_min" "$d_sec"
        local cache_str="${step_cache[$step]:-}"
        [ "$cache_str" = "hit" ] && cache_str="$(tui_c green "HIT")" || cache_str="$(tui_c yellow "MISS")"
        local valid_str="${step_valid[$step]:-0}"
        [ "$valid_str" = "1" ] && valid_str="$(tui_c done "[V]")" || valid_str="$(tui_c yellow "[?]")"
        status_str="$(tui_c done "[OK]") $valid_str $d_str $cache_str"
        ;;
      failed)
        status_str="$(tui_c fail "[!!]") -- -- --"
        ;;
      skipped)
        status_str="$(tui_c skip "[--]") -- -- skip"
        ;;
      *)
        status_str="$(tui_c pending "[..]") -- -- --"
        ;;
    esac
    printf '%s  %s  %-22s %s\n' "$TUI_BOX_V" "$status_str" "$step" " "
    visible=$((visible + 1))
    step_idx=$((step_idx + 1))
  done
  for ((; visible < disp_rows; visible++)); do
    printf '%s%*s\n' "$TUI_BOX_V" $((w - 1)) "$TUI_BOX_V"
  done

  # ── Generated files section ──────────────────────────────
  tui_box_sep $w
  printf '%s %s\n' "$TUI_BOX_V" "$(tui_c bold "FICHIERS GENERES")"
  local has_files=0
  for f in "$OUTPUT_DIR"/*.iso "$OUTPUT_DIR"/*.sha256; do
    if [ -f "$f" ]; then
      local fsize; fsize=$(du -sh "$f" 2>/dev/null | cut -f1)
      printf '%s   %s  %s\n' "$TUI_BOX_V" "$(tui_c bold "$(basename "$f")")" "$(tui_h "$fsize")"
      has_files=1
    fi
  done
  [ "$has_files" = "0" ] && printf '%s   %s\n' "$TUI_BOX_V" "$(tui_h "No output files generated")"

  # ── Warnings section ─────────────────────────────────────
  tui_box_sep $w
  printf '%s %s\n' "$TUI_BOX_V" "$(tui_c bold "AVERTISSEMENTS")"
  local warn_shown=0
  for step in "${BUILD_STEPS[@]}"; do
    local lf; lf=$(state_log_path "$step")
    if [ -f "$lf" ]; then
      local warns; warns=$(grep -in 'warning\|warn' "$lf" 2>/dev/null | head -3 || true)
      if [ -n "$warns" ]; then
        while IFS= read -r wline; do
          local wtext; wtext=$(echo "$wline" | sed 's/^[^:]*:[^:]*://' | xargs)
          printf '%s   %s %s\n' "$TUI_BOX_V" "$(tui_c yellow "[!]")" "$(tui_h "${wtext:0:$((w-12))}")"
          warn_shown=$((warn_shown + 1))
          [ "$warn_shown" -ge 5 ] && break
        done <<< "$warns"
      fi
    fi
    [ "$warn_shown" -ge 5 ] && break
  done
  [ "$warn_shown" -eq 0 ] && printf '%s   %s\n' "$TUI_BOX_V" "$(tui_h "No warnings")"
  tui_box_bot $w

  # ── Actions ──────────────────────────────────────────────
  echo
  printf '  %s\n' "$(tui_h "[1] QEMU Boot    [2] Report   [3] Save Config   [4] Menu   [5] Clean   [q] Quit")"
  echo
  local key
  key=$(tui_read_key)
  case "$key" in
    1) echo "qemu" ;;
    3) echo "config_save" ;;
    4|ESC) echo "menu" ;;
    5) echo "clean" ;;
    q|Q) echo "quit" ;;
    *) echo "menu" ;;
  esac
}

# ─── Status Dashboard ───────────────────────────────────────

tui_status_dashboard() {
  tui_clear
  tui_hide_cursor
  local w=$TUI_COLS l=$TUI_LINES
  state_init

  tui_box_top $w
  tui_center $w "$(tui_c bold "TABLEAU DE BORD")"; echo
  tui_center $w "$(tui_h "Etat des checkpoints, validation, et ressources")"; echo
  tui_box_sep $w

  # Step table
  tui_box_section "Build Steps" $w
  printf '%s  %-22s  %-10s  %-4s  %-6s  %-5s  %s\n' "$TUI_BOX_V" "$(tui_c bold "Etape")" "$(tui_c bold "Statut")" "$(tui_c bold "Val.")" "$(tui_c bold "Cache")" "$(tui_c bold "Arch")" "$(tui_c bold "Date")"
  printf '%s  %s\n' "$TUI_BOX_V" "$(tui_hr '-' $((w - 8)))"

  local step status dot valid_str cache_str ts archs
  for step in "${BUILD_STEPS[@]}"; do
    dot="  " status="$(tui_c pending "[..]")" valid_str="--" cache_str="--" ts="--" archs="--"
    local done_file
    done_file="$(tui_state_file "$step" done)"
    if state_status_file "$step" running >/dev/null 2>&1; then
      status="$(tui_c running "[>>]")"
      dot=" >"
    elif state_status_file "$step" failed >/dev/null 2>&1; then
      status="$(tui_c fail "[!!]")"
      dot=" !"
    elif state_status_file "$step" skipped >/dev/null 2>&1; then
      status="$(tui_c skip "[--]")"
      dot=" -"
      ts="$(tui_state_get "$step" skipped skipped_at 2>/dev/null | sed 's/T/ /; s/Z//' || echo "")"
      ts="${ts:5:11}"
      [ -z "$ts" ] && ts="--"
    elif [ -n "$done_file" ]; then
      status="$(tui_c done "[OK]")"
      dot=" v"
      if checkpoint_valid "$step" >/dev/null 2>&1; then
        valid_str="$(tui_c done "[V]")"
      else
        valid_str="$(tui_c yellow "[?]")"
      fi
      local cache_val; cache_val=$(state_get_field cache "$done_file" 2>/dev/null || echo "")
      [ "$cache_val" = "hit" ] && cache_str="$(tui_c green "disk")" || cache_str="$(tui_c yellow "miss")"
      local raw_ts; raw_ts=$(state_get_field completed_at "$done_file" 2>/dev/null | sed 's/T/ /; s/Z//' || echo "")
      ts="${raw_ts:5:11}"  # MM-DD HH:MM
      [ -z "$ts" ] && ts="--"
    fi
    printf '%s  %s %-22s  %-10s  %-4s  %-6s  %-5s  %s\n' "$TUI_BOX_V" "$dot" "$step" "$status" "$valid_str" "$cache_str" "$archs" "$(tui_h "$ts")"
  done
  tui_box_section_bot $w

  # Storage section
  tui_box_section "Stockage" $w
  local work_size="?" cache_size="?" output_size="?"
  [ -d "$WORK_DIR" ] && work_size=$(du -sh "$WORK_DIR" 2>/dev/null | cut -f1)
  [ -d "$CACHE_DIR" ] && cache_size=$(du -sh "$CACHE_DIR" 2>/dev/null | cut -f1)
  [ -d "$OUTPUT_DIR" ] && output_size=$(du -sh "$OUTPUT_DIR" 2>/dev/null | cut -f1)
  local docker_size=$(docker images --format '{{.Size}}' --filter=reference='samaris-os-builder:*' 2>/dev/null | head -1 || echo "N/A")

  # Display storage with simple bars
  local bar_w=$((w - 50))
  [ "$bar_w" -lt 10 ] && bar_w=10
  # Workdir bar (assume 50GB max)
  local work_pct=0
  local work_mb; work_mb=$(du -sm "$WORK_DIR" 2>/dev/null | cut -f1 || echo 0)
  [ "$work_mb" -gt 0 ] && work_pct=$((work_mb * 100 / 51200))
  [ "$work_pct" -gt 100 ] && work_pct=100
  printf '%s  Work volume:  %-10s  %s\n' "$TUI_BOX_V" "$work_size" "$(tui_progress_bar "$work_pct" "$bar_w")"
  printf '%s  Cache:        %s\n' "$TUI_BOX_V" "$cache_size"
  printf '%s  Output:       %s\n' "$TUI_BOX_V" "$output_size"
  printf '%s  Docker image: %s\n' "$TUI_BOX_V" "$docker_size"
  echo
  printf '%s  Work dir:  %s\n' "$TUI_BOX_V" "$(tui_h "$WORK_DIR")"
  printf '%s  State dir: %s\n' "$TUI_BOX_V" "$(tui_h "$STATE_DIR")"
  printf '%s  Overlay:   %s\n' "$TUI_BOX_V" "$(tui_h "$OVERLAY_DIR")"
  tui_box_section_bot $w

  tui_box_bot $w
  echo
  printf '  %s\n' "$(tui_h "[R] Rafraichir  [V] Valider tout  [L] Voir les logs  [Esc] Retour au menu")"
  echo

  local key
  key=$(tui_read_key)
  case "$key" in
    r|R) echo "status" ;;
    v|V)
      echo; log "Validating all checkpoints..."
      for step in "${BUILD_STEPS[@]}"; do
        if state_done "$step"; then
          if checkpoint_valid "$step" >/dev/null 2>&1; then
            printf '  %s %s\n' "$(tui_c done "[OK]")" "$step"
          else
            printf '  %s %s\n' "$(tui_c fail "[!!]")" "$(tui_c yellow "$step")"
          fi
        fi
      done
      tui_wait_enter
      echo "status"
      ;;
    l|L) echo "log_viewer" ;;
    q|Q) echo "quit" ;;
    ESC|*) echo "menu" ;;
  esac
}

# ─── Run Single Step ────────────────────────────────────────

tui_run_step() {
  tui_clear
  tui_hide_cursor
  local w=$TUI_COLS
  local numbered_steps=() step_descriptions=()
  local i=1

  tui_run_step_description() {
    local s="$1"
    local sf="$STEPS_DIR/$s.sh"
    [ -f "$sf" ] && grep -m1 '^# Description:' "$sf" 2>/dev/null | sed 's/^# Description: *//' || echo ""
  }

  for step in "${BUILD_STEPS[@]}" "17-qemu"; do
    numbered_steps+=("$i:$step")
    local desc; desc=$(tui_run_step_description "$step")
    step_descriptions+=("$desc")
    i=$((i+1))
  done

  local force_flag=0 docker_flag=1 dryrun_flag=0
  local focus=0 max_visible=$((TUI_LINES - 12))
  local scroll_offset=0 total_items=${#numbered_steps[@]}

  render_run_step() {
    tui_clear
    tui_hide_cursor
    tui_box_top $w
    tui_center $w "$(tui_c bold "EXECUTER UNE ETAPE")"; echo
    tui_box_sep $w

    tui_box_section "Etapes disponibles" $w
    local visible=0 i=$scroll_offset
    while [ "$visible" -lt "$max_visible" ] && [ "$i" -lt "$total_items" ]; do
      local entry="${numbered_steps[$i]}"
      local num="${entry%%:*}"
      local step="${entry#*:}"
      local prefix="  "
      [ "$i" = "$focus" ] && prefix=" >"
      local status="$(tui_c pending "[..]")"
      if state_skipped "$step"; then
        status="$(tui_c skip "[--]")"
      elif state_done "$step"; then
        if checkpoint_valid "$step" >/dev/null 2>&1; then
          status="$(tui_c done "[OK]")"
        else
          status="$(tui_c yellow "stale")"
        fi
      fi
      local desc="${step_descriptions[$i]}"
      printf '%s%2d  %-22s %s %s\n' "$prefix" "$num" "$step" "$status" "$(tui_h "$desc")"
      visible=$((visible + 1))
      i=$((i + 1))
    done
    tui_box_section_bot $w

    # Execution options
    tui_box_section "Options d'execution" $w
    printf '%s %s Forcer la reexecution (ignorer le checkpoint)\n' "$TUI_BOX_V" "$([ "$force_flag" = 1 ] && tui_c check_on || echo "$TUI_CHK_OFF")"
    printf '%s %s Utiliser Docker\n' "$TUI_BOX_V" "$([ "$docker_flag" = 1 ] && tui_c check_on || echo "$TUI_CHK_OFF")"
    printf '%s %s Simuler (dry-run)\n' "$TUI_BOX_V" "$([ "$dryrun_flag" = 1 ] && tui_c check_on || echo "$TUI_CHK_OFF")"
    tui_box_section_bot $w

    tui_box_bot $w
    echo
    printf '  %s\n' "$(tui_h "[↑↓] Naviguer  [F] Force  [D] Docker  [Y] Dry-run  [Enter] Selectionner  [Esc] Retour")"
    if [ "$focus" -ge 0 ] && [ "$focus" -lt "$total_items" ]; then
      local focused_step="${numbered_steps[$focus]#*:}"
      printf '  %s %s\n' "$(tui_c bold "Selection:")" "$focused_step"
    fi
  }

  local selecting_step=1
  while [ "$selecting_step" = "1" ]; do
    render_run_step
    local key
    key=$(tui_read_key)
    case "$key" in
      UP) [ "$focus" -gt 0 ] && focus=$((focus - 1))
          [ "$focus" -lt "$scroll_offset" ] && scroll_offset=$focus ;;
      DOWN) [ "$focus" -lt $((total_items - 1)) ] && focus=$((focus + 1))
            [ "$focus" -ge $((scroll_offset + max_visible)) ] && scroll_offset=$((focus - max_visible + 1)) ;;
      ENTER)
        if [ "$focus" -ge 0 ] && [ "$focus" -lt "$total_items" ]; then
          local selected_step="${numbered_steps[$focus]#*:}"
          echo "$selected_step|$force_flag|$docker_flag|$dryrun_flag"
          return 0
        fi
        ;;
      f|F) force_flag=$((1 - force_flag)) ;;
      d|D) docker_flag=$((1 - docker_flag)) ;;
      y|Y) dryrun_flag=$((1 - dryrun_flag)) ;;
      ESC) return 1 ;;
    esac
  done
}

# ─── Config Manager ─────────────────────────────────────────

tui_config_manager() {
  local w=$TUI_COLS action=""
  mkdir -p "$CONFIG_DIR" 2>/dev/null || true

  while [ "$action" != "menu" ]; do
    tui_clear
    tui_hide_cursor
    local configs
    configs=$(build_config_list)

    tui_box_top $w
    tui_center $w "$(tui_c bold "GESTIONNAIRE DE CONFIGURATIONS")"; echo
    tui_center $w "$(tui_h "Sauvegarder ou restaurer une configuration ISO")"
    echo
    tui_box_sep $w

    if [ -n "$configs" ]; then
      tui_box_section "Configs disponibles" $w
      printf '%s  %-3s %-20s %-16s %-8s %-8s %-9s\n' "$TUI_BOX_V" "No" "Nom" "Architecture" "AI" "Volts" "Taille" "Date"
      printf '%s  %s\n' "$TUI_BOX_V" "$(tui_hr '-' $((w - 8)))"
      local i=1
      while IFS= read -r c; do
        local info; info=$(build_config_describe "$c" 2>/dev/null || echo "$c|?|?|?|?|?")
        IFS='|' read -r name arch ai volts size mdate <<< "$info"
        printf '%s  %-3d %-20s %-16s %-8s %-8s %-9s\n' "$TUI_BOX_V" "$i" "$(tui_c bold "$name")" "$(tui_h "$arch")" "$(tui_h "$ai")" "$(tui_h "$volts")" "$(tui_c yellow "$size")" "$(tui_h "$mdate")"
        i=$((i+1))
      done <<< "$configs"
      tui_box_section_bot $w
    else
      printf '%s  %s\n' "$TUI_BOX_V" "$(tui_h "No saved configurations.")"
    fi

    echo
    tui_box_section "Actions" $w
    printf '%s  1)  Charger une configuration\n' "$TUI_BOX_V"
    printf '%s  2)  Sauvegarder la configuration actuelle\n' "$TUI_BOX_V"
    printf '%s  3)  Supprimer une configuration\n' "$TUI_BOX_V"
    printf '%s  4)  Renommer une configuration\n' "$TUI_BOX_V"
    printf '%s  5)  Exporter en JSON\n' "$TUI_BOX_V"
    printf '%s  6)  Retour au menu\n' "$TUI_BOX_V"
    tui_box_section_bot $w

    echo
    printf '%s\n' "$(tui_h "Repertoire: $CONFIG_DIR")"
    printf '  Entrez le numero de l'\''action [1-6]: '
    local key
    key=$(tui_read_key)
    echo

    case "$key" in
      1)
        if [ -n "$configs" ]; then
          printf '  Config name: '
          read -r cname
          build_config_load_by_name "$cname" && printf '  %s\n' "$(tui_c done "Loaded")"
          tui_wait_enter
        fi
        ;;
      2)
        printf '  Config name: '
        read -r cname
        [ -n "$cname" ] && build_config_save "$cname" && printf '  %s\n' "$(tui_c done "Saved")"
        tui_wait_enter
        ;;
      3)
        if [ -n "$configs" ]; then
          printf '  Config name: '
          read -r cname
          build_config_delete "$cname" && printf '  %s\n' "$(tui_c done "Deleted")"
          tui_wait_enter
        fi
        ;;
      4)
        if [ -n "$configs" ]; then
          printf '  Current name: '
          read -r oldname
          printf '  New name: '
          read -r newname
          if [ -n "$oldname" ] && [ -n "$newname" ]; then
            build_config_rename "$oldname" "$newname" && printf '  %s\n' "$(tui_c done "Renamed")" || printf '  %s\n' "$(tui_c fail "Failed")"
          fi
          tui_wait_enter
        fi
        ;;
      5)
        if [ -n "$configs" ]; then
          printf '  Config name: '
          read -r cname
          local inf; inf="$(build_config_name_to_path "$cname" "$CONFIG_DIR")"
          if [ -f "$inf" ]; then
            local out="${inf}.json"
            build_config_export_json "$cname" > "$out" 2>/dev/null && printf '  %s Exported: %s\n' "$(tui_c done "[OK]")" "$out"
          fi
          tui_wait_enter
        fi
        ;;
      6|ESC|q) action="menu" ;;
    esac
  done
  echo "menu"
}

build_config_export_json() {
  local name="$1" dir="${2:-$CONFIG_DIR}"
  local f
  f="$(build_config_name_to_path "$name" "$dir")"
  [ -f "$f" ] || return 1
  local ts; ts=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
  printf '{\n'
  printf '  "exported_at": "%s",\n' "$ts"
  printf '  "config_name": "%s",\n' "$name"
  printf '  "project": "Samaris OS 1.0 Mountain Lake Alpha One",\n'
  printf '  "settings": {\n'
  while IFS='=' read -r k v; do
    [ -z "$k" ] && continue
    local jk; jk=$(echo "$k" | tr '[:upper:]' '[:lower:]' | sed 's/build_config_//')
    printf '    "%s": "%s",\n' "$jk" "$v"
  done < "$f"
  printf '    "export_tool": "samaris-iso-generator"\n'
  printf '  }\n'
  printf '}\n'
}

# ─── Clean Prompt ───────────────────────────────────────────

tui_clean_prompt() {
  local clean_workdir=1 clean_checkpoints=1 clean_cache=0 clean_output=0 clean_docker=0
  local mode="quick" focus=0
  local items=("workdir" "checkpoints" "cache" "output" "docker")
  local item_labels=("Workdir ($WORK_DIR)" "Checkpoints ($STATE_DIR)" "Cache ($CACHE_DIR)" "Output ISO ($OUTPUT_DIR)" "Docker image")
  local w=$TUI_COLS

  compute_size() {
    local total=0
    [ "$clean_workdir" = 1 ] && [ -d "$WORK_DIR" ] && total=$((total + $(du -sm "$WORK_DIR" 2>/dev/null | cut -f1 || echo 0)))
    [ "$clean_checkpoints" = 1 ] && [ -d "$STATE_DIR" ] && total=$((total + $(du -sm "$STATE_DIR" 2>/dev/null | cut -f1 || echo 0)))
    [ "$clean_cache" = 1 ] && [ -d "$CACHE_DIR" ] && total=$((total + $(du -sm "$CACHE_DIR" 2>/dev/null | cut -f1 || echo 0)))
    [ "$clean_output" = 1 ] && [ -d "$OUTPUT_DIR" ] && total=$((total + $(du -sm "$OUTPUT_DIR" 2>/dev/null | cut -f1 || echo 0)))
    [ "$clean_docker" = 1 ] && total=$((total + 1500))
    echo $total
  }

  get_size() {
    local what="$1" s=0
    case "$what" in
      workdir) [ -d "$WORK_DIR" ] && s=$(du -sm "$WORK_DIR" 2>/dev/null | cut -f1) || s=0 ;;
      checkpoints) [ -d "$STATE_DIR" ] && s=$(du -sm "$PUBLIC_STATE_DIR" 2>/dev/null | cut -f1) || s=0 ;;
      cache) [ -d "$CACHE_DIR" ] && s=$(du -sm "$CACHE_DIR" 2>/dev/null | cut -f1) || s=0 ;;
      output) [ -d "$OUTPUT_DIR" ] && s=$(du -sm "$OUTPUT_DIR" 2>/dev/null | cut -f1) || s=0 ;;
      docker) s=1500 ;;
    esac
    local friendly
    if [ "$s" -ge 1000 ]; then
      friendly="$(echo "scale=1; $s / 1024" | bc -l 2>/dev/null || echo "${s}M")"
      friendly="${friendly} GB"
    else
      friendly="${s} MB"
    fi
    echo "$friendly"
  }

  render_clean() {
    tui_clear
    tui_hide_cursor
    printf '\n%s\n' "$(tui_c red_bold "  NETTOYAGE")"
    printf '  %s\n\n' "$(tui_c yellow "ATTENTION: Cette action est irreversible !")"

    tui_box_section "Ce qui sera supprime" $w
    local i=0
    local vals=($clean_workdir $clean_checkpoints $clean_cache $clean_output $clean_docker)
    for item in "${items[@]}"; do
      local prefix="  "
      [ "$mode" = "custom" ] && [ "$i" = "$focus" ] && prefix=" >"
      local check; [ "${vals[$i]}" = "1" ] && check="$(tui_c check_on)" || check="$TUI_CHK_OFF"
      local size_str; size_str=$(get_size "$item")
      printf '%s %s %-30s %s\n' "$prefix" "$check" "${item_labels[$i]}" "$(tui_h "$size_str")"
      i=$((i + 1))
    done
    local total_mb; total_mb=$(compute_size)
    local total_str; [ "$total_mb" -ge 1000 ] && total_str="$(echo "scale=1; $total_mb / 1024" | bc -l 2>/dev/null || echo "${total_mb}M") GB" || total_str="${total_mb} MB"
    printf '  %s\n' "$(tui_hr '-' $((w-6)))"
    printf '  %s %s\n\n' "$(tui_c bold "Total selectionne:")" "$total_str"
    tui_box_section_bot $w

    echo
    printf '  %s\n' "$(tui_h "Actions rapides:")"
    printf '  1)  Nettoyage rapide       (workdir + checkpoints)        %s\n' "$(get_size workdir)"
    printf '  2)  Nettoyage complet      (tout sauf Docker image)       %s\n' "$(get_size cache)"
    printf '  3)  Nettoyage total        (tout, y compris Docker)       %s\n' "~$(get_size docker)"
    printf '  4)  Personnalise           (choisir chaque element)\n'
    printf '  n)  Annuler\n'
    echo
    if [ "$mode" = "custom" ]; then
      printf '  %s\n' "$(tui_h "Mode personnalise: [ESPACE] cocher/decocher  [↑↓] naviguer  [ENTER] executer")"
    else
      printf '  Action [1-4, n]: '
    fi
  }

  while true; do
    render_clean
    local key
    key=$(tui_read_key)

    if [ "$mode" = "custom" ]; then
      case "$key" in
        UP) [ "$focus" -gt 0 ] && focus=$((focus - 1)) ;;
        DOWN) [ "$focus" -lt 4 ] && focus=$((focus + 1)) ;;
        SPACE)
          case "$focus" in
            0) clean_workdir=$((1 - clean_workdir)) ;;
            1) clean_checkpoints=$((1 - clean_checkpoints)) ;;
            2) clean_cache=$((1 - clean_cache)) ;;
            3) clean_output=$((1 - clean_output)) ;;
            4) clean_docker=$((1 - clean_docker)) ;;
          esac
          ;;
        ENTER) break ;;
        n|N) clean_workdir=0 clean_checkpoints=0 clean_cache=0 clean_output=0 clean_docker=0; break ;;
        q|Q|ESC) clean_workdir=0 clean_checkpoints=0 clean_cache=0 clean_output=0 clean_docker=0; break ;;
      esac
    else
      case "$key" in
        1) clean_workdir=1 clean_checkpoints=1 clean_cache=0 clean_output=0 clean_docker=0; break ;;
        2) clean_workdir=1 clean_checkpoints=1 clean_cache=1 clean_output=1 clean_docker=0; break ;;
        3) clean_workdir=1 clean_checkpoints=1 clean_cache=1 clean_output=1 clean_docker=1; break ;;
        4) mode="custom"; focus=0 ;;
        n|N|ESC) clean_workdir=0 clean_checkpoints=0 clean_cache=0 clean_output=0 clean_docker=0; break ;;
      esac
    fi
  done

  local doit=0
  [ "$clean_workdir" = 1 ] || [ "$clean_checkpoints" = 1 ] || \
  [ "$clean_cache" = 1 ] || [ "$clean_output" = 1 ] || [ "$clean_docker" = 1 ] && doit=1

  if [ "$doit" = "1" ]; then
    echo
    if tui_confirm "  Confirmer le nettoyage? (y/N)"; then
      [ "$clean_checkpoints" = 1 ] && state_clear_all
      [ "$clean_workdir" = 1 ] && safe_remove_dir "$WORK_DIR"
      [ "$clean_cache" = 1 ] && safe_remove_dir "$CACHE_DIR"
      [ "$clean_output" = 1 ] && safe_clean_output_files
      [ "$clean_docker" = 1 ] && docker rmi samaris-os-builder:trixie 2>/dev/null || true
      printf '  %s Nettoye\n' "$(tui_c done "[OK]")"
    fi
  fi
  tui_wait_enter
  echo "menu"
}

# ─── QEMU Config ────────────────────────────────────────────

tui_qemu_config() {
  local arch="x86_64" ram=4096 cpus=4 kvm=1 display="gtk"
  local gpu_virtio=1 gpu_3d=0 disk_attach=0 disk_size=8 network="user" extra_args=""
  local iso="${OUTPUT_DIR}/${OUTPUT_ISO}"
  local w=$TUI_COLS
  local focus=0 total_fields=12

  build_qemu_cmdline() {
    local cmd="qemu-system-${arch}"
    [ "$arch" = "aarch64" ] && cmd="qemu-system-aarch64 -M virt"
    cmd="$cmd -m ${ram}M -smp $cpus"
    [ "$kvm" = "1" ] && [ "$arch" = "x86_64" ] && cmd="$cmd -accel kvm"
    [ "$kvm" = "1" ] && [ "$arch" = "aarch64" ] && cmd="$cmd -accel hvf"
    case "$display" in
      gtk) cmd="$cmd -display gtk" ;;
      sdl) cmd="$cmd -display sdl" ;;
      vnc) cmd="$cmd -vnc :1" ;;
      headless) cmd="$cmd -display none -nographic" ;;
    esac
    [ "$gpu_virtio" = "1" ] && cmd="$cmd -vga virtio"
    [ "$gpu_3d" = "1" ] && cmd="$cmd -device virtio-gpu-pci"
    cmd="$cmd -cdrom \"$iso\""
    [ "$disk_attach" = "1" ] && cmd="$cmd -drive file=samaris-disk-${disk_size}G.qcow2,format=qcow2"
    case "$network" in
      user) cmd="$cmd -netdev user,id=net0 -device virtio-net,netdev=net0" ;;
      tap)  cmd="$cmd -netdev tap,id=net0 -device virtio-net,netdev=net0" ;;
    esac
    [ -n "$extra_args" ] && cmd="$cmd $extra_args"
    printf '%s' "$cmd"
  }

  render_qemu() {
    tui_clear
    tui_hide_cursor

    tui_box_top $w
    tui_center $w "$(tui_c bold "LANCEMENT QEMU")"; echo
    tui_center $w "$(tui_h "Configurer et demarrer l'ISO dans QEMU")"
    echo

    # Machine section
    tui_box_section "Machine" $w
    local arch_label="Architecture:"
    if [ "$focus" = 0 ]; then printf ' >'; else printf '  '; fi
    if [ "$arch" = "x86_64" ]; then printf ' %s' "$(tui_c rad_on)"; else printf ' %s' "$TUI_RAD_OFF"; fi
    printf ' x86_64  '
    if [ "$arch" = "aarch64" ]; then printf '%s' "$(tui_c rad_on)"; else printf '%s' "$TUI_RAD_OFF"; fi
    printf ' aarch64\n'

    if [ "$focus" = 1 ]; then printf ' >'; else printf '  '; fi
    local ram_slider_w=$(( (w - 42) / 2 ))
    [ "$ram_slider_w" -lt 5 ] && ram_slider_w=5
    local ram_pct=$(( (ram - 512) * 100 / (16384 - 512) ))
    printf ' RAM:   [%s  %d MB%s]\n' "$(tui_progress "$ram_pct" "$ram_slider_w")" "$ram" "$(tui_h " (512-16384)")"

    if [ "$focus" = 2 ]; then printf ' >'; else printf '  '; fi
    local cpu_pct=$(( (cpus - 1) * 100 / (16 - 1) ))
    printf ' CPUs:  [%s  %d%s]\n' "$(tui_progress "$cpu_pct" "$ram_slider_w")" "$cpus" "$(tui_h " (1-16)")"

    if [ "$focus" = 3 ]; then printf ' >'; else printf '  '; fi
    printf ' Accel: %s KVM%s\n' "$([ "$kvm" = "1" ] && tui_c check_on || echo "$TUI_CHK_OFF")" "$(tui_h " (note: Linux only)")"
    tui_box_section_bot $w

    # Display section
    tui_box_section "Display" $w
    if [ "$focus" = 4 ]; then printf ' >'; else printf '  '; fi
    printf ' Affichage:'
    local disp_modes=("gtk" "sdl" "vnc" "headless")
    for dm in "${disp_modes[@]}"; do
      if [ "$display" = "$dm" ]; then printf ' %s' "$(tui_c rad_on)"; else printf ' %s' "$TUI_RAD_OFF"; fi
      printf ' %-8s' "$dm"
    done
    printf '\n'

    if [ "$focus" = 5 ]; then printf ' >'; else printf '  '; fi
    printf ' GPU:      %s virtio-vga' "$([ "$gpu_virtio" = "1" ] && tui_c check_on || echo "$TUI_CHK_OFF")"
    [ "$focus" = 6 ] && printf '  >' || printf '   '
    printf ' %s 3D acceleration\n' "$([ "$gpu_3d" = "1" ] && tui_c check_on || echo "$TUI_CHK_OFF")"
    tui_box_section_bot $w

    # Storage section
    tui_box_section "Storage" $w
    local iso_size="?"
    [ -f "$iso" ] && iso_size=$(du -sh "$iso" 2>/dev/null | cut -f1)
    printf '%s   ISO:  %s  (%s)\n' "$TUI_BOX_V" "$(tui_c bold "$iso")" "$(tui_h "$iso_size")"
    if [ "$focus" = 7 ]; then printf ' >'; else printf '  '; fi
    printf ' Disk: %s Ajouter un disque virtuel' "$([ "$disk_attach" = "1" ] && tui_c check_on || echo "$TUI_CHK_OFF")"
    [ "$disk_attach" = "1" ] && printf '  taille: %d GB' "$disk_size"
    printf '\n'
    tui_box_section_bot $w

    # Network section
    tui_box_section "Network" $w
    if [ "$focus" = 8 ]; then printf ' >'; else printf '  '; fi
    local net_modes=("user" "tap" "none")
    for nm in "${net_modes[@]}"; do
      if [ "$network" = "$nm" ]; then printf ' %s' "$(tui_c rad_on)"; else printf ' %s' "$TUI_RAD_OFF"; fi
      case "$nm" in
        user) printf ' User (NAT)  ' ;;
        tap)  printf ' Tap (bridge) ' ;;
        none) printf ' Aucun        ' ;;
      esac
    done
    printf '\n'
    tui_box_section_bot $w

    # Advanced section
    tui_box_section "Advanced" $w
    if [ "$focus" = 9 ]; then printf ' >'; else printf '  '; fi
    local extra_display="${extra_args:-[________________]}"
    printf ' Arguments: %s\n' "$(tui_h "${extra_display:0:$((w - 22))}")"
    tui_box_section_bot $w

    # Footer
    tui_box_bot $w
    echo
    printf '  %s\n' "$(tui_h "[↑↓] Naviguer  [←→] Ajuster  [Space] Toggle  [Enter] Lancer  [Esc] Retour")"
    [ ! -f "$iso" ] && printf '  %s\n' "$(tui_c yellow "[!] ISO file not found: $iso")"
    echo
    local cmdline; cmdline=$(build_qemu_cmdline)
    printf '  %s\n' "$(tui_h "qemu cmd: ${cmdline:0:$((w-12))}")"
  }

  while true; do
    render_qemu
    local key
    key=$(tui_read_key)

    case "$key" in
      UP) [ "$focus" -gt 0 ] && focus=$((focus - 1)) ;;
      DOWN) [ "$focus" -lt $((total_fields - 1)) ] && focus=$((focus + 1)) ;;
      LEFT)
        case "$focus" in
          0) arch="x86_64" ;;
          1) ram=$((ram - 256)); [ "$ram" -lt 512 ] && ram=512 ;;
          2) cpus=$((cpus - 1)); [ "$cpus" -lt 1 ] && cpus=1 ;;
          4)
            case "$display" in
              gtk) display="headless" ;;
              sdl) display="gtk" ;;
              vnc) display="sdl" ;;
              headless) display="vnc" ;;
            esac
            ;;
          7) [ "$disk_attach" = "1" ] && disk_size=$((disk_size - 2)); [ "$disk_size" -lt 2 ] && disk_size=2 ;;
          8)
            case "$network" in
              user) network="none" ;;
              tap) network="user" ;;
              none) network="tap" ;;
            esac
            ;;
        esac
        ;;
      RIGHT)
        case "$focus" in
          0) arch="aarch64" ;;
          1) ram=$((ram + 256)); [ "$ram" -gt 16384 ] && ram=16384 ;;
          2) cpus=$((cpus + 1)); [ "$cpus" -gt 16 ] && cpus=16 ;;
          4)
            case "$display" in
              gtk) display="sdl" ;;
              sdl) display="vnc" ;;
              vnc) display="headless" ;;
              headless) display="gtk" ;;
            esac
            ;;
          7) [ "$disk_attach" = "1" ] && disk_size=$((disk_size + 2)); [ "$disk_size" -gt 100 ] && disk_size=100 ;;
          8)
            case "$network" in
              user) network="tap" ;;
              tap) network="none" ;;
              none) network="user" ;;
            esac
            ;;
        esac
        ;;
      SPACE)
        case "$focus" in
          3) kvm=$((1 - kvm)) ;;
          5) gpu_virtio=$((1 - gpu_virtio)) ;;
          6) gpu_3d=$((1 - gpu_3d)) ;;
          7) disk_attach=$((1 - disk_attach)) ;;
        esac
        ;;
      ENTER)
        if [ -f "$iso" ]; then
          printf '  %s Lancement QEMU...\n' "$(tui_c done "[OK]")"
          echo "qemu:$arch:$ram:$cpus:$display:$gpu_virtio:$gpu_3d:$disk_attach:$disk_size:$network"
          return 0
        fi
        ;;
      ESC) echo "menu"; return 0 ;;
    esac
  done
}

# ─── Environment Check ─────────────────────────────────────

tui_env_check() {
  tui_clear
  tui_hide_cursor
  local w=$TUI_COLS

  tui_box_top $w
  tui_center $w "$(tui_c bold "VERIFICATION ENVIRONNEMENT")"; echo
  echo

  # Host section
  tui_box_section "Hote" $w
  local deps=(
    "bash:bash:version"
    "xorriso:xorriso:version"
    "debootstrap:debootstrap:version"
    "rsync:rsync:version"
    "git:git:version"
    "curl:curl:version"
    "cmake:cmake:version"
    "docker:docker:version"
    "python3:python3:version"
    "qemu-aarch64-static:qemu-aarch64-static:version"
  )

  local all_ok=0 docker_missing=0
  for dep in "${deps[@]}"; do
    local name="${dep%%:*}"
    local bin="${dep#*:}"; bin="${bin%:*}"
    if command -v "$bin" >/dev/null 2>&1; then
      local ver
      ver=$("$bin" --version 2>/dev/null | head -1 | grep -oE '[0-9]+\.[0-9]+(\.[0-9]+)?' | head -1 || echo "?")
      local binpath; binpath=$(command -v "$bin" 2>/dev/null || echo "?")
      printf '%s  %-8s  %-8s  %-16s  %s\n' "$TUI_BOX_V" "$(tui_c done "[OK]")" "$(tui_c dim "$ver")" "$name" "$(tui_h "$binpath")"
    else
      local note=""
      [ "$bin" = "docker" ] && note="(disponible dans Docker uniquement)" && docker_missing=1
      printf '%s  %-8s  %-8s  %-16s  %s\n' "$TUI_BOX_V" "$(tui_c skip "[--]")" "" "$(tui_c yellow "$name")" "$(tui_h "$note")"
      all_ok=$((all_ok + 1))
    fi
  done
  tui_box_section_bot $w

  # Docker section
  tui_box_section "Docker" $w
  local docker_ver="non trouve" docker_img="" docker_vol=""
  if command -v docker >/dev/null 2>&1; then
    docker_ver=$(docker --version 2>/dev/null | grep -oE '[0-9]+\.[0-9]+\.[0-9]+' | head -1 || echo "?")
    docker_img=$(docker images --format '{{.Repository}}:{{.Tag}} ({{.Size}})' --filter=reference='samaris-os-builder:*' 2>/dev/null | head -1 || echo "none")
    docker_vol=$(docker volume inspect samaris-os-work --format '{{.Name}} {{.Driver}} {{.Scope}}' 2>/dev/null || echo "not created")
    [ -z "$docker_img" ] && docker_img="none"
  fi
  printf '%s  %-8s  Docker Engine    %s\n' "$TUI_BOX_V" "$(tui_c done "[OK]")" "$(tui_c dim "$docker_ver")"
  printf '%s  %-8s  Image builder    %s\n' "$TUI_BOX_V" "$( [ "$docker_img" != "none" ] && tui_c done "[OK]" || tui_c skip "[--]")" "$docker_img"
  printf '%s  %-8s  Volume work      %s\n' "$TUI_BOX_V" "$( [ "$docker_vol" != "not created" ] && tui_c done "[OK]" || tui_c skip "[--]")" "$docker_vol"
  tui_box_section_bot $w

  # Result section
  tui_box_section "Resultat" $w
  if [ "$all_ok" -eq 0 ]; then
    printf '%s  %s\n' "$TUI_BOX_V" "$(tui_c green_bold "[OK]  Environnement pret pour la construction")"
  else
    printf '%s  %s\n' "$TUI_BOX_V" "$(tui_c yellow "[!]  $all_ok dependance(s) manquante(s)")"
  fi
  [ "$docker_missing" = "1" ] && printf '%s  %s\n' "$TUI_BOX_V" "$(tui_c yellow "     [!] docker non disponible sur l'hote (mais present dans l'image Docker)")"
  tui_box_section_bot $w

  tui_box_bot $w
  echo
  printf '  %s\n' "$(tui_h "[R] Re-verifier  [Esc] Retour au menu")"
  local key
  key=$(tui_read_key)
  case "$key" in
    r|R) echo "check" ;;
    *) echo "menu" ;;
  esac
}

# ─── Log Viewer ──────────────────────────────────────────────

tui_log_viewer() {
  local w=$TUI_COLS
  local logs=()
  local step
  for step in "${BUILD_STEPS[@]}"; do
    local lf; lf=$(state_log_path "$step")
    [ -f "$lf" ] && logs+=("$step")
  done

  if [ ${#logs[@]} -eq 0 ]; then
    tui_box_top $w
    printf '%s  %s\n' "$TUI_BOX_V" "$(tui_c yellow "[!] No log files found")"
    tui_box_bot $w
    tui_wait_enter
    echo "menu"
    return 0
  fi

  tui_clear
  tui_hide_cursor
  tui_box_top $w
  tui_center $w "$(tui_c bold "LOGS DE CONSTRUCTION")"; echo
  tui_box_sep $w
  tui_box_section "Selectionner une etape" $w
  local i=1
  for step in "${logs[@]}"; do
    local lf; lf=$(state_log_path "$step")
    local lines; lines=$(wc -l < "$lf" 2>/dev/null || echo 0)
    printf '%s  %2d)  %-22s  %d lines\n' "$TUI_BOX_V" "$i" "$step" "$lines"
    i=$((i+1))
  done
  tui_box_section_bot $w
  tui_box_bot $w
  echo
  printf '  Step number [1-%d, 0=cancel]: ' "${#logs[@]}"
  local sel
  read -r sel

  if [ "$sel" -ge 1 ] && [ "$sel" -le "${#logs[@]}" ] 2>/dev/null; then
    local selected="${logs[$((sel-1))]}"
    local lf; lf=$(state_log_path "$selected")
    local total_lines; total_lines=$(wc -l < "$lf" 2>/dev/null || echo 0)
    local scroll_offset=0 search_term=""
    local viewport=$((TUI_LINES - 6))
    [ "$viewport" -lt 5 ] && viewport=5

    while true; do
      tui_clear
      tui_hide_cursor
      tui_box_top $w
      tui_center $w "$(tui_c bold "LOG: $selected")  $(tui_h "($total_lines lines)")"; echo
      tui_box_sep $w

      local start_line=$((scroll_offset + 1))
      local end_line=$((scroll_offset + viewport))
      [ "$end_line" -gt "$total_lines" ] && end_line=$total_lines

      local idx=0
      while IFS= read -r line; do
        idx=$((idx + 1))
        [ "$idx" -lt "$start_line" ] && continue
        [ "$idx" -gt "$end_line" ] && break
        local display="$line"
        [ -n "$search_term" ] && display=$(echo "$display" | grep --color=always -i "$search_term" 2>/dev/null || echo "$display")
        local truncated="${display:0:$((w - 6))}"
        printf '%s  %s%*s\n' "$TUI_BOX_V" "$(tui_h "$truncated")" $((w - 6 - ${#truncated})) ""
      done < "$lf"

      local fill=$((viewport - (end_line - start_line + 1)))
      [ "$fill" -lt 0 ] && fill=0
      for ((; fill > 0; fill--)); do
        printf '%s%*s\n' "$TUI_BOX_V" $((w - 1)) "$TUI_BOX_V"
      done

      local footer="[↑↓] Scroll  [F] Chercher${search_term:+  (/${search_term})}  [Q] Fermer"
      tui_box_sep $w
      printf '%s  %s%*s\n' "$TUI_BOX_V" "$(tui_h "$footer")" $((w - 6 - ${#footer})) ""
      tui_box_bot $w

      local key
      key=$(tui_read_key)
      case "$key" in
        UP) [ "$scroll_offset" -gt 0 ] && scroll_offset=$((scroll_offset - 1)) ;;
        DOWN) [ "$scroll_offset" -lt $((total_lines - viewport)) ] && scroll_offset=$((scroll_offset + 1)) ;;
        PAGE_UP) scroll_offset=$((scroll_offset - viewport)); [ "$scroll_offset" -lt 0 ] && scroll_offset=0 ;;
        PAGE_DOWN) scroll_offset=$((scroll_offset + viewport))
                   [ "$scroll_offset" -gt $((total_lines - viewport)) ] && scroll_offset=$((total_lines - viewport))
                   [ "$scroll_offset" -lt 0 ] && scroll_offset=0 ;;
        f|F)
          tui_show_cursor
          printf '\n  Search: '
          read -r search_term
          tui_hide_cursor
          # Jump to first match
          if [ -n "$search_term" ]; then
            local match_line; match_line=$(grep -n -i "$search_term" "$lf" 2>/dev/null | head -1 | cut -d: -f1 || echo "")
            [ -n "$match_line" ] && scroll_offset=$((match_line - 2))
            [ "$scroll_offset" -lt 0 ] && scroll_offset=0
          fi
          ;;
        q|Q|ESC) break ;;
      esac
    done
  fi
  echo "menu"
}

# ─── Splash Screen (Screen 1) ──────────────────────────────

TUI_SPLASH_SHOWN=0

tui_splash() {
  [ "$TUI_SPLASH_SHOWN" = "1" ] && return
  TUI_SPLASH_SHOWN=1
  tui_clear
  tui_hide_cursor
  local w=$TUI_COLS

  local samaris_logo=(
    "███████╗ █████╗ ███╗   ███╗ █████╗ ██████╗ ██╗███████╗"
    "██╔════╝██╔══██╗████╗ ████║██╔══██╗██╔══██╗██║██╔════╝"
    "███████╗███████║██╔████╔██║███████║██████╔╝██║███████╗"
    "╚════██║██╔══██║██║╚██╔╝██║██╔══██║██╔══██╗██║╚════██║"
    "███████║██║  ██║██║ ╚═╝ ██║██║  ██║██║  ██║██║███████║"
    "╚══════╝╚═╝  ╚═╝╚═╝     ╚═╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝╚══════╝"
  )
  local version_bar="ISO GENERATOR  —  v1.0.0   Mountain Lake Alpha One   Debian Trixie"

  tui_box_top $w; echo
  for line in "${samaris_logo[@]}"; do
    tui_center "$w" "$(tui_c bold "$line")"; echo
  done
  echo
  tui_center "$w" "$(tui_c bold "$version_bar")"; echo; echo

  # ── Environment Detection ──────────────────────────────
  printf '%s  %s\n' "$TUI_BOX_V" "$(tui_c bold "$(tui_icon refresh) Environment Detection")"
  local deps=(
    "bash:bash" "xorriso:xorriso" "debootstrap:debootstrap"
    "rsync:rsync" "curl:curl" "git:git"
    "cmake:cmake" "docker:docker" "python3:python3"
  )
  local cols=$(( (w - 8) / 2 ))
  local i=0 lc="" rc="" lcs=$cols rcs=$cols
  for dep in "${deps[@]}"; do
    local name="${dep%%:*}" bin="${dep#*:}"
    local tag
    if command -v "$bin" >/dev/null 2>&1; then
      local ver; ver=$("$bin" --version 2>/dev/null | head -1 | grep -oE '[0-9]+\.[0-9]+' | head -1 || echo "?")
      tag="$(tui_c green "$(tui_icon check)") $(printf '%-10s' "$name") $(tui_c dim "$ver")"
    else
      tag="$(tui_c yellow "$(tui_icon warn)") $(printf '%-10s' "$name") $(tui_c dim "not found")"
    fi
    if [ "$((i % 2))" -eq 0 ]; then lc="$tag"; lcs=$cols
    else rc="$tag"; rcs=$cols
      printf '%s  %-*s  %s  %s\n' "$TUI_BOX_V" "$lcs" "$lc" "$TUI_BOX_V" "$rc"
    fi
    i=$((i+1))
  done
  [ "$((i % 2))" -ne 0 ] && printf '%s  %-*s  %s  %*s\n' "$TUI_BOX_V" "$lcs" "$lc" "$TUI_BOX_V" "$rcs" ""

  # ── Build Status ──────────────────────────────────────
  local done_count=0 total_steps="${#BUILD_STEPS[@]}"
  for step in "${BUILD_STEPS[@]}"; do state_complete "$step" && done_count=$((done_count+1)); done
  local last_build="No previous build"
  local latest_file="" latest_step=""
  latest_file="$(tui_latest_done_file)"
  if [ -n "$latest_file" ]; then
    latest_step="$(basename "$latest_file" .done)"
    local ts; ts=$(state_get_field completed_at "$latest_file" 2>/dev/null | sed 's/T/ /; s/Z//' || echo "?")
    ts="${ts%:*}"; ts="${ts%:*}"; [ -z "$ts" ] && ts="?"
    last_build="${latest_step} at ${ts}"
  fi
  local docker_vol
  docker_vol=$(docker volume inspect samaris-os-work 2>/dev/null | sed -n 's/.*"Mountpoint": "\([^"]*\)".*/\1/p' || echo "not found")

  printf '%s  %s\n' "$TUI_BOX_V" "$(tui_c bold "$(tui_icon chart) Build Status")"
  printf '%s  %-*s  %s\n' "$TUI_BOX_V" $((cols*2)) "Dernier build : $(tui_h "$last_build")" "$TUI_BOX_V"
  printf '%s  %-*s  %s\n' "$TUI_BOX_V" $((cols*2)) "Étapes : $(tui_c $([ "$done_count" -eq "$total_steps" ] && echo green || echo yellow) "$done_count/$total_steps")$(tui_h " completes")$(tui_progress_colored "$((done_count*100/(total_steps>0?total_steps:1)))" 10)" "$TUI_BOX_V"
  printf '%s  %-*s  %s\n' "$TUI_BOX_V" $((cols*2)) "Docker volume : $(tui_c dim "$docker_vol")" "$TUI_BOX_V"

  tui_box_bot $w; echo
  tui_center "$w" "$(tui_h "Press ENTER to continue...")"; echo
  tui_read_key >/dev/null
  tui_show_cursor
}

# ─── TUI Main Controller ───────────────────────────────────

tui_main() {
  tui_ensure_init
  build_config_init
  tui_splash
  local action
  while true; do
    action=$(tui_main_menu)
    case "$action" in
      build)
        if tui_build_config; then
          log "Starting build with config..."
          tui_run_build_with_dashboard
          action=$(tui_build_summary)
          case "$action" in
            qemu) run_step "17-qemu" || true ;;
            config_save)
              printf '\n  Config name: '
              read -r cname
              [ -n "$cname" ] && build_config_save "$cname"
              ;;
            clean) action=$(tui_clean_prompt) ;;
          esac
        fi
        ;;
      status)
        action=$(tui_status_dashboard)
        [ "$action" = "log_viewer" ] && action=$(tui_log_viewer)
        ;;
      run)
        local run_result step force_flag docker_flag dryrun_flag
        run_result=$(tui_run_step)
        IFS='|' read -r step force_flag docker_flag dryrun_flag <<< "$run_result"
        if [ -n "$step" ]; then
          tui_clear
          tui_hide_cursor
          printf '\n%s %s\n\n' "$(tui_c running "[>>]")" "$(tui_c bold "Running: $step")"
          if [ "${force_flag:-0}" = "1" ]; then
            export RUN_FORCE=1
            state_clear_step "$step"
          else
            export RUN_FORCE=0
          fi
          export BUILD_DRY_RUN="${dryrun_flag:-0}"
          if [ "${docker_flag:-0}" = "1" ] && [ "${SAMARIS_IN_DOCKER:-0}" != "1" ]; then
            local run_args=(run "$step")
            [ "$RUN_FORCE" = "1" ] && run_args+=(--force)
            [ "$BUILD_DRY_RUN" = "1" ] && run_args+=(--dry-run)
            run_in_docker "${run_args[@]}" || true
          else
            run_step "$step" || true
          fi
          tui_wait_enter
        fi
        ;;
      check)
        action=$(tui_env_check)
        ;;
      clean)
        action=$(tui_clean_prompt)
        ;;
      config)
        action=$(tui_config_manager)
        ;;
      qemu)
        local qemu_result
        qemu_result=$(tui_qemu_config)
        case "$qemu_result" in
          qemu:*)
            local _tag qemu_arch qemu_ram qemu_cpus qemu_display qemu_gpu qemu_gpu3d qemu_disk qemu_disk_size qemu_network
            IFS=':' read -r _tag qemu_arch qemu_ram qemu_cpus qemu_display qemu_gpu qemu_gpu3d qemu_disk qemu_disk_size qemu_network <<< "$qemu_result"
            if [ -n "$qemu_arch" ]; then
              tui_clear
              printf '\n%s\n\n' "$(tui_c bold "  BOOTING QEMU")"
              printf '  %s\n' "$(tui_h "arch=$qemu_arch ram=${qemu_ram}M cpus=$qemu_cpus display=$qemu_display")"
              echo
              QEMU_RAM="$qemu_ram" \
              QEMU_CPUS="$qemu_cpus" \
              QEMU_DISPLAY="$qemu_display" \
              QEMU_GPU_VIRTIO="$qemu_gpu" \
              QEMU_GPU_3D="$qemu_gpu3d" \
              QEMU_DISK_ATTACH="$qemu_disk" \
              QEMU_DISK_SIZE="$qemu_disk_size" \
              QEMU_NETWORK="$qemu_network" \
                run_step "17-qemu" "$qemu_arch" || true
              tui_wait_enter
            fi
            ;;
        esac
        ;;
      quit)
        tui_clear
        tui_show_cursor
        log "Goodbye"
        exit 0
        ;;
    esac
  done
}
