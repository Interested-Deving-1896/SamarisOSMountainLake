pub mod engine;
pub mod error;
pub mod lifecycle;
pub mod manager;
pub mod result;
pub mod state;

pub use crate::core::manager::VoltRamManager;
pub use crate::core::result::VrmResult;
pub use crate::core::state::VrmState;
pub use crate::config::schema::VrmConfig;
