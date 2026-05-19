# Volt ASC Safety

## Safety Caps

Absolute maximum values that cannot be exceeded:

| Parameter | Cap |
|-----------|-----|
| Max workers | 48 |
| Orbit quota | 16 GB |
| Desktop quota | 512 MB |
| VRM cache | 4 GB |
| VUM cache | 4 GB |
| Min desktop workers | 1 |

## Invariants

1. `min_workers` > 0
2. Desktop quota >= 64 MB
3. No quota exceeds total RAM
4. Pressure policy: green < yellow < orange < red
5. `min_workers` <= `max_workers`

## Budget Safety

1. Total Samaris allocation never exceeds `samaris_budget_cap()`
2. Desktop is never sacrificed first
3. Override validation rejects dangerous values
4. Safe mode forces conservative caps
5. Fallback detections produce conservative budgets
