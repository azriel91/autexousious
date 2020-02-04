use amethyst::ecs::{storage::DenseVecStorage, Component};
use asset_model::loaded::AssetId;

/// Selected `AssetId` or `Random`.
#[derive(Clone, Component, Copy, Debug, PartialEq)]
pub enum AssetSelection {
    /// User has selected *Random*.
    Random,
    /// User has selected an asset with the associated ID.
    Id(AssetId),
}
