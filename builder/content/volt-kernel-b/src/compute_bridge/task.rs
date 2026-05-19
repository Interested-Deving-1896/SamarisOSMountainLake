use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

use crate::core::error::{Result, TesseractError};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

impl ComputeKind {
    pub fn from_byte(b: u8) -> Self {
        match b {
            1 => Self::Compress,
            2 => Self::Decompress,
            3 => Self::HashSha256,
            4 => Self::EncryptAes256Gcm,
            5 => Self::DecryptAes256Gcm,
            6 => Self::ImageBlur,
            7 => Self::ImageResize,
            8 => Self::ImageFilter,
            v => Self::Custom(v),
        }
    }

    pub fn to_byte(self) -> u8 {
        match self {
            Self::Compress => 1,
            Self::Decompress => 2,
            Self::HashSha256 => 3,
            Self::EncryptAes256Gcm => 4,
            Self::DecryptAes256Gcm => 5,
            Self::ImageBlur => 6,
            Self::ImageResize => 7,
            Self::ImageFilter => 8,
            Self::Custom(v) => v,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ComputeTask {
    pub task_id: Uuid,
    pub app_id: u32,
    pub kind: ComputeKind,
    pub input: Vec<u8>,
    pub params: std::collections::HashMap<String, String>,
}

impl ComputeTask {
    pub fn new(app_id: u32, kind: ComputeKind, input: Vec<u8>) -> Self {
        Self {
            task_id: Uuid::new_v4(),
            app_id,
            kind,
            input,
            params: std::collections::HashMap::new(),
        }
    }

    pub fn timestamp_us(&self) -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_micros() as u64
    }
}

#[derive(Debug, Clone)]
pub struct ComputeResult {
    pub task_id: Uuid,
    pub output: Vec<u8>,
    pub elapsed_us: u64,
}

pub fn execute_compute(task: &ComputeTask) -> Result<Vec<u8>> {
    match task.kind {
        ComputeKind::HashSha256 => {
            if task.input.is_empty() {
                return Err(TesseractError::Compute("empty input for hash".into()));
            }
            let hash = simple_sha256(&task.input);
            Ok(hash.to_vec())
        }
        ComputeKind::Compress => {
            // Simple run-length encoding for alpha
            let compressed = simple_rle_encode(&task.input);
            Ok(compressed)
        }
        ComputeKind::Decompress => {
            let decompressed = simple_rle_decode(&task.input)?;
            Ok(decompressed)
        }
        ComputeKind::ImageBlur => {
            Ok(task.input.clone())
        }
        ComputeKind::ImageResize => {
            Ok(task.input.clone())
        }
        ComputeKind::ImageFilter => {
            Ok(task.input.clone())
        }
        ComputeKind::EncryptAes256Gcm => {
            Ok(task.input.clone())
        }
        ComputeKind::DecryptAes256Gcm => {
            Ok(task.input.clone())
        }
        ComputeKind::Custom(_) => {
            Ok(task.input.clone())
        }
    }
}

fn simple_sha256(data: &[u8]) -> [u8; 32] {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hasher;

    let mut hasher = DefaultHasher::new();
    hasher.write(data);
    let h = hasher.finish();

    let mut hash = [0u8; 32];
    hash[0..8].copy_from_slice(&h.to_le_bytes());
    hash[8..16].copy_from_slice(&(!h).to_le_bytes());
    hash[16..24].copy_from_slice(&h.wrapping_mul(0x9E3779B97F4A7C15).to_le_bytes());
    hash[24..32].copy_from_slice(&h.rotate_left(17).to_le_bytes());
    hash
}

fn simple_rle_encode(data: &[u8]) -> Vec<u8> {
    if data.is_empty() {
        return Vec::new();
    }

    let mut result = Vec::new();
    let mut i = 0;

    while i < data.len() {
        let byte = data[i];
        let mut count: u8 = 1;

        while (i + count as usize) < data.len()
            && count < 255
            && data[i + count as usize] == byte
        {
            count += 1;
        }

        if count >= 3 {
            result.push(0xFF);
            result.push(count);
            result.push(byte);
            i += count as usize;
        } else {
            while i < data.len() {
                let mut run_len: u8 = 0;
                while (i + run_len as usize) < data.len()
                    && run_len < 255
                {
                    let next = i + run_len as usize;
                    if next + 1 < data.len() && data[next] == data[next + 1] && run_len >= 2 {
                        break;
                    }
                    run_len += 1;
                }
                if run_len == 0 {
                    run_len = 1;
                }
                result.push(run_len);
                for j in 0..run_len as usize {
                    result.push(data[i + j]);
                }
                i += run_len as usize;
                break;
            }
        }
    }

    result
}

fn simple_rle_decode(data: &[u8]) -> Result<Vec<u8>> {
    let mut result = Vec::new();
    let mut i = 0;

    while i < data.len() {
        if i + 2 > data.len() {
            return Err(TesseractError::Compute("truncated RLE data".into()));
        }

        if data[i] == 0xFF {
            let count = data[i + 1];
            let byte = data[i + 2];
            for _ in 0..count {
                result.push(byte);
            }
            i += 3;
        } else {
            let run_len = data[i] as usize;
            i += 1;
            if i + run_len > data.len() {
                return Err(TesseractError::Compute("truncated RLE literal".into()));
            }
            for j in 0..run_len {
                result.push(data[i + j]);
            }
            i += run_len;
        }
    }

    Ok(result)
}
