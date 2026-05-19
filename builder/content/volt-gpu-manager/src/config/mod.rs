pub mod schema;
pub mod loader;
pub mod defaults;
pub mod validation;

pub use schema::VgmConfig;
pub use loader::{load_config, load_config_or_default};
