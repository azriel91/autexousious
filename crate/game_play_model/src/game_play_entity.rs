use amethyst::utils::removal::Removal;

use crate::GamePlayEntityId;

/// Marker type for entities to be deleted when the `GameModeSelectionState` is paused or stopped.
pub type GamePlayEntity = Removal<GamePlayEntityId>;
