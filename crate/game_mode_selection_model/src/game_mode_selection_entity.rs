use amethyst_utils::removal::Removal;

use GameModeSelectionEntityId;

/// Marker type for entities to be deleted when the `GameModeSelectionState` is paused or stopped.
pub type GameModeSelectionEntity = Removal<GameModeSelectionEntityId>;
