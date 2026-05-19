use uuid::Uuid;

use crate::core::error::Result;
use crate::protocol::header::SbpHeader;
use crate::protocol::opcodes::Opcode;

#[derive(Debug, Clone)]
pub struct TesseractCommand {
    pub header: SbpHeader,
    pub payload: Vec<u8>,
    pub correlation_id: Uuid,
}

impl TesseractCommand {
    pub fn new(header: SbpHeader, payload: Vec<u8>) -> Self {
        Self {
            header,
            payload,
            correlation_id: Uuid::new_v4(),
        }
    }

    pub fn opcode(&self) -> Result<Opcode> {
        Opcode::from_byte(self.header.opcode)
    }

    pub fn app_id(&self) -> u32 {
        self.header.app_id
    }

    pub fn priority(&self) -> u8 {
        self.header.priority
    }

    pub fn payload_len(&self) -> u32 {
        self.header.payload_len
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let header_bytes = self.header.encode();
        let mut buf = Vec::with_capacity(header_bytes.len() + self.payload.len());
        buf.extend_from_slice(&header_bytes);
        buf.extend_from_slice(&self.payload);
        buf
    }

    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        if data.len() < 16 {
            return Err(crate::core::error::TesseractError::Protocol(
                "truncated header".into(),
            ));
        }
        let header = SbpHeader::decode(&data[..16].try_into().unwrap())?;
        let payload_len = header.payload_len as usize;
        let payload = if data.len() >= 16 + payload_len {
            data[16..16 + payload_len].to_vec()
        } else {
            Vec::new()
        };
        Ok(Self {
            header,
            payload,
            correlation_id: Uuid::new_v4(),
        })
    }

    pub fn is_response(&self) -> bool {
        self.header.flags.contains(crate::protocol::header::Flags::RESPONSE)
    }

    pub fn is_error(&self) -> bool {
        self.header.flags.contains(crate::protocol::header::Flags::ERROR)
    }
}

#[derive(Debug, Clone)]
pub enum CommandPayload {
    RenderRect {
        x: i32,
        y: i32,
        w: u32,
        h: u32,
        border_radius: f32,
        shadow_blur: f32,
        shadow_offset_x: f32,
        shadow_offset_y: f32,
        fill_r: u8,
        fill_g: u8,
        fill_b: u8,
        fill_a: u8,
        border_r: u8,
        border_g: u8,
        border_b: u8,
        border_width: f32,
    },
    ComputeTask {
        kind: ComputeKind,
        data: Vec<u8>,
    },
    MemAlloc {
        size: u64,
    },
    MemFree {
        address: u64,
    },
    StreamMedia {
        stream_type: StreamType,
        data: Vec<u8>,
        pts_us: u64,
    },
    Query {
        kind: QueryKind,
    },
    Heartbeat,
    ContextCreate {
        name: String,
    },
    ContextShare {
        target_app_id: u32,
    },
    SystemMetricResponse,
    RenderResult {
        pixel_data: Vec<u8>,
        width: u32,
        height: u32,
    },
    RawBytes(Vec<u8>),
    Empty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComputeKind {
    Compress,
    Decompress,
    HashSha256,
    EncryptAes256Gcm,
    DecryptAes256Gcm,
    ImageBlur,
    ImageResize,
    ImageFilter,
    Custom(u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StreamType {
    Video { codec: u8 },
    Audio { sample_rate: u32, channels: u8 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueryKind {
    Cores,
    Gpu,
    Thermal,
}
