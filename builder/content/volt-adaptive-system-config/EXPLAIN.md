# Volt ASC Explain Report

The explain report is a Markdown document showing every detected hardware parameter, machine classification, generated decision, and the reason behind it.

## Format

```markdown
# Volt ASC Explain Report

## Detected Hardware
- CPU cores: 8
- CPU arch: x86_64
- RAM total: 8192 MB
- ...

## Machine Classes
- standard_laptop
- battery_powered
- thermal_sensitive

## Generated Decisions
| Parameter | Value | Reason |
|---|---|---|
| Kernel B workers | 6 | 8 CPU cores → 75% → 6 workers |
| DWP min workers | 4 | 8/3 ≈ 2.67, max(2, 3), min(12) = 4 |
| DWP max workers | 10 | 8*3/4 = 6, max(4, 6), min(48) = 10 |

## Warnings
- GPU not detected → fallback used, confidence: 0.0
```

## Clamping

If a value exceeds safety caps, it's clamped and reported:
```
Desktop quota: 1024 MB → clamped to 512 MB (safety cap exceeded)
```
