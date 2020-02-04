use amethyst::ecs::{storage::DenseVecStorage, Component};
use asset_model::loaded::AssetId;
use asset_selection_model::play::AssetSelection;
use derivative::Derivative;

/// Selected map ID or random for a particular controller.
#[derive(Clone, Component, Copy, Debug, Derivative, PartialEq)]
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

impl From<AssetSelection> for MapSelection {
    fn from(asset_selection: AssetSelection) -> Self {
        match asset_selection {
            AssetSelection::Random => MapSelection::Random(None),
            AssetSelection::Id(asset_id) => MapSelection::Id(asset_id),
        }
    }
}
