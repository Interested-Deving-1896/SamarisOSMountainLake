use crate::core::error::Result;
use crate::protocol::command::{CommandPayload, ComputeKind, QueryKind, StreamType};
use crate::protocol::opcodes::Opcode;

/// Simple binary payload format (FlatBuffers schema-compatible).
/// Payload layout:
///   [int32_len: u32][int32_data: i32 × N][uint32_len: u32][uint32_data: u32 × N]
///   [byte_len: u32][byte_data: u8 × N][str_len: u32][str_data: u8 × N]

#[derive(Debug, Clone)]
pub struct ParsedPacket {
    pub opcode: u8,
    pub priority: u8,
    pub app_id: String,
    pub correlation_id: u64,
    pub has_response: bool,
    pub has_error: bool,
    pub status_code: u32,
    pub int32_values: Vec<i32>,
    pub uint32_values: Vec<u32>,
    pub byte_data: Vec<u8>,
    pub string_value: String,
}

fn encode_length(len: usize, buf: &mut Vec<u8>) {
    buf.extend_from_slice(&(len as u32).to_le_bytes());
}

fn encode_u32_slice(data: &[u32], buf: &mut Vec<u8>) {
    encode_length(data.len(), buf);
    for &item in data {
        buf.extend_from_slice(&item.to_le_bytes());
    }
}

fn encode_i32_slice(data: &[i32], buf: &mut Vec<u8>) {
    encode_length(data.len(), buf);
    for &item in data {
        buf.extend_from_slice(&item.to_le_bytes());
    }
}

fn encode_bytes(data: &[u8], buf: &mut Vec<u8>) {
    encode_length(data.len(), buf);
    buf.extend_from_slice(data);
}

fn encode_string(s: &str, buf: &mut Vec<u8>) {
    let bytes = s.as_bytes();
    encode_length(bytes.len(), buf);
    buf.extend_from_slice(bytes);
}

fn decode_u32_slice(data: &[u8], pos: &mut usize) -> Vec<u32> {
    if *pos + 4 > data.len() {
        return Vec::new();
    }
    let len = u32::from_le_bytes(data[*pos..*pos + 4].try_into().unwrap_or([0; 4])) as usize;
    *pos += 4;
    let mut result = Vec::with_capacity(len);
    for _ in 0..len {
        if *pos + 4 > data.len() {
            break;
        }
        result.push(u32::from_le_bytes(data[*pos..*pos + 4].try_into().unwrap_or([0; 4])));
        *pos += 4;
    }
    result
}

fn decode_i32_slice(data: &[u8], pos: &mut usize) -> Vec<i32> {
    if *pos + 4 > data.len() {
        return Vec::new();
    }
    let len = u32::from_le_bytes(data[*pos..*pos + 4].try_into().unwrap_or([0; 4])) as usize;
    *pos += 4;
    let mut result = Vec::with_capacity(len);
    for _ in 0..len {
        if *pos + 4 > data.len() {
            break;
        }
        result.push(i32::from_le_bytes(data[*pos..*pos + 4].try_into().unwrap_or([0; 4])));
        *pos += 4;
    }
    result
}

fn decode_bytes(data: &[u8], pos: &mut usize) -> Vec<u8> {
    if *pos + 4 > data.len() {
        return Vec::new();
    }
    let len = u32::from_le_bytes(data[*pos..*pos + 4].try_into().unwrap_or([0; 4])) as usize;
    *pos += 4;
    let end = (*pos + len).min(data.len());
    let result = data[*pos..end].to_vec();
    *pos = end;
    result
}

fn decode_string(data: &[u8], pos: &mut usize) -> String {
    String::from_utf8_lossy(&decode_bytes(data, pos)).to_string()
}

pub fn command_to_payload(cmd: &CommandPayload, _opcode: Opcode) -> Vec<u8> {
    let mut buf = Vec::new();
    match cmd {
        CommandPayload::RenderRect {
            x, y, w, h, border_radius, shadow_blur,
            shadow_offset_x, shadow_offset_y,
            fill_r, fill_g, fill_b, fill_a,
            border_r, border_g, border_b, border_width,
        } => {
            encode_i32_slice(&[*x, *y, *w as i32, *h as i32], &mut buf);
            encode_u32_slice(&[
                border_radius.to_bits(), shadow_blur.to_bits(),
                *shadow_offset_x as u32, *shadow_offset_y as u32,
                *border_width as u32,
            ], &mut buf);
            encode_bytes(&[*fill_r, *fill_g, *fill_b, *fill_a, *border_r, *border_g, *border_b], &mut buf);
            encode_string("", &mut buf);
        }
        CommandPayload::ComputeTask { kind, data } => {
            let kb = match kind {
                ComputeKind::Compress => 1u8, ComputeKind::Decompress => 2,
                ComputeKind::HashSha256 => 3, ComputeKind::EncryptAes256Gcm => 4,
                ComputeKind::DecryptAes256Gcm => 5, ComputeKind::ImageBlur => 6,
                ComputeKind::ImageResize => 7, ComputeKind::ImageFilter => 8,
                ComputeKind::Custom(v) => *v,
            };
            encode_i32_slice(&[kb as i32], &mut buf);
            encode_u32_slice(&[], &mut buf);
            encode_bytes(data, &mut buf);
            encode_string("", &mut buf);
        }
        CommandPayload::MemAlloc { size } => {
            encode_i32_slice(&[*size as i32], &mut buf);
            encode_u32_slice(&[], &mut buf);
            encode_bytes(&[], &mut buf);
            encode_string("", &mut buf);
        }
        CommandPayload::MemFree { address } => {
            encode_i32_slice(&[*address as i32], &mut buf);
            encode_u32_slice(&[], &mut buf);
            encode_bytes(&[], &mut buf);
            encode_string("", &mut buf);
        }
        CommandPayload::StreamMedia { stream_type, data, pts_us } => {
            let (st_code, sr) = match stream_type {
                StreamType::Video { codec } => (0u8, *codec as u32),
                StreamType::Audio { sample_rate, .. } => (1u8, *sample_rate),
            };
            encode_i32_slice(&[st_code as i32], &mut buf);
            encode_u32_slice(&[sr, *pts_us as u32], &mut buf);
            encode_bytes(data, &mut buf);
            encode_string("", &mut buf);
        }
        CommandPayload::Query { kind } => {
            let k = match kind { QueryKind::Cores => 0u8, QueryKind::Gpu => 1, QueryKind::Thermal => 2 };
            encode_i32_slice(&[k as i32], &mut buf);
            encode_u32_slice(&[], &mut buf);
            encode_bytes(&[], &mut buf);
            encode_string("", &mut buf);
        }
        CommandPayload::Heartbeat => {
            encode_i32_slice(&[], &mut buf);
            encode_u32_slice(&[], &mut buf);
            encode_bytes(&[], &mut buf);
            encode_string("", &mut buf);
        }
        CommandPayload::ContextCreate { name } => {
            encode_i32_slice(&[], &mut buf);
            encode_u32_slice(&[], &mut buf);
            encode_bytes(&[], &mut buf);
            encode_string(name, &mut buf);
        }
        CommandPayload::ContextShare { target_app_id } => {
            encode_i32_slice(&[*target_app_id as i32], &mut buf);
            encode_u32_slice(&[], &mut buf);
            encode_bytes(&[], &mut buf);
            encode_string("", &mut buf);
        }
        CommandPayload::RenderResult { pixel_data, width, height } => {
            encode_i32_slice(&[*width as i32, *height as i32], &mut buf);
            encode_u32_slice(&[], &mut buf);
            encode_bytes(pixel_data, &mut buf);
            encode_string("", &mut buf);
        }
        CommandPayload::RawBytes(data) => {
            encode_i32_slice(&[], &mut buf);
            encode_u32_slice(&[], &mut buf);
            encode_bytes(data, &mut buf);
            encode_string("", &mut buf);
        }
        CommandPayload::SystemMetricResponse | CommandPayload::Empty => {
            encode_i32_slice(&[], &mut buf);
            encode_u32_slice(&[], &mut buf);
            encode_bytes(&[], &mut buf);
            encode_string("", &mut buf);
        }
    }
    buf
}

pub fn parse_packet(data: &[u8]) -> Result<ParsedPacket> {
    let mut pos = 0usize;
    let int32_values = decode_i32_slice(data, &mut pos);
    let uint32_values = decode_u32_slice(data, &mut pos);
    let byte_data = decode_bytes(data, &mut pos);
    let string_value = decode_string(data, &mut pos);
    Ok(ParsedPacket {
        opcode: 0, priority: 2,
        app_id: String::new(),
        correlation_id: 0,
        has_response: false, has_error: false, status_code: 0,
        int32_values, uint32_values, byte_data, string_value,
    })
}

pub fn payload_to_command(opcode: Opcode, data: &[u8]) -> Result<CommandPayload> {
    let packet = parse_packet(data)?;
    match opcode {
        Opcode::GpuRender => {
            if packet.int32_values.len() >= 4 && packet.uint32_values.len() >= 5 && packet.byte_data.len() >= 7 {
                Ok(CommandPayload::RenderRect {
                    x: packet.int32_values[0], y: packet.int32_values[1],
                    w: packet.int32_values[2] as u32, h: packet.int32_values[3] as u32,
                    border_radius: f32::from_bits(packet.uint32_values[0]),
                    shadow_blur: f32::from_bits(packet.uint32_values[1]),
                    shadow_offset_x: f32::from_bits(packet.uint32_values[2]),
                    shadow_offset_y: f32::from_bits(packet.uint32_values[3]),
                    fill_r: packet.byte_data[0], fill_g: packet.byte_data[1],
                    fill_b: packet.byte_data[2], fill_a: packet.byte_data[3],
                    border_r: packet.byte_data[4], border_g: packet.byte_data[5],
                    border_b: packet.byte_data[6],
                    border_width: f32::from_bits(packet.uint32_values.get(4).copied().unwrap_or(0)),
                })
            } else { Ok(CommandPayload::Empty) }
        }
        Opcode::GpuCompute | Opcode::CpuExec => {
            let kind = match packet.int32_values.first().copied().unwrap_or(0) {
                1 => ComputeKind::Compress, 2 => ComputeKind::Decompress,
                3 => ComputeKind::HashSha256, 4 => ComputeKind::EncryptAes256Gcm,
                5 => ComputeKind::DecryptAes256Gcm, 6 => ComputeKind::ImageBlur,
                7 => ComputeKind::ImageResize, 8 => ComputeKind::ImageFilter,
                v => ComputeKind::Custom(v as u8),
            };
            Ok(CommandPayload::ComputeTask { kind, data: packet.byte_data.clone() })
        }
        Opcode::MemAlloc => Ok(CommandPayload::MemAlloc { size: packet.int32_values.first().copied().unwrap_or(0) as u64 }),
        Opcode::MemFree => Ok(CommandPayload::MemFree { address: packet.int32_values.first().copied().unwrap_or(0) as u64 }),
        Opcode::StreamVideo => {
            Ok(CommandPayload::StreamMedia {
                stream_type: StreamType::Video { codec: packet.uint32_values.first().copied().unwrap_or(0) as u8 },
                data: packet.byte_data.clone(),
                pts_us: packet.uint32_values.get(1).copied().unwrap_or(0) as u64,
            })
        }
        Opcode::StreamAudio => {
            Ok(CommandPayload::StreamMedia {
                stream_type: StreamType::Audio {
                    sample_rate: packet.uint32_values.first().copied().unwrap_or(44100),
                    channels: packet.byte_data.first().copied().unwrap_or(2),
                },
                data: packet.byte_data.clone(),
                pts_us: packet.uint32_values.get(1).copied().unwrap_or(0) as u64,
            })
        }
        Opcode::QueryCores => Ok(CommandPayload::Query { kind: QueryKind::Cores }),
        Opcode::QueryGpu => Ok(CommandPayload::Query { kind: QueryKind::Gpu }),
        Opcode::Heartbeat => Ok(CommandPayload::Heartbeat),
        Opcode::ThermalStatus => Ok(CommandPayload::Query { kind: QueryKind::Thermal }),
        Opcode::ContextCreate => Ok(CommandPayload::ContextCreate { name: packet.string_value.clone() }),
        Opcode::ContextShare => Ok(CommandPayload::ContextShare { target_app_id: packet.int32_values.first().copied().unwrap_or(0) as u32 }),
        Opcode::CpuReserve | Opcode::CpuRelease => Ok(CommandPayload::Empty),
    }
}
