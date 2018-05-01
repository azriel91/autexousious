use std::collections::HashMap;

use game_input::ControllerId;

/// Stores the selected characters for each controller.
///
/// The usize is the index of the selected `Object` in the loaded `GameConfig`.
pub type CharacterSelection = HashMap<ControllerId, usize>;
