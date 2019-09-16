use std::iter::FromIterator;

use bimap::BiMap;
use derive_deref::{Deref, DerefMut};
use derive_new::new;

use crate::config::{AssetSlug, AssetType};

/// Mappings from asset slug to asset type, and asset type to slug.
///
/// This is essentially a wrapper around `BiMap`, with the `asset_slug()` and `asset_type()` methods.
#[derive(Clone, Debug, Default, Deref, DerefMut, PartialEq, new)]
pub struct AssetSlugTypeMappings {
    /// Bi-directional mapping from asset slug to asset type.
    #[new(default)]
    pub asset_slug_to_type: BiMap<AssetSlug, AssetType>,
}

impl AssetSlugTypeMappings {
    /// Returns a `AssetSlugTypeMappings` with pre-allocated capacity.
    ///
    /// The mappings are guaranteed to hold `capacity` elements without re-allocating.
    pub fn with_capacity(capacity: usize) -> Self {
        AssetSlugTypeMappings {
            asset_slug_to_type: BiMap::with_capacity(capacity),
        }
    }

    /// Returns the asset slug for the given asset type.
    pub fn asset_slug(&self, asset_type: AssetType) -> Option<&AssetSlug> {
        self.asset_slug_to_type.get_by_right(&asset_type)
    }

    /// Returns the asset type for the given asset slug.
    pub fn asset_type(&self, asset_slug: &AssetSlug) -> Option<&AssetType> {
        self.asset_slug_to_type.get_by_left(asset_slug)
    }
}

impl FromIterator<(AssetSlug, AssetType)> for AssetSlugTypeMappings {
    fn from_iter<T: IntoIterator<Item = (AssetSlug, AssetType)>>(iter: T) -> AssetSlugTypeMappings {
        let asset_slug_to_type = BiMap::from_iter(iter);
        AssetSlugTypeMappings { asset_slug_to_type }
    }
}
