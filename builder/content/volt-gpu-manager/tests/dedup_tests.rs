use volt_gpu_manager::resources::{GpuDedupTable, GpuFingerprint, GpuResourceId};

#[test]
fn identical_resources_dedup() {
    let table = GpuDedupTable::new();
    let fp = GpuFingerprint::from_data(b"same data");
    let id = GpuResourceId::new();
    table.insert(fp, id).unwrap();
    let found = table.lookup(&fp);
    assert_eq!(found, Some(id));
}

#[test]
fn different_not_dedup() {
    let table = GpuDedupTable::new();
    let fp_a = GpuFingerprint::from_data(b"data A");
    let fp_b = GpuFingerprint::from_data(b"data B");
    assert_ne!(fp_a, fp_b);
    table.insert(fp_a, GpuResourceId::new()).unwrap();
    assert!(table.lookup(&fp_b).is_none());
}

#[test]
fn hash_collision_uses_verifier() {
    let fp = GpuFingerprint::from_data(b"verify me");
    let data = b"verify me";
    let result = volt_gpu_manager::resources::GpuVerifier::verify(data, &fp);
    assert!(result.is_ok());
}

#[test]
fn refcount_increments() {
    let rc = volt_gpu_manager::resources::RefCounter::new();
    let prev = rc.increment();
    assert_eq!(prev, 2);
    assert_eq!(rc.count(), 2);
}

#[test]
fn refcount_decrements() {
    let rc = volt_gpu_manager::resources::RefCounter::with_count(5);
    let prev = rc.decrement();
    assert_eq!(prev, 4);
    assert_eq!(rc.count(), 4);
}
