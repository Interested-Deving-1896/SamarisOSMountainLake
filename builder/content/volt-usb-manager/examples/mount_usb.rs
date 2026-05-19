use std::fs;
use std::path::Path;

use volt_usb_manager::config::loader::load_config;
use volt_usb_manager::config::schema::VumConfig;
use volt_usb_manager::core::manager::VoltUsbManager;
use volt_usb_manager::core::state::VumState;
use volt_usb_manager::runtime::service::RuntimeService;

const CONFIG_PATH: &str = "/etc/volt/usb-manager.toml";

fn create_example_config() -> VumConfig {
    let base = std::env::temp_dir().join("volt_mount_example");
    fs::create_dir_all(&base).ok();

    VumConfig {
        manager: volt_usb_manager::config::schema::ManagerConfig {
            mount_point: base.join("mnt").to_str().unwrap().to_string(),
            backing_path: base.join("backing").to_str().unwrap().to_string(),
            runtime_dir: base.join("run").to_str().unwrap().to_string(),
            state_dir: base.join("state").to_str().unwrap().to_string(),
            log_level: "info".to_string(),
            safe_mode: false,
        },
        ..VumConfig::default()
    }
}

fn main() {
    let config = if Path::new(CONFIG_PATH).exists() {
        load_config(CONFIG_PATH).unwrap_or_else(|_| {
            eprintln!("Warning: could not load {}, using example config", CONFIG_PATH);
            create_example_config()
        })
    } else {
        let config = create_example_config();
        println!("Using example config with mount point: {}", config.manager.mount_point);
        config
    };

    fs::create_dir_all(&config.manager.mount_point).expect("Cannot create mount point");
    fs::create_dir_all(&config.manager.backing_path).expect("Cannot create backing path");

    let mut mgr = VoltUsbManager::new(config.clone());
    mgr.init().expect("Failed to initialize Volt USB Manager");
    println!("Manager initialized: {:?}", mgr.state());

    if mgr.state() == VumState::ConfigLoaded || mgr.state() == VumState::Uninitialized {
        println!("Starting runtime service...");
        let service = RuntimeService::new(config).expect("Failed to create runtime service");
        service.run().expect("Runtime service exited with error");
    } else {
        eprintln!("Manager in unexpected state: {:?}", mgr.state());
    }
}
