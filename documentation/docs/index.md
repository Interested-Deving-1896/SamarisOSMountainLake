<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="../assets/branding/samaris-logo.webp">
    <img src="../assets/branding/samaris-logo.webp" alt="Samaris OS" width="120">
  </picture>
</p>

<h1 align="center">Samaris OS Documentation</h1>

<p align="center">
  <a href="../README.md">README</a> ·
  <a href="../CHANGELOG.md">Changelog</a> ·
  <a href="quickstart.md">Quickstart</a> ·
  <a href="../ROADMAP.md">Roadmap</a> ·
  <a href="spec/README.md">VOLT Specification</a>
</p>

<br>

## Architecture

| | |
|---|---|
| System Stack | [Overview](architecture/overview.md) |
| Kernel A | [Node.js Kernel](architecture/kernel-node.md) |
| Kernel B | [Tesseract Engine (Rust)](architecture/kernel-rust.md) |
| Daemons | [Volt Daemons](architecture/volt-daemons.md) |
| Bridge | [Volt Unifier & SBP](architecture/volt-unifier.md) |
| AI | [AI Stack](architecture/ai-stack.md) |
| Shell | [Electron Shell](architecture/electron-shell.md) |
| Windowing | [Window System](architecture/window-system.md) |
| AirBar | [AirBar System Panel](architecture/airbar.md) |
| Filesystem | [Filesystem](architecture/filesystem.md) |
| Data Flow | [Data Flow](architecture/data-flow.md) |
| Security | [Security](architecture/security.md) |

## System

| | |
|---|---|
| ISO | [ISO Contents](system/iso-contents.md) |
| Drivers | [Driver Support](system/drivers.md) |
| AI Models | [AI Models](system/ai-models.md) |
| Services | [Systemd Services](system/systemd-services.md) |
| Boot Splash | [Boot Splash](system/boot-splash.md) |
| Session | [Desktop Session](system/desktop-session.md) |
| Layout | [Filesystem Layout](system/filesystem-layout.md) |
| Hardware | [Hardware Matrix](system/hardware-matrix.md) |
| Build | [Build System](system/build-system.md) |
| Plymouth | [Boot Splash Theme](system/boot-plymouth.md) |

## Applications

| | | | |
|---|---|---|---|
| [Orbit AI](apps/orbit.md) | [Finder](apps/finder.md) | [Peregrine](apps/peregrine.md) | [Terminal](apps/terminal.md) |
| [Settings](apps/settings.md) | [Network](apps/network.md) | [PDF Viewer](apps/pdf-viewer.md) | [Mail](apps/mail.md) |
| [Music](apps/music.md) | [Photos](apps/photos.md) | [Archive](apps/archive.md) | [App Store](apps/app-store.md) |
| [Notes](apps/notes.md) | [Videos](apps/videos.md) | [DOOM](apps/doom.md) | [Downloads](apps/downloads.md) |
| [Text Editor](apps/text-editor.md) | [About](apps/about.md) | [Encryption](apps/encryption.md) | [Firewall](apps/firewall.md) |
| [Permissions](apps/permissions-manager.md) | [Print](apps/print.md) | [Trash](apps/trash.md) | [Utilities](apps/utilities.md) |
| [Installed Web App](apps/installed-web-app.md) | [Others](apps/others.md) | | |

## Modules

### Daemons (Rust)

| | |
|---|---|
| VRM | [RAM Manager](modules/daemons/vrm.md) |
| VGM | [GPU Manager](modules/daemons/vgm.md) |
| VUM | [USB Manager](modules/daemons/vum.md) |
| DWP | [Dynamic Worker Pool](modules/daemons/dwp.md) |
| ASC | [Adaptive System Config](modules/daemons/asc.md) |
| Kernel B | [Tesseract Engine](modules/daemons/kernel-b.md) |
| VDM | [Display Manager](modules/daemons/vdm.md) |
| Bench | [Bench Rust Engine](modules/daemons/bench.md) |
| Audit | [Backend Audit](modules/daemons/backend-audit.md) |

### System (Desktop)

| | |
|---|---|
| UI | [Architecture](modules/system/ui-architecture.md) |
| Electron | [Services](modules/system/electron-services.md) |
| Theme | [Theme System](modules/system/theme-system.md) |
| Icons | [Desktop Icons](modules/system/desktop-icons.md) |
| Icons | [Samaris Icons](modules/system/samaris-icons.md) |
| Spotlight | [Search](modules/system/spotlight.md) |
| Audio | [Audio System](modules/system/audio-system.md) |
| Onboarding | [First Boot Wizard](modules/system/onboarding.md) |

### Benchmarks

| | |
|---|---|
| Bench | [Performance Suite](modules/bench.md) |

## APIs

| | |
|---|---|
| WebSocket | [Kernel WebSocket Protocol](apis/kernel-ws.md) |
| SBP | [SBP Binary Protocol](apis/sbp-protocol.md) |
| Filesystem | [Filesystem API](apis/fs-api.md) |
| IPC | [Electron IPC Reference](apis/ipc-electron.md) |
| Volt | [Volt Client API](apis/volt-client-api.md) |

## Configuration

| | |
|---|---|
| Kernel B | [Tesseract Engine](config/kernel-b.toml.md) |
| VRM | [RAM Manager](config/vrm.toml.md) |
| DWP | [Worker Pool](config/dwp.toml.md) |
| VGM | [GPU Manager](config/vgm.toml.md) |
| VUM | [USB Manager](config/vum.toml.md) |
| ASC | [Adaptive System Config](config/asc.toml.md) |
| Orbit | [AI Assistant](config/orbit-config.md) |

## Guides

| | |
|---|---|
| Start | [Quickstart](quickstart.md) |
| Start | [Getting Started](guides/getting-started.md) |
| Install | [Installing the ISO](guides/installing-iso.md) |
| Boot | [First Boot](guides/first-boot.md) |
| App | [Adding an App](guides/adding-an-app.md) |
| Style | [Styling Guide](guides/styling-guide.md) |
| Debug | [Debugging](guides/debugging.md) |
| Contribute | [Contributing](guides/contributing.md) |

## VOLT Specification

| | |
|---|---|
| README | [VOLT Overview](spec/README.md) |
| Chapters | [24 Chapters](spec/) — Foundation, Core Runtime, Subsystems, Desktop, Operations |

---

<p align="center">
  <a href="https://samaris.tech">samaris.tech</a> ·
  <a href="../README.md">README</a> ·
  contact.samaris.os@gmail.com
</p>
