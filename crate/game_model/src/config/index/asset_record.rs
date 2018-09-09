use std::path::PathBuf;

use config::AssetSlug;

/// Contains the meta information about an asset.
///
/// Includes:
///
/// * Asset slug (see [`AssetSlug`][asset_slug]).
/// * Path to the asset directory.
///
/// [asset_slug]: config/struct.AssetSlug.html
#[derive(Clone, Debug, PartialEq, new)]
pub struct AssetRecord {
    /// Human readable slug to the asset.
    pub asset_slug: AssetSlug,
    /// Relative path of the asset to the assets directory.
    ///
    /// e.g. "default/objects/characters/heat"
    pub path: PathBuf,
}
