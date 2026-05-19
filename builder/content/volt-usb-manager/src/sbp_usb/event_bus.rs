use crossbeam::channel::{unbounded, Receiver, Sender};

use crate::core::error::VumError;
use crate::core::result::VumResult;
use crate::sbp_usb::message::SbpUsbMessage;

pub struct SbpUsbEventBus {
    tx: Sender<SbpUsbMessage>,
    rx: Receiver<SbpUsbMessage>,
}

impl SbpUsbEventBus {
    pub fn new() -> Self {
        let (tx, rx) = unbounded();
        SbpUsbEventBus { tx, rx }
    }

    pub fn publish(&self, msg: SbpUsbMessage) -> VumResult<()> {
        self.tx.send(msg).map_err(|e| {
            VumError::InternalInvariantViolation(format!(
                "event bus send failed: {}",
                e
            ))
        })
    }

    pub fn events(&self) -> Receiver<SbpUsbMessage> {
        self.rx.clone()
    }
}

impl Default for SbpUsbEventBus {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sbp_usb::opcode::SbpUsbOpcode;

    #[test]
    fn test_new_event_bus() {
        let bus = SbpUsbEventBus::new();
        let rx = bus.events();
        assert!(rx.is_empty());
    }

    #[test]
    fn test_publish_and_receive() {
        let bus = SbpUsbEventBus::new();
        let msg = SbpUsbMessage::new(SbpUsbOpcode::UsbHeartbeat, 0, vec![]);
        bus.publish(msg.clone()).unwrap();
        let rx = bus.events();
        let received = rx.recv().unwrap();
        assert_eq!(received.opcode, SbpUsbOpcode::UsbHeartbeat);
    }

    #[test]
    fn test_multiple_events() {
        let bus = SbpUsbEventBus::new();
        for i in 0..5 {
            let msg = SbpUsbMessage::new(SbpUsbOpcode::UsbHeartbeat, i, vec![]);
            bus.publish(msg).unwrap();
        }
        let rx = bus.events();
        for i in 0..5 {
            let received = rx.recv().unwrap();
            assert_eq!(received.app_id, i);
        }
    }

    #[test]
    fn test_event_bus_default() {
        let bus = SbpUsbEventBus::default();
        let rx = bus.events();
        assert!(rx.is_empty());
    }

    #[test]
    fn test_publish_after_receive_clone() {
        let bus = SbpUsbEventBus::new();
        let rx1 = bus.events();
        let msg = SbpUsbMessage::new(SbpUsbOpcode::UsbDeviceEvent, 0, vec![0x01]);
        bus.publish(msg).unwrap();
        let rx2 = bus.events();
        assert_eq!(rx1.recv().unwrap().app_id, 0);
        assert!(rx2.is_empty());
    }
}
