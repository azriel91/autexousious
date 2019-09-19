use std::{collections::HashMap, iter::FromIterator};

use derive_deref::{Deref, DerefMut};
use derive_new::new;

use crate::{config::AssetType, loaded::AssetId};

/// Mappings from asset ID to asset type.
#[derive(Clone, Debug, Default, Deref, DerefMut, PartialEq, new)]
pub struct AssetTypeMappings {
    /// Bi-directional mapping from asset ID to asset type.
    #[new(default)]
    pub asset_id_to_type: HashMap<AssetId, AssetType>,
}

impl AssetTypeMappings {
    /// Returns a `AssetTypeMappings` with pre-allocated capacity.
    ///
    /// The mappings are guaranteed to hold `capacity` elements without re-allocating.
    pub fn with_capacity(capacity: usize) -> Self {
        AssetTypeMappings {
            asset_id_to_type: HashMap::with_capacity(capacity),
        }
    }
}

impl FromIterator<(AssetId, AssetType)> for AssetTypeMappings {
    fn from_iter<T: IntoIterator<Item = (AssetId, AssetType)>>(iter: T) -> AssetTypeMappings {
        let asset_id_to_type = HashMap::from_iter(iter);
        AssetTypeMappings { asset_id_to_type }
    }
}
