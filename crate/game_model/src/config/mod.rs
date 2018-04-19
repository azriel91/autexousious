//! Contains types and functionality to index configuration on disk.

pub use self::config_type::ConfigType;
pub use self::discovery::index_configuration;
pub use self::index::ConfigIndex;
pub use self::index::ConfigRecord;

mod config_type;
mod discovery;
mod index;
