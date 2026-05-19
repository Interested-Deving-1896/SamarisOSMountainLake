use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WriteAckKind {
    Buffered,
    Durable,
    Error,
}

#[derive(Debug, Clone)]
pub struct WriteAck {
    pub write_id: u64,
    pub kind: WriteAckKind,
    pub timestamp_us: u64,
}

impl WriteAck {
    fn now_us() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_micros() as u64
    }

    pub fn buffered(write_id: u64) -> Self {
        Self {
            write_id,
            kind: WriteAckKind::Buffered,
            timestamp_us: Self::now_us(),
        }
    }

    pub fn durable(write_id: u64) -> Self {
        Self {
            write_id,
            kind: WriteAckKind::Durable,
            timestamp_us: Self::now_us(),
        }
    }

    pub fn error(write_id: u64) -> Self {
        Self {
            write_id,
            kind: WriteAckKind::Error,
            timestamp_us: Self::now_us(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ack_buffered() {
        let ack = WriteAck::buffered(42);
        assert_eq!(ack.write_id, 42);
        assert_eq!(ack.kind, WriteAckKind::Buffered);
        assert!(ack.timestamp_us > 0);
    }

    #[test]
    fn test_ack_durable() {
        let ack = WriteAck::durable(99);
        assert_eq!(ack.write_id, 99);
        assert_eq!(ack.kind, WriteAckKind::Durable);
    }

    #[test]
    fn test_ack_error() {
        let ack = WriteAck::error(7);
        assert_eq!(ack.write_id, 7);
        assert_eq!(ack.kind, WriteAckKind::Error);
    }

    #[test]
    fn test_ack_timestamps_increasing() {
        let a1 = WriteAck::buffered(1);
        let a2 = WriteAck::buffered(2);
        assert!(a2.timestamp_us >= a1.timestamp_us);
    }

    #[test]
    fn test_ack_clone() {
        let ack = WriteAck::durable(100);
        let cloned = ack.clone();
        assert_eq!(ack.write_id, cloned.write_id);
        assert_eq!(ack.kind, cloned.kind);
        assert_eq!(ack.timestamp_us, cloned.timestamp_us);
    }

    #[test]
    fn test_write_ack_kind_equality() {
        assert_eq!(WriteAckKind::Buffered, WriteAckKind::Buffered);
        assert_ne!(WriteAckKind::Buffered, WriteAckKind::Durable);
        assert_ne!(WriteAckKind::Durable, WriteAckKind::Error);
    }

    #[test]
    fn test_write_ack_kind_names() {
        match WriteAckKind::Buffered {
            WriteAckKind::Buffered => {}
            _ => panic!("wrong variant"),
        }
    }
}
