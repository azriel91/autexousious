use asset_model::loaded::AssetId;

/// Selected map ID or random for a particular controller.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MapSelection {
    /// User has selected *Random*.
    Random(Option<AssetId>),
    /// User has selected a map.
    Id(AssetId),
}

impl MapSelection {
    /// Returns the `AssetId` of the selection.
    pub fn asset_id(self) -> Option<AssetId> {
        match self {
            MapSelection::Random(asset_id) => asset_id,
            MapSelection::Id(asset_id) => Some(asset_id),
        }
    }
}
