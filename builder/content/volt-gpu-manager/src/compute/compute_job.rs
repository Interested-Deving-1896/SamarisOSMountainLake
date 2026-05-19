use uuid::Uuid;
use crate::scheduler::priority::GpuPriority;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GpuComputeJobKind {
    Blur,
    Shadow,
    Composite,
    Transform2D,
    MatMul,
    TexturePack,
    MipmapGenerate,
    VramCompress,
    VramDecompress,
    VideoAssist,
}

impl GpuComputeJobKind {
    pub fn name(&self) -> &'static str {
        match self {
            GpuComputeJobKind::Blur => "blur",
            GpuComputeJobKind::Shadow => "shadow",
            GpuComputeJobKind::Composite => "composite",
            GpuComputeJobKind::Transform2D => "transform_2d",
            GpuComputeJobKind::MatMul => "matmul",
            GpuComputeJobKind::TexturePack => "texture_pack",
            GpuComputeJobKind::MipmapGenerate => "mipmap_generate",
            GpuComputeJobKind::VramCompress => "vram_compress",
            GpuComputeJobKind::VramDecompress => "vram_decompress",
            GpuComputeJobKind::VideoAssist => "video_assist",
        }
    }
}

#[derive(Debug, Clone)]
pub struct GpuComputeJob {
    pub job_id: Uuid,
    pub kind: GpuComputeJobKind,
    pub priority: GpuPriority,
    pub input_size: u64,
    pub app_id: u64,
}

impl GpuComputeJob {
    pub fn new(kind: GpuComputeJobKind, priority: GpuPriority, app_id: u64) -> Self {
        Self {
            job_id: Uuid::new_v4(),
            kind,
            priority,
            input_size: 0,
            app_id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_job_has_unique_id() {
        let a = GpuComputeJob::new(GpuComputeJobKind::MatMul, GpuPriority::High, 1);
        let b = GpuComputeJob::new(GpuComputeJobKind::MatMul, GpuPriority::High, 1);
        assert_ne!(a.job_id, b.job_id);
    }

    #[test]
    fn test_job_kind_name() {
        assert_eq!(GpuComputeJobKind::Blur.name(), "blur");
        assert_eq!(GpuComputeJobKind::VideoAssist.name(), "video_assist");
    }

    #[test]
    fn test_job_priority() {
        let job = GpuComputeJob::new(GpuComputeJobKind::Composite, GpuPriority::Critical, 42);
        assert_eq!(job.priority, GpuPriority::Critical);
        assert_eq!(job.app_id, 42);
    }

    #[test]
    fn test_all_kinds_have_names() {
        let kinds = vec![
            GpuComputeJobKind::Blur,
            GpuComputeJobKind::Shadow,
            GpuComputeJobKind::Composite,
            GpuComputeJobKind::Transform2D,
            GpuComputeJobKind::MatMul,
            GpuComputeJobKind::TexturePack,
            GpuComputeJobKind::MipmapGenerate,
            GpuComputeJobKind::VramCompress,
            GpuComputeJobKind::VramDecompress,
            GpuComputeJobKind::VideoAssist,
        ];
        for k in kinds {
            assert!(!k.name().is_empty());
        }
    }
}
