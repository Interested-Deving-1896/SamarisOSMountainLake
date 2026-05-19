use crate::sbp_mem::opcode::SbpOpcode;
use crate::core::error::VrmError;
use crate::core::result::VrmResult;

pub const SBP_MAGIC: u32 = 0x31524D56;

#[derive(Debug, Clone)]
pub struct SbpMessage {
    pub opcode: SbpOpcode,
    pub request_id: u64,
    pub app_id: u64,
    pub payload: Vec<u8>,
}

impl SbpMessage {
    pub fn new(opcode: SbpOpcode, app_id: u64, payload: Vec<u8>) -> Self {
        Self {
            opcode,
            request_id: 0,
            app_id,
            payload,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let raw_opcode = self.opcode as u8;
        let mut buf = Vec::with_capacity(32 + self.payload.len());
        buf.extend_from_slice(&SBP_MAGIC.to_le_bytes());
        buf.push(0x01); // version
        buf.push(raw_opcode);
        buf.push(0u8); buf.push(0u8); // flags
        buf.extend_from_slice(&self.request_id.to_le_bytes());
        buf.extend_from_slice(&self.app_id.to_le_bytes());
        buf.extend_from_slice(&(self.payload.len() as u32).to_le_bytes());
        let checksum = Self::compute_checksum(&buf, &self.payload);
        buf.extend_from_slice(&checksum.to_le_bytes());
        buf.extend_from_slice(&self.payload);
        buf
    }

    pub fn from_bytes(data: &[u8]) -> VrmResult<Self> {
        if data.len() < 32 {
            return Err(VrmError::InvalidSbpMessage("too short".into()));
        }
        let magic = u32::from_le_bytes(data[0..4].try_into().unwrap());
        if magic != SBP_MAGIC {
            return Err(VrmError::InvalidSbpMessage("bad magic".into()));
        }
        let raw_opcode = data[5];
        let opcode = SbpOpcode::from_byte(raw_opcode)?;
        let request_id = u64::from_le_bytes(data[8..16].try_into().unwrap());
        let app_id = u64::from_le_bytes(data[16..24].try_into().unwrap());
        let payload_len = u32::from_le_bytes(data[24..28].try_into().unwrap()) as usize;
        let stored_checksum = u32::from_le_bytes(data[28..32].try_into().unwrap());
        let payload = if payload_len > 0 && 32 + payload_len <= data.len() {
            data[32..32 + payload_len].to_vec()
        } else {
            Vec::new()
        };
        let mut header = data[..28].to_vec();
        let computed = Self::compute_checksum(&header, &payload);
        if stored_checksum != computed {
            return Err(VrmError::InvalidSbpMessage("checksum mismatch".into()));
        }
        Ok(Self { opcode, request_id, app_id, payload })
    }

    fn compute_checksum(header: &[u8], payload: &[u8]) -> u32 {
        let mut c = 0u32;
        for &b in header { c = c.wrapping_add(b as u32); }
        for &b in payload { c = c.wrapping_add(b as u32); }
        c
    }
}
