use std::collections::HashMap;

use game_input::ControllerId;

/// Stores the selected characters for each controller.
///
/// The usize is the index of the selected `Character` in the loaded `Character`s.
pub type CharacterSelections = HashMap<ControllerId, usize>;
