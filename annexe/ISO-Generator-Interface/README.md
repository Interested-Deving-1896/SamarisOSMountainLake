# SAMARIS ISO GENERATOR — Interface Design

## Apercu

Ce dossier contient les maquettes completes de l'interface utilisateur
du SAMARIS ISO GENERATOR, une TUI (Terminal User Interface) interactive
pour construire des ISOs Samaris OS "a la carte".

## Contenu

```
ISO-Generator-Interface/
├── index.html           # Visualiseur HTML interactif (tous les ecrans)
├── README.md            # Ce fichier
├── render.sh            # Script pour afficher un ecran dans le terminal
├── sample.conf          # Exemple de configuration sauvegardee
└── screens/             # Fichiers texte individuels par ecran
    ├── 01-splash.txt
    ├── 02-main-menu.txt
    ├── 03-build-config.txt
    ├── 04-live-dashboard.txt
    ├── 05-build-summary.txt
    ├── 06-status-dashboard.txt
    ├── 07-run-step.txt
    ├── 08-config-manager.txt
    ├── 09-clean-prompt.txt
    ├── 10-qemu-boot.txt
    ├── 11-env-check.txt
    └── 12-log-viewer.txt
```

## Les 12 ecrans

| No  | Ecran               | Description                                    |
|-----|----------------------|------------------------------------------------|
| 00  | Splash / Intro       | Auto-detection environnement, logo ASCII       |
| 01  | Main Menu            | Hub central : 7 actions + raccourcis           |
| 02  | Build Config         | Configuration "a la carte" complete            |
| 03  | Live Dashboard       | Construction en temps reel (2 arches)          |
| 04  | Build Summary        | Resultats, temps, cache hits, warnings          |
| 05  | Status Dashboard     | Checkpoints, validation, stockage              |
| 06  | Run Single Step      | Selection et execution d'une etape             |
| 07  | Config Manager       | Sauver/charger/supprimer des configurations    |
| 08  | Clean Prompt         | Nettoyage avec selection des elements          |
| 09  | QEMU Boot            | Options de demarrage dans QEMU                 |
| 10  | Env Check            | Verification des dependances de build          |
| 11  | Log Viewer           | Logs interactifs d'une etape                   |

## Utilisation

### Visualiseur HTML

Ouvrez `index.html` dans un navigateur pour parcourir tous les ecrans
avec navigation clavier et theme terminal.

### Raccourcis clavier (visualiseur HTML)

- `1` a `9`, `0` : Aller directement a un ecran
- `↑` / `↓` : Navigation sequentielle
- `Home` / `End` : Premier / dernier ecran
- Clic souris : Selection dans la barre laterale

### Affichage dans le terminal

```bash
./render.sh              # Affiche tous les ecrans (appuyez sur ESPACE)
./render.sh 03           # Affiche l'ecran 03 (build-config)
./render.sh splash       # Affiche l'ecran splash
```

## Design

- Boites de dialogue avec caracteres Unicode (╔═╗║╚╝)
- Fallback ASCII si le terminal ne supporte pas UTF-8
- Codes couleurs ANSI pour les statuts (vert/rouge/jaune)
- Barres de progression en caracteres █░
- Curseur clignotant sur les invites de saisie

## Prochaine etape

Implementation du moteur TUI dans `builder/ISOGenerator/lib/` :

- `07-tui.sh`    : Moteur de rendu (boites, couleurs, input)
- `08-config.sh` : Modele de configuration (save/load/calc)
- `09-menu.sh`   : Gestionnaires d'ecrans (render_*, handle_*)
