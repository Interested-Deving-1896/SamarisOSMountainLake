use crate::resources::resource_id::GpuResourceId;
use crate::resources::resource_type::GpuResourceType;
use crate::resources::resource_usage::GpuResourceUsage;
use crate::vram::tier::VramResidencyTier;
use crate::scheduler::GpuPriority;

#[derive(Debug, Clone)]
pub struct GpuResourceMeta {
    pub resource_id: GpuResourceId,
    pub app_id: u64,
    pub name: String,
    pub resource_type: GpuResourceType,
    pub usage: GpuResourceUsage,
    pub original_size: u64,
    pub current_size: u64,
    pub tier: VramResidencyTier,
    pub priority: GpuPriority,
    pub pinned: bool,
    pub compression_allowed: bool,
    pub last_access_ms: u64,
    pub access_count: u64,
}

impl GpuResourceMeta {
    pub fn new(
        id: GpuResourceId,
        app_id: u64,
        name: &str,
        rtype: GpuResourceType,
        usage: GpuResourceUsage,
        size: u64,
    ) -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;
        let can_compress = usage.can_compress();
        Self {
            resource_id: id,
            app_id,
            name: name.to_string(),
            resource_type: rtype,
            usage,
            original_size: size,
            current_size: size,
            tier: VramResidencyTier::T1Active,
            priority: GpuPriority::Normal,
            pinned: false,
            compression_allowed: can_compress,
            last_access_ms: now,
            access_count: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_meta(size: u64) -> GpuResourceMeta {
        GpuResourceMeta::new(
            GpuResourceId::new(),
            1,
            "test_resource",
            GpuResourceType::Texture2D,
            GpuResourceUsage::Background,
            size,
        )
    }

    #[test]
    fn test_new_meta() {
        let id = GpuResourceId::new();
        let meta = GpuResourceMeta::new(
            id,
            42,
            "my_texture",
            GpuResourceType::Texture2D,
            GpuResourceUsage::Background,
            65536,
        );
        assert_eq!(meta.resource_id, id);
        assert_eq!(meta.app_id, 42);
        assert_eq!(meta.name, "my_texture");
        assert_eq!(meta.original_size, 65536);
        assert_eq!(meta.current_size, 65536);
    }

    #[test]
    fn test_default_tier_and_priority() {
        let meta = make_meta(1024);
        assert_eq!(meta.tier, VramResidencyTier::T1Active);
        assert_eq!(meta.priority, GpuPriority::Normal);
    }

    #[test]
    fn test_compression_allowed_for_background() {
        let meta = GpuResourceMeta::new(
            GpuResourceId::new(),
            1,
            "bg",
            GpuResourceType::Texture2D,
            GpuResourceUsage::Background,
            100,
        );
        assert!(meta.compression_allowed);
    }

    #[test]
    fn test_compression_not_allowed_for_desktop_frame() {
        let meta = GpuResourceMeta::new(
            GpuResourceId::new(),
            1,
            "frame",
            GpuResourceType::Texture2D,
            GpuResourceUsage::DesktopFrame,
            100,
        );
        assert!(!meta.compression_allowed);
    }

    #[test]
    fn test_pinned_defaults_false() {
        let meta = make_meta(512);
        assert!(!meta.pinned);
    }

    #[test]
    fn test_last_access_ms_set() {
        let meta = make_meta(256);
        assert!(meta.last_access_ms > 0);
    }

    #[test]
    fn test_access_count_starts_zero() {
        let meta = make_meta(128);
        assert_eq!(meta.access_count, 0);
    }

    #[test]
    fn test_clone() {
        let meta = make_meta(1024);
        let cloned = meta.clone();
        assert_eq!(meta.name, cloned.name);
        assert_eq!(meta.original_size, cloned.original_size);
    }

    #[test]
    fn test_stores_resource_type() {
        let meta = GpuResourceMeta::new(
            GpuResourceId::new(),
            1,
            "shader",
            GpuResourceType::ShaderModule,
            GpuResourceUsage::Cache,
            2000,
        );
        assert_eq!(meta.resource_type, GpuResourceType::ShaderModule);
    }
}
