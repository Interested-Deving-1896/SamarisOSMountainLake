#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GpuResourceUsage {
    DesktopFrame,
    UiAsset,
    OrbitCompute,
    ImageProcessing,
    Video,
    Background,
    Cache,
    Temporary,
}

impl GpuResourceUsage {
    pub fn name(&self) -> &'static str {
        match self {
            GpuResourceUsage::DesktopFrame => "DesktopFrame",
            GpuResourceUsage::UiAsset => "UiAsset",
            GpuResourceUsage::OrbitCompute => "OrbitCompute",
            GpuResourceUsage::ImageProcessing => "ImageProcessing",
            GpuResourceUsage::Video => "Video",
            GpuResourceUsage::Background => "Background",
            GpuResourceUsage::Cache => "Cache",
            GpuResourceUsage::Temporary => "Temporary",
        }
    }

    pub fn is_critical_frame(&self) -> bool {
        matches!(
            self,
            GpuResourceUsage::DesktopFrame | GpuResourceUsage::OrbitCompute
        )
    }

    pub fn can_compress(&self) -> bool {
        matches!(
            self,
            GpuResourceUsage::Background
                | GpuResourceUsage::Cache
                | GpuResourceUsage::Temporary
                | GpuResourceUsage::Video
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usage_names() {
        assert_eq!(GpuResourceUsage::DesktopFrame.name(), "DesktopFrame");
        assert_eq!(GpuResourceUsage::Temporary.name(), "Temporary");
    }

    #[test]
    fn test_is_critical_frame() {
        assert!(GpuResourceUsage::DesktopFrame.is_critical_frame());
        assert!(GpuResourceUsage::OrbitCompute.is_critical_frame());
        assert!(!GpuResourceUsage::Cache.is_critical_frame());
    }

    #[test]
    fn test_can_compress() {
        assert!(GpuResourceUsage::Background.can_compress());
        assert!(GpuResourceUsage::Cache.can_compress());
        assert!(GpuResourceUsage::Temporary.can_compress());
        assert!(!GpuResourceUsage::DesktopFrame.can_compress());
        assert!(!GpuResourceUsage::UiAsset.can_compress());
    }

    #[test]
    fn test_all_usages_have_names() {
        let usages = vec![
            GpuResourceUsage::DesktopFrame,
            GpuResourceUsage::UiAsset,
            GpuResourceUsage::OrbitCompute,
            GpuResourceUsage::ImageProcessing,
            GpuResourceUsage::Video,
            GpuResourceUsage::Background,
            GpuResourceUsage::Cache,
            GpuResourceUsage::Temporary,
        ];
        for u in usages {
            assert!(!u.name().is_empty());
        }
    }

    #[test]
    fn test_clone() {
        let a = GpuResourceUsage::Video;
        assert_eq!(a.clone(), a);
    }

    #[test]
    fn test_non_critical_not_compressible() {
        assert!(!GpuResourceUsage::UiAsset.can_compress());
        assert!(!GpuResourceUsage::ImageProcessing.can_compress());
        assert!(!GpuResourceUsage::UiAsset.is_critical_frame());
    }
}
