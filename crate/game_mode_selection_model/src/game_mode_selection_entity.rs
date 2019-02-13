use amethyst::utils::removal::Removal;

use crate::GameModeSelectionEntityId;

/// Marker type for entities to be deleted when the `GameModeSelectionState` is paused or stopped.
pub type GameModeSelectionEntity = Removal<GameModeSelectionEntityId>;
