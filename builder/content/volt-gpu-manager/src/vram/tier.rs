#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VramResidencyTier {
    T1Active,
    T2Compressed,
    T3Fallback,
}

impl VramResidencyTier {
    pub fn name(&self) -> &'static str {
        match self {
            VramResidencyTier::T1Active => "T1Active",
            VramResidencyTier::T2Compressed => "T2Compressed",
            VramResidencyTier::T3Fallback => "T3Fallback",
        }
    }

    pub fn is_compressible(&self) -> bool {
        matches!(self, VramResidencyTier::T1Active)
    }

    pub fn is_bindable(&self) -> bool {
        matches!(self, VramResidencyTier::T1Active)
    }

    pub fn requires_restore(&self) -> bool {
        matches!(self, VramResidencyTier::T2Compressed | VramResidencyTier::T3Fallback)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tier_names() {
        assert_eq!(VramResidencyTier::T1Active.name(), "T1Active");
        assert_eq!(VramResidencyTier::T2Compressed.name(), "T2Compressed");
        assert_eq!(VramResidencyTier::T3Fallback.name(), "T3Fallback");
    }

    #[test]
    fn test_t1_active_properties() {
        let t = VramResidencyTier::T1Active;
        assert!(t.is_compressible());
        assert!(t.is_bindable());
        assert!(!t.requires_restore());
    }

    #[test]
    fn test_t2_compressed_properties() {
        let t = VramResidencyTier::T2Compressed;
        assert!(!t.is_compressible());
        assert!(!t.is_bindable());
        assert!(t.requires_restore());
    }

    #[test]
    fn test_t3_fallback_properties() {
        let t = VramResidencyTier::T3Fallback;
        assert!(!t.is_compressible());
        assert!(!t.is_bindable());
        assert!(t.requires_restore());
    }

    #[test]
    fn test_tier_eq_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(VramResidencyTier::T1Active);
        set.insert(VramResidencyTier::T1Active);
        set.insert(VramResidencyTier::T2Compressed);
        assert_eq!(set.len(), 2);
        assert_eq!(VramResidencyTier::T1Active, VramResidencyTier::T1Active);
        assert_ne!(VramResidencyTier::T1Active, VramResidencyTier::T2Compressed);
    }

    #[test]
    fn test_all_tiers_have_unique_names() {
        use std::collections::HashSet;
        let tiers = [VramResidencyTier::T1Active, VramResidencyTier::T2Compressed, VramResidencyTier::T3Fallback];
        let mut names = HashSet::new();
        for t in &tiers {
            assert!(names.insert(t.name()));
        }
        assert_eq!(names.len(), 3);
    }
}
