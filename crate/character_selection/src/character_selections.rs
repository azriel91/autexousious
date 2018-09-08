use std::collections::HashMap;

use game_input::ControllerId;
use game_model::config::AssetSlug;

use CharacterSelectionsState;

/// Stores the selected characters for each controller.
///
/// The asset slug is the selected `Character` in `CharacterAssets`.
#[derive(Clone, Debug, Default, PartialEq, new)]
pub struct CharacterSelections {
    /// State of the selections.
    pub state: CharacterSelectionsState,
    /// Map of controller ID to character slug.
    pub selections: HashMap<ControllerId, AssetSlug>,
}
