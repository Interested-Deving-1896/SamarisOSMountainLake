use bitflags::bitflags;

use crate::sbp_usb::opcode::SbpUsbOpcode;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct SbpUsbPermission: u32 {
        const CAP_READ_STATUS   = 1 << 0;
        const CAP_READ_FILE     = 1 << 1;
        const CAP_WRITE_FILE    = 1 << 2;
        const CAP_ADMIN_STORAGE = 1 << 3;
        const INTERNAL          = 1 << 31;
    }
}

impl SbpUsbPermission {
    pub fn required_for(opcode: SbpUsbOpcode) -> Self {
        opcode.permission()
    }

    pub fn admin() -> Self {
        SbpUsbPermission::CAP_ADMIN_STORAGE
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sbp_usb::opcode::SbpUsbOpcode;

    #[test]
    fn test_required_for_matches_opcode_permission() {
        for code in 0x30..=0x3F {
            let op = SbpUsbOpcode::from_byte(code).unwrap();
            assert_eq!(
                SbpUsbPermission::required_for(op),
                op.permission()
            );
        }
    }

    #[test]
    fn test_admin_helper() {
        let perm = SbpUsbPermission::admin();
        assert!(perm.contains(SbpUsbPermission::CAP_ADMIN_STORAGE));
        assert!(!perm.contains(SbpUsbPermission::CAP_READ_STATUS));
        assert!(!perm.contains(SbpUsbPermission::INTERNAL));
    }

    #[test]
    fn test_bitflag_combinations() {
        let combined = SbpUsbPermission::CAP_READ_STATUS
            | SbpUsbPermission::CAP_READ_FILE
            | SbpUsbPermission::CAP_WRITE_FILE;
        assert!(combined.contains(SbpUsbPermission::CAP_READ_STATUS));
        assert!(combined.contains(SbpUsbPermission::CAP_READ_FILE));
        assert!(combined.contains(SbpUsbPermission::CAP_WRITE_FILE));
        assert!(!combined.contains(SbpUsbPermission::CAP_ADMIN_STORAGE));
    }

    #[test]
    fn test_internal_bit_position() {
        let internal = SbpUsbPermission::INTERNAL;
        assert_eq!(internal.bits(), 1u32 << 31);
    }

    #[test]
    fn test_empty_permission() {
        let empty = SbpUsbPermission::empty();
        assert!(empty.is_empty());
    }
}
