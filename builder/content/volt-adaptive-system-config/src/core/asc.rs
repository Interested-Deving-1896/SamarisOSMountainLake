use parking_lot::RwLock;

use crate::budget::system_budget::SystemBudget;
use crate::classify::machine_class::MachineClass;
use crate::classify::profile_kind::ProfileKind;
use crate::config::schema::AscConfig;
use crate::core::error::AscError;
use crate::core::lifecycle::Lifecycle;
use crate::core::result::AscResult;
use crate::core::state::AscState;
use crate::explain::report::ExplainReport;
use crate::generator::generated_config::GeneratedConfig;
use crate::hardware::probe::HardwareProbe;
use crate::hardware::profile::HardwareProfile;

pub struct VoltAsc {
    config: AscConfig,
    lifecycle: RwLock<Lifecycle>,
    hardware: RwLock<Option<HardwareProfile>>,
    classes: RwLock<Vec<MachineClass>>,
    budget: RwLock<Option<SystemBudget>>,
    generated: RwLock<Option<GeneratedConfig>>,
    explain: RwLock<Option<ExplainReport>>,
}

impl VoltAsc {
    pub fn new(config: AscConfig) -> Self {
        Self {
            config,
            lifecycle: RwLock::new(Lifecycle::new()),
            hardware: RwLock::new(None),
            classes: RwLock::new(Vec::new()),
            budget: RwLock::new(None),
            generated: RwLock::new(None),
            explain: RwLock::new(None),
        }
    }

    pub fn probe(&self) -> AscResult<HardwareProfile> {
        let mut lc = self.lifecycle.write();
        lc.start()?;

        let probe = HardwareProbe::new();
        let profile = probe.detect()?;

        let mut hw = self.hardware.write();
        *hw = Some(profile.clone());

        lc.transition(crate::core::state::AscState::Profiling)?;
        Ok(profile)
    }

    pub fn classify(&self) -> AscResult<Vec<MachineClass>> {
        let hw = self.hardware.read();
        let profile = hw.as_ref().ok_or(AscError::InvalidHardwareProfile(
            "call probe() before classify()".into(),
        ))?;

        let classes = crate::classify::machine_class::classify(profile);
        let mut cl = self.classes.write();
        *cl = classes.clone();

        let mut lc = self.lifecycle.write();
        lc.transition(crate::core::state::AscState::Budgeting)?;

        Ok(classes)
    }

    pub fn budget(&self) -> AscResult<SystemBudget> {
        let hw = self.hardware.read();
        let profile = hw.as_ref().ok_or(AscError::InvalidHardwareProfile(
            "call probe() before budget()".into(),
        ))?;

        let classes = self.classes.read();
        let _kind = ProfileKind::from_config(&self.config.adaptive.profile);

        let budget = SystemBudget::compute(profile, &classes, 256);
        let reconciled = budget.reconcile(profile)?;

        let mut b = self.budget.write();
        *b = Some(reconciled.clone());

        let mut lc = self.lifecycle.write();
        lc.transition(crate::core::state::AscState::Generating)?;

        Ok(reconciled)
    }

    pub fn generate(&self) -> AscResult<GeneratedConfig> {
        let hw = self.hardware.read();
        let profile = hw.as_ref().ok_or(AscError::InvalidHardwareProfile(
            "call probe() before generate()".into(),
        ))?;

        let classes = self.classes.read();
        let budget = self.budget.read();
        let budget = budget.as_ref().ok_or(AscError::BudgetReconciliationFailed(
            "call budget() before generate()".into(),
        ))?;

        let kind = ProfileKind::from_config(&self.config.adaptive.profile);
        let config = GeneratedConfig::from_profile(profile, &classes, budget, kind);

        let mut g = self.generated.write();
        *g = Some(config.clone());

        let mut lc = self.lifecycle.write();
        lc.transition(crate::core::state::AscState::Validating)?;
        lc.complete()?;

        Ok(config)
    }

    pub fn explain(&self) -> AscResult<ExplainReport> {
        let hw = self.hardware.read();
        let profile = hw.as_ref().ok_or(AscError::InvalidHardwareProfile(
            "call probe() before explain()".into(),
        ))?;
        let classes = self.classes.read();
        let budget = self.budget.read();
        let generated = self.generated.read();

        let report = ExplainReport::new(profile, &classes, budget.as_ref(), generated.as_ref());

        let mut e = self.explain.write();
        *e = Some(report.clone());

        Ok(report)
    }

    pub fn hardware_profile(&self) -> Option<HardwareProfile> {
        self.hardware.read().clone()
    }

    pub fn generated_config(&self) -> Option<GeneratedConfig> {
        self.generated.read().clone()
    }

    pub fn state(&self) -> AscState {
        self.lifecycle.read().state()
    }

    pub fn config(&self) -> &AscConfig {
        &self.config
    }

    pub fn full_pipeline(&self) -> AscResult<GeneratedConfig> {
        self.probe()?;
        self.classify()?;
        self.budget()?;
        let config = self.generate()?;
        Ok(config)
    }
}
