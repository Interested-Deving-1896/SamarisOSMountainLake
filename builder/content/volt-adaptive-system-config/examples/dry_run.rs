use volt_adaptive_system_config::config::schema::AscConfig;
use volt_adaptive_system_config::core::asc::VoltAsc;

fn main() {
    let config = AscConfig::default();
    let asc = VoltAsc::new(config);
    let generated = asc.full_pipeline().expect("Pipeline failed");
    let toml = toml::to_string_pretty(&generated).unwrap();
    println!("{}", toml);
    println!("# DRY-RUN: No files written.");
}
