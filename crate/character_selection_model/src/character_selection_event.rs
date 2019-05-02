use game_input_model::ControllerId;

use crate::CharacterSelection;

/// Event signalling a change in character selection state.
#[derive(Clone, Debug, PartialEq)]
pub enum CharacterSelectionEvent {
    /// Signal to return from `CharacterSelectionState`.
    Return,
    /// Character has been selected.
    Select {
        /// ID of the controller of the selection.
        controller_id: ControllerId,
        /// ID of the selected character.
        character_selection: CharacterSelection,
    },
    /// Character has been deselected.
    Deselect {
        /// ID of the controller of the selection.
        controller_id: ControllerId,
    },
    /// Character selections have been confirmed.
    Confirm,
}
