use clap::Parser;

#[derive(Parser)]
#[command(
    name = "volt-display-manager",
    version = env!("CARGO_PKG_VERSION"),
    about = "VOLT Display Manager — Automatic display detection and adaptation for Samaris OS"
)]
struct Cli {
    #[arg(long)]
    status: bool,

    #[arg(long)]
    detect: bool,

    /// Detect, plan, apply, and persist (full cycle)
    #[arg(long)]
    apply: bool,

    /// Force safe mode (fallback config)
    #[arg(long)]
    safe: bool,

    /// Start hotplug watcher (long-running)
    #[arg(long)]
    watch: bool,

    /// Dump raw xrandr output + parsed data
    #[arg(long)]
    dump: bool,

    /// Print runtime generated TOML
    #[arg(long)]
    runtime: bool,

    /// Print user preferences TOML
    #[arg(long, default_value_t = false)]
    user_config: bool,

    /// Runtime config path (default: /run/samaris/display.generated.toml)
    #[arg(long, default_value = "/run/samaris/display.generated.toml")]
    config_path: String,

    /// Event notification path (default: /run/samaris/display.event.json)
    #[arg(long, default_value = "/run/samaris/display.event.json")]
    event_path: String,

    #[arg(long, short = 'V')]
    version: bool,
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_target(false)
        .init();

    let cli = Cli::parse();

    // Configure VDM paths
    volt_display_manager::set_runtime_config_path(cli.config_path.clone());
    volt_display_manager::set_event_path(cli.event_path.clone());

    let result = run(cli);
    if let Err(e) = result {
        eprintln!("VDM ERROR: {e}");
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    if cli.status {
        volt_display_manager::display::cli::print_status()?;
    }

    if cli.detect {
        let screens = volt_display_manager::detect()?;
        println!("{}", serde_json::to_string_pretty(&screens)?);
    }

    if cli.apply {
        match volt_display_manager::detect_and_apply() {
            Ok(config) => println!("{}", serde_json::to_string_pretty(&config)?),
            Err(e) => {
                eprintln!("Apply failed: {e}");
                eprintln!("Attempting safe mode fallback...");
                volt_display_manager::safe_mode()?;
            }
        }
    }

    if cli.safe {
        volt_display_manager::safe_mode()?;
        println!("Safe mode activated.");
    }

    if cli.watch {
        volt_display_manager::watch_hotplug()?;
    }

    if cli.dump {
        volt_display_manager::display::cli::print_dump()?;
    }

    if cli.runtime {
        volt_display_manager::display::cli::print_runtime()?;
    }

    if cli.user_config {
        volt_display_manager::display::cli::print_user_config()?;
    }

    if cli.version {
        println!("volt-display-manager v{}", env!("CARGO_PKG_VERSION"));
    }

    let any = cli.status || cli.detect || cli.apply || cli.safe || cli.watch || cli.dump || cli.runtime || cli.user_config || cli.version;
    if !any {
        volt_display_manager::display::cli::print_status()?;
    }

    Ok(())
}
