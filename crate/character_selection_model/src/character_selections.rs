use std::collections::HashMap;

use character_model::loaded::Character;
use derive_new::new;
use game_input::ControllerId;
use game_model::loaded::SlugAndHandle;

/// Stores the selected characters for each controller.
///
/// The asset slug is the selected `Character` in `CharacterAssets`.
#[derive(Clone, Debug, Default, PartialEq, new)]
pub struct CharacterSelections {
    /// Map of controller ID to character slug and handle.
    pub selections: HashMap<ControllerId, SlugAndHandle<Character>>,
}
