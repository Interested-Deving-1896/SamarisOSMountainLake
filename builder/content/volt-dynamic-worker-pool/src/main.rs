use std::sync::Arc;

use clap::Parser;
use tracing_subscriber::EnvFilter;

use volt_dynamic_worker_pool::config::loader::load_config;
use volt_dynamic_worker_pool::config::validation::validate_config;
use volt_dynamic_worker_pool::core::pool::DynamicWorkerPool;
use volt_dynamic_worker_pool::job::job::Job;
use volt_dynamic_worker_pool::job::job_id::JobId;
use volt_dynamic_worker_pool::priority::level::PriorityLevel;
use volt_dynamic_worker_pool::runtime::cli::Cli;
use volt_dynamic_worker_pool::runtime::service::RuntimeService;
use volt_dynamic_worker_pool::runtime::signal::SignalHandler;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_target(false)
        .init();

    let cli = Cli::parse();

    match cli.command_str() {
        Some("check-config") | Some("--check-config") => {
            let config = load_config(cli.config.as_deref()).expect("Failed to load config");
            validate_config(&config).expect("Invalid config");
            println!("Config is valid.");
        }
        Some("status") | Some("--status") => {
            let config = load_config(cli.config.as_deref()).expect("Failed to load config");
            let pool = DynamicWorkerPool::new(config);
            let _ = pool.start();
            let metrics = pool.metrics();
            println!("{:#?}", metrics);
        }
        Some("simulate-load") | Some("--simulate-load") => {
            simulate_load(cli.config.as_deref());
        }
        Some("simulate-orbit-burst") | Some("--simulate-orbit-burst") => {
            simulate_orbit_burst(cli.config.as_deref());
        }
        Some("simulate-desktop-pressure") | Some("--simulate-desktop-pressure") => {
            simulate_desktop_pressure(cli.config.as_deref());
        }
        Some("simulate-scaling") | Some("--simulate-scaling") => {
            simulate_scaling(cli.config.as_deref());
        }
        Some("simulate-adapters") | Some("--simulate-adapters") => {
            simulate_adapters(cli.config.as_deref());
        }
        Some("integration-plan") | Some("--integration-plan") => {
            print_integration_plan();
        }
        Some("version") | Some("--version") => {
            println!("Volt Dynamic Worker Pool v{}", env!("CARGO_PKG_VERSION"));
        }
        Some("run") | Some("daemon") | Some("--daemon") => {
            run_daemon(cli.config.as_deref());
        }
        _ => {
            let config = load_config(cli.config.as_deref()).expect("Failed to load config");
            let pool = DynamicWorkerPool::new(config);
            let _ = pool.start();
            println!("Volt Dynamic Worker Pool v{} running", env!("CARGO_PKG_VERSION"));
            println!("State: {:?}", pool.state());
            println!("Workers: {} (min) / {} (max)",
                pool.hardware().min_workers,
                pool.hardware().max_workers);
            let _ = pool.shutdown();
        }
    }
}

fn run_daemon(config_path: Option<&str>) {
    let config = load_config(config_path).expect("Failed to load config");
    let pool = Arc::new(DynamicWorkerPool::new(config.clone()));
    let service = RuntimeService::new(config, pool);
    let signal = SignalHandler::new();
    println!("Volt Dynamic Worker Pool v{} daemon starting", env!("CARGO_PKG_VERSION"));
    service.run(&signal).expect("Worker Pool daemon failed");
    println!("Volt Dynamic Worker Pool daemon stopped");
}

fn simulate_load(config_path: Option<&str>) {
    let config = load_config(config_path).expect("Failed to load config");
    let pool = DynamicWorkerPool::new(config);
    pool.start().expect("Failed to start pool");

    println!("Submitting 100 Normal jobs...");
    for i in 0..100 {
        let job = Job::new(JobId::new(), format!("load-job-{}", i), PriorityLevel::Normal, 1024);
        let _ = pool.submit_job(job);
    }
    println!("Submitting 10 Idle jobs...");
    for i in 0..10 {
        let job = Job::new(JobId::new(), format!("idle-job-{}", i), PriorityLevel::Low, 512);
        let _ = pool.submit_job(job);
    }

    let metrics = pool.metrics();
    println!("Metrics after load:");
    println!("  Submitted: {}", metrics.total_jobs_submitted);
    println!("  Queue depth: {}", metrics.queue_depth);
    println!("  Active workers: {}", metrics.active_workers);
    println!("  Idle workers: {}", metrics.idle_workers);
    pool.shutdown().expect("Failed to shutdown");
}

fn simulate_orbit_burst(config_path: Option<&str>) {
    let config = load_config(config_path).expect("Failed to load config");
    let pool = DynamicWorkerPool::new(config);
    pool.start().expect("Failed to start pool");

    use volt_dynamic_worker_pool::orbit::burst_controller::OrbitBurstRequest;
    let req = OrbitBurstRequest::new("orbit-infer-1".into(), 3, 300, "inference".into());
    let decision = pool.request_orbit_burst(req);
    println!("Orbit burst decision: {:?}", decision);

    let metrics = pool.metrics();
    println!("Orbit bursts: {}", metrics.orbit_burst_count);
    pool.shutdown().expect("Failed to shutdown");
}

fn simulate_desktop_pressure(config_path: Option<&str>) {
    let config = load_config(config_path).expect("Failed to load config");
    let pool = DynamicWorkerPool::new(config);
    pool.start().expect("Failed to start pool");

    println!("Setting desktop pressure to 0.85 (High)...");
    pool.set_desktop_pressure(0.85);

    let metrics = pool.metrics();
    println!("Desktop pressure: {}", metrics.desktop_pressure);
    println!("Yield count: {}", metrics.yield_count);

    pool.shutdown().expect("Failed to shutdown");
}

fn simulate_scaling(config_path: Option<&str>) {
    let config = load_config(config_path).expect("Failed to load config");
    let pool = DynamicWorkerPool::new(config);
    pool.start().expect("Failed to start pool");

    println!("Hardware profile:");
    println!("  Min workers: {}", pool.hardware().min_workers);
    println!("  Max workers: {}", pool.hardware().max_workers);
    println!("  CPU cores: {}", pool.hardware().cpu_cores);

    pool.shutdown().expect("Failed to shutdown");
}

fn simulate_adapters(config_path: Option<&str>) {
    let config = load_config(config_path).expect("Failed to load config");
    let pool = DynamicWorkerPool::new(config);
    pool.start().expect("Failed to start pool");

    use volt_dynamic_worker_pool::modules::module_id::ModuleId;
    use volt_dynamic_worker_pool::modules::module_profile::ModuleProfile;
    use volt_dynamic_worker_pool::priority::level::PriorityLevel;

    let modules = vec![
        ("kernel_b", PriorityLevel::High),
        ("kernel_a", PriorityLevel::Normal),
        ("desktop", PriorityLevel::High),
        ("orbit", PriorityLevel::Critical),
        ("vrm", PriorityLevel::Low),
        ("vum", PriorityLevel::Normal),
        ("vgm", PriorityLevel::Normal),
        ("background", PriorityLevel::Low),
    ];

    for (name, pri) in modules {
        let profile = ModuleProfile::new(ModuleId::new(name), pri);
        match pool.register_module(profile) {
            Ok(()) => println!("Registered adapter: {}", name),
            Err(e) => println!("Failed to register {}: {}", name, e),
        }
    }

    pool.shutdown().expect("Failed to shutdown");
}

fn print_integration_plan() {
    println!("=== Volt Dynamic Worker Pool — Integration Plan ===");
    println!();
    println!("PHASE 1 — Standalone (current)");
    println!("  - Pool runs independently");
    println!("  - CLI simulations work");
    println!("  - No Samaris module integration yet");
    println!();
    println!("PHASE 2 — Adapter-Ready");
    println!("  - Enable 'adapters' feature");
    println!("  - Each Samaris module gets a WorkerPoolAdapter stub");
    println!("  - API contracts are verified via adapter_contract_tests");
    println!();
    println!("PHASE 3 — Partial Integration");
    println!("  - Kernel B / Tesseract Engine connects via KernelBAdapter");
    println!("  - Desktop sends frame_pressure signals via DesktopAdapter");
    println!("  - Orbit submits inference jobs via OrbitAdapter");
    println!("  - VRM compressions use pool (optional, VUM/VGM follow)");
    println!();
    println!("PHASE 4 — Full Runtime Integration");
    println!("  - volt-dynamic-worker-pool becomes the central scheduler");
    println!("  - All Samaris modules use pool for all async work");
    println!("  - Integration mode switches to 'full_runtime'");
    println!("  - Old per-module thread pools are deprecated");
    println!();
    println!("Module-specific integration:");
    println!("  Orbit  → submit Critical jobs via request_orbit_burst()");
    println!("  Desktop → send frame_pressure via set_desktop_pressure()");
    println!("  VRM    → submit Idle compression/dedup jobs");
    println!("  VUM    → submit Normal flush/journal jobs");
    println!("  VGM    → submit Normal GPU-helper jobs");
    println!("  Kernel B → use pool as compute scheduler supervisor");
    println!("  Kernel A → submit Normal Electron/Kernel A tasks");
    println!("  Background → submit Idle maintenance jobs");
}
