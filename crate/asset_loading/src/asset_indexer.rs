use asset_model::config::{AssetIndex, AssetType, AssetTypeVariants};
use heck::SnakeCase;
use strum::IntoEnumIterator;

use crate::{FlatIndexer, NamespaceDirectory, ObjectIndexer};

/// Indexes assets within a single namespace directory.
#[derive(Debug)]
pub struct AssetIndexer;

impl AssetIndexer {
    /// Returns an asset index for a single namespace.
    ///
    /// # Parameters
    ///
    /// * `namespace_dir`: Namespace directory to index.
    pub fn index(namespace_dir: &NamespaceDirectory) -> AssetIndex {
        AssetTypeVariants::iter().fold(AssetIndex::default(), |mut asset_index, asset_type| {
            let asset_type_dir = namespace_dir
                .path
                .join(&asset_type.to_string().to_snake_case());

            match asset_type {
                AssetTypeVariants::Object => {
                    asset_index.extend(ObjectIndexer::index(
                        &namespace_dir.namespace,
                        &asset_type_dir,
                    ));
                }
                AssetTypeVariants::Map => {
                    asset_index.insert(
                        AssetType::Map,
                        FlatIndexer::index(&namespace_dir.namespace, &asset_type_dir),
                    );
                }
            }

            asset_index
        })
    }
}
