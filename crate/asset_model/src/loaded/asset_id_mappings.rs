use std::{collections::HashMap, iter::FromIterator};

use derive_new::new;
use slotmap::DenseSlotMap;

use crate::{config::AssetSlug, loaded::AssetId};

/// Mappings from asset slug to ID, and ID to slug.
///
/// Asset slugs are intended to be inserted / removed, but not modified.
#[derive(Clone, Debug, Default, new)]
pub struct AssetIdMappings {
    /// Mapping from asset ID to slug.
    #[new(default)]
    asset_id_to_slug: DenseSlotMap<AssetId, AssetSlug>,
    /// Mapping from asset slug to id.
    #[new(default)]
    asset_slug_to_id: HashMap<AssetSlug, AssetId>,
}

impl AssetIdMappings {
    /// Returns a `AssetIdMappings` with pre-allocated capacity.
    ///
    /// The mappings are guaranteed to hold `capacity` elements without re-allocating.
    pub fn with_capacity(capacity: usize) -> Self {
        AssetIdMappings {
            asset_id_to_slug: DenseSlotMap::with_capacity_and_key(capacity),
            asset_slug_to_id: HashMap::with_capacity(capacity),
        }
    }

    /// Returns the number of elements the mappings can hold without reallocating.
    pub fn capacity(&self) -> usize {
        self.asset_id_to_slug.capacity()
    }

    /// Returns the asset slug for the given ID.
    pub fn slug(&self, asset_id: AssetId) -> Option<&AssetSlug> {
        self.asset_id_to_slug.get(asset_id)
    }

    /// Returns the asset ID for the given asset slug.
    pub fn id(&self, asset_slug: &AssetSlug) -> Option<&AssetId> {
        self.asset_slug_to_id.get(asset_slug)
    }

    /// Returns `true` if there are no mappings.
    pub fn is_empty(&self) -> bool {
        self.asset_slug_to_id.is_empty()
    }

    /// Inserts an asset slug into the mappings and returns the generated ID.
    ///
    /// # Parameters
    ///
    /// * `asset_slug`: Asset slug to insert.
    ///
    /// # Panics
    ///
    /// Panics if the number of mappings equals 2<sup>32</sup> - 2.
    pub fn insert(&mut self, asset_slug: AssetSlug) -> AssetId {
        let asset_id = self.asset_id_to_slug.insert(asset_slug.clone());
        self.asset_slug_to_id.insert(asset_slug, asset_id);

        asset_id
    }

    /// Returns an iterator of asset IDs to slug.
    pub fn iter(&self) -> impl Iterator<Item = (AssetId, &AssetSlug)> {
        self.asset_id_to_slug.iter()
    }

    /// Returns an iterator visiting all `AssetId`s in arbitrary order.
    pub fn keys<'a>(&'a self) -> impl Iterator<Item = AssetId> + 'a {
        self.asset_id_to_slug.keys()
    }

    /// Returns an iterator visiting all `AssetSlug`s in arbitrary order.
    pub fn values(&self) -> impl Iterator<Item = &AssetSlug> {
        self.asset_id_to_slug.values()
    }

    /// Returns the number of mappings.
    pub fn len(&self) -> usize {
        self.asset_id_to_slug.len()
    }

    /// Removes the ID mapping for the given asset slug, returning it if it exists.
    pub fn remove(&mut self, asset_id: AssetId) -> Option<AssetSlug> {
        let asset_slug = self.asset_id_to_slug.remove(asset_id);
        if let Some(asset_slug) = asset_slug.as_ref() {
            self.asset_slug_to_id.remove(asset_slug);
        }

        asset_slug
    }

    /// Reserves capacity for at least `additional` more mappings to be inserted.
    ///
    /// This may reserve more space to avoid frequent reallocations.
    ///
    /// # Panics
    ///
    /// Panics if the new allocation size overflows `usize`.
    pub fn reserve(&mut self, additional: usize) {
        self.asset_id_to_slug.reserve(additional);
        self.asset_slug_to_id.reserve(additional);
    }
}

impl FromIterator<AssetSlug> for AssetIdMappings {
    fn from_iter<T: IntoIterator<Item = AssetSlug>>(iter: T) -> AssetIdMappings {
        let (asset_id_to_slug, asset_slug_to_id) = iter.into_iter().fold(
            (DenseSlotMap::with_key(), HashMap::new()),
            |(mut asset_id_to_slug, mut asset_slug_to_id), asset_slug| {
                let asset_id = asset_id_to_slug.insert(asset_slug.clone());
                asset_slug_to_id.insert(asset_slug, asset_id);

                (asset_id_to_slug, asset_slug_to_id)
            },
        );

        AssetIdMappings {
            asset_id_to_slug,
            asset_slug_to_id,
        }
    }
}
