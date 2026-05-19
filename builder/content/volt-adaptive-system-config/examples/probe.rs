fn main() {
    let profile = volt_adaptive_system_config::hardware::probe::HardwareProbe::new()
        .detect()
        .expect("Hardware probe failed");
    let json = serde_json::to_string_pretty(&profile).unwrap();
    println!("{}", json);
}
