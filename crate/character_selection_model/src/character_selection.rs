use amethyst::ecs::{storage::DenseVecStorage, Component};
use asset_model::loaded::AssetId;

/// Selected character ID or random for a particular controller.
#[derive(Clone, Component, Copy, Debug, PartialEq)]
pub enum CharacterSelection {
    /// User has selected *Random*.
    Random,
    /// User has selected a character.
    Id(AssetId),
}
