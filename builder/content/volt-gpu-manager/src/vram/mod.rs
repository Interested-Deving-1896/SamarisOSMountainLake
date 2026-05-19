pub mod compressed_pool;
pub mod eviction;
pub mod pressure;
pub mod quotas;
pub mod residency;
pub mod restore;
pub mod scratch;
pub mod t1_active;
pub mod t2_compressed;
pub mod t3_fallback;
pub mod tier;

pub use tier::VramResidencyTier;
pub use residency::VramResidencyManager;
pub use pressure::VramPressure;
pub use eviction::EvictionTarget;
pub use eviction::VramEviction;
