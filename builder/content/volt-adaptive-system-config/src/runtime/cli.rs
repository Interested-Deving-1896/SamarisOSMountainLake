use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "volt-asc")]
#[command(version = "1.0.0")]
#[command(about = "Volt Adaptive System Configuration — Hardware-Aware Policy Compiler for Samaris OS")]
pub struct Cli {
    #[arg(short, long, default_value = None)]
    pub config: Option<String>,

    #[arg(short, long, default_value = None)]
    pub write: Option<String>,

    #[arg(long, default_value = None)]
    pub profile: Option<String>,

    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    pub command: Vec<String>,
}

impl Cli {
    pub fn command_str(&self) -> Option<&str> {
        self.command.first().map(|s| s.as_str())
    }

    pub fn is_command(&self, name: &str) -> bool {
        self.command_str().map_or(false, |c| c == name || c == &format!("--{}", name))
    }
}
