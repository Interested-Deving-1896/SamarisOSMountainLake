#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GpuResourceType {
    Buffer,
    Texture2D,
    Texture3D,
    RenderTarget,
    DepthTarget,
    ShaderModule,
    Pipeline,
    GlyphAtlas,
    TextureAtlas,
    CompressedBlob,
}

impl GpuResourceType {
    pub fn name(&self) -> &'static str {
        match self {
            GpuResourceType::Buffer => "Buffer",
            GpuResourceType::Texture2D => "Texture2D",
            GpuResourceType::Texture3D => "Texture3D",
            GpuResourceType::RenderTarget => "RenderTarget",
            GpuResourceType::DepthTarget => "DepthTarget",
            GpuResourceType::ShaderModule => "ShaderModule",
            GpuResourceType::Pipeline => "Pipeline",
            GpuResourceType::GlyphAtlas => "GlyphAtlas",
            GpuResourceType::TextureAtlas => "TextureAtlas",
            GpuResourceType::CompressedBlob => "CompressedBlob",
        }
    }

    pub fn needs_mipmaps(&self) -> bool {
        matches!(
            self,
            GpuResourceType::Texture2D | GpuResourceType::TextureAtlas
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_type_names() {
        assert_eq!(GpuResourceType::Buffer.name(), "Buffer");
        assert_eq!(GpuResourceType::Texture2D.name(), "Texture2D");
        assert_eq!(GpuResourceType::CompressedBlob.name(), "CompressedBlob");
    }

    #[test]
    fn test_needs_mipmaps() {
        assert!(GpuResourceType::Texture2D.needs_mipmaps());
        assert!(GpuResourceType::TextureAtlas.needs_mipmaps());
        assert!(!GpuResourceType::Buffer.needs_mipmaps());
        assert!(!GpuResourceType::RenderTarget.needs_mipmaps());
        assert!(!GpuResourceType::ShaderModule.needs_mipmaps());
    }

    #[test]
    fn test_all_types_have_names() {
        let types = vec![
            GpuResourceType::Buffer,
            GpuResourceType::Texture2D,
            GpuResourceType::Texture3D,
            GpuResourceType::RenderTarget,
            GpuResourceType::DepthTarget,
            GpuResourceType::ShaderModule,
            GpuResourceType::Pipeline,
            GpuResourceType::GlyphAtlas,
            GpuResourceType::TextureAtlas,
            GpuResourceType::CompressedBlob,
        ];
        for t in types {
            assert!(!t.name().is_empty());
        }
    }

    #[test]
    fn test_clone() {
        let a = GpuResourceType::Texture3D;
        assert_eq!(a.clone(), a);
    }

    #[test]
    fn test_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(GpuResourceType::Buffer);
        set.insert(GpuResourceType::Buffer);
        set.insert(GpuResourceType::Texture2D);
        assert_eq!(set.len(), 2);
    }
}
