#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocationKind {
    Generic,
    UI,
    Image,
    Audio,
    Video,
    Binary,
    Text,
    Crypto,
    FileCache,
    Shm,
}

impl AllocationKind {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Generic => "generic",
            Self::UI => "ui",
            Self::Image => "image",
            Self::Audio => "audio",
            Self::Video => "video",
            Self::Binary => "binary",
            Self::Text => "text",
            Self::Crypto => "crypto",
            Self::FileCache => "file_cache",
            Self::Shm => "shm",
        }
    }

    pub fn is_compressible(&self) -> bool {
        matches!(
            self,
            Self::Image | Self::Audio | Self::Video | Self::Text | Self::FileCache
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kind_name() {
        assert_eq!(AllocationKind::Generic.name(), "generic");
        assert_eq!(AllocationKind::UI.name(), "ui");
        assert_eq!(AllocationKind::Audio.name(), "audio");
        assert_eq!(AllocationKind::Shm.name(), "shm");
    }

    #[test]
    fn test_is_compressible() {
        assert!(AllocationKind::Image.is_compressible());
        assert!(AllocationKind::Audio.is_compressible());
        assert!(AllocationKind::Video.is_compressible());
        assert!(!AllocationKind::Generic.is_compressible());
        assert!(!AllocationKind::Binary.is_compressible());
        assert!(!AllocationKind::Crypto.is_compressible());
        assert!(!AllocationKind::Shm.is_compressible());
    }
}
