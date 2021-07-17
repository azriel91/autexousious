use asset_model::loaded::{AssetId, AssetIdMappings};
use state_registry::StateId;

/// Functions to query `AssetId`s for `State`s.
#[derive(Debug)]
pub struct StateAssetUtils;

impl StateAssetUtils {
    /// Returns the State's UI collective asset ID, if any.
    ///
    /// # Parameters
    ///
    /// * `asset_id_mappings`: Bidrectional mappings beteen Asset ID and asset
    ///   slug.
    /// * `state_id`: ID of the active state.
    pub fn asset_id(asset_id_mappings: &AssetIdMappings, state_id: StateId) -> Option<AssetId> {
        let state_id_name = state_id.to_string();
        asset_id_mappings.iter().find_map(|(id, slug)| {
            if slug.name == state_id_name {
                Some(id)
            } else {
                None
            }
        })
    }
}
