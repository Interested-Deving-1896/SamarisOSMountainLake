use crate::core::result::VumResult;
use crate::core::error::VumError;
use crate::sbp_usb::permissions::SbpUsbPermission;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SbpUsbOpcode {
    UsbStatus = 0x30,
    UsbRead = 0x31,
    UsbWrite = 0x32,
    UsbFlush = 0x33,
    UsbCacheStatus = 0x34,
    UsbPrefetch = 0x35,
    UsbEject = 0x36,
    UsbHeartbeat = 0x37,
    UsbMount = 0x38,
    UsbUnmount = 0x39,
    UsbJournalStatus = 0x3A,
    UsbRecoveryRun = 0x3B,
    UsbDurabilityStatus = 0x3C,
    UsbWriteAckEvent = 0x3D,
    UsbDeviceEvent = 0x3E,
    UsbMetricsSnapshot = 0x3F,
}

impl SbpUsbOpcode {
    pub fn from_byte(b: u8) -> VumResult<Self> {
        match b {
            0x30 => Ok(SbpUsbOpcode::UsbStatus),
            0x31 => Ok(SbpUsbOpcode::UsbRead),
            0x32 => Ok(SbpUsbOpcode::UsbWrite),
            0x33 => Ok(SbpUsbOpcode::UsbFlush),
            0x34 => Ok(SbpUsbOpcode::UsbCacheStatus),
            0x35 => Ok(SbpUsbOpcode::UsbPrefetch),
            0x36 => Ok(SbpUsbOpcode::UsbEject),
            0x37 => Ok(SbpUsbOpcode::UsbHeartbeat),
            0x38 => Ok(SbpUsbOpcode::UsbMount),
            0x39 => Ok(SbpUsbOpcode::UsbUnmount),
            0x3A => Ok(SbpUsbOpcode::UsbJournalStatus),
            0x3B => Ok(SbpUsbOpcode::UsbRecoveryRun),
            0x3C => Ok(SbpUsbOpcode::UsbDurabilityStatus),
            0x3D => Ok(SbpUsbOpcode::UsbWriteAckEvent),
            0x3E => Ok(SbpUsbOpcode::UsbDeviceEvent),
            0x3F => Ok(SbpUsbOpcode::UsbMetricsSnapshot),
            _ => Err(VumError::UnsupportedOpcode(b)),
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            SbpUsbOpcode::UsbStatus => "UsbStatus",
            SbpUsbOpcode::UsbRead => "UsbRead",
            SbpUsbOpcode::UsbWrite => "UsbWrite",
            SbpUsbOpcode::UsbFlush => "UsbFlush",
            SbpUsbOpcode::UsbCacheStatus => "UsbCacheStatus",
            SbpUsbOpcode::UsbPrefetch => "UsbPrefetch",
            SbpUsbOpcode::UsbEject => "UsbEject",
            SbpUsbOpcode::UsbHeartbeat => "UsbHeartbeat",
            SbpUsbOpcode::UsbMount => "UsbMount",
            SbpUsbOpcode::UsbUnmount => "UsbUnmount",
            SbpUsbOpcode::UsbJournalStatus => "UsbJournalStatus",
            SbpUsbOpcode::UsbRecoveryRun => "UsbRecoveryRun",
            SbpUsbOpcode::UsbDurabilityStatus => "UsbDurabilityStatus",
            SbpUsbOpcode::UsbWriteAckEvent => "UsbWriteAckEvent",
            SbpUsbOpcode::UsbDeviceEvent => "UsbDeviceEvent",
            SbpUsbOpcode::UsbMetricsSnapshot => "UsbMetricsSnapshot",
        }
    }

    pub fn requires_response(&self) -> bool {
        match self {
            SbpUsbOpcode::UsbHeartbeat
            | SbpUsbOpcode::UsbDeviceEvent
            | SbpUsbOpcode::UsbWriteAckEvent => false,
            _ => true,
        }
    }

    pub fn permission(&self) -> SbpUsbPermission {
        match self {
            SbpUsbOpcode::UsbStatus
            | SbpUsbOpcode::UsbCacheStatus
            | SbpUsbOpcode::UsbHeartbeat
            | SbpUsbOpcode::UsbMetricsSnapshot => SbpUsbPermission::CAP_READ_STATUS,
            SbpUsbOpcode::UsbRead | SbpUsbOpcode::UsbPrefetch => SbpUsbPermission::CAP_READ_FILE,
            SbpUsbOpcode::UsbWrite => SbpUsbPermission::CAP_WRITE_FILE,
            SbpUsbOpcode::UsbFlush
            | SbpUsbOpcode::UsbEject
            | SbpUsbOpcode::UsbMount
            | SbpUsbOpcode::UsbUnmount
            | SbpUsbOpcode::UsbRecoveryRun
            | SbpUsbOpcode::UsbJournalStatus
            | SbpUsbOpcode::UsbDurabilityStatus => SbpUsbPermission::CAP_ADMIN_STORAGE,
            SbpUsbOpcode::UsbWriteAckEvent | SbpUsbOpcode::UsbDeviceEvent => {
                SbpUsbPermission::INTERNAL
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_byte_valid() {
        for code in 0x30..=0x3F {
            assert!(SbpUsbOpcode::from_byte(code).is_ok());
        }
    }

    #[test]
    fn test_from_byte_invalid() {
        assert!(SbpUsbOpcode::from_byte(0x00).is_err());
        assert!(SbpUsbOpcode::from_byte(0x29).is_err());
        assert!(SbpUsbOpcode::from_byte(0x40).is_err());
        assert!(SbpUsbOpcode::from_byte(0xFF).is_err());
    }

    #[test]
    fn test_name_not_empty() {
        for code in 0x30..=0x3F {
            let op = SbpUsbOpcode::from_byte(code).unwrap();
            assert!(!op.name().is_empty());
        }
    }

    #[test]
    fn test_requires_response_heartbeat_false() {
        let hb = SbpUsbOpcode::UsbHeartbeat;
        assert!(!hb.requires_response());
    }

    #[test]
    fn test_requires_response_status_true() {
        assert!(SbpUsbOpcode::UsbStatus.requires_response());
    }

    #[test]
    fn test_permission_read_status() {
        let perm = SbpUsbOpcode::UsbStatus.permission();
        assert!(perm.contains(SbpUsbPermission::CAP_READ_STATUS));
    }

    #[test]
    fn test_permission_read_file() {
        let perm = SbpUsbOpcode::UsbRead.permission();
        assert!(perm.contains(SbpUsbPermission::CAP_READ_FILE));
    }

    #[test]
    fn test_permission_write_file() {
        let perm = SbpUsbOpcode::UsbWrite.permission();
        assert!(perm.contains(SbpUsbPermission::CAP_WRITE_FILE));
    }

    #[test]
    fn test_permission_admin_storage() {
        let perm = SbpUsbOpcode::UsbEject.permission();
        assert!(perm.contains(SbpUsbPermission::CAP_ADMIN_STORAGE));
    }

    #[test]
    fn test_permission_internal() {
        let perm = SbpUsbOpcode::UsbDeviceEvent.permission();
        assert!(perm.contains(SbpUsbPermission::INTERNAL));
    }
}
