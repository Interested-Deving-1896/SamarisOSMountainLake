#!/usr/bin/env bash
# SAMARIS ISO GENERATOR — Terminal UI Renderer
# Affiche les maquettes d'ecran directement dans le terminal
#
# Usage:
#   ./render.sh              # Diaporama de tous les ecrans
#   ./render.sh 03           # Ecran 03 (build-config)
#   ./render.sh splash       # Ecran splash
#   ./render.sh list         # Liste les ecrans disponibles

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SCREENS_DIR="$SCRIPT_DIR/screens"

declare -A SCREEN_MAP
SCREEN_MAP["splash"]="01-splash.txt"
SCREEN_MAP["main-menu"]="02-main-menu.txt"
SCREEN_MAP["build-config"]="03-build-config.txt"
SCREEN_MAP["main"]="02-main-menu.txt"
SCREEN_MAP["live-dashboard"]="04-live-dashboard.txt"
SCREEN_MAP["dashboard"]="04-live-dashboard.txt"
SCREEN_MAP["live"]="04-live-dashboard.txt"
SCREEN_MAP["build-summary"]="05-build-summary.txt"
SCREEN_MAP["summary"]="05-build-summary.txt"
SCREEN_MAP["status-dashboard"]="06-status-dashboard.txt"
SCREEN_MAP["status"]="06-status-dashboard.txt"
SCREEN_MAP["run-step"]="07-run-step.txt"
SCREEN_MAP["run"]="07-run-step.txt"
SCREEN_MAP["step"]="07-run-step.txt"
SCREEN_MAP["config-manager"]="08-config-manager.txt"
SCREEN_MAP["config"]="08-config-manager.txt"
SCREEN_MAP["clean-prompt"]="09-clean-prompt.txt"
SCREEN_MAP["clean"]="09-clean-prompt.txt"
SCREEN_MAP["qemu-boot"]="10-qemu-boot.txt"
SCREEN_MAP["qemu"]="10-qemu-boot.txt"
SCREEN_MAP["env-check"]="11-env-check.txt"
SCREEN_MAP["check"]="11-env-check.txt"
SCREEN_MAP["env"]="11-env-check.txt"
SCREEN_MAP["log-viewer"]="12-log-viewer.txt"
SCREEN_MAP["log"]="12-log-viewer.txt"
SCREEN_MAP["logs"]="12-log-viewer.txt"

SCREEN_ORDER=(
  "splash" "main-menu" "build-config" "live-dashboard"
  "build-summary" "status-dashboard" "run-step" "config-manager"
  "clean-prompt" "qemu-boot" "env-check" "log-viewer"
)

# Couleurs ANSI
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
DIM='\033[2m'
BOLD='\033[1m'
NC='\033[0m'

has_color() { [ -t 1 ] && [ "$(tput colors 2>/dev/null || echo 0)" -ge 8 ]; }

list_screens() {
  echo -e "${BOLD}SAMARIS ISO GENERATOR — Ecrans disponibles${NC}"
  echo ""
  for i in "${!SCREEN_ORDER[@]}"; do
    local name="${SCREEN_ORDER[$i]}"
    local file="${SCREEN_MAP[$name]}"
    printf "  %02d  %-20s  %s\n" $((i + 1)) "$name" "$file"
  done
  echo ""
  echo "Usage: $0 [numero|nom]"
}

render_screen() {
  local file="$1"
  local screen_file="$SCREENS_DIR/$file"

  if [ ! -f "$screen_file" ]; then
    echo -e "${RED}Erreur: ecran '$file' introuvable${NC}" >&2
    echo "Cherche dans: $screen_file" >&2
    return 1
  fi

  local width
  width="$(tput cols 2>/dev/null || echo 80)"

  # Clear screen
  clear 2>/dev/null || printf '\033[2J\033[H'

  # Afficher le fichier
  if has_color; then
    # Colorisation basee sur le contenu
    sed \
      -e 's/\[OK\]/\x1b[32m[OK]\x1b[0m/g' \
      -e 's/\[>>\]/\x1b[33m[>>]\x1b[0m/g' \
      -e 's/\[\.\.\]/\x1b[2m[..]\x1b[0m/g' \
      -e 's/\[XX\]/\x1b[31m[XX]\x1b[0m/g' \
      -e 's/\[VV\]/\x1b[32m[VV]\x1b[0m/g' \
      -e 's/\[--\]/\x1b[2m[--]\x1b[0m/g' \
      -e 's/\[X\]/\x1b[33m[X]\x1b[0m/g' \
      -e 's/\[!\]/\x1b[33m[!]\x1b[0m/g' \
      -e 's/\[V\]/\x1b[32m[V]\x1b[0m/g' \
      -e 's/\(O\)/\x1b[33m(O)\x1b[0m/g' \
      -e 's/HIT/\x1b[32mHIT\x1b[0m/g' \
      -e 's/MISS/\x1b[31mMISS\x1b[0m/g' \
      -e 's/done/\x1b[32mdone\x1b[0m/g' \
      -e 's/running/\x1b[33mrunning\x1b[0m/g' \
      -e 's/pending/\x1b[2mpending\x1b[0m/g' \
      -e 's/skip/\x1b[2mskip\x1b[0m/g' \
      -e 's/SUCCES/\x1b[32mSUCCES\x1b[0m/g' \
      -e 's/AVERTISSEMENT/\x1b[33mAVERTISSEMENT\x1b[0m/g' \
      -e 's/ATTENTION/\x1b[31mATTENTION\x1b[0m/g' \
      "$screen_file"
  else
    cat "$screen_file"
  fi

  echo ""
  echo -e "${DIM}--- Appuyez sur ESPACE pour continuer, q pour quitter ---${NC}" >&2
}

# Parse arguments
if [ $# -eq 0 ]; then
  # Diaporama
  for name in "${SCREEN_ORDER[@]}"; do
    render_screen "${SCREEN_MAP[$name]}"
    read -r -s -n1 key
    if [ "$key" = "q" ]; then
      clear
      exit 0
    fi
  done
  clear
  echo -e "${GREEN}Fin du diaporama.${NC}"
  exit 0
fi

case "${1:-}" in
  list|ls|l)
    list_screens
    ;;
  help|-h|--help)
    sed -n '2,8p' "$0"
    ;;
  [0-9]|[0-9][0-9])
    idx=$((10#${1} - 1))
    if [ "$idx" -ge 0 ] && [ "$idx" -lt "${#SCREEN_ORDER[@]}" ]; then
      name="${SCREEN_ORDER[$idx]}"
      render_screen "${SCREEN_MAP[$name]}"
    else
      echo -e "${RED}Numero d'ecran invalide: $1 (1-${#SCREEN_ORDER[@]})${NC}" >&2
      exit 1
    fi
    ;;
  *)
    local name="${1,,}"  # lowercase
    if [ -n "${SCREEN_MAP[$name]:-}" ]; then
      render_screen "${SCREEN_MAP[$name]}"
    else
      # Try as filename
      if [ -f "$SCREENS_DIR/$name" ]; then
        render_screen "$name"
      elif [ -f "$SCREENS_DIR/$name.txt" ]; then
        render_screen "$name.txt"
      else
        echo -e "${RED}Ecran inconnu: $1${NC}" >&2
        echo "Utilisez '$0 list' pour voir les ecrans disponibles." >&2
        exit 1
      fi
    fi
    ;;
esac
