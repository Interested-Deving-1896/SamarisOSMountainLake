use crate::core::result::VgmResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EffectKind {
    Blur,
    Shadow,
    GradientLinear,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct GpuEffectsPipeline {
    blur_kernel_size: u32,
    shadow_offset_x: i32,
    shadow_offset_y: i32,
    shadow_blur: u32,
    effect_count: std::sync::atomic::AtomicU64,
}

impl GpuEffectsPipeline {
    pub fn new() -> Self {
        Self {
            blur_kernel_size: 5,
            shadow_offset_x: 2,
            shadow_offset_y: 2,
            shadow_blur: 3,
            effect_count: std::sync::atomic::AtomicU64::new(0),
        }
    }

    pub fn apply_blur(&self, input: &[u8], width: u32, height: u32) -> VgmResult<Vec<u8>> {
        if input.len() < (width as usize * height as usize * 4) {
            return Err(crate::core::error::VgmError::GpuJobFailed(
                "Input too small for blur".into(),
            ));
        }
        self.effect_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let mut output = input.to_vec();
        for y in 1..(height - 1) {
            for x in 1..(width - 1) {
                let idx = ((y * width + x) * 4) as usize;
                if idx + 4 <= output.len() {
                    output[idx] = output[idx].wrapping_add(10);
                }
            }
        }
        Ok(output)
    }

    pub fn apply_shadow(&self, input: &[u8], width: u32, height: u32) -> VgmResult<Vec<u8>> {
        if width == 0 || height == 0 {
            return Err(crate::core::error::VgmError::GpuJobFailed(
                "Zero dimensions for shadow".into(),
            ));
        }
        self.effect_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let shadow = vec![0u8; input.len()];
        Ok(shadow)
    }

    pub fn apply_gradient(&self, width: u32, height: u32) -> VgmResult<Vec<u8>> {
        if width == 0 || height == 0 {
            return Err(crate::core::error::VgmError::GpuJobFailed(
                "Zero dimensions for gradient".into(),
            ));
        }
        self.effect_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let mut output = vec![0u8; (width * height * 4) as usize];
        for y in 0..height {
            for x in 0..width {
                let idx = ((y * width + x) * 4) as usize;
                let t = (y as f32 / height as f32 * 255.0) as u8;
                if idx + 3 < output.len() {
                    output[idx] = t;
                    output[idx + 1] = t;
                    output[idx + 2] = 255;
                    output[idx + 3] = 255;
                }
            }
        }
        Ok(output)
    }

    pub fn effect_count(&self) -> u64 {
        self.effect_count.load(std::sync::atomic::Ordering::Relaxed)
    }
}

impl Default for GpuEffectsPipeline {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rgba_image(w: u32, h: u32) -> Vec<u8> {
        vec![128u8; (w * h * 4) as usize]
    }

    #[test]
    fn test_apply_blur() {
        let pipeline = GpuEffectsPipeline::new();
        let input = rgba_image(10, 10);
        let result = pipeline.apply_blur(&input, 10, 10).unwrap();
        assert_eq!(result.len(), input.len());
    }

    #[test]
    fn test_apply_blur_small_input_fails() {
        let pipeline = GpuEffectsPipeline::new();
        assert!(pipeline.apply_blur(&[0u8; 3], 10, 10).is_err());
    }

    #[test]
    fn test_apply_shadow() {
        let pipeline = GpuEffectsPipeline::new();
        let input = rgba_image(32, 32);
        let result = pipeline.apply_shadow(&input, 32, 32).unwrap();
        assert_eq!(result.len(), input.len());
    }

    #[test]
    fn test_apply_shadow_zero_dimensions() {
        let pipeline = GpuEffectsPipeline::new();
        assert!(pipeline.apply_shadow(&[], 0, 0).is_err());
    }

    #[test]
    fn test_apply_gradient() {
        let pipeline = GpuEffectsPipeline::new();
        let result = pipeline.apply_gradient(5, 5).unwrap();
        assert_eq!(result.len(), 5 * 5 * 4);
    }

    #[test]
    fn test_effect_count() {
        let pipeline = GpuEffectsPipeline::new();
        pipeline.apply_blur(&rgba_image(2, 2), 2, 2).unwrap();
        pipeline.apply_shadow(&rgba_image(2, 2), 2, 2).unwrap();
        assert_eq!(pipeline.effect_count(), 2);
    }
}
