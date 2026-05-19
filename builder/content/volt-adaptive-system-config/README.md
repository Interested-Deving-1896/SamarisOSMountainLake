# Volt Adaptive System Configuration v1.0

**Hardware-Aware Policy Compiler for Samaris OS**

One OS. All hardware. Zero required manual tuning.

## Role

Volt ASC is the hardware calibration layer of Samaris OS. At boot:

1. **Detect** hardware (CPU, RAM, GPU, storage, USB, VM, laptop/battery)
2. **Classify** the machine type (desktop, laptop, server, VM, USB boot, low RAM)
3. **Compute** safe system budgets for all Samaris modules
4. **Generate** a coherent configuration for Kernel B, VRM, VUM, DWP, Orbit, Desktop
5. **Validate** safety constraints and budget caps
6. **Explain** every decision in a readable report

## Build

```bash
cargo build --release
```

## Run

```bash
# Probe hardware and show profile
./target/release/volt-asc probe

# Generate full config
./target/release/volt-asc generate

# Explain all decisions
./target/release/volt-asc explain

# Dry-run (show config without writing)
./target/release/volt-asc dry-run

# Validate config
./target/release/volt-asc check

# Write generated config to disk
./target/release/volt-asc write

# Force a profile
./target/release/volt-asc --profile safe generate
```

## Outputs

| File | Path |
|------|------|
| Generated config | `/run/samaris/adaptive.generated.toml` |
| Hardware profile | `/var/lib/samaris/asc/last-hardware-profile.json` |
| Explain report | `/var/lib/samaris/asc/last-explain-report.md` |
