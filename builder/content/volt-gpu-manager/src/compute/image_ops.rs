use crate::compute::compute_job::GpuComputeJobKind;

pub struct GpuImageOp {
    pub kind: GpuComputeJobKind,
    pub width: u32,
    pub height: u32,
    pub input: Vec<u8>,
}

impl GpuImageOp {
    pub fn new(kind: GpuComputeJobKind, w: u32, h: u32) -> Self {
        let pixel_count = w as u64 * h as u64;
        let size = (pixel_count * 4) as usize;
        Self {
            kind,
            width: w,
            height: h,
            input: vec![0u8; size],
        }
    }

    pub fn pixel_count(&self) -> u64 {
        self.width as u64 * self.height as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_image_op() {
        let op = GpuImageOp::new(GpuComputeJobKind::Blur, 1920, 1080);
        assert_eq!(op.width, 1920);
        assert_eq!(op.height, 1080);
    }

    #[test]
    fn test_pixel_count() {
        let op = GpuImageOp::new(GpuComputeJobKind::Blur, 640, 480);
        assert_eq!(op.pixel_count(), 307200);
    }

    #[test]
    fn test_input_size_is_rgba() {
        let op = GpuImageOp::new(GpuComputeJobKind::Composite, 100, 100);
        assert_eq!(op.input.len(), 100 * 100 * 4);
    }

    #[test]
    fn test_kind_is_preserved() {
        let op = GpuImageOp::new(GpuComputeJobKind::Shadow, 800, 600);
        assert_eq!(op.kind, GpuComputeJobKind::Shadow);
    }

    #[test]
    fn test_zero_resolution() {
        let op = GpuImageOp::new(GpuComputeJobKind::Transform2D, 0, 0);
        assert_eq!(op.pixel_count(), 0);
        assert!(op.input.is_empty());
    }
}
