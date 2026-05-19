use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct RenderFrame {
    pub frame_id: Uuid,
    pub framebuffer: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub frame_index: u64,
    pub timestamp_ms: u64,
    pub metadata: FrameMetadata,
}

#[derive(Debug, Clone, Default)]
pub struct FrameMetadata {
    pub layer_count: u32,
    pub has_background: bool,
    pub has_shadow: bool,
    pub has_blur: bool,
    pub has_text: bool,
    pub draw_call_count: u32,
    pub gpu_time_us: u64,
}

impl RenderFrame {
    pub fn new(width: u32, height: u32, frame_index: u64) -> Self {
        let pixel_count = width as u64 * height as u64 * 4;
        Self {
            frame_id: Uuid::new_v4(),
            framebuffer: vec![0u8; pixel_count as usize],
            width,
            height,
            frame_index,
            timestamp_ms: 0,
            metadata: FrameMetadata::default(),
        }
    }

    pub fn clear(&mut self) {
        self.framebuffer.fill(0);
    }

    pub fn is_empty(&self) -> bool {
        self.width == 0 || self.height == 0
    }

    pub fn aspect_ratio(&self) -> f64 {
        if self.height == 0 {
            return 0.0;
        }
        self.width as f64 / self.height as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_frame() {
        let frame = RenderFrame::new(1920, 1080, 1);
        assert_eq!(frame.width, 1920);
        assert_eq!(frame.height, 1080);
        assert_eq!(frame.frame_index, 1);
    }

    #[test]
    fn test_framebuffer_size() {
        let frame = RenderFrame::new(100, 100, 0);
        assert_eq!(frame.framebuffer.len(), 100 * 100 * 4);
    }

    #[test]
    fn test_clear() {
        let mut frame = RenderFrame::new(10, 10, 0);
        frame.framebuffer[42] = 0xFF;
        frame.clear();
        assert!(frame.framebuffer.iter().all(|&b| b == 0));
    }

    #[test]
    fn test_is_empty() {
        let frame = RenderFrame::new(0, 0, 0);
        assert!(frame.is_empty());
        let frame = RenderFrame::new(1, 1, 0);
        assert!(!frame.is_empty());
    }

    #[test]
    fn test_aspect_ratio() {
        let frame = RenderFrame::new(1920, 1080, 0);
        assert!((frame.aspect_ratio() - 16.0 / 9.0).abs() < 0.001);
    }

    #[test]
    fn test_unique_frame_ids() {
        let a = RenderFrame::new(1, 1, 0);
        let b = RenderFrame::new(1, 1, 0);
        assert_ne!(a.frame_id, b.frame_id);
    }
}
