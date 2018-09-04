//! Contains types and functionality to index configuration on disk.

pub use self::asset_ref::{AssetRef, AssetRefBuilder};
pub use self::config_type::ConfigType;
pub use self::discovery::index_configuration;
pub use self::game_config::GameConfig;
pub use self::index::{ConfigIndex, ConfigRecord};

mod asset_ref;
mod config_type;
mod discovery;
mod game_config;
mod index;
