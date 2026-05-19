use clap::Parser;
use tracing_subscriber::EnvFilter;

use volt_adaptive_system_config::runtime::cli::Cli;
use volt_adaptive_system_config::runtime::service::RuntimeService;

use volt_adaptive_system_config::core::result::AscResult;

fn main() -> AscResult<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_target(false)
        .init();

    let cli = Cli::parse();

    let config_path = cli.config.as_deref();

    let result: AscResult<()> = match cli.command_str() {
        Some("probe") | Some("--probe") => {
            let svc = RuntimeService::new(config_path)?;
            let profile = svc.probe()?;
            let json = serde_json::to_string_pretty(&profile)
                .map_err(|e| volt_adaptive_system_config::core::error::AscError::WriteFailed(e.to_string()))?;
            println!("{}", json);
            Ok(())
        }
        Some("generate") | Some("--generate") => {
            let svc = RuntimeService::new(config_path)?;
            let config = svc.generate()?;
            let toml = toml::to_string_pretty(&config)
                .map_err(|e| volt_adaptive_system_config::core::error::AscError::WriteFailed(e.to_string()))?;
            println!("{}", toml);
            Ok(())
        }
        Some("explain") | Some("--explain") => {
            let svc = RuntimeService::new(config_path)?;
            let _ = svc.generate()?;
            let report = svc.explain()?;
            println!("{}", report.render());
            Ok(())
        }
        Some("dry-run") | Some("--dry-run") => {
            let svc = RuntimeService::new(config_path)?;
            let config = svc.dry_run()?;
            let toml = toml::to_string_pretty(&config)
                .map_err(|e| volt_adaptive_system_config::core::error::AscError::WriteFailed(e.to_string()))?;
            println!("{}", toml);
            println!("# DRY-RUN: No files written.");
            Ok(())
        }
        Some("check") | Some("--check") => {
            let svc = RuntimeService::new(config_path)?;
            svc.check()?;
            println!("Config is valid.");
            Ok(())
        }
        Some("write") | Some("--write") => {
            let path = cli.write.as_deref();
            let svc = RuntimeService::new(config_path)?;
            svc.write_config(path)?;
            println!("Config written to: {}",
                path.unwrap_or(volt_adaptive_system_config::runtime::paths::DEFAULT_GENERATED_CONFIG_PATH));
            Ok(())
        }
        Some("version") | Some("--version") => {
            println!("Volt Adaptive System Configuration v{}", env!("CARGO_PKG_VERSION"));
            Ok(())
        }
        _ => {
            let svc = RuntimeService::new(config_path)?;
            let config = svc.generate()?;
            let toml = toml::to_string_pretty(&config)
                .map_err(|e| volt_adaptive_system_config::core::error::AscError::WriteFailed(e.to_string()))?;
            println!("{}", toml);
            Ok(())
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
    Ok(())
}
