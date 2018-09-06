use std::path::PathBuf;

use config::AssetRef;

/// Contains the meta information about an asset.
///
/// Includes:
///
/// * Asset reference (see [`AssetRef`][asset_ref]).
/// * Path to the asset configuration directory.
///
/// [asset_ref]: config/struct.AssetRef.html
#[derive(Clone, Debug, PartialEq, new)]
pub struct ConfigRecord {
    /// Asset reference.
    pub asset_ref: AssetRef,
    /// Directory path of the configuration relative to the assets directory.
    ///
    /// e.g. "default/objects/characters/heat"
    pub directory: PathBuf,
}
