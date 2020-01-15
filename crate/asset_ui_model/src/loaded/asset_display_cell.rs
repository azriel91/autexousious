use amethyst::ecs::{storage::VecStorage, Component};
use asset_model::loaded::AssetId;

/// Display cell for a particular asset.
///
/// # Type Parameters
///
/// * `T`: Type to indicate the assets to display.
#[derive(Clone, Component, Debug, PartialEq)]
#[storage(VecStorage)]
pub struct AssetDisplayCell {
    /// ID of the asset to display.
    pub asset_id: AssetId,
}
