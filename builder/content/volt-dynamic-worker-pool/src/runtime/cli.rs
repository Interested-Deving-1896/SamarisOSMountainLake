use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "volt-dynamic-worker-pool")]
#[command(version = "2.0.0")]
pub struct Cli {
    #[arg(short, long, default_value = None)]
    pub config: Option<String>,

    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    pub command: Vec<String>,
}

impl Cli {
    pub fn command_str(&self) -> Option<&str> {
        self.command.first().map(|s| s.as_str())
    }

    pub fn is_simulate(&self, name: &str) -> bool {
        self.command_str().map_or(false, |c| {
            c == name || c == &format!("--{}", name)
        })
    }
}
