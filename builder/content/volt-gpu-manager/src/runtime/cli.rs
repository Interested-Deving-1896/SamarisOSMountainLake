use clap::Parser;

#[derive(Debug, Parser)]
#[command(
    name = "volt-gpu-manager",
    version,
    about = "Samaris OS Volt GPU Manager — GPU Orchestration Layer"
)]
pub struct Cli {
    #[arg(
        long = "config",
        help = "Path to configuration file (TOML format)"
    )]
    pub config: Option<String>,

    #[arg(
        long = "check-config",
        help = "Load and validate configuration, then exit"
    )]
    pub check_config: bool,

    #[arg(
        long = "status",
        help = "Display GPU manager status and exit"
    )]
    pub status: bool,

    #[arg(
        long = "probe",
        help = "Detect available GPU backends and hardware"
    )]
    pub probe: bool,

    #[arg(
        long = "vram-status",
        help = "Display VRAM tier usage information"
    )]
    pub vram_status: bool,

    #[arg(
        long = "thermal-status",
        help = "Display thermal sensor and throttling state"
    )]
    pub thermal_status: bool,

    #[arg(
        long = "warmup-shaders",
        help = "Precompile shader cache entries and exit"
    )]
    pub warmup_shaders: bool,

    #[arg(
        long = "simulate-frame",
        help = "Run a simulated frame workload through the scheduler"
    )]
    pub simulate_frame: bool,

    #[arg(
        long = "simulate-compress-restore",
        help = "Run a T1->T2 compression and T2->T1 restore simulation"
    )]
    pub simulate_compress_restore: bool,

    #[arg(
        long = "metrics",
        help = "Collect and display current GPU metrics snapshot"
    )]
    pub metrics: bool,
}
