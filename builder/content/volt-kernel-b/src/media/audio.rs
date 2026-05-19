use crate::core::error::{Result, TesseractError};

pub struct AudioProcessor;

impl AudioProcessor {
    pub fn new() -> Self {
        Self
    }

    pub fn process_frame(&self, data: &[u8], _sample_rate: u32, _channels: u8) -> Result<Vec<u8>> {
        if data.is_empty() {
            return Err(TesseractError::Media("empty audio frame".into()));
        }

        Ok(data.to_vec())
    }

    pub fn resample(&self, data: &[u8], from_rate: u32, to_rate: u32) -> Result<Vec<u8>> {
        if data.is_empty() {
            return Err(TesseractError::Media("empty audio data for resample".into()));
        }

        if from_rate == 0 || to_rate == 0 {
            return Err(TesseractError::Media("invalid sample rate".into()));
        }

        if from_rate == to_rate {
            return Ok(data.to_vec());
        }

        let ratio = to_rate as f64 / from_rate as f64;
        let input_samples = data.len() / 2;
        let output_samples = (input_samples as f64 * ratio) as usize;
        let mut output = vec![0u8; output_samples * 2];

        for i in 0..output_samples {
            let src_idx = ((i as f64) / ratio) as usize;
            if src_idx < input_samples {
                let src_byte = src_idx * 2;
                let dst_byte = i * 2;
                if src_byte + 1 < data.len() && dst_byte + 1 < output.len() {
                    output[dst_byte] = data[src_byte];
                    output[dst_byte + 1] = data[src_byte + 1];
                }
            }
        }

        Ok(output)
    }

    pub fn mix(&self, buffers: &[&[u8]]) -> Result<Vec<u8>> {
        if buffers.is_empty() {
            return Err(TesseractError::Media("no audio buffers to mix".into()));
        }

        let max_len = buffers.iter().map(|b| b.len()).max().unwrap_or(0);
        if max_len == 0 {
            return Ok(Vec::new());
        }

        let mut output = vec![0u8; max_len];
        for i in (0..max_len).step_by(2) {
            let mut sum: i32 = 0;
            for buf in buffers {
                if i + 1 < buf.len() {
                    let sample = i16::from_le_bytes([buf[i], buf[i + 1]]);
                    sum += sample as i32;
                }
            }
            sum = sum.clamp(i16::MIN as i32, i16::MAX as i32);
            let [lo, hi] = (sum as i16).to_le_bytes();
            output[i] = lo;
            output[i + 1] = hi;
        }

        Ok(output)
    }
}
