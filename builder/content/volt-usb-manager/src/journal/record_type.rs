#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum RecordType {
    BeginWrite = 1,
    CommitWrite = 2,
    AbortWrite = 3,
    BeginDelete = 4,
    CommitDelete = 5,
    BeginRename = 6,
    CommitRename = 7,
    Checkpoint = 8,
    CleanShutdown = 9,
}

impl RecordType {
    pub fn from_byte(b: u8) -> Option<Self> {
        match b {
            1 => Some(Self::BeginWrite),
            2 => Some(Self::CommitWrite),
            3 => Some(Self::AbortWrite),
            4 => Some(Self::BeginDelete),
            5 => Some(Self::CommitDelete),
            6 => Some(Self::BeginRename),
            7 => Some(Self::CommitRename),
            8 => Some(Self::Checkpoint),
            9 => Some(Self::CleanShutdown),
            _ => None,
        }
    }

    pub fn is_begin(&self) -> bool {
        matches!(self, Self::BeginWrite | Self::BeginDelete | Self::BeginRename)
    }

    pub fn is_commit(&self) -> bool {
        matches!(self, Self::CommitWrite | Self::CommitDelete | Self::CommitRename)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_byte_valid() {
        assert_eq!(RecordType::from_byte(1), Some(RecordType::BeginWrite));
        assert_eq!(RecordType::from_byte(9), Some(RecordType::CleanShutdown));
    }

    #[test]
    fn test_from_byte_invalid() {
        assert_eq!(RecordType::from_byte(0), None);
        assert_eq!(RecordType::from_byte(10), None);
        assert_eq!(RecordType::from_byte(255), None);
    }

    #[test]
    fn test_is_begin() {
        assert!(RecordType::BeginWrite.is_begin());
        assert!(RecordType::BeginDelete.is_begin());
        assert!(RecordType::BeginRename.is_begin());
        assert!(!RecordType::CommitWrite.is_begin());
        assert!(!RecordType::Checkpoint.is_begin());
    }

    #[test]
    fn test_is_commit() {
        assert!(RecordType::CommitWrite.is_commit());
        assert!(RecordType::CommitDelete.is_commit());
        assert!(RecordType::CommitRename.is_commit());
        assert!(!RecordType::BeginWrite.is_commit());
        assert!(!RecordType::AbortWrite.is_commit());
        assert!(!RecordType::CleanShutdown.is_commit());
    }

    #[test]
    fn test_repr_values() {
        assert_eq!(RecordType::BeginWrite as u8, 1);
        assert_eq!(RecordType::CleanShutdown as u8, 9);
    }

    #[test]
    fn test_all_variants_roundtrip() {
        let variants = [
            RecordType::BeginWrite,
            RecordType::CommitWrite,
            RecordType::AbortWrite,
            RecordType::BeginDelete,
            RecordType::CommitDelete,
            RecordType::BeginRename,
            RecordType::CommitRename,
            RecordType::Checkpoint,
            RecordType::CleanShutdown,
        ];
        for v in &variants {
            let byte = *v as u8;
            let back = RecordType::from_byte(byte).unwrap();
            assert_eq!(*v, back);
        }
    }
}
