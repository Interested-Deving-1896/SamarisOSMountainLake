use crate::core::error::{Result, TesseractError};
use crate::protocol::opcodes::Opcode;

pub const SBP_MAGIC: [u8; 2] = [0x56, 0x4F];
pub const SBP_VERSION: u8 = 5;
pub const SBP_HEADER_SIZE: usize = 16;

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Flags: u16 {
        const RESPONSE        = 0x0001;
        const ERROR           = 0x0002;
        const PRIORITY_OVERRIDE = 0x0080;
        const NO_RESPONSE     = 0x0100;
    }
}


#[derive(Debug, Clone, Copy)]
pub struct SbpHeader {
    pub magic: [u8; 2],
    pub version: u8,
    pub opcode: u8,
    pub flags: Flags,
    pub priority: u8,
    pub app_id: u32,
    pub payload_len: u32,
    pub checksum: u8,
    pub reserved: u8,
}

impl SbpHeader {
    pub fn new(opcode: Opcode, priority: u8, app_id: u32, payload_len: u32) -> Self {
        Self {
            magic: SBP_MAGIC,
            version: SBP_VERSION,
            opcode: opcode as u8,
            flags: Flags::empty(),
            priority,
            app_id,
            payload_len,
            checksum: 0,
            reserved: 0,
        }
    }

    pub fn encode(&self) -> [u8; SBP_HEADER_SIZE] {
        let mut buf = [0u8; SBP_HEADER_SIZE];
        buf[0..2].copy_from_slice(&self.magic);
        buf[2] = self.version;
        buf[3] = self.opcode;
        buf[4..6].copy_from_slice(&self.flags.bits().to_le_bytes());
        buf[6] = self.priority;
        buf[7..11].copy_from_slice(&self.app_id.to_le_bytes());
        buf[11..15].copy_from_slice(&self.payload_len.to_le_bytes());
        let checksum = Self::compute_checksum(&buf[..15]);
        buf[15] = checksum;
        buf
    }

    pub fn decode(buf: &[u8; SBP_HEADER_SIZE]) -> Result<Self> {
        let checksum = buf[15];
        let computed = Self::compute_checksum(&buf[..15]);
        if checksum != computed {
            return Err(TesseractError::Protocol(format!(
                "checksum mismatch: got 0x{checksum:02X}, expected 0x{computed:02X}"
            )));
        }
        if buf[0..2] != SBP_MAGIC {
            return Err(TesseractError::Protocol("invalid magic".into()));
        }
        if buf[2] != SBP_VERSION {
            return Err(TesseractError::Protocol(format!(
                "unsupported version: {}",
                buf[2]
            )));
        }
        Ok(Self {
            magic: [buf[0], buf[1]],
            version: buf[2],
            opcode: buf[3],
            flags: Flags::from_bits_truncate(u16::from_le_bytes([buf[4], buf[5]])),
            priority: buf[6],
            app_id: u32::from_le_bytes([buf[7], buf[8], buf[9], buf[10]]),
            payload_len: u32::from_le_bytes([buf[11], buf[12], buf[13], buf[14]]),
            checksum,
            reserved: 0,
        })
    }

    pub fn with_flags(mut self, flags: Flags) -> Self {
        self.flags = flags;
        self
    }

    pub fn with_payload_len(mut self, len: u32) -> Self {
        self.payload_len = len;
        self
    }

    fn compute_checksum(data: &[u8]) -> u8 {
        data.iter().fold(0u8, |acc, &b| acc ^ b)
    }
}
