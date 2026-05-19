use volt_usb_manager::cache::cache_entry::CacheEntry;
use volt_usb_manager::cache::cache_key::CacheKey;
use volt_usb_manager::cache::read_cache::ReadCache;

fn make_key(id: u8) -> CacheKey {
    CacheKey::new(&format!("/cache_test/{}", id), id as u64, 100)
}

#[test]
fn test_read_cache_insert_get() {
    let mut cache = ReadCache::new(10);
    let key = make_key(1);
    let data = vec![10, 20, 30, 40, 50];
    let entry = CacheEntry::new(key.clone(), "/cache_test/1", data.clone());
    cache.insert(key.clone(), entry).unwrap();
    let result = cache.get(&key);
    assert_eq!(result, Some(data));
    assert_eq!(cache.hit_count(), 1);
    assert_eq!(cache.miss_count(), 0);
}

#[test]
fn test_miss_then_hit() {
    let mut cache = ReadCache::new(10);
    let key = make_key(7);
    let miss = cache.get(&key);
    assert_eq!(miss, None);
    assert_eq!(cache.miss_count(), 1);
    let entry = CacheEntry::new(key.clone(), "/cache_test/7", vec![99]);
    cache.insert(key.clone(), entry).unwrap();
    let hit = cache.get(&key);
    assert_eq!(hit, Some(vec![99]));
    assert_eq!(cache.hit_count(), 1);
    assert_eq!(cache.miss_count(), 1);
}

#[test]
fn test_lru_eviction() {
    let mut cache = ReadCache::new(1);
    let key1 = make_key(1);
    let key2 = make_key(2);
    let big = vec![0u8; 800 * 1024];
    let entry1 = CacheEntry::new(key1.clone(), "/1", big.clone());
    cache.insert(key1.clone(), entry1).unwrap();
    let entry2 = CacheEntry::new(key2.clone(), "/2", big);
    cache.insert(key2.clone(), entry2).unwrap();
    assert!(cache.get(&key1).is_none() || cache.entry_count() <= 1);
}

#[test]
fn test_pinned_entry_not_evicted() {
    let mut cache = ReadCache::new(10);
    let pinned_key = make_key(1);
    let other1_key = make_key(2);
    let other2_key = make_key(3);
    let data = vec![0u8; 100];
    cache.pin(pinned_key.clone());
    let entry = CacheEntry::new(pinned_key.clone(), "/pinned", data.clone());
    cache.insert(pinned_key.clone(), entry).unwrap();
    cache.insert(other1_key.clone(), CacheEntry::new(other1_key.clone(), "/other1", data.clone())).unwrap();
    cache.insert(other2_key.clone(), CacheEntry::new(other2_key.clone(), "/other2", data)).unwrap();
    assert!(cache.get(&pinned_key).is_some());
    assert!(cache.get(&other1_key).is_some() || cache.get(&other2_key).is_some());
}

#[test]
fn test_compressed_cache_roundtrip() {
    use volt_usb_manager::compression::algorithm::CompressionAlgorithm;
    let mut cache = ReadCache::new(10);
    let key = make_key(9);
    let data = vec![0xABu8; 5000];
    cache
        .insert_compressed(key.clone(), data.clone(), CompressionAlgorithm::Zstd { level: 3 })
        .unwrap();
    let result = cache.get(&key);
    assert_eq!(result, Some(data));
}

#[test]
fn test_evict_all_clears_non_pinned() {
    let mut cache = ReadCache::new(10);
    for i in 0..3 {
        let key = make_key(i);
        let entry = CacheEntry::new(key, &format!("/{}", i), vec![i; 100]);
        cache.insert(make_key(i), entry).unwrap();
    }
    cache.evict_all().unwrap();
    assert_eq!(cache.entry_count(), 0);
    assert_eq!(cache.used_bytes(), 0);
}

#[test]
fn test_cache_eviction_triggers() {
    let mut cache = ReadCache::new(1);
    for i in 0..5 {
        let key = make_key(i);
        let entry = CacheEntry::new(key, &format!("/evict/{}", i), vec![0u8; 300 * 1024]);
        cache.insert(make_key(i), entry).unwrap();
    }
    assert!(cache.entry_count() <= 3);
}

#[test]
fn test_entry_too_large_for_cache() {
    let mut cache = ReadCache::new(1);
    let key = make_key(1);
    let entry = CacheEntry::new(key, "/big", vec![0u8; 2 * 1024 * 1024]);
    let result = cache.insert(make_key(1), entry);
    assert!(result.is_err());
}

#[test]
fn test_hit_ratio_tracking() {
    let mut cache = ReadCache::new(10);
    let key = make_key(1);
    let entry = CacheEntry::new(key.clone(), "/t", vec![1]);
    cache.insert(key.clone(), entry).unwrap();
    let _ = cache.get(&key);
    let _ = cache.get(&make_key(99));
    assert!((cache.hit_ratio() - 0.5).abs() < 0.001);
}
