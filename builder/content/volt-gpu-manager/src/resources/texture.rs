use crate::resources::resource_id::GpuResourceId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GpuTextureFormat {
    R8Unorm,
    Rgba8Unorm,
    Rgba8Srgb,
    Bgra8Unorm,
    Bgra8Srgb,
    R32Float,
    Rgba32Float,
    D32Float,
    D24PlusS8,
    Bc1,
    Bc3,
    Bc5,
    Bc7,
}

impl GpuTextureFormat {
    pub fn name(&self) -> &'static str {
        match self {
            GpuTextureFormat::R8Unorm => "R8Unorm",
            GpuTextureFormat::Rgba8Unorm => "Rgba8Unorm",
            GpuTextureFormat::Rgba8Srgb => "Rgba8Srgb",
            GpuTextureFormat::Bgra8Unorm => "Bgra8Unorm",
            GpuTextureFormat::Bgra8Srgb => "Bgra8Srgb",
            GpuTextureFormat::R32Float => "R32Float",
            GpuTextureFormat::Rgba32Float => "Rgba32Float",
            GpuTextureFormat::D32Float => "D32Float",
            GpuTextureFormat::D24PlusS8 => "D24PlusS8",
            GpuTextureFormat::Bc1 => "Bc1",
            GpuTextureFormat::Bc3 => "Bc3",
            GpuTextureFormat::Bc5 => "Bc5",
            GpuTextureFormat::Bc7 => "Bc7",
        }
    }

    pub fn bytes_per_pixel(&self) -> u32 {
        match self {
            GpuTextureFormat::R8Unorm => 1,
            GpuTextureFormat::Rgba8Unorm | GpuTextureFormat::Rgba8Srgb => 4,
            GpuTextureFormat::Bgra8Unorm | GpuTextureFormat::Bgra8Srgb => 4,
            GpuTextureFormat::R32Float => 4,
            GpuTextureFormat::Rgba32Float => 16,
            GpuTextureFormat::D32Float => 4,
            GpuTextureFormat::D24PlusS8 => 4,
            GpuTextureFormat::Bc1 => 8,
            GpuTextureFormat::Bc3 => 16,
            GpuTextureFormat::Bc5 => 16,
            GpuTextureFormat::Bc7 => 16,
        }
    }

    pub fn is_compressed(&self) -> bool {
        matches!(
            self,
            GpuTextureFormat::Bc1
                | GpuTextureFormat::Bc3
                | GpuTextureFormat::Bc5
                | GpuTextureFormat::Bc7
        )
    }
}

#[derive(Debug, Clone)]
pub struct GpuTexture {
    pub id: GpuResourceId,
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub format: GpuTextureFormat,
    pub mips: u32,
}

impl GpuTexture {
    pub fn new(
        id: GpuResourceId,
        width: u32,
        height: u32,
        depth: u32,
        format: GpuTextureFormat,
        mips: u32,
    ) -> Self {
        Self {
            id,
            width,
            height,
            depth,
            format,
            mips: if mips == 0 {
                1
            } else {
                mips.min((width.max(height) as f64).log2().ceil() as u32 + 1)
            },
        }
    }

    pub fn byte_size(&self) -> u64 {
        let mut total = 0u64;
        let mut w = self.width;
        let mut h = self.height;
        for _ in 0..self.mips {
            let pixels = (w.max(1) * h.max(1) * self.depth.max(1)) as u64;
            let level_size = if self.format.is_compressed() {
                let blocks_x = ((w.max(1) + 3) / 4) as u64;
                let blocks_y = ((h.max(1) + 3) / 4) as u64;
                blocks_x * blocks_y * self.format.bytes_per_pixel() as u64
            } else {
                pixels * self.format.bytes_per_pixel() as u64
            };
            total += level_size;
            w /= 2;
            h /= 2;
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_texture() {
        let id = GpuResourceId::new();
        let tex = GpuTexture::new(id, 256, 256, 1, GpuTextureFormat::Rgba8Unorm, 1);
        assert_eq!(tex.id, id);
        assert_eq!(tex.width, 256);
        assert_eq!(tex.height, 256);
        assert_eq!(tex.mips, 1);
    }

    #[test]
    fn test_byte_size_rgba8() {
        let tex = GpuTexture::new(
            GpuResourceId::new(),
            128,
            128,
            1,
            GpuTextureFormat::Rgba8Unorm,
            1,
        );
        assert_eq!(tex.byte_size(), (128 * 128 * 4) as u64);
    }

    #[test]
    fn test_byte_size_with_mipmaps() {
        let tex = GpuTexture::new(
            GpuResourceId::new(),
            64,
            64,
            1,
            GpuTextureFormat::Rgba8Unorm,
            7,
        );
        let expected = (64 * 64 + 32 * 32 + 16 * 16 + 8 * 8 + 4 * 4 + 2 * 2 + 1) * 4;
        assert_eq!(tex.byte_size(), expected as u64);
    }

    #[test]
    fn test_byte_size_compressed_format() {
        let tex = GpuTexture::new(
            GpuResourceId::new(),
            8,
            8,
            1,
            GpuTextureFormat::Bc1,
            1,
        );
        let expected = ((8u32 + 3) / 4) * ((8u32 + 3) / 4) * 8;
        assert_eq!(tex.byte_size(), expected as u64);
    }

    #[test]
    fn test_format_names() {
        assert_eq!(GpuTextureFormat::Rgba8Srgb.name(), "Rgba8Srgb");
        assert_eq!(GpuTextureFormat::Bc7.name(), "Bc7");
    }

    #[test]
    fn test_is_compressed() {
        assert!(GpuTextureFormat::Bc3.is_compressed());
        assert!(!GpuTextureFormat::R8Unorm.is_compressed());
    }

    #[test]
    fn test_bytes_per_pixel() {
        assert_eq!(GpuTextureFormat::R32Float.bytes_per_pixel(), 4);
        assert_eq!(GpuTextureFormat::Rgba32Float.bytes_per_pixel(), 16);
        assert_eq!(GpuTextureFormat::Bc1.bytes_per_pixel(), 8);
    }

    #[test]
    fn test_mips_auto_clamp() {
        let tex = GpuTexture::new(
            GpuResourceId::new(),
            8,
            8,
            1,
            GpuTextureFormat::R8Unorm,
            100,
        );
        // log2(8) = 3, so max mips = 4
        assert!(tex.mips <= 4);
    }

    #[test]
    fn test_zero_mips_defaults_to_one() {
        let tex = GpuTexture::new(
            GpuResourceId::new(),
            64,
            64,
            1,
            GpuTextureFormat::Rgba8Unorm,
            0,
        );
        assert_eq!(tex.mips, 1);
    }
}
