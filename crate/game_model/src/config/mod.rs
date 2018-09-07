//! Types representing asset configuration.

pub use self::asset_ref::{AssetRef, AssetRefBuilder};
pub use self::config_type::ConfigType;
pub use self::index::{AssetIndex, AssetRecord};

mod asset_ref;
mod config_type;
mod index;
