use std::fs;

use volt_usb_manager::cache::cache_entry::CacheEntry;
use volt_usb_manager::cache::cache_key::CacheKey;
use volt_usb_manager::cache::read_cache::ReadCache;
use volt_usb_manager::prefetch::boot_assets::BootAssetPrefetcher;

fn main() {
    let base = std::env::temp_dir().join("volt_prefetch_example");
    fs::create_dir_all(&base).ok();
    fs::create_dir_all(base.join("assets")).ok();
    fs::create_dir_all(base.join("wallpapers")).ok();
    fs::create_dir_all(base.join("brand")).ok();

    fs::write(base.join("index.html"), "<html></html>").unwrap();
    fs::write(base.join("assets/app.js"), "// js").unwrap();
    fs::write(base.join("assets/app.css"), "/* css */").unwrap();
    fs::write(base.join("wallpapers/desktop.png"), "png").unwrap();
    fs::write(base.join("brand/logo.png"), "png").unwrap();

    let mut cache = ReadCache::new(256);
    let root = base.to_str().unwrap().to_string();

    let assets = BootAssetPrefetcher::discover_assets(&root);
    println!("Discovered {} boot assets:", assets.len());
    for asset in &assets {
        println!("  {}", asset);
    }

    let plan = BootAssetPrefetcher::prefetch(&assets, 10 * 1024 * 1024).unwrap();
    println!("Prefetch plan: {} files, {} total bytes, {} prefetched bytes",
        plan.files.len(), plan.total_bytes, plan.prefetched_bytes);

    for file_path in &plan.files {
        if let Ok(data) = fs::read(file_path) {
            let key = CacheKey::new(file_path, 0, data.len() as u64);
            let entry = CacheEntry::new(key.clone(), file_path, data);
            if plan.pinned.iter().any(|p| p == file_path) {
                cache.pin(key.clone());
                println!("  (pinned) ",);
            }
            cache.insert(key, entry).expect("Failed to insert into cache");
            println!("  Cached: {}", file_path);
        }
    }

    println!("Cache entries: {}, used: {} bytes", cache.entry_count(), cache.used_bytes());
    println!("Boot asset prefetch complete.");
}
