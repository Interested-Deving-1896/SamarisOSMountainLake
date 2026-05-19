pub mod audit;
pub mod quotas;
pub mod sandbox;

use std::sync::Arc;

use parking_lot::RwLock;

use crate::core::config::TesseractConfig;
use crate::core::error::{Result, TesseractError};
use crate::protocol::TesseractCommand;
use crate::security::audit::AuditLog;
use crate::security::quotas::ResourceQuotas;
use crate::security::sandbox::CommandSandbox;

pub struct SecurityManager {
    sandbox: CommandSandbox,
    quotas: Arc<RwLock<ResourceQuotas>>,
    audit: Arc<RwLock<AuditLog>>,
}

impl SecurityManager {
    pub fn new(config: &TesseractConfig) -> Self {
        Self {
            sandbox: CommandSandbox::new(),
            quotas: Arc::new(RwLock::new(ResourceQuotas::new(config))),
            audit: Arc::new(RwLock::new(AuditLog::new(config.audit_max_entries))),
        }
    }

    pub fn authorize(&self, cmd: &TesseractCommand) -> Result<()> {
        self.sandbox.validate(cmd)?;

        let app_id = cmd.app_id();
        {
            let mut quotas = self.quotas.write();
            quotas.check_command_quota(app_id).map_err(|e| {
                TesseractError::QuotaExceeded(format!("app 0x{app_id:08X}: {e}"))
            })?;
        }

        {
            let mut audit = self.audit.write();
            audit.log(audit::AuditEntry {
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_micros() as u64,
                app_id,
                opcode: cmd.header.opcode,
                action: "authorize".into(),
                allowed: true,
                reason: String::new(),
            });
        }

        Ok(())
    }

    pub fn quotas(&self) -> &Arc<RwLock<ResourceQuotas>> {
        &self.quotas
    }

    pub fn audit(&self) -> &Arc<RwLock<AuditLog>> {
        &self.audit
    }
}
