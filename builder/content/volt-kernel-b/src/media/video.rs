use crate::core::error::{Result, TesseractError};

pub struct VideoProcessor;

impl VideoProcessor {
    pub fn new() -> Self {
        Self
    }

    pub fn process_frame(&self, data: &[u8]) -> Result<Vec<u8>> {
        if data.is_empty() {
            return Err(TesseractError::Media("empty video frame".into()));
        }

        Ok(data.to_vec())
    }

    pub fn decode_frame(&self, data: &[u8], _codec: u8) -> Result<Vec<u8>> {
        if data.is_empty() {
            return Err(TesseractError::Media("empty encoded frame".into()));
        }

        Ok(data.to_vec())
    }

    pub fn encode_frame(&self, data: &[u8], _codec: u8) -> Result<Vec<u8>> {
        if data.is_empty() {
            return Err(TesseractError::Media("empty raw frame".into()));
        }

        Ok(data.to_vec())
    }

    pub fn get_frame_info(&self, data: &[u8]) -> FrameInfo {
        FrameInfo {
            size_bytes: data.len(),
            width: 0,
            height: 0,
            format: "raw".into(),
        }
    }
}

pub struct FrameInfo {
    pub size_bytes: usize,
    pub width: u32,
    pub height: u32,
    pub format: String,
}
