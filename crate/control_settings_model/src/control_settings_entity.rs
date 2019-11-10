use amethyst::utils::removal::Removal;

use crate::ControlSettingsEntityId;

/// Marker type for entities to be deleted when the `GameModeSelectionState` is paused or stopped.
pub type ControlSettingsEntity = Removal<ControlSettingsEntityId>;
