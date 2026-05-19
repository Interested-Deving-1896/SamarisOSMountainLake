use crate::compression::algorithm::CompressionAlgorithm;
use crate::compression::compressed_page::CompressedPage;
use crate::compression::lz4_backend::Lz4Backend;
use crate::compression::zstd_backend::ZstdBackend;
use crate::core::result::VrmResult;

pub struct Compressor;

impl Compressor {
    pub fn compress(data: &[u8], algo: CompressionAlgorithm) -> VrmResult<CompressedPage> {
        if data.len() < 64 {
            return Ok(CompressedPage::new(
                data.len() as u64,
                CompressionAlgorithm::None,
                data.to_vec(),
            ));
        }

        match algo {
            CompressionAlgorithm::None => Ok(CompressedPage::new(
                data.len() as u64,
                CompressionAlgorithm::None,
                data.to_vec(),
            )),
            CompressionAlgorithm::Zstd { level } => {
                let compressed = ZstdBackend::compress(data, level)?;
                Ok(CompressedPage::new(data.len() as u64, algo, compressed))
            }
            CompressionAlgorithm::Lz4 => {
                let compressed = Lz4Backend::compress(data)?;
                Ok(CompressedPage::new(data.len() as u64, algo, compressed))
            }
        }
    }

    pub fn decompress(page: &CompressedPage) -> VrmResult<Vec<u8>> {
        match page.algorithm {
            CompressionAlgorithm::None => Ok(page.data.clone()),
            CompressionAlgorithm::Zstd { .. } => ZstdBackend::decompress(&page.data),
            CompressionAlgorithm::Lz4 => Lz4Backend::decompress(&page.data),
        }
    }

    pub fn select_algorithm(compressible: bool, is_binary: bool) -> CompressionAlgorithm {
        if !compressible {
            return CompressionAlgorithm::None;
        }
        if is_binary {
            CompressionAlgorithm::Zstd { level: 3 }
        } else {
            CompressionAlgorithm::Lz4
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_data_skips_compression() {
        let data = vec![0u8; 10];
        let result = Compressor::compress(&data, CompressionAlgorithm::Zstd { level: 3 }).unwrap();
        assert_eq!(result.algorithm, CompressionAlgorithm::None);
    }

    #[test]
    fn test_none_algorithm() {
        let data = vec![1u8; 100];
        let result = Compressor::compress(&data, CompressionAlgorithm::None).unwrap();
        assert_eq!(result.algorithm, CompressionAlgorithm::None);
        assert_eq!(result.data, data);
    }

    #[test]
    fn test_decompress_none() {
        let data = vec![2u8; 50];
        let page = CompressedPage::new(50, CompressionAlgorithm::None, data.clone());
        let result = Compressor::decompress(&page).unwrap();
        assert_eq!(result, data);
    }

    #[test]
    fn test_select_algorithm() {
        assert_eq!(
            Compressor::select_algorithm(false, true),
            CompressionAlgorithm::None
        );
        assert_eq!(
            Compressor::select_algorithm(true, false),
            CompressionAlgorithm::Lz4
        );
        assert_eq!(
            Compressor::select_algorithm(true, true),
            CompressionAlgorithm::Zstd { level: 3 }
        );
    }
}
