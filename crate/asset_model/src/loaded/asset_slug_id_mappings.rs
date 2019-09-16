use std::iter::FromIterator;

use bimap::BiMap;
use derive_deref::{Deref, DerefMut};
use derive_new::new;

use crate::{config::AssetSlug, loaded::AssetSlugId};

/// Mappings from asset slug to ID, and ID to slug.
///
/// This is essentially a wrapper around `BiMap`, with the `slug()` and `id()` methods.
#[derive(Clone, Debug, Default, Deref, DerefMut, PartialEq, new)]
pub struct AssetSlugIdMappings {
    /// Bi-directional mapping from asset slug to id.
    #[new(default)]
    pub asset_slug_to_id: BiMap<AssetSlug, AssetSlugId>,
}

impl AssetSlugIdMappings {
    /// Returns a `AssetSlugIdMappings` with pre-allocated capacity.
    ///
    /// The mappings are guaranteed to hold `capacity` elements without re-allocating.
    pub fn with_capacity(capacity: usize) -> Self {
        AssetSlugIdMappings {
            asset_slug_to_id: BiMap::with_capacity(capacity),
        }
    }

    /// Returns the asset slug for the given ID.
    pub fn slug(&self, asset_slug_id: AssetSlugId) -> Option<&AssetSlug> {
        self.asset_slug_to_id.get_by_right(&asset_slug_id)
    }

    /// Returns the asset slug ID for the given asset slug.
    pub fn id(&self, asset_slug: &AssetSlug) -> Option<&AssetSlugId> {
        self.asset_slug_to_id.get_by_left(asset_slug)
    }
}

impl FromIterator<(AssetSlug, AssetSlugId)> for AssetSlugIdMappings {
    fn from_iter<T: IntoIterator<Item = (AssetSlug, AssetSlugId)>>(iter: T) -> AssetSlugIdMappings {
        let asset_slug_to_id = BiMap::from_iter(iter);
        AssetSlugIdMappings { asset_slug_to_id }
    }
}
