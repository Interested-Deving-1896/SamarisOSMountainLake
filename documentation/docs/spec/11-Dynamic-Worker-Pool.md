# 11. Dynamic Worker Pool

## 11.1 Overview

The Volt Dynamic Worker Pool (DWP) is the cooperative adaptive priority scheduler of Samaris OS. Written in Rust as an independent daemon (`volt-dynamic-worker-pool`), it manages background computation across the system, ensuring that desktop responsiveness is never sacrificed for background task throughput. The DWP implements a fairness-guaranteed scheduling model with thermal backoff, desktop frame guarding, and burst support for AI inference workloads.

## 11.2 Scheduling Model

The DWP uses a **cooperative adaptive priority** scheduler with the following characteristics:

- **Cooperative**: workers yield voluntarily at configurable budget intervals (default: 50 µs)
- **Adaptive**: worker count scales dynamically based on system load, CPU utilisation, and thermal state
- **Priority-based**: tasks are scheduled strictly by priority level, with starvation prevention

### Priority Hierarchy

| Priority | Assigned To | Behaviour |
|----------|-------------|-----------|
| Critical | Desktop rendering, Orbit AI | Highest scheduling precedence |
| High | Electron window manager, Kernel A, Kernel B | Guaranteed execution within budget |
| Normal | VUM, VGM | Standard time-slicing |
| Idle | VRM, background tasks | Executed only when no higher-priority work is pending |

### Starvation Prevention

The fairness subsystem implements priority aging:

- Tasks waiting longer than 200 ms receive a priority boost
- Starvation limit at 1000 ms triggers guaranteed scheduling
- Aged tasks inherit scheduling from the next higher priority class

## 11.3 Adaptive Scaling

The worker count adjusts based on multiple signals:

| Signal | Scale Up Condition | Scale Down Condition |
|--------|-------------------|---------------------|
| Queue depth | Factor > 2.0× steady state | Factor < 1.0× steady state |
| CPU utilisation | > 80% | < 30% |
| Thermal pressure | Scale down on throttle | — |
| Desktop frame budget | Reduce background on frame pressure | Normal when frames healthy |

Scaling operates with a configurable cooldown (default: 5000 ms) to prevent oscillation.

## 11.4 Desktop Frame Guard

A critical safety mechanism that protects UI responsiveness:

- Monitors frame rendering budget (target: 16 ms for 60 FPS)
- Detects frame budget overruns (> 16 ms)
- On pressure: reduces Orbit AI burst allocation, throttles background tasks
- Latency guard triggers at 8 ms to preemptively reduce load
- Prevents background computation from causing visible UI stutter

## 11.5 Thermal Integration

The DWP responds to thermal pressure from the system:

| Thermal State | Response |
|---------------|----------|
| Normal | Full worker count, burst operations allowed |
| Elevated | Scale down worker count proportionally |
| High | Disable Orbit burst, reduce to minimum workers |
| Critical | Cooperative shutdown of non-essential tasks |

## 11.6 Orbit Burst Support

The DWP provides dedicated burst capacity for AI inference workloads:

- Default allocation: 75% of available workers to Orbit during burst
- Burst window: 100 ms with 2000 ms cooldown
- Maximum 3 consecutive bursts before mandatory cooldown
- Burst is deprioritised under desktop frame pressure
- Reduces AI inference latency during interactive assistant usage

## 11.7 Configuration

The DWP is configured via `/opt/volt/worker-pool/config.toml`. Key sections include:

- Scheduler mode, preemption, yield budget, idle timeout
- Scaling: mode (adaptive/fixed), queue thresholds, CPU thresholds, cooldown
- Hardware: core detection, RAM detection, thermal detection
- Reservations: desktop minimum workers, system minimum, Orbit burst parameters
- Desktop guard: frame budget, latency guard, pressure response
- Priorities: per-module priority assignment
- Fairness: aging interval, starvation limit, boost behaviour
- Thermal: backoff enablement, response triggers
- Adapter configuration for module integration
