use asset_model::loaded::AssetId;
use derivative::Derivative;

/// Selected map ID or random for a particular controller.
#[derive(Clone, Copy, Debug, Derivative, PartialEq)]
#[derivative(Default)]
pub enum MapSelection {
    /// No map is currently selected.
    #[derivative(Default)]
    None,
    /// User has selected *Random*.
    Random(Option<AssetId>),
    /// User has selected a map.
    Id(AssetId),
}

impl MapSelection {
    /// Returns the `AssetId` of the selection.
    pub fn asset_id(self) -> Option<AssetId> {
        match self {
            MapSelection::None => None,
            MapSelection::Random(asset_id) => asset_id,
            MapSelection::Id(asset_id) => Some(asset_id),
        }
    }
}
