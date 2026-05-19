use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "volt-usb-manager",
    about = "Samaris OS — Volt USB Manager — Journaled RAM-First Removable Storage Manager",
    version = env!("CARGO_PKG_VERSION"),
    long_version = env!("CARGO_PKG_VERSION"),
    disable_version_flag = true,
)]
pub struct Cli {
    #[arg(long, value_name = "FILE", help = "Path to configuration file")]
    pub config: Option<String>,

    #[arg(long, value_name = "FILE", help = "Load and validate configuration, then exit")]
    pub check_config: Option<String>,

    #[arg(long, help = "Print current status and exit")]
    pub status: bool,

    #[arg(long, help = "Mount and run the service")]
    pub mount: bool,

    #[arg(long, help = "Unmount the device")]
    pub unmount: bool,

    #[arg(long, help = "Flush write buffer to device")]
    pub flush: bool,

    #[arg(long, help = "Flush buffers and eject device")]
    pub eject: bool,

    #[arg(long, help = "Recover journal and replay records")]
    pub recover: bool,

    #[arg(long, help = "Simulate write operations in a temporary directory")]
    pub simulate_write: bool,

    #[arg(long, help = "Simulate recovery in a temporary directory")]
    pub simulate_recovery: bool,

    #[arg(long, help = "Print version information")]
    pub version: bool,
}

impl Cli {
    pub fn parse_args() -> Self {
        Cli::parse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_default() {
        let cli = Cli {
            config: None,
            check_config: None,
            status: false,
            mount: false,
            unmount: false,
            flush: false,
            eject: false,
            recover: false,
            simulate_write: false,
            simulate_recovery: false,
            version: false,
        };
        assert!(cli.config.is_none());
        assert!(!cli.mount);
    }
}
