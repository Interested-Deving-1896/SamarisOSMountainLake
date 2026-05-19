# Samaris OS

An experimental sovereign computing platform exploring calm, local-first, portable personal computing.

## What it is

Samaris OS is a real bootable operating environment built on Linux with:
- Custom desktop shell rendered through Chromium kiosk mode
- Native Node.js services for system orchestration
- React-based UI system with persistent sessions
- Wine compatibility layer prototype
- GitHub-powered App Store prototype
- Experimental local-first AI interface (Orbit AI)

## Current State

**Samaris OS 1.0 Mountain Lake Alpha One**

- Public Alpha — experimental, VM-tested primarily
- x86_64 only (ARM64 in development)
- Hardware compatibility validation ongoing
- Encryption architecture implemented but temporarily disabled

This is NOT:
- A production operating system
- A Windows/macOS replacement
- A hardened security platform
- Enterprise-ready

## Architecture

```
BIOS/UEFI → GRUB → Linux Kernel → Node.js Services → Chromium → React Desktop
```

- Linux: System foundation
- Node.js: Native orchestration, persistence, filesystem access
- Chromium: GPU-accelerated rendering layer
- React: Desktop shell (dock, AirBar, Finder, windows, login)

## Positioning

"An experimental sovereign computing platform exploring calm, local-first, portable personal computing."

## License

© 2026 Khaled Ben Taieb.  
Licensed under the GNU General Public License v3.0.

See the [LICENSE](LICENSE) file for details.

## Development

```bash
npm install
npm run dev
```

## Pages

- **/** — Home
- **/software** — What's inside, architecture
- **/download** — Download Alpha One
- **/license** — GPLv3 license
- **/faq** — Frequently asked
- **/business** — Business inquiries

## Contact

contact.samaris.os@gmail.com