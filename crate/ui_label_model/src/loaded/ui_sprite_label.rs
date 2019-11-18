use amethyst::ecs::{storage::DenseVecStorage, Component};
use asset_model::ItemComponent;
use derive_new::new;
use kinematic_model::config::PositionInit;
use sequence_model::loaded::SequenceId;
use serde::{Deserialize, Serialize};

/// Defines a sprite sequence to display.
#[derive(Clone, Debug, Deserialize, ItemComponent, PartialEq, Serialize, new)]
#[serde(deny_unknown_fields)]
#[storage(DenseVecStorage)]
pub struct UiSpriteLabel {
    /// Position of the label relative to its parent.
    pub position: PositionInit,
    /// `SequenceId` that the `UiSpriteLabel` should begin with.
    pub sequence_id: SequenceId,
}
