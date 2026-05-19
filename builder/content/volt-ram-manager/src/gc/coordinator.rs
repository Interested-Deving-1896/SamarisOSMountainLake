use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use crate::config::schema::GcConfig;
use crate::core::result::VrmResult;
use crate::gc::animation_guard::AnimationGuard;
use crate::gc::cooldown::GcCooldown;
use crate::gc::report::GcReport;
use crate::gc::v8_signal::V8PressureSignal;
use crate::pages::page::Page;
use crate::pages::page_table::PageTable;
use crate::tiers::tier::MemoryTier;

pub struct GcCoordinator {
    config: GcConfig,
    cooldown: GcCooldown,
    running: AtomicBool,
    total_collected: AtomicU64,
    cycle_count: AtomicU64,
    animation_guard: AnimationGuard,
}

impl GcCoordinator {
    pub fn new(config: GcConfig) -> Self {
        Self {
            cooldown: GcCooldown::new(config.interval_ms),
            config,
            running: AtomicBool::new(false),
            total_collected: AtomicU64::new(0),
            cycle_count: AtomicU64::new(0),
            animation_guard: AnimationGuard::new(),
        }
    }

    pub fn run_cycle(&self, page_table: &PageTable) -> VrmResult<GcReport> {
        if self.animation_guard.is_active() {
            return Ok(GcReport::new());
        }

        if !self.cooldown.is_ready() {
            return Ok(GcReport::new());
        }

        let aggressive = self.should_run_aggressive(page_table);
        let max_pages = if aggressive {
            self.config.max_pages_per_cycle.saturating_mul(2)
        } else {
            self.config.max_pages_per_cycle
        } as usize;

        let candidates = self.collect_candidates(page_table, aggressive, max_pages);
        if candidates.is_empty() {
            return Ok(GcReport::new());
        }

        let mut report = if aggressive {
            GcReport::new_aggressive()
        } else {
            GcReport::new()
        };

        for page in &candidates {
            let _ = page_table.remove(page.id());
            report.pages_freed += 1;
            report.bytes_reclaimed += page.size();
        }

        let elapsed = self.cooldown.elapsed();
        report.duration_ms = elapsed.as_millis() as u64;

        self.total_collected.fetch_add(report.bytes_reclaimed, Ordering::SeqCst);
        self.cycle_count.fetch_add(1, Ordering::SeqCst);

        Ok(report)
    }

    fn collect_candidates(&self, page_table: &PageTable, aggressive: bool, max: usize) -> Vec<Page> {
        let mut candidates: Vec<Page> = Vec::new();
        for page in page_table.get_by_tier(MemoryTier::T3Compressed) {
            if candidates.len() >= max {
                break;
            }
            if !page.is_pinned() {
                candidates.push(page);
            }
        }
        if candidates.len() < max && aggressive {
            for page in page_table.get_by_tier(MemoryTier::T2Direct) {
                if candidates.len() >= max {
                    break;
                }
                if !page.is_pinned() {
                    candidates.push(page);
                }
            }
        }
        candidates
    }

    pub fn is_gc_needed(&self, page_table: &PageTable) -> bool {
        let total = page_table.total_bytes();
        if total == 0 {
            return false;
        }
        let compressed = page_table.total_bytes_by_tier(MemoryTier::T3Compressed);
        let ratio = compressed as f64 / total as f64 * 100.0;
        ratio >= self.config.threshold_percent
    }

    pub fn should_run_aggressive(&self, page_table: &PageTable) -> bool {
        let total = page_table.total_bytes();
        if total == 0 {
            return false;
        }
        let compressed = page_table.total_bytes_by_tier(MemoryTier::T3Compressed);
        let ratio = compressed as f64 / total as f64 * 100.0;
        ratio >= self.config.aggressive_threshold
    }

    pub fn handle_v8_signal(&mut self, signal: V8PressureSignal) -> VrmResult<GcReport> {
        match signal {
            V8PressureSignal::Critical => {
                self.cooldown.force_ready();
                Ok(GcReport::new_aggressive())
            }
            V8PressureSignal::Moderate => {
                self.cooldown.force_ready();
                Ok(GcReport::new())
            }
            V8PressureSignal::Low => Ok(GcReport::new()),
        }
    }

    pub fn cooldown(&self) -> &GcCooldown {
        &self.cooldown
    }

    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    pub fn total_collected_bytes(&self) -> u64 {
        self.total_collected.load(Ordering::SeqCst)
    }

    pub fn cycle_count(&self) -> u64 {
        self.cycle_count.load(Ordering::SeqCst)
    }

    pub fn animation_guard(&self) -> &AnimationGuard {
        &self.animation_guard
    }

    pub fn config(&self) -> &GcConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pages::page::Page;

    fn make_page(tier: MemoryTier, size: u64) -> Page {
        Page::new(1u64, tier, vec![0u8; size as usize])
    }

    #[test]
    fn test_new_coordinator() {
        let config = GcConfig::default();
        let coord = GcCoordinator::new(config);
        assert!(!coord.is_running());
        assert_eq!(coord.total_collected_bytes(), 0);
        assert_eq!(coord.cycle_count(), 0);
    }

    #[test]
    fn test_gc_needed_false_when_empty() {
        let config = GcConfig::default();
        let coord = GcCoordinator::new(config);
        let table = PageTable::new();
        assert!(!coord.is_gc_needed(&table));
    }

    #[test]
    fn test_run_cycle_empty_table() {
        let config = GcConfig::default();
        let coord = GcCoordinator::new(config);
        let table = PageTable::new();
        let report = coord.run_cycle(&table).unwrap();
        assert!(report.is_empty());
    }

    #[test]
    fn test_collect_candidates_t3_only() {
        let config = GcConfig::default();
        let coord = GcCoordinator::new(config);
        let table = PageTable::new();
        let p = make_page(MemoryTier::T3Compressed, 256);
        table.insert(p).unwrap();
        let candidates = coord.collect_candidates(&table, false, 100);
        assert_eq!(candidates.len(), 1);
    }

    #[test]
    fn test_collect_candidates_skips_pinned() {
        let config = GcConfig::default();
        let coord = GcCoordinator::new(config);
        let table = PageTable::new();
        let mut p = make_page(MemoryTier::T3Compressed, 256);
        p.meta.flags = 0b0001;
        table.insert(p).unwrap();
        let candidates = coord.collect_candidates(&table, false, 100);
        assert!(candidates.is_empty());
    }

    #[test]
    fn test_handle_v8_signal_critical() {
        let config = GcConfig::default();
        let mut coord = GcCoordinator::new(config);
        let report = coord.handle_v8_signal(V8PressureSignal::Critical).unwrap();
        assert!(report.aggressive);
    }

    #[test]
    fn test_cycle_count_increments() {
        let config = GcConfig::default();
        let mut coord = GcCoordinator::new(config);
        coord.cooldown.force_ready();
        let table = PageTable::new();
        table.insert(make_page(MemoryTier::T3Compressed, 128)).unwrap();
        let _ = coord.run_cycle(&table).unwrap();
        assert_eq!(coord.cycle_count(), 1);
    }
}
