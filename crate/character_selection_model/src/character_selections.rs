use std::collections::HashMap;

use asset_model::loaded::AssetId;
use derive_new::new;
use game_input_model::config::ControllerId;

/// Stores the selected characters for each controller.
///
/// The asset ID refers to the selected `Character`.
#[derive(Clone, Debug, Default, PartialEq, new)]
pub struct CharacterSelections {
    /// Map of controller ID to character asset ID.
    pub selections: HashMap<ControllerId, AssetId>,
}
