# Volt ASC Architecture

```
┌──────────────┐
│   VOLT BOOT  │
└──────┬───────┘
       │ volt-asc --probe
       ▼
┌──────────────┐
│  Hardware    │  CPU, RAM, GPU, Storage, USB, VM, Battery
│  Probe       │  Best-effort Linux detection + fallbacks
└──────┬───────┘
       │ HardwareProfile
       ▼
┌──────────────┐
│  Classifier  │  MachineClass (cumulable): low_ram, laptop, vm, usb_boot...
└──────┬───────┘
       │ MachineClass[]
       ▼
┌──────────────┐
│  Policy      │  Kernel B, VRM, VUM, Worker Pool, Orbit, Desktop
│  Engine      │  Formula-based, testable, documented
└──────┬───────┘
       │ SystemBudget
       ▼
┌──────────────┐
│  Budget      │  samaris_budget_cap(), reconciliation, safety margin
│  Allocator   │  Desktop protected, Orbit reduced first
└──────┬───────┘
       │ SystemBudget (reconciled)
       ▼
┌──────────────┐
│  Config      │  GeneratedConfig → TOML sections for all modules
│  Generator   │  Kernel B, Worker Pool, VRM, VUM, Budget, ASC meta
└──────┬───────┘
       │ GeneratedConfig
       ▼
┌──────────────┐
│  Validator   │  Safety caps, invariants, constraints, conflicts
└──────┬───────┘
       │ Validated config
       ▼
┌──────────────┐
│  Explain     │  human-readable report with reasons for every decision
│  Report      │  Markdown output
└──────┬───────┘
       │
       ▼
/run/samaris/adaptive.generated.toml
/var/lib/samaris/asc/last-hardware-profile.json
/var/lib/samaris/asc/last-explain-report.md
       │
       ▼
Kernel B / VRM / VUM / DWP / Orbit / Desktop
```

## Module Groups

| Group | Files | Responsibility |
|-------|-------|----------------|
| `core/` | asc, state, lifecycle, error, result | Central orchestrator |
| `hardware/` | profile, probe, cpu, memory, gpu, storage, usb, vm, battery, thermal, confidence | Hardware detection |
| `classify/` | machine_class, profile_kind, boot_medium | Machine classification |
| `policies/` | kernel_b, vrm, vum, worker_pool, orbit, desktop, global_budget | Policy formulas |
| `budget/` | system_budget, ram_budget, cpu_budget, storage_budget, reconciliation | Budget allocation |
| `generator/` | generated_config, toml_writer, *config | Config generation |
| `validation/` | constraints, safety_caps, invariants, conflicts | Safety validation |
| `explain/` | report, decision, reason, diff | Explainable decisions |
| `profiles/` | balanced, powersave, performance, safe, debug, vm, usb_boot, low_ram | Profile modifiers |
| `config/` | schema, loader, overrides, validation | User configuration |
| `runtime/` | cli, service, paths | CLI and runtime |
