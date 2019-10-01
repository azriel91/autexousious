use amethyst::ecs::{World, WorldExt};
use asset_model::{
    config::{AssetSlug, AssetType},
    loaded::{AssetId, AssetIdMappings, AssetTypeMappings},
};

/// Functions to retrieve asset data from a running world.
#[derive(Debug)]
pub struct AssetQueries;

impl AssetQueries {
    /// Returns the `AssetId` of the first asset of a given type.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` of the running application.
    /// * `asset_type`: `AssetType` whose first asset ID to retrieve.
    pub fn first_id(world: &World, asset_type: AssetType) -> AssetId {
        let asset_type_mappings = world.read_resource::<AssetTypeMappings>();
        asset_type_mappings
            .iter_ids(&asset_type)
            .next()
            .copied()
            .expect("Expected at least one character to be loaded.")
    }

    /// Returns the `AssetId` of the last asset of a given type.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` of the running application.
    /// * `asset_type`: `AssetType` whose last asset ID to retrieve.
    pub fn last_id(world: &World, asset_type: AssetType) -> AssetId {
        let asset_type_mappings = world.read_resource::<AssetTypeMappings>();
        asset_type_mappings
            .iter_ids(&asset_type)
            .next_back()
            .copied()
            .expect("Expected at least one character to be loaded.")
    }

    /// Returns the `AssetId` of the asset with the given slug.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` of the running application.
    /// * `asset_slug`: `AssetSlug` whose asset ID to retrieve.
    pub fn id(world: &World, asset_slug: &AssetSlug) -> AssetId {
        let asset_id_mappings = world.read_resource::<AssetIdMappings>();
        asset_id_mappings
            .id(asset_slug)
            .copied()
            .unwrap_or_else(|| panic!("Asset ID for `{}` not found.", asset_slug))
    }
}
