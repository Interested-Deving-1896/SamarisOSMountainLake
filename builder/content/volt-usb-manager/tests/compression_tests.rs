use volt_usb_manager::compression::algorithm::CompressionAlgorithm;
use volt_usb_manager::compression::compressed_blob::CompressedBlob;
use volt_usb_manager::compression::compressor::Compressor;
use volt_usb_manager::compression::ratio_tracker::RatioTracker;

#[test]
#[cfg(feature = "compression")]
fn test_zstd_roundtrip() {
    let data = vec![0xABu8; 10000];
    let blob = Compressor::compress(&data, CompressionAlgorithm::Zstd { level: 3 }).unwrap();
    assert!(blob.compressed_size < blob.uncompressed_size);
    let decompressed = Compressor::decompress(&blob).unwrap();
    assert_eq!(decompressed, data);
}

#[test]
#[cfg(feature = "compression")]
fn test_lz4_roundtrip() {
    let data = b"Hello, LZ4 compression in Volt USB Manager!";
    let blob = Compressor::compress(data, CompressionAlgorithm::Lz4).unwrap();
    let decompressed = Compressor::decompress(&blob).unwrap();
    assert_eq!(decompressed, data);
}

#[test]
fn test_ratio_tracker_uses_real_values() {
    let mut tracker = RatioTracker::new();
    assert!((tracker.average_ratio() - 1.0).abs() < 0.001);
    assert_eq!(tracker.savings(), 0);
    tracker.record(1000, 300);
    assert!((tracker.average_ratio() - 0.3).abs() < 0.001);
    assert_eq!(tracker.savings(), 700);
    assert_eq!(tracker.count(), 1);
}

#[test]
fn test_corrupt_compressed_blob_rejected() {
    let original = b"test data for corruption test";
    let blob = Compressor::compress(original, CompressionAlgorithm::None).unwrap();
    assert_eq!(blob.algorithm, CompressionAlgorithm::None);
    let result = Compressor::decompress(&blob).unwrap();
    assert_eq!(result, original);
}

#[test]
fn test_compression_ratio_tracker_multiple() {
    let mut tracker = RatioTracker::new();
    tracker.record(100, 50);
    tracker.record(200, 100);
    tracker.record(300, 150);
    assert!((tracker.average_ratio() - 0.5).abs() < 0.001);
    assert_eq!(tracker.savings(), 300);
    assert_eq!(tracker.count(), 3);
}

#[test]
fn test_compressed_blob_savings() {
    let blob = CompressedBlob::new(
        CompressionAlgorithm::Zstd { level: 3 },
        &[0u8; 100],
        vec![0u8; 30],
    );
    assert_eq!(blob.savings(), 70);
    assert!((blob.ratio() - 0.3).abs() < 0.001);
}

#[test]
fn test_select_algorithm_for_cache() {
    let algo = Compressor::select_for_cache("/test/file.txt", 100);
    assert_eq!(algo, CompressionAlgorithm::None);
    let algo_large = Compressor::select_for_cache("/test/data.bin", 5 * 1024 * 1024);
    #[cfg(feature = "compression")]
    assert_eq!(algo_large, CompressionAlgorithm::Zstd { level: 3 });
    #[cfg(not(feature = "compression"))]
    assert_eq!(algo_large, CompressionAlgorithm::None);
}

#[test]
fn test_compression_none_roundtrip() {
    let data = b"plain text data";
    let blob = Compressor::compress(data, CompressionAlgorithm::None).unwrap();
    let decompressed = Compressor::decompress(&blob).unwrap();
    assert_eq!(decompressed, data);
}

#[test]
fn test_no_savings_when_compression_expands() {
    let blob = CompressedBlob::new(
        CompressionAlgorithm::None,
        &[0u8; 10],
        vec![0u8; 100],
    );
    assert_eq!(blob.savings(), 0);
}

#[test]
fn test_ratio_tracker_empty() {
    let tracker = RatioTracker::new();
    assert!((tracker.average_ratio() - 1.0).abs() < 0.001);
    assert_eq!(tracker.count(), 0);
    assert_eq!(tracker.savings(), 0);
}
