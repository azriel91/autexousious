use amethyst::ecs::{storage::DenseVecStorage, Component};
use asset_model::ItemComponent;
use derive_new::new;
use kinematic_model::config::PositionInit;
use serde::{Deserialize, Serialize};

/// Defines text to display.
#[derive(Clone, Debug, Default, Deserialize, ItemComponent, PartialEq, Serialize, new)]
#[serde(default, deny_unknown_fields)]
#[storage(DenseVecStorage)]
pub struct UiLabel {
    /// Position of the label relative to its parent.
    pub position: PositionInit,
    /// Text to display.
    pub text: String,
}
