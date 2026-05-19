use crate::config::schema::{
    AdaptersSection, DesktopGuardSection, FairnessSection, HardwareSection, MetricsSection,
    PrioritiesSection, ReservationsSection, ScalingSection, ThermalSection, WorkerPoolConfig,
    WorkerPoolSection,
};

pub fn default_config() -> WorkerPoolConfig {
    WorkerPoolConfig {
        worker_pool: WorkerPoolSection {
            scheduler: "cooperative_adaptive_priority".into(),
            preemption_enabled: true,
            yield_budget_us: 50,
            idle_timeout_ms: 5000,
            max_workers_cap: 48,
            safe_mode: true,
            integration_mode: "standalone".into(),
            scaling: ScalingSection {
                mode: "adaptive".into(),
                scale_up_queue_factor: 2.0,
                scale_down_queue_factor: 1.0,
                scale_up_cpu_threshold: 0.8,
                scale_down_cpu_threshold: 0.3,
                scale_cooldown_ms: 5000,
                min_workers_override: None,
                max_workers_override: None,
            },
            hardware: HardwareSection {
                detect_cpu_cores: true,
                detect_ram: true,
                detect_thermal: true,
                default_cpu_cores: 4,
            },
            reservations: ReservationsSection {
                desktop_min_workers: 1,
                system_min_workers: 1,
                orbit_default_fraction: 0.75,
                orbit_burst_window_ms: 100,
                orbit_burst_cooldown_ms: 2000,
                orbit_max_consecutive_bursts: 3,
            },
            desktop_guard: DesktopGuardSection {
                enabled: true,
                frame_budget_ms: 16,
                latency_guard_ms: 8,
                reduce_orbit_on_frame_pressure: true,
                reduce_background_on_frame_pressure: true,
            },
            priorities: PrioritiesSection {
                orbit: "critical".into(),
                desktop: "high".into(),
                electron: "normal".into(),
                kernel_a: "high".into(),
                kernel_b: "high".into(),
                vrm: "idle".into(),
                vum: "normal".into(),
                vgm: "normal".into(),
                background: "idle".into(),
            },
            fairness: FairnessSection {
                aging_enabled: true,
                aging_after_ms: 200,
                starvation_limit_ms: 1000,
                priority_boost_on_starvation: true,
            },
            thermal: ThermalSection {
                thermal_backoff_enabled: true,
                scale_down_on_thermal_pressure: true,
                disable_orbit_burst_on_thermal_pressure: true,
            },
        },
        adapters: AdaptersSection {
            enabled: true,
            kernel_b: "stub".into(),
            kernel_a: "stub".into(),
            desktop: "stub".into(),
            orbit: "stub".into(),
            vrm: "stub".into(),
            vum: "stub".into(),
            vgm: "stub".into(),
        },
        metrics: MetricsSection {
            enabled: true,
            latency_histograms: true,
            utilization_tracking: true,
            queue_tracking: true,
        },
    }
}
