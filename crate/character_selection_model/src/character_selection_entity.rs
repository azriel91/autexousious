use amethyst::utils::removal::Removal;

use crate::CharacterSelectionEntityId;

/// Marker type for entities to be deleted when the `CharacterSelectionState` is paused or stopped.
pub type CharacterSelectionEntity = Removal<CharacterSelectionEntityId>;
