use amethyst_utils::removal::Removal;

use CharacterSelectionEntityId;

/// Marker type for entities to be deleted when the `CharacterSelectionState` is paused or stopped.
pub type CharacterSelectionEntity = Removal<CharacterSelectionEntityId>;
