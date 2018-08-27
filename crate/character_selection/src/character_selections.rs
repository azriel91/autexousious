use std::collections::HashMap;

use game_input::ControllerId;

use CharacterSelectionsState;

/// Stores the selected characters for each controller.
///
/// The usize is the index of the selected `Character` in the loaded `Character`s.
#[derive(Clone, Debug, Default, PartialEq, new)]
pub struct CharacterSelections {
    /// State of the selections.
    pub state: CharacterSelectionsState,
    /// Map of controller ID to character ID.
    pub selections: HashMap<ControllerId, usize>,
}
