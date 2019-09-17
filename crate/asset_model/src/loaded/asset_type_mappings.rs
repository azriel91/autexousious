use std::iter::FromIterator;

use bimap::BiMap;
use derive_deref::{Deref, DerefMut};
use derive_new::new;

use crate::{config::AssetType, loaded::AssetId};

/// Mappings from asset ID to asset type, and asset type to ID.
///
/// This is essentially a wrapper around `BiMap`, with the `asset_id()` and `asset_type()` methods.
#[derive(Clone, Debug, Default, Deref, DerefMut, PartialEq, new)]
pub struct AssetTypeMappings {
    /// Bi-directional mapping from asset ID to asset type.
    #[new(default)]
    pub asset_id_to_type: BiMap<AssetId, AssetType>,
}

impl AssetTypeMappings {
    /// Returns a `AssetTypeMappings` with pre-allocated capacity.
    ///
    /// The mappings are guaranteed to hold `capacity` elements without re-allocating.
    pub fn with_capacity(capacity: usize) -> Self {
        AssetTypeMappings {
            asset_id_to_type: BiMap::with_capacity(capacity),
        }
    }

    /// Returns the asset ID for the given asset type.
    pub fn asset_id(&self, asset_type: AssetType) -> Option<&AssetId> {
        self.asset_id_to_type.get_by_right(&asset_type)
    }

    /// Returns the asset type for the given asset ID.
    pub fn asset_type(&self, asset_id: &AssetId) -> Option<&AssetType> {
        self.asset_id_to_type.get_by_left(asset_id)
    }
}

impl FromIterator<(AssetId, AssetType)> for AssetTypeMappings {
    fn from_iter<T: IntoIterator<Item = (AssetId, AssetType)>>(iter: T) -> AssetTypeMappings {
        let asset_id_to_type = BiMap::from_iter(iter);
        AssetTypeMappings { asset_id_to_type }
    }
}
