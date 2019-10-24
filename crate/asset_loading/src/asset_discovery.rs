use std::{mem, path::Path};

use asset_model::config::{AssetIndex, AssetType, AssetTypeVariant};
use object_type::ObjectType;
use strum::IntoEnumIterator;

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
            |mut asset_index_combined, mut asset_index| {
                AssetTypeVariant::iter().for_each(|asset_type_variant| match asset_type_variant {
                    AssetTypeVariant::Object => {
                        ObjectType::iter().for_each(|object_type| {
                            Self::asset_index_merge(
                                &mut asset_index_combined,
                                &mut asset_index,
                                AssetType::Object(object_type),
                            );
                        });
                    }
                    AssetTypeVariant::Map => {
                        Self::asset_index_merge(
                            &mut asset_index_combined,
                            &mut asset_index,
                            AssetType::Map,
                        );
                    }
                });

                asset_index_combined
            },
        )
    }

    fn asset_index_merge(
        asset_index_combined: &mut AssetIndex,
        asset_records_new: &mut AssetIndex,
        asset_type: AssetType,
    ) {
        if let Some(asset_records) = asset_records_new.get_mut(&asset_type) {
            if let Some(asset_records_existing) = asset_index_combined.get_mut(&asset_type) {
                asset_records_existing.extend(asset_records.drain(..));
            } else {
                asset_index_combined.insert(asset_type, mem::replace(asset_records, Vec::new()));
            }
        }
    }
}
