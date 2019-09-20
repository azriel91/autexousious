use std::{collections::HashMap, iter::FromIterator};

use derive_new::new;

use crate::{config::AssetType, loaded::AssetId};

/// Mappings from asset ID to asset type.
#[derive(Clone, Debug, Default, PartialEq, new)]
pub struct AssetTypeMappings {
    /// Mappings from asset ID to asset type.
    #[new(default)]
    asset_id_to_type: HashMap<AssetId, AssetType>,
    /// Mappings from asset type to the asset ID of that type.
    #[new(default)]
    asset_type_to_ids: HashMap<AssetType, Vec<AssetId>>,
}

impl AssetTypeMappings {
    /// Returns a `AssetTypeMappings` with pre-allocated capacity.
    ///
    /// The mappings are guaranteed to hold `capacity` elements without re-allocating.
    pub fn with_capacity(capacity: usize) -> Self {
        AssetTypeMappings {
            asset_id_to_type: HashMap::with_capacity(capacity),
            asset_type_to_ids: HashMap::with_capacity(capacity),
        }
    }

    /// Returns the number of elements the mappings can hold without reallocating.
    pub fn capacity(&self) -> usize {
        self.asset_id_to_type.capacity()
    }

    /// Returns a reference to the value corresponding to the key.
    pub fn get(&self, asset_id: &AssetId) -> Option<&AssetType> {
        self.asset_id_to_type.get(asset_id)
    }

    /// Returns `true` if there are no mappings.
    pub fn is_empty(&self) -> bool {
        self.asset_id_to_type.is_empty()
    }

    /// Inserts a key-value pair into the map.
    ///
    /// If the map did not have this key present, [`None`] is returned.
    ///
    /// If the map did have this key present, the value is updated, and the old
    /// value is returned. The key is not updated.
    pub fn insert(&mut self, asset_id: AssetId, asset_type: AssetType) -> Option<AssetType> {
        let previous_type = self.asset_id_to_type.insert(asset_id, asset_type);

        let asset_ids = self
            .asset_type_to_ids
            .entry(asset_type)
            .or_insert_with(Vec::new);
        asset_ids.push(asset_id);

        if let Some(previous_type) = previous_type {
            self.remove_from_ids(previous_type, &asset_id);
        }

        previous_type
    }

    /// Returns an iterator of asset ID to type.
    pub fn iter(&self) -> impl Iterator<Item = (&AssetId, &AssetType)> {
        self.asset_id_to_type.iter()
    }

    /// Returns an iterator of asset type to ids.
    ///
    /// # Parameters
    ///
    /// * `asset_type`: Asset type whose IDs to iterate over.
    pub fn iter_ids(&self, asset_type: &AssetType) -> impl Iterator<Item = &AssetId> {
        self.asset_type_to_ids
            .get(asset_type)
            .map(|asset_ids| asset_ids.iter())
            .unwrap_or_else(|| [].iter())
    }

    /// Returns an iterator of asset type to ids.
    pub fn iter_ids_all(&self) -> impl Iterator<Item = (&AssetType, &Vec<AssetId>)> {
        self.asset_type_to_ids.iter()
    }

    /// Returns an iterator visiting all `AssetId`s in arbitrary order.
    pub fn keys(&self) -> impl Iterator<Item = &AssetId> {
        self.asset_id_to_type.keys()
    }

    /// Returns an iterator visiting all `AssetType`s in arbitrary order.
    pub fn values(&self) -> impl Iterator<Item = &AssetType> {
        self.asset_id_to_type.values()
    }

    /// Returns the number of mappings.
    pub fn len(&self) -> usize {
        self.asset_id_to_type.len()
    }

    /// Removes the ID mapping for the given asset type, returning it if it exists.
    pub fn remove(&mut self, asset_id: &AssetId) -> Option<AssetType> {
        let previous_type = self.asset_id_to_type.remove(asset_id);

        if let Some(previous_type) = previous_type {
            self.remove_from_ids(previous_type, asset_id);
        }

        previous_type
    }

    /// Reserves capacity for at least `additional` more mappings to be inserted.
    ///
    /// This may reserve more space to avoid frequent reallocations.
    ///
    /// # Panics
    ///
    /// Panics if the new allocation size overflows `usize`.
    pub fn reserve(&mut self, additional: usize) {
        self.asset_id_to_type.reserve(additional);
    }

    fn remove_from_ids(&mut self, asset_type: AssetType, asset_id: &AssetId) {
        let asset_type_ids = self
            .asset_type_to_ids
            .get_mut(&asset_type)
            .expect("Expected previous type mapping to exist.");

        asset_type_ids
            .iter()
            .position(|existing_id| existing_id == asset_id)
            .map(|i| asset_type_ids.remove(i));
    }
}

impl FromIterator<(AssetId, AssetType)> for AssetTypeMappings {
    fn from_iter<T: IntoIterator<Item = (AssetId, AssetType)>>(iter: T) -> AssetTypeMappings {
        let asset_id_to_type = HashMap::from_iter(iter);
        let asset_type_to_ids = asset_id_to_type.iter().fold(
            HashMap::with_capacity(asset_id_to_type.len()),
            |mut asset_type_to_ids, (asset_id, asset_type)| {
                asset_type_to_ids
                    .entry(*asset_type)
                    .or_insert_with(Vec::new)
                    .push(*asset_id);

                asset_type_to_ids
            },
        );
        AssetTypeMappings {
            asset_id_to_type,
            asset_type_to_ids,
        }
    }
}
