use sha2::{Digest, Sha256};

use crate::core::error::VumError;
use crate::core::result::VumResult;
use crate::journal::record_type::RecordType;

pub const RECORD_MAGIC: u32 = 0x4A524E4C;

#[derive(Debug, Clone)]
pub struct JournalRecord {
    pub magic: u32,
    pub version: u8,
    pub record_type: RecordType,
    pub record_id: u64,
    pub timestamp_us: u64,
    pub path: String,
    pub data: Vec<u8>,
    pub data_hash: [u8; 32],
    pub checksum: u32,
}

impl JournalRecord {
    pub fn new(record_type: RecordType, record_id: u64, path: &str, data: Vec<u8>) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(&data);
        let data_hash = hasher.finalize().into();

        let mut record = Self {
            magic: RECORD_MAGIC,
            version: 1,
            record_type,
            record_id,
            timestamp_us: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_micros() as u64,
            path: path.to_string(),
            data,
            data_hash,
            checksum: 0,
        };
        record.checksum = record.compute_checksum();
        record
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let path_bytes = self.path.as_bytes();
        let mut buf = Vec::with_capacity(self.size());
        buf.extend_from_slice(&self.magic.to_le_bytes());
        buf.push(self.version);
        buf.push(self.record_type as u8);
        buf.extend_from_slice(&self.record_id.to_le_bytes());
        buf.extend_from_slice(&self.timestamp_us.to_le_bytes());
        buf.extend_from_slice(&(path_bytes.len() as u32).to_le_bytes());
        buf.extend_from_slice(path_bytes);
        buf.extend_from_slice(&(self.data.len() as u32).to_le_bytes());
        buf.extend_from_slice(&self.data);
        buf.extend_from_slice(&self.data_hash);
        buf.extend_from_slice(&self.checksum.to_le_bytes());
        buf
    }

    pub fn from_bytes(data: &[u8]) -> VumResult<Self> {
        if data.len() < 66 {
            return Err(VumError::JournalCorrupt(
                "Record too short for header".into(),
            ));
        }
        let mut offset = 0;
        let magic = u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap());
        offset += 4;
        if magic != RECORD_MAGIC {
            return Err(VumError::JournalCorrupt(format!(
                "Invalid magic: expected {:#X}, got {:#X}",
                RECORD_MAGIC, magic
            )));
        }
        let version = data[offset];
        offset += 1;
        let record_type =
            RecordType::from_byte(data[offset])
                .ok_or_else(|| VumError::JournalCorrupt(format!("Invalid record type byte: {}", data[offset])))?;
        offset += 1;
        let record_id = u64::from_le_bytes(data[offset..offset + 8].try_into().unwrap());
        offset += 8;
        let timestamp_us = u64::from_le_bytes(data[offset..offset + 8].try_into().unwrap());
        offset += 8;
        let path_len = u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap()) as usize;
        offset += 4;
        if offset + path_len > data.len() {
            return Err(VumError::JournalCorrupt("Path length exceeds buffer".into()));
        }
        let path = String::from_utf8(data[offset..offset + path_len].to_vec())
            .map_err(|_| VumError::JournalCorrupt("Invalid UTF-8 in path".into()))?;
        offset += path_len;
        if offset + 4 > data.len() {
            return Err(VumError::JournalCorrupt("Missing data length field".into()));
        }
        let data_len = u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap()) as usize;
        offset += 4;
        let record_data_end = offset + data_len + 32 + 4;
        if record_data_end > data.len() {
            return Err(VumError::JournalCorrupt("Data length exceeds buffer".into()));
        }
        let record_data = data[offset..offset + data_len].to_vec();
        offset += data_len;
        let mut data_hash = [0u8; 32];
        data_hash.copy_from_slice(&data[offset..offset + 32]);
        offset += 32;
        let stored_cs = u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap());

        let record = Self {
            magic,
            version,
            record_type,
            record_id,
            timestamp_us,
            path,
            data: record_data,
            data_hash,
            checksum: stored_cs,
        };
        if !record.verify_checksum() {
            return Err(VumError::JournalChecksumFailed);
        }
        Ok(record)
    }

    pub fn compute_checksum(&self) -> u32 {
        let path_bytes = self.path.as_bytes();
        let mut hasher = crc32fast::Hasher::new();
        hasher.update(&self.magic.to_le_bytes());
        hasher.update(&[self.version]);
        hasher.update(&[self.record_type as u8]);
        hasher.update(&self.record_id.to_le_bytes());
        hasher.update(&self.timestamp_us.to_le_bytes());
        hasher.update(&(path_bytes.len() as u32).to_le_bytes());
        hasher.update(path_bytes);
        hasher.update(&(self.data.len() as u32).to_le_bytes());
        hasher.update(&self.data);
        hasher.update(&self.data_hash);
        hasher.finalize()
    }

    pub fn verify_checksum(&self) -> bool {
        self.checksum == self.compute_checksum()
    }

    pub fn size(&self) -> usize {
        4 + 1 + 1 + 8 + 8 + 4 + self.path.len() + 4 + self.data.len() + 32 + 4
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_record() -> JournalRecord {
        JournalRecord::new(RecordType::BeginWrite, 42, "/test/file.bin", vec![1, 2, 3, 4, 5])
    }

    #[test]
    fn test_record_new_and_fields() {
        let r = make_record();
        assert_eq!(r.magic, RECORD_MAGIC);
        assert_eq!(r.version, 1);
        assert_eq!(r.record_type, RecordType::BeginWrite);
        assert_eq!(r.record_id, 42);
        assert_eq!(r.path, "/test/file.bin");
        assert_eq!(r.data, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_record_checksum_auto_verify() {
        let r = make_record();
        assert!(r.verify_checksum());
    }

    #[test]
    fn test_record_to_bytes_roundtrip() {
        let r = make_record();
        let bytes = r.to_bytes();
        let r2 = JournalRecord::from_bytes(&bytes).unwrap();
        assert_eq!(r.record_id, r2.record_id);
        assert_eq!(r.record_type, r2.record_type);
        assert_eq!(r.path, r2.path);
        assert_eq!(r.data, r2.data);
        assert_eq!(r.timestamp_us, r2.timestamp_us);
    }

    #[test]
    fn test_record_from_bytes_invalid_magic() {
        let r = make_record();
        let mut bytes = r.to_bytes();
        bytes[0] = 0xFF;
        bytes[1] = 0xFF;
        bytes[2] = 0xFF;
        bytes[3] = 0xFF;
        let result = JournalRecord::from_bytes(&bytes);
        assert!(result.is_err());
    }

    #[test]
    fn test_record_from_bytes_truncated() {
        let r = make_record();
        let bytes = r.to_bytes();
        let truncated = &bytes[..20];
        let result = JournalRecord::from_bytes(truncated);
        assert!(result.is_err());
    }

    #[test]
    fn test_record_size_consistency() {
        let r = make_record();
        let bytes = r.to_bytes();
        assert_eq!(bytes.len(), r.size());
    }

    #[test]
    fn test_record_compute_checksum_different() {
        let r1 = JournalRecord::new(RecordType::BeginWrite, 1, "/a", vec![0]);
        let r2 = JournalRecord::new(RecordType::BeginWrite, 2, "/b", vec![1]);
        assert_ne!(r1.checksum, r2.checksum);
    }

    #[test]
    fn test_record_new_with_empty_data() {
        let r = JournalRecord::new(RecordType::Checkpoint, 0, "/cp", vec![]);
        assert!(r.verify_checksum());
        assert_eq!(r.data.len(), 0);
    }

    #[test]
    fn test_record_from_bytes_tampered_data() {
        let r = make_record();
        let mut bytes = r.to_bytes();
        let path_len = u32::from_le_bytes(bytes[22..26].try_into().unwrap()) as usize;
        let data_offset = 26 + path_len;
        let data_len_pos = data_offset;
        let data_len = u32::from_le_bytes(bytes[data_len_pos..data_len_pos + 4].try_into().unwrap()) as usize;
        let data_start = data_len_pos + 4;
        if data_len > 0 {
            bytes[data_start] ^= 0xFF;
        }
        let result = JournalRecord::from_bytes(&bytes);
        assert!(result.is_err());
    }

    #[test]
    fn test_record_empty_path() {
        let r = JournalRecord::new(RecordType::BeginWrite, 1, "", vec![10]);
        assert_eq!(r.path, "");
        assert!(r.verify_checksum());
        let bytes = r.to_bytes();
        let r2 = JournalRecord::from_bytes(&bytes).unwrap();
        assert_eq!(r2.path, "");
    }
}
