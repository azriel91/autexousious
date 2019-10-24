use std::convert::TryFrom;

use asset_model::config::{AssetIndex, AssetType, AssetTypeVariant};
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
        AssetTypeVariant::iter().fold(
            AssetIndex::default(),
            |mut asset_index, asset_type_variant| {
                let asset_type_dir = namespace_dir
                    .path
                    .join(&asset_type_variant.to_string().to_snake_case());

                if let AssetTypeVariant::Object = asset_type_variant {
                    asset_index.extend(ObjectIndexer::index(
                        &namespace_dir.namespace,
                        &asset_type_dir,
                    ));
                } else {
                    let asset_type = AssetType::try_from(asset_type_variant).unwrap_or_else(|e| {
                        panic!("Expected `AssetType::try_from({:?})` to succeed.", e)
                    });
                    asset_index.insert(
                        asset_type,
                        FlatIndexer::index(&namespace_dir.namespace, &asset_type_dir),
                    );
                }

                asset_index
            },
        )
    }
}
