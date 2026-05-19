use volt_adaptive_system_config::config::schema::AscConfig;
use volt_adaptive_system_config::core::asc::VoltAsc;

fn main() {
    let config = AscConfig::default();
    let asc = VoltAsc::new(config);
    let _ = asc.full_pipeline().expect("Pipeline failed");
    let report = asc.explain().expect("Explain failed");
    println!("{}", report.render());
}
