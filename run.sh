#!/usr/bin/env bash
set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
GENERATOR="$PROJECT_ROOT/builder/ISOGenerator/generator.sh"

usage() {
  cat <<'USAGE'
╔══════════════════════════════════════════════════════════════╗
║             SAMARIS OS — ISO Generator v2.0                 ║
║        Application terminale de génération ISO              ║
╠══════════════════════════════════════════════════════════════╣
║                                                              ║
║  UTILISATION :                                               ║
║    ./run.sh                    → Application terminale TUI   ║
║    ./run.sh help               → Cette aide                  ║
║                                                              ║
║  COMMANDES :                                                 ║
║                                                              ║
║    ./run.sh iso [options]      → Construction ISO complète   ║
║    ./run.sh check              → Vérifier l'environnement    ║
║    ./run.sh status [--validate]→ État des checkpoints        ║
║    ./run.sh steps              → Liste des étapes            ║
║    ./run.sh next               → Prochaine étape             ║
║    ./run.sh run STEP           → Exécuter une étape          ║
║    ./run.sh clean              → Nettoyer l'espace de travail║
║    ./run.sh build [--config]   → Build avec configuration    ║
║    ./run.sh tui                → Forcer le mode interactif   ║
║    ./run.sh qemu               → Booter l'ISO dans QEMU      ║
║    ./run.sh all                → check + iso + status        ║
║    ./run.sh app                → Dev UI Electron (hors ISO)  ║
║                                                              ║
║  OPTIONS :                                                   ║
║    --docker       → Construire dans Docker (recommandé Mac)  ║
║    --from STEP    → Reprendre à partir d'une étape           ║
║    --only STEP    → Exécuter une seule étape                 ║
║    --force        → Ignorer les checkpoints                  ║
║    --config FILE  → Charger un fichier de configuration      ║
║    --dry-run      → Afficher sans exécuter les étapes        ║
║                                                              ║
║  EXEMPLES :                                                  ║
║    ./run.sh iso --docker --config \                         ║
║      builder/configs/minimal.conf                            ║
║    ./run.sh run 05-packages --docker --force                 ║
║    ./run.sh status --validate                                ║
║                                                              ║
║  ÉTAPES DU PIPELINE (25) :                                   ║
║    ./run.sh steps             → Affiche toutes les étapes    ║
║                                                              ║
╚══════════════════════════════════════════════════════════════╝
USAGE
}

# --- Détection du mode interactif ---
if [ "$#" -eq 0 ]; then
  if [ -t 0 ] && [ -t 1 ]; then
    exec "$GENERATOR" tui
  else
    usage
    exit 0
  fi
fi

cmd="$1"
shift 2>/dev/null || true

passthrough() {
  exec "$GENERATOR" "$cmd" "$@"
}

case "$cmd" in
  help|-h|--help)
    usage
    ;;
  iso|check|status|steps|next|run|clean|qemu|build|tui)
    passthrough "$@"
    ;;
  list)
    exec "$GENERATOR" steps "$@"
    ;;
  all)
    echo ""
    if [ -t 1 ] && command -v tput >/dev/null 2>&1; then
      bold="$(tput bold)"; reset="$(tput sgr0)"; green="$(tput setaf 2)"
    else
      bold=""; reset=""; green=""
    fi
    echo "  ${bold}Samaris OS — Full Pipeline${reset}"
    echo "  ─────────────────────────────────"
    echo ""
    echo "  [1/3] Vérification de l'environnement..."
    "$GENERATOR" check "$@" || { echo "  ❌ CHECK FAILED"; exit 1; }
    echo ""
    echo "  [2/3] Construction de l'ISO..."
    "$GENERATOR" iso "$@" || { echo "  ❌ BUILD FAILED"; exit 1; }
    echo ""
    echo "  [3/3] Résumé de la construction..."
    "$GENERATOR" status "$@"
    echo ""
    echo "  ${green}✅ Pipeline terminé${reset}"
    ;;
  app)
    cd "$PROJECT_ROOT/builder/content/ui"
    if [ ! -d node_modules ]; then npm install; fi
    npm run dev &
    ui_pid=$!
    trap 'kill "$ui_pid" 2>/dev/null || true' EXIT
    cd "$PROJECT_ROOT/builder/content/electron"
    if [ ! -d node_modules ]; then npm install; fi
    NODE_ENV=development npm start
    ;;
  *)
    echo "❌ Commande inconnue : $cmd" >&2
    echo "   Tapez './run.sh help' pour voir les commandes disponibles." >&2
    exit 64
    ;;
esac
