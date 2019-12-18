use game_input_model::ControllerId;

use crate::CharacterSelection;

/// Event signalling a change in character selection state.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CharacterSelectionEvent {
    /// Signal to return from `CharacterSelectionState`.
    Return,
    /// Player has joined / become active.
    Join {
        /// ID of the controller.
        controller_id: ControllerId,
    },
    /// Player has left / become inactive.
    Leave {
        /// ID of the controller.
        controller_id: ControllerId,
    },
    /// Character has been selected.
    Switch {
        /// ID of the controller.
        controller_id: ControllerId,
        /// ID of the selected character.
        character_selection: CharacterSelection,
    },
    /// Character has been selected.
    Select {
        /// ID of the controller.
        controller_id: ControllerId,
        /// ID of the selected character.
        character_selection: CharacterSelection,
    },
    /// Character has been deselected.
    Deselect {
        /// ID of the controller.
        controller_id: ControllerId,
    },
    /// Character selections have been confirmed.
    Confirm,
}
