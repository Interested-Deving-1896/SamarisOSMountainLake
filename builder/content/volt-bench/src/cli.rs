use clap::Parser;

#[derive(Parser, Clone, Debug)]
#[command(name = "bench", version, about = "Samaris OS Benchmark Suite")]
pub struct BenchCli {
    #[arg(long)]
    pub run: bool,
    #[arg(long)]
    pub quick: bool,
    #[arg(long)]
    pub full: bool,
    #[arg(long)]
    pub stress: bool,
    #[arg(long)]
    pub watch: bool,
    #[arg(long)]
    pub ci: bool,
    #[arg(long)]
    pub score: bool,
    #[arg(long)]
    pub system: bool,
    #[arg(long)]
    pub ui: bool,
    #[arg(long)]
    pub vrm: bool,
    #[arg(long)]
    pub vgm: bool,
    #[arg(long)]
    pub dwp: bool,
    #[arg(long)]
    pub vum: bool,
    #[arg(long)]
    pub orbit: bool,
    #[arg(long)]
    pub peregrine: bool,
    #[arg(long)]
    pub finder: bool,
    #[arg(long)]
    pub kernel: bool,
    #[arg(long, value_name = "FORMAT")]
    pub export: Option<String>,
    #[arg(long)]
    pub history: bool,
    #[arg(long)]
    pub latest: bool,
    #[arg(long)]
    pub compare: bool,
    #[arg(long, value_name = "FILE")]
    pub import_baseline: Option<String>,
    #[arg(long)]
    pub optimizer_export: bool,
    #[arg(long)]
    pub dump: bool,
}

impl BenchCli {
    pub fn mode(&self) -> &str {
        if self.quick { "quick" }
        else if self.full { "full" }
        else if self.stress { "stress" }
        else if self.watch { "watch" }
        else if self.ci { "ci" }
        else if self.run { "quick" }
        else { "none" }
    }
}
