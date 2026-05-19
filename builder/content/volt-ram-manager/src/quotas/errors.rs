use crate::core::error::VrmError;

pub fn quota_exceeded(app_id: u64, used: u64, limit: u64) -> VrmError {
    VrmError::QuotaExceeded { app_id, used, limit }
}

pub fn app_not_registered(app_id: u64) -> VrmError {
    VrmError::AppNotRegistered(app_id)
}

pub fn app_already_registered(app_id: u64) -> VrmError {
    VrmError::AppAlreadyRegistered(app_id)
}

pub fn invalid_config(msg: impl Into<String>) -> VrmError {
    VrmError::InvalidConfig(msg.into())
}
