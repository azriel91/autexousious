//! Types representing asset configuration.

pub use self::asset_slug::{AssetSlug, AssetSlugBuilder};
pub use self::config_type::ConfigType;
pub use self::index::{AssetIndex, AssetRecord};

mod asset_slug;
mod config_type;
mod index;
