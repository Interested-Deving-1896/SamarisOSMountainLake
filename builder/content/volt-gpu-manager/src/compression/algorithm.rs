#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GpuCompressionAlgorithm {
    None,
    VgmZstdL1,
    VgmLz4,
    NativeTextureCompression,
    CpuFallbackZstd,
    /// Legacy alias — maps to VgmZstdL1 in compressor
    Zstd,
    /// Legacy alias — maps to VgmLz4 in compressor
    Lz4,
    /// Legacy CRC-only wrapper
    Crc,
}

impl GpuCompressionAlgorithm {
    pub fn from_name(name: &str) -> Self {
        match name.to_lowercase().as_str() {
            "none" => GpuCompressionAlgorithm::None,
            "vgm_zstd_l1" | "vgmzstdl1" | "zstd" => GpuCompressionAlgorithm::VgmZstdL1,
            "vgm_lz4" | "vgmlz4" | "lz4" => GpuCompressionAlgorithm::VgmLz4,
            "native_texture_compression" | "nativetexture" | "ntc" => {
                GpuCompressionAlgorithm::NativeTextureCompression
            }
            "cpu_fallback_zstd" | "cpufallbackzstd" | "fallback" => {
                GpuCompressionAlgorithm::CpuFallbackZstd
            }
            "crc" => GpuCompressionAlgorithm::Crc,
            _ => GpuCompressionAlgorithm::None,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            GpuCompressionAlgorithm::None => "none",
            GpuCompressionAlgorithm::VgmZstdL1 => "vgm_zstd_l1",
            GpuCompressionAlgorithm::VgmLz4 => "vgm_lz4",
            GpuCompressionAlgorithm::NativeTextureCompression => "native_texture_compression",
            GpuCompressionAlgorithm::CpuFallbackZstd => "cpu_fallback_zstd",
            GpuCompressionAlgorithm::Zstd => "zstd",
            GpuCompressionAlgorithm::Lz4 => "lz4",
            GpuCompressionAlgorithm::Crc => "crc",
        }
    }

    pub fn is_gpu_accelerated(&self) -> bool {
        matches!(self, GpuCompressionAlgorithm::VgmZstdL1 | GpuCompressionAlgorithm::VgmLz4 | GpuCompressionAlgorithm::NativeTextureCompression)
    }

    pub fn is_native_texture(&self) -> bool {
        matches!(self, GpuCompressionAlgorithm::NativeTextureCompression)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_name_valid() {
        assert_eq!(GpuCompressionAlgorithm::from_name("none"), GpuCompressionAlgorithm::None);
        assert_eq!(GpuCompressionAlgorithm::from_name("zstd"), GpuCompressionAlgorithm::VgmZstdL1);
        assert_eq!(GpuCompressionAlgorithm::from_name("VgmLz4"), GpuCompressionAlgorithm::VgmLz4);
        assert_eq!(GpuCompressionAlgorithm::from_name("NATIVE_TEXTURE_COMPRESSION"), GpuCompressionAlgorithm::NativeTextureCompression);
        assert_eq!(GpuCompressionAlgorithm::from_name("fallback"), GpuCompressionAlgorithm::CpuFallbackZstd);
    }

    #[test]
    fn test_from_name_unknown_defaults_none() {
        assert_eq!(GpuCompressionAlgorithm::from_name("bogus"), GpuCompressionAlgorithm::None);
        assert_eq!(GpuCompressionAlgorithm::from_name(""), GpuCompressionAlgorithm::None);
    }

    #[test]
    fn test_name_roundtrip_primary() {
        for algo in &[GpuCompressionAlgorithm::None, GpuCompressionAlgorithm::VgmZstdL1, GpuCompressionAlgorithm::VgmLz4, GpuCompressionAlgorithm::NativeTextureCompression, GpuCompressionAlgorithm::CpuFallbackZstd] {
            assert_eq!(GpuCompressionAlgorithm::from_name(algo.name()), *algo);
        }
    }

    #[test]
    fn test_legacy_aliases_map_to_primary() {
        assert_eq!(GpuCompressionAlgorithm::from_name("zstd"), GpuCompressionAlgorithm::VgmZstdL1);
        assert_eq!(GpuCompressionAlgorithm::from_name("lz4"), GpuCompressionAlgorithm::VgmLz4);
    }

    #[test]
    fn test_legacy_names_differ() {
        assert_eq!(GpuCompressionAlgorithm::Zstd.name(), "zstd");
        assert_eq!(GpuCompressionAlgorithm::Lz4.name(), "lz4");
        assert_eq!(GpuCompressionAlgorithm::Crc.name(), "crc");
    }

    #[test]
    fn test_is_gpu_accelerated() {
        assert!(!GpuCompressionAlgorithm::None.is_gpu_accelerated());
        assert!(GpuCompressionAlgorithm::VgmZstdL1.is_gpu_accelerated());
        assert!(GpuCompressionAlgorithm::VgmLz4.is_gpu_accelerated());
        assert!(GpuCompressionAlgorithm::NativeTextureCompression.is_gpu_accelerated());
        assert!(!GpuCompressionAlgorithm::CpuFallbackZstd.is_gpu_accelerated());
    }

    #[test]
    fn test_is_native_texture() {
        assert!(!GpuCompressionAlgorithm::None.is_native_texture());
        assert!(GpuCompressionAlgorithm::NativeTextureCompression.is_native_texture());
        assert!(!GpuCompressionAlgorithm::VgmZstdL1.is_native_texture());
    }

    #[test]
    fn test_clone_copy_eq() {
        let a = GpuCompressionAlgorithm::VgmLz4;
        let b = a;
        assert_eq!(a, b);
        assert_ne!(a, GpuCompressionAlgorithm::None);
    }
}
