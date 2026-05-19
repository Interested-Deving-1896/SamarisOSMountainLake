use volt_usb_manager::core::manager::VoltUsbManager;
use volt_usb_manager::config::schema::VumConfig;
use volt_usb_manager::sbp_usb::message::{MessageFlags, SbpUsbMessage};
use volt_usb_manager::sbp_usb::opcode::SbpUsbOpcode;

fn main() {
    let config = VumConfig::default();
    let mgr = VoltUsbManager::new(config);

    let status_msg = SbpUsbMessage::new(SbpUsbOpcode::UsbStatus, 0, vec![]);
    println!("Sending SBP-USB status request...");
    let resp = mgr.handle_sbp(status_msg).expect("Failed to handle status request");

    println!("Response flags: {:?}", resp.message.flags);
    assert!(resp.message.flags.contains(MessageFlags::RESPONSE));
    assert!(!resp.message.flags.contains(MessageFlags::ERROR));

    let status_str = String::from_utf8_lossy(&resp.message.payload);
    println!("Status payload (JSON):");
    println!("{}", status_str);

    let heartbeat_msg = SbpUsbMessage::new(SbpUsbOpcode::UsbHeartbeat, 1, vec![]);
    let hb_resp = mgr.handle_sbp(heartbeat_msg).expect("Failed to handle heartbeat");
    assert!(hb_resp.message.flags.contains(MessageFlags::EVENT));
    println!("Heartbeat response received (event flag set)");

    let snapshot = mgr.snapshot();
    println!("Uptime: {}ms", snapshot.uptime_ms);
    println!("State: {:?}", snapshot.state);
    println!("Cache hits: {}", snapshot.cache_hit_count);
    println!("Pending writes: {}", snapshot.pending_write_count);
}
