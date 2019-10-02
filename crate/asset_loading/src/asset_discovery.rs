use std::path::Path;

use asset_model::config::AssetIndex;

use crate::{AssetIndexer, NamespaceDiscoverer};

/// Discovers assets across multiple namespaces.
#[derive(Debug)]
pub struct AssetDiscovery;

impl AssetDiscovery {
    /// Returns the asset index of the `assets` directory.
    ///
    /// # Parameters
    ///
    /// * `assets_dir`: Path to the assets directory to index.
    pub fn asset_index(assets_dir: &Path) -> AssetIndex {
        let namespace_directories = NamespaceDiscoverer::discover(assets_dir);
        namespace_directories.iter().map(AssetIndexer::index).fold(
            AssetIndex::default(),
            |mut combined, asset_index| {
                combined.maps.extend(asset_index.maps);
                combined.objects.extend(asset_index.objects);

                combined
            },
        )
    }
}
