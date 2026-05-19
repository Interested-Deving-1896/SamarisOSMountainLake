# Backend Audit — Automated Service Verification

**Structural audit framework for Samaris OS Kernel A services, handlers, and inter-module communication.**

Backend Audit is a Node.js-based verification tool that systematically tests every service, handler, core module, and protocol bridge in the Kernel A ecosystem. It is not a unit test runner — it is an integration audit that validates structure, contract adherence, error handling, and cross-module consistency.

<br>

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Backend Audit Runner                       │
│                                                              │
│  core/     — Mock factory, shared test utilities             │
│  auditors/ — Per-module audit suites                         │
│  reporters/— Markdown + JSON report generation               │
│  config.js — Paths, iteration counts, thresholds             │
│  engine/   — Test lifecycle, warmup, iteration control       │
└─────────────────────────────────────────────────────────────┘
```

<br>

## Audit Categories

### Core Modules

| Auditor | Module | What It Validates |
|---------|--------|-------------------|
| `Auth` | `core/auth.js` | Session creation, validation, expiry, token format |
| `EventBus` | `core/eventBus.js` | Subscription, emission, unsubscribe, wildcard patterns |
| `Scheduler` | `core/scheduler.js` | Task scheduling, prioritisation, cancellation, error handling |

### Models

| Auditor | What It Validates |
|---------|-------------------|
| `Models` | Schema validation, required fields, type checks, defaults |

### Services (31 auditors)

| Service | File |
|---------|------|
| PermissionManager | `services/permissionManager.js` |
| FileSystemService | `services/fileSystem.js` |
| VaultService | `services/vaultService.js` |
| UserService | `services/userService.js` |
| ArchiveService | `services/archiveService.js` |
| MediaService | `services/mediaService.js` |
| AudioService | `services/audioService.js` |
| BatteryService | `services/batteryService.js` |
| NetworkService | `services/networkService.js` |
| PowerService | `services/powerService.js` |
| SystemMetricsService | `services/systemMetricsService.js` |
| ProcessManager | `services/processManager.js` |
| RuntimeManager | `services/runtimeManager.js` |
| WindowManager | `services/windowManager.js` |
| DiskService | `services/diskService.js` |
| FirewallService | `services/firewallService.js` |
| PrintService | `services/printService.js` |
| SessionFeaturesService | `services/sessionFeaturesService.js` |
| SearchService | `services/searchService.js` |
| DevStateService | `services/devStateService.js` |
| StorageService | `services/storageService.js` |
| WineService | `services/wineService.js` |
| BrowserService | `services/browserService.js` |
| KernelBClient | `services/kernelBClient.js` |
| EncryptionService | `services/encryptionService.js` |
| AppStoreService | `services/appStoreService.js` |
| OrbitRuntimeService | `services/orbitRuntime.js` |
| TTSService | `services/ttsService.js` |
| STTService | `services/sttService.js` |
| ConnectivityService | `services/connectivityService.js` |
| MailService | `services/mailService.js` |

### Handlers (27 auditors)

| Auditor | Handler | Domain |
|---------|---------|--------|
| system | `handlers/system.js` | OS, kernel, hardware queries |
| fs | `handlers/fs.js` | Read/write/delete, directory listing |
| audio | `handlers/audio.js` | Stream control, volume, metadata |
| battery | `handlers/battery.js` | Charge level, status, estimated time |
| archive | `handlers/archive.js` | Create, extract, inspect archives |
| media | `handlers/media.js` | Audio/video/image metadata extraction |
| user | `handlers/user.js` | User accounts, settings, preferences |
| session | `handlers/session.js` | User session lifecycle |
| device | `handlers/device.js` | Connected peripherals, hardware inventory |
| display | `handlers/display.js` | Screen configuration queries |
| power | `handlers/power.js` | Shutdown, reboot, suspend |
| network | `handlers/network.js` | WiFi scanning, connection management |
| app | `handlers/app.js` | Launch, suspend, resume, terminate |
| runtime | `handlers/runtime.js` | Service start/stop/restart |
| process | `handlers/process.js` | Process listing, termination, resource usage |
| search | `handlers/search.js` | Full-text search |
| firewall | `handlers/firewall.js` | Rules, ports, network filtering |
| disk | `handlers/disk.js` | Partition info, usage statistics |
| storage | `handlers/storage.js` | Block device and mount management |
| print | `handlers/print.js` | Print job management |
| onboarding | `handlers/onboarding.js` | First-boot setup wizard |
| encryption | `handlers/encryption.js` | File/volume encryption management |
| tts | `handlers/tts.js` | TTS synthesis via OuteTTS |
| stt | `handlers/stt.js` | Audio transcription via Whisper |
| wine | `handlers/wine.js` | Windows app execution via Wine |
| orbit | `handlers/orbit.js` | LLM inference requests, context management |
| browser | `handlers/browser.js` | Peregrine browser integration |

### Volt Unifier (10 auditors)

| Auditor | What It Validates |
|---------|-------------------|
| Unifier Constants | Opcode definitions, magic bytes, flag constants |
| SBP Message | Message encoding/decoding, checksums, integrity |
| SBP Router | Message routing, request-response correlation, timeouts |
| ModuleRegistry | Module registration, status tracking, heartbeat |
| Unifier EventBus | Internal event pub-sub, bridge events |
| CapabilityGuard | Permission enforcement, opcode authorisation |
| Unifier Lifecycle | Initialisation, shutdown, error recovery |
| Unifier Bridges | Bridge registration, message forwarding, health |
| Unifier Health | Health check polling, degradation detection |
| Unifier Metrics | Histogram collection, snapshot generation |

### Cross-Cutting Audits

| Auditor | What It Validates |
|---------|-------------------|
| **Kernel B** | SBP v5 protocol adherence, scheduler correctness, thermal watchdog |
| **Inter-Kernel** | Kernel A ↔ Kernel B communication, dual-protocol IPC detection |
| **SBP v5 Protocol** | Frame structure, opcode range, checksum, FlatBuffers serialisation |
| **CPU & Memory Profiling** | Module initialisation time, memory footprint, async overhead |
| **Integration** | End-to-end: FS write/read cycle, permission/auth flow, vault encrypt/decrypt, router dispatch |

<br>

## Report Format

After running, Backend Audit produces two reports in `results/`:

1. **Markdown report** — Human-readable with per-service tables, pass/fail counts, and summary
2. **JSON report** — Machine-readable for CI integration

Example output:
```
── Services ──

   PermissionManager... 12 tests · 12✅ 0❌ 0⚠️ (45ms)
   FileSystemService... 8 tests · 8✅ 0❌ 0⚠️ (32ms)
   ...

══ AUDIT COMPLETE ══

   Duration: 12.4s
   Total:    312 tests
   ✅ Passed: 308 (98.7%)
   ❌ Failed: 4
   ⚠️ Skipped: 0
   Score:    98/100
```

<br>

## Usage

```bash
# Run full audit
cd builder/content/backend-audit
node index.js

# Custom kernel root
KERNEL_ROOT=/opt/volt/kernel node index.js

# Configure iterations/timeouts
# Edit config.js:
#   defaultIterations: 5,
#   defaultWarmup: 2,
#   timeoutMs: 15000
```
