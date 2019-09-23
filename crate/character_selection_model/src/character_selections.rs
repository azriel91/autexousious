use std::collections::HashMap;

use asset_model::loaded::AssetId;
use derive_new::new;
use game_input_model::ControllerId;

/// Stores the selected characters for each controller.
///
/// The asset slug refers to the selected `CharacterPrefab` in `CharacterPrefabs`.
#[derive(Clone, Debug, Default, PartialEq, new)]
pub struct CharacterSelections {
    /// Map of controller ID to character asset slug.
    pub selections: HashMap<ControllerId, AssetId>,
}
