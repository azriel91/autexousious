use std::{convert::TryFrom, mem, path::Path};

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
        let mut asset_index_combined = namespace_directories.iter().map(AssetIndexer::index).fold(
            AssetIndex::default(),
            |mut asset_index_combined, mut asset_index| {
                AssetTypeVariant::iter().for_each(|asset_type_variant| {
                    if let AssetTypeVariant::Object = asset_type_variant {
                        ObjectType::iter().for_each(|object_type| {
                            Self::asset_index_merge(
                                &mut asset_index_combined,
                                &mut asset_index,
                                AssetType::Object(object_type),
                            );
                        });
                    } else {
                        Self::asset_index_merge(
                            &mut asset_index_combined,
                            &mut asset_index,
                            AssetType::try_from(asset_type_variant).unwrap_or_else(|e| {
                                panic!("Expected `AssetType::try_from({:?})` to succeed.", e)
                            }),
                        )
                    }
                });

                asset_index_combined
            },
        );

        asset_index_combined.values_mut().for_each(|asset_records| {
            asset_records.sort_unstable_by(|r1, r2| r1.asset_slug.cmp(&r2.asset_slug))
        });

        asset_index_combined
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
