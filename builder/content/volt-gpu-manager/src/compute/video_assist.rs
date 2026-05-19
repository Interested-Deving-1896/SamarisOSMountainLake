use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct GpuVideoAssistJob {
    pub job_id: Uuid,
    pub frame_index: u64,
    pub width: u32,
    pub height: u32,
    pub format: VideoFrameFormat,
    pub frame_data: Vec<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VideoFrameFormat {
    NV12,
    I420,
    RGBA8,
    BGRA8,
}

impl VideoFrameFormat {
    pub fn name(&self) -> &'static str {
        match self {
            VideoFrameFormat::NV12 => "nv12",
            VideoFrameFormat::I420 => "i420",
            VideoFrameFormat::RGBA8 => "rgba8",
            VideoFrameFormat::BGRA8 => "bgra8",
        }
    }

    pub fn bytes_per_pixel(&self) -> u32 {
        match self {
            VideoFrameFormat::NV12 => 1,
            VideoFrameFormat::I420 => 1,
            VideoFrameFormat::RGBA8 => 4,
            VideoFrameFormat::BGRA8 => 4,
        }
    }
}

impl GpuVideoAssistJob {
    pub fn new(frame_index: u64, width: u32, height: u32, format: VideoFrameFormat) -> Self {
        let pixel_count = width as u64 * height as u64;
        let data_size = (pixel_count * format.bytes_per_pixel() as u64) as usize;
        Self {
            job_id: Uuid::new_v4(),
            frame_index,
            width,
            height,
            format,
            frame_data: vec![0u8; data_size],
        }
    }

    pub fn frame_size_bytes(&self) -> u64 {
        self.width as u64 * self.height as u64 * self.format.bytes_per_pixel() as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_video_job() {
        let job = GpuVideoAssistJob::new(42, 1920, 1080, VideoFrameFormat::NV12);
        assert_eq!(job.frame_index, 42);
        assert_eq!(job.width, 1920);
        assert_eq!(job.height, 1080);
    }

    #[test]
    fn test_frame_size_rgba() {
        let job = GpuVideoAssistJob::new(0, 640, 480, VideoFrameFormat::RGBA8);
        assert_eq!(job.frame_size_bytes(), 640 * 480 * 4);
    }

    #[test]
    fn test_frame_size_nv12() {
        let job = GpuVideoAssistJob::new(0, 640, 480, VideoFrameFormat::NV12);
        assert_eq!(job.frame_size_bytes(), 640 * 480 * 1);
    }

    #[test]
    fn test_unique_job_ids() {
        let a = GpuVideoAssistJob::new(1, 100, 100, VideoFrameFormat::I420);
        let b = GpuVideoAssistJob::new(1, 100, 100, VideoFrameFormat::I420);
        assert_ne!(a.job_id, b.job_id);
    }

    #[test]
    fn test_format_name() {
        assert_eq!(VideoFrameFormat::NV12.name(), "nv12");
        assert_eq!(VideoFrameFormat::RGBA8.name(), "rgba8");
        assert_eq!(VideoFrameFormat::I420.name(), "i420");
        assert_eq!(VideoFrameFormat::BGRA8.name(), "bgra8");
    }
}
