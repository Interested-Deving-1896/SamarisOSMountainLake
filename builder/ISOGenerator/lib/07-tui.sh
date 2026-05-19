#!/usr/bin/env bash
# SAMARIS ISO GENERATOR — Terminal UI Engine
# Pure bash TUI with ANSI codes, zero external dependencies

[ -n "${SAMARIS_TUI_LOADED:-}" ] && return 0
SAMARIS_TUI_LOADED=1

TUI_COLS=80
TUI_LINES=24
TUI_HAS_COLORS=0
TUI_HAS_UTF8=0

# Box characters (set by init)
TUI_BOX_TL="+" TUI_BOX_TR="+" TUI_BOX_BL="+" TUI_BOX_BR="+"
TUI_BOX_H="-" TUI_BOX_V="|"
TUI_BOX_ML="+" TUI_BOX_MR="+" TUI_BOX_TM="+" TUI_BOX_BM="+"
TUI_BAR_FILL="#" TUI_BAR_EMPTY="."
TUI_CHK_ON="[X]" TUI_CHK_OFF="[ ]"
TUI_RAD_ON="(O)" TUI_RAD_OFF="( )"

tui_detect() {
  TUI_COLS=$(tput cols 2>/dev/null || echo 80)
  TUI_LINES=$(tput lines 2>/dev/null || echo 24)
  if [ "$(tput colors 2>/dev/null || echo 0)" -ge 8 ]; then
    TUI_HAS_COLORS=1
  fi
  # UTF-8 detection: only enable if locale explicitly says UTF-8
  # AND terminal is not in a known limited environment
  local cm; cm=$(locale charmap 2>/dev/null || echo ASCII)
  if [ "$cm" = "UTF-8" ] && [ -n "${TERM_PROGRAM:-}" ]; then
    TUI_HAS_UTF8=1
  fi
}

tui_c() {
  [ "$TUI_HAS_COLORS" != 1 ] && { printf '%s' "$*"; return; }
  [ -z "${1:-}" ] && { printf '%s' "$*"; return; }
  case "$1" in
    green)    tput setaf 2; shift; printf '%s' "$*"; tput sgr0 ;;
    red)      tput setaf 1; shift; printf '%s' "$*"; tput sgr0 ;;
    yellow)   tput setaf 3; shift; printf '%s' "$*"; tput sgr0 ;;
    blue)     tput setaf 4; shift; printf '%s' "$*"; tput sgr0 ;;
    cyan)     tput setaf 6; shift; printf '%s' "$*"; tput sgr0 ;;
    magenta)  tput setaf 5; shift; printf '%s' "$*"; tput sgr0 ;;
    white)    tput setaf 7; shift; printf '%s' "$*"; tput sgr0 ;;
    dim)      tput dim; shift; printf '%s' "$*"; tput sgr0 ;;
    bold)     tput bold; shift; printf '%s' "$*"; tput sgr0 ;;
    green_bold)  tput bold; tput setaf 2; shift; printf '%s' "$*"; tput sgr0 ;;
    red_bold)    tput bold; tput setaf 1; shift; printf '%s' "$*"; tput sgr0 ;;
    yellow_bold) tput bold; tput setaf 3; shift; printf '%s' "$*"; tput sgr0 ;;
    blue_bold)   tput bold; tput setaf 4; shift; printf '%s' "$*"; tput sgr0 ;;
    ok)       tput setaf 2; printf '%s' "[OK]"; tput sgr0 ;;
    fail)     tput setaf 1; printf '%s' "[!!]"; tput sgr0 ;;
    warn)     tput setaf 3; printf '%s' "[!]"; tput sgr0 ;;
    skip)     tput dim; printf '%s' "[--]"; tput sgr0 ;;
    pending)  tput dim; printf '%s' "[..]"; tput sgr0 ;;
    running)  tput setaf 3; printf '%s' "[>>]"; tput sgr0 ;;
    done)     tput setaf 2; printf '%s' "[OK]"; tput sgr0 ;;
    check_on) tput setaf 2; printf '%s' "$TUI_CHK_ON"; tput sgr0 ;;
    check_off) printf '%s' "$TUI_CHK_OFF" ;;
    rad_on)   tput setaf 3; printf '%s' "$TUI_RAD_ON"; tput sgr0 ;;
    rad_off)  printf '%s' "$TUI_RAD_OFF" ;;
    *)        printf '%s' "$*" ;;
  esac
}

tui_h() { tput dim 2>/dev/null; printf '%s' "$*"; tput sgr0 2>/dev/null; }

tui_utf8_box() {
  TUI_BOX_TL=$(printf '\xe2\x95\x94') TUI_BOX_TR=$(printf '\xe2\x95\x97')
  TUI_BOX_BL=$(printf '\xe2\x95\x9a') TUI_BOX_BR=$(printf '\xe2\x95\x9d')
  TUI_BOX_H=$(printf '\xe2\x95\x90')  TUI_BOX_V=$(printf '\xe2\x95\x91')
  TUI_BOX_ML=$(printf '\xe2\x95\xa0') TUI_BOX_MR=$(printf '\xe2\x95\xa3')
  TUI_BOX_TM=$(printf '\xe2\x95\xa4') TUI_BOX_BM=$(printf '\xe2\x95\xa7')
  TUI_BAR_FILL=$(printf '\xe2\x96\x88') TUI_BAR_EMPTY=$(printf '\xe2\x96\x91')
  TUI_CHK_ON="["$(printf '\xe2\x9c\x93')"]" TUI_CHK_OFF="[ ]"
  TUI_RAD_ON="("$(printf '\xe2\x97\x89')")" TUI_RAD_OFF="( )"
}

tui_ascii_box() {
  TUI_BOX_TL="+" TUI_BOX_TR="+" TUI_BOX_BL="+" TUI_BOX_BR="+"
  TUI_BOX_H="-" TUI_BOX_V="|"
  TUI_BOX_ML="+" TUI_BOX_MR="+" TUI_BOX_TM="+" TUI_BOX_BM="+"
  TUI_BAR_FILL="#" TUI_BAR_EMPTY="."
  TUI_CHK_ON="[X]" TUI_CHK_OFF="[ ]"
  TUI_RAD_ON="(O)" TUI_RAD_OFF="( )"
}

tui_init() {
  tui_detect
  if [ "$TUI_HAS_UTF8" = 1 ]; then tui_utf8_box; else tui_ascii_box; fi
}

tui_clear() { printf '\033[2J\033[H'; }
tui_goto() { printf '\033[%d;%dH' "$1" "$2"; }
tui_hide_cursor() { printf '\033[?25l'; }
tui_show_cursor() { printf '\033[?25h'; }

tui_hr() {
  local ch="${1:-$TUI_BOX_H}" n="${2:-$TUI_COLS}"
  local s=""; printf -v s '%*s' "$n" ''; printf '%s' "${s// /$ch}"
}

tui_box_top() {
  local w=$1; local h=$TUI_BOX_H
  printf '%s' "$TUI_BOX_TL"; tui_hr "$h" $((w-2)); printf '%s\n' "$TUI_BOX_TR"
}
tui_box_bot() {
  local w=$1; local h=$TUI_BOX_H
  printf '%s' "$TUI_BOX_BL"; tui_hr "$h" $((w-2)); printf '%s\n' "$TUI_BOX_BR"
}
tui_box_sep() {
  local w=$1; local h=$TUI_BOX_H
  printf '%s' "$TUI_BOX_ML"; tui_hr "$h" $((w-2)); printf '%s\n' "$TUI_BOX_MR"
}
tui_box_line() {
  printf '%s %s %s\n' "$TUI_BOX_V" "$*" "$TUI_BOX_V"
}

tui_draw_box() {
  local w=$1 h=$2; shift 2
  tui_box_top $w
  local i=0
  for line in "$@"; do
    if [ "$i" -ge $((h-2)) ]; then break; fi
    local pad=$((w - 3 - ${#line}))
    [ "$pad" -lt 1 ] && pad=1
    printf '%s %s%*s\n' "$TUI_BOX_V" "$line" "$pad" "$TUI_BOX_V"
    i=$((i+1))
  done
  for ((; i < h-2; i++)); do
    printf '%s%*s\n' "$TUI_BOX_V" $((w-1)) "$TUI_BOX_V"
  done
  tui_box_bot $w
}

tui_progress() {
  local pct=$1 w=${2:-20}
  [ "$pct" -gt 100 ] && pct=100; [ "$pct" -lt 0 ] && pct=0
  local f=$((pct*w/100))
  local e=$((w-f))
  local fb="" eb=""
  [ "$f" -gt 0 ] && printf -v fb '%*s' "$f" '' && fb="${fb// /$TUI_BAR_FILL}"
  [ "$e" -gt 0 ] && printf -v eb '%*s' "$e" '' && eb="${eb// /$TUI_BAR_EMPTY}"
  printf '%s%s' "$fb" "$eb"
}

tui_progress_line() {
  local pct=$1 w=${2:-20}
  printf '%s  %3d%%%%' "$(tui_progress "$pct" "$w")" "$pct"
}

tui_read_key() {
  local key rest
  IFS= read -r -s -n1 key 2>/dev/null || true
  if [ "$key" = $'\033' ]; then
    if read -r -s -n2 -t 0.1 rest 2>/dev/null; then
      case "$rest" in
        '[A') echo 'UP' ;; '[B') echo 'DOWN' ;;
        '[C') echo 'RIGHT' ;; '[D') echo 'LEFT' ;;
        '[H') echo 'HOME' ;; '[F') echo 'END' ;;
        '[5~') echo 'PAGE_UP' ;; '[6~') echo 'PAGE_DOWN' ;;
        'OP') echo 'F1' ;; 'OQ') echo 'F2' ;;
        'OR') echo 'F3' ;; 'OS') echo 'F4' ;;
        *) echo 'ESC' ;;
      esac
    else echo 'ESC'; fi
  elif [ "$key" = $'\n' ] || [ "$key" = $'\r' ] || [ -z "$key" ]; then echo 'ENTER'
  elif [ "$key" = $'\t' ]; then echo 'TAB'
  elif [ "$key" = $'\177' ] || [ "$key" = $'\b' ]; then echo 'BACKSPACE'
  elif [ "$key" = ' ' ]; then echo 'SPACE'
  else echo "$key"; fi
}

tui_wait_enter() {
  printf '  %s' "$(tui_h "Press ENTER to continue...")"
  tui_read_key >/dev/null
  printf '\r%*s\r' 80 ''
}

tui_dialog() {
  local w=$1; shift
  tui_box_top "$w"
  for line in "$@"; do
    local pad=$((w - 3 - ${#line}))
    [ "$pad" -lt 1 ] && pad=1
    printf '%s%s%*s\n' "$TUI_BOX_V" "$line" "$pad" "$TUI_BOX_V"
  done
  tui_box_bot "$w"
}

tui_confirm() {
  local prompt="$1" default="${2:-n}"
  printf '%s [y/N] ' "$prompt"
  local k; k=$(tui_read_key)
  case "$k" in y|Y|o|O) echo; return 0 ;; *) echo; return 1 ;; esac
}

tui_center() {
  local w="${1:-$TUI_COLS}"
  shift 2>/dev/null || true
  local s="$*"
  local l=${#s}
  local p=0
  [ "$w" -gt "$l" ] 2>/dev/null && p=$(((w - l) / 2))
  [ "$p" -lt 0 ] && p=0
  printf '%*s%s%*s' "$p" '' "$s" $((w - l - p)) ''
}

tui_title() {
  local w=$1; shift
  local s="$*"
  tui_box_top "$w"
  tui_center "$w" "$(tui_c bold "$s")"; echo
  tui_box_bot "$w"
}

tui_banner() {
  local w=$1; shift
  tui_box_top "$w"
  for l in "$@"; do tui_box_line "$l"; done
  tui_box_bot "$w"
}

tui_read_key_timeout() {
  local timeout=${1:-0.5}
  local key rest
  IFS= read -r -s -n1 -t "$timeout" key 2>/dev/null || true
  if [ -z "$key" ]; then echo 'TIMEOUT'; return; fi
  if [ "$key" = $'\033' ]; then
    if read -r -s -n2 -t 0.1 rest 2>/dev/null; then
      case "$rest" in
        '[A') echo 'UP' ;; '[B') echo 'DOWN' ;;
        '[C') echo 'RIGHT' ;; '[D') echo 'LEFT' ;;
        '[H') echo 'HOME' ;; '[F') echo 'END' ;;
        '[5~') echo 'PAGE_UP' ;; '[6~') echo 'PAGE_DOWN' ;;
        'OP') echo 'F1' ;; 'OQ') echo 'F2' ;;
        'OR') echo 'F3' ;; 'OS') echo 'F4' ;;
        *) echo 'ESC' ;;
      esac
    else echo 'ESC'; fi
  elif [ "$key" = $'\n' ] || [ "$key" = $'\r' ] || [ -z "$key" ]; then echo 'ENTER'
  elif [ "$key" = $'\t' ]; then echo 'TAB'
  elif [ "$key" = $'\177' ] || [ "$key" = $'\b' ]; then echo 'BACKSPACE'
  elif [ "$key" = ' ' ]; then echo 'SPACE'
  else echo "$key"; fi
}

tui_box_section() {
  local title="$1" w=${2:-$TUI_COLS}
  w=$((w - 2))
  local tlen=${#title}
  local rem=$((w - tlen - 4))
  [ "$rem" -lt 0 ] && rem=0
  printf '%s ' "$TUI_BOX_TL"
  printf '%s' "$TUI_BOX_H"
  printf '%s' "$title"
  printf ' '
  local s=""; printf -v s '%*s' "$rem" ''; printf '%s' "${s// /$TUI_BOX_H}"
  printf ' %s\n' "$TUI_BOX_TR"
}

tui_box_section_bot() {
  local w=${1:-$TUI_COLS}
  w=$((w - 2))
  printf '%s' "$TUI_BOX_BL"
  local n=$((w - 2))
  local s=""; printf -v s '%*s' "$n" ''; printf '%s' "${s// /$TUI_BOX_H}"
  printf '%s\n' "$TUI_BOX_BR"
}

tui_progress_bar() {
  local pct=$1 w=${2:-20} label="${3:-}"
  [ "$pct" -gt 100 ] && pct=100; [ "$pct" -lt 0 ] && pct=0
  local f=$((pct*w/100))
  local e=$((w-f))
  local fb=""; [ "$f" -gt 0 ] && printf -v fb '%*s' "$f" '' && fb="${fb// /$TUI_BAR_FILL}"
  local eb=""; [ "$e" -gt 0 ] && printf -v eb '%*s' "$e" '' && eb="${eb// /$TUI_BAR_EMPTY}"
  printf '%s%s%s  %3d%%%s' "$fb" "$eb" "$TUI_BOX_V" "$pct" "${label:+ $label}"
}

tui_box_line_raw() {
  printf '%s%s%s\n' "$TUI_BOX_V" "$*" "$TUI_BOX_V"
}

tui_hr_to() {
  local ch="${1:-$TUI_BOX_H}" n="${2:-$TUI_COLS}"
  local s=""; printf -v s '%*s' "$n" ''; printf '%s' "${s// /$ch}"
}

TUI_INIT_DONE=0
tui_ensure_init() {
  [ "$TUI_INIT_DONE" = "1" ] && return
  TUI_INIT_DONE=1
  tui_init
}

tui_icon() {
  [ "$TUI_HAS_UTF8" = 0 ] && return 0
  [ -z "${1:-}" ] && return 0
  case "$1" in
    check|ok)       printf '\xe2\x9c\x85 ' ;;    cross|fail)     printf '\xe2\x9d\x8c ' ;;
    warn*|warning)  printf '\xe2\x9a\xa0\xef\xb8\x8f ' ;;  build|hammer)   printf '\xf0\x9f\x94\xa8 ' ;;
    chart|stats)    printf '\xf0\x9f\x93\x8a ' ;;  play|run)       printf '\xe2\x96\xb6\xef\xb8\x8f ' ;;
    stop|abort)     printf '\xf0\x9f\x9b\x91 ' ;;  pause)          printf '\xe2\x8f\xb8\xef\xb8\x8f ' ;;
    clean|broom)    printf '\xf0\x9f\xa7\xb9 ' ;;  save|disk)      printf '\xf0\x9f\x92\xbe ' ;;
    load|folder)    printf '\xf0\x9f\x93\x82 ' ;;  config|gear)    printf '\xe2\x9a\x99\xef\xb8\x8f ' ;;
    qemu|desktop)   printf '\xf0\x9f\x96\xa5\xef\xb8\x8f ' ;;  menu|list)      printf '\xe2\x98\xb0 ' ;;
    quit|exit)      printf '\xf0\x9f\x9a\xaa ' ;;  terminal|logs)  printf '\xf0\x9f\x93\x8b ' ;;
    search|find)    printf '\xf0\x9f\x94\x8d ' ;;  refresh)        printf '\xf0\x9f\x94\x84 ' ;;
    time|clock)     printf '\xf0\x9f\x95\x90 ' ;;  iso|cd)         printf '\xf0\x9f\x93\x80 ' ;;
    docker)         printf '\xf0\x9f\x90\xb3 ' ;;  arch|cpu)       printf '\xf0\x9f\x92\xbb ' ;;
    ai|brain)       printf '\xf0\x9f\xa7\xa0 ' ;;  network|wifi)   printf '\xf0\x9f\x93\xa1 ' ;;
    print|cups)     printf '\xf0\x9f\x96\xa8\xef\xb8\x8f ' ;;  power|battery)  printf '\xf0\x9f\x94\x8b ' ;;
    desktop|ui)     printf '\xf0\x9f\x96\xa5 ' ;;  browser|web)    printf '\xf0\x9f\x8c\x90 ' ;;
    disk|storage)   printf '\xf0\x9f\x92\xbe ' ;;  rust|kernel)    printf '\xf0\x9f\xa6\x80 ' ;;
    packages|deb)   printf '\xf0\x9f\x93\xa6 ' ;;  *)              return 0 ;;
  esac
}

tui_progress_colored() {
  local pct=$1 w=${2:-20}
  [ "$pct" -gt 100 ] && pct=100; [ "$pct" -lt 0 ] && pct=0
  local f=$((pct*w/100))
  local e=$((w-f))
  local fb="" eb=""
  [ "$f" -gt 0 ] && printf -v fb '%*s' "$f" '' && fb="${fb// /$TUI_BAR_FILL}"
  [ "$e" -gt 0 ] && printf -v eb '%*s' "$e" '' && eb="${eb// /$TUI_BAR_EMPTY}"
  if [ "$pct" -ge 90 ]; then tui_c green "${fb}${eb}"
  elif [ "$pct" -ge 60 ]; then tui_c cyan "${fb}${eb}"
  elif [ "$pct" -ge 30 ]; then tui_c yellow "${fb}${eb}"
  else tui_c white "${fb}${eb}"; fi
  printf ' %3d%%%%' "$pct"
}

# ─── Section Header ────────────────────────────────────────

tui_section_header() {
  local title="$1" w=${2:-$TUI_COLS} ch="${3:-$TUI_BOX_H}"
  local tlen=${#title} pad=$(( (w - tlen - 4) / 2 ))
  [ "$pad" -lt 0 ] && pad=0
  printf '  '
  local s=""; printf -v s '%*s' "$pad" ''; printf '%s' "${s// /$ch}"
  printf ' %s ' "$(tui_c bold "$title")"
  local rem=$((w - tlen - pad - 5))
  [ "$rem" -lt 0 ] && rem=0
  printf -v s '%*s' "$rem" ''; printf '%s\n' "${s// /$ch}"
}

# ─── Two-Panel Layout ──────────────────────────────────────

tui_two_panel() {
  local l_title="$1" r_title="$2" w=$3 l_content="$4" r_content="$5"
  local half=$(( (w - 5) / 2 ))
  printf '%s' "$TUI_BOX_V"
  printf ' %-*s ' "$half" "$(tui_c bold "$l_title")"
  printf '%s' "$TUI_BOX_V"
  printf ' %-*s ' "$half" "$(tui_c bold "$r_title")"
  printf '%s\n' "$TUI_BOX_V"
  local l_lines r_lines
  mapfile -t l_lines <<< "$l_content"
  mapfile -t r_lines <<< "$r_content"
  local max=$(( ${#l_lines[@]} > ${#r_lines[@]} ? ${#l_lines[@]} : ${#r_lines[@]} ))
  for ((i=0; i<max; i++)); do
    local ll="${l_lines[$i]:-}" rl="${r_lines[$i]:-}"
    printf '%s %-*s %s %-*s %s\n' "$TUI_BOX_V" "$half" "$ll" "$TUI_BOX_V" "$half" "$rl" "$TUI_BOX_V"
  done
}

# ─── Slider ─────────────────────────────────────────────────

tui_slider() {
  local val=$1 min=$2 max=$3 w=${4:-20} label="${5:-}"
  local range=$((max - min)) pos=$((val - min))
  [ "$range" -le 0 ] && range=1
  local pct=$((pos * 100 / range))
  local f=$((pct * (w-2) / 100))
  local e=$(((w-2) - f))
  [ "$f" -lt 0 ] && f=0; [ "$e" -lt 0 ] && e=0
  local fb=""; [ "$f" -gt 0 ] && printf -v fb '%*s' "$f" '' && fb="${fb// /$TUI_BAR_FILL}"
  local eb=""; [ "$e" -gt 0 ] && printf -v eb '%*s' "$e" '' && eb="${eb// /$TUI_BAR_EMPTY}"
  printf '[%s%s%s] %d%s' "$fb" "$TUI_BOX_ML" "$eb" "$val" "${label:+ $label}"
}

# ─── Checkbox / Radio — enhanced ──────────────────────────

tui_checkbox() {
  local checked=$1
  [ "$TUI_HAS_UTF8" = "1" ] && printf "$(tui_c $([ "$checked" = 1 ] && echo "green" || echo "dim") '[')$(tui_c $([ "$checked" = 1 ] && echo "check_on" || echo "dim") ' ')$(tui_c $([ "$checked" = 1 ] && echo "green" || echo "dim") ']')" || printf '%s' "$([ "$checked" = 1 ] && echo "$TUI_CHK_ON" || echo "$TUI_CHK_OFF")"
}

tui_radio() {
  local selected=$1
  [ "$TUI_HAS_UTF8" = "1" ] && printf "$(tui_c $([ "$selected" = 1 ] && echo "green" || echo "dim") '(')$(tui_c $([ "$selected" = 1 ] && echo "green" || echo "dim") '●' || echo ' ')$(tui_c $([ "$selected" = 1 ] && echo "green" || echo "dim") ')')" || printf '%s' "$([ "$selected" = 1 ] && echo "$TUI_RAD_ON" || echo "$TUI_RAD_OFF")"
}

# ─── Status Footer ──────────────────────────────────────────

tui_status_footer() {
  local w=$TUI_COLS keys=("${@}")
  local line=""
  for key in "${keys[@]}"; do
    line="${line}$(tui_h "[$key]")  "
  done
  printf '%s%*s%s\n' "$TUI_BOX_V" $((w-2)) "$TUI_BOX_V"
  printf '%s  %-*s %s\n' "$TUI_BOX_V" $((w-6)) "$line" "$TUI_BOX_V"
}

# ─── Colored Status Tag ─────────────────────────────────────

tui_status_tag() {
  local status="$1"
  case "$status" in
    done|ok|success)     printf '%s' "$(tui_c green_bold "$(tui_icon check) OK")" ;;
    fail|error|failed)   printf '%s' "$(tui_c red_bold "$(tui_icon fail) FAIL")" ;;
    warn*|warning)       printf '%s' "$(tui_c yellow_bold "$(tui_icon warn) $status")" ;;
    running|progress)    printf '%s' "$(tui_c yellow_bold "$(tui_icon play) $status")" ;;
    skip*)               printf '%s' "$(tui_c dim "$(tui_icon play) skip")" ;;
    pending|waiting)     printf '%s' "$(tui_c dim " .. ")" ;;
    *)                   printf '%s' "$(tui_c white "$status")" ;;
  esac
}

# ─── Bubble / Badge ────────────────────────────────────────

tui_badge() {
  local text="$1" color="${2:-dim}"
  printf ' %s ' "$(tui_c "$color" " $text ")"
}

# ─── Tag / Pill ─────────────────────────────────────────────

tui_pill() {
  local text="$1" color="${2:-green}"
  printf '%s' "$(tui_c "$color" " $text ")"
}
