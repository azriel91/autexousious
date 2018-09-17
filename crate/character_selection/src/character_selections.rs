use std::collections::HashMap;

use game_input::ControllerId;
use game_model::loaded::SlugAndHandle;
use object_model::loaded::Character;

use CharacterSelectionsStatus;

/// Stores the selected characters for each controller.
///
/// The asset slug is the selected `Character` in `CharacterAssets`.
#[derive(Clone, Debug, Default, PartialEq, new)]
pub struct CharacterSelections {
    /// State of the selections.
    pub state: CharacterSelectionsStatus,
    /// Map of controller ID to character slug and handle.
    pub selections: HashMap<ControllerId, SlugAndHandle<Character>>,
}
