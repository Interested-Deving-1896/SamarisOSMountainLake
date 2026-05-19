# Thermal Management

## Thermal States

| Level | Temperature | Behavior |
|-------|------------|----------|
| Unknown | N/A | Conservative defaults; no throttling |
| Normal | ≤70°C | Full performance |
| Warm | ≤75°C | Normal operation, no restrictions |
| Hot | ≤80°C | Idle priority paused |
| Throttle | ≤85°C | Idle paused, burst disabled |
| Critical | ≤90°C | Only Critical priority runs |
| Emergency | >95°C | All non-Critical stopped, CPU fallback |
| Fatal | >100°C | Immediate shutdown |

## ThermalPolicy

The `ThermalPolicy` struct controls behavior at each threshold:

- `backoff_at_80`: Throttle-back non-critical work at 80°C (default: `true`)
- `disable_orbit_burst_at_85`: Disable Orbit burst compute at 85°C (default: `true`)
- `desktop_only_at_95`: Only desktop rendering at 95°C (default: `true`)
- `cpu_fallback_at_100`: CPU fallback activation at 100°C (default: `true`)

## Priority Blocking by Thermal State

```rust
fn should_block_priority(state: &ThermalState, priority: GpuPriority) -> bool
```

| Thermal Level | Critical | High | Normal | Idle |
|--------------|----------|------|--------|------|
| Normal | No | No | No | No |
| Warm | No | No | No | No |
| Hot | No | No | No | Yes |
| Throttle | No | No | No | Yes |
| Critical | No | Yes | Yes | Yes |
| Emergency | Yes | Yes | Yes | Yes |
| Fatal | Yes | Yes | Yes | Yes |

## Backoff Behavior

`ThermalBackoff` implements a progressive backoff mechanism:
- Tracks consecutive thermal events
- Increases backoff interval exponentially (capped)
- Resets when temperature returns to Normal

## Backend Sensor Availability

- **Wgpu/Metal backend**: GPU temperature sensors vary by platform.
  On macOS, Metal provides thermal status via `MTLDevice`.
  On Windows/Linux with Vulkan, temperature may be unavailable.
- **Null backend**: Reports `ThermalLevel::Unknown` with temperature 0°C.
- **No sensor**: Returns `ThermalSensorUnavailable` error.

## Limitations

1. Temperature sensor availability depends on platform and GPU vendor
2. Without real sensors, the thermal system operates in Unknown state
3. CPU fallback at Emergency level requires CPU-based compression support
4. Polling interval is configurable but minimal interval is 100ms
