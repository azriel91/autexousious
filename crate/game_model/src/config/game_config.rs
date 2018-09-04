use std::collections::HashMap;

use map_model::loaded::MapHandle;
use object_model::loaded::CharacterHandle;

use config::AssetRef;

#[derive(Clone, Debug, PartialEq, Eq, new)]
pub struct GameConfig {
    /// Handles to the loaded characters.
    pub character_handles: HashMap<AssetRef, CharacterHandle>,
    /// Handles to the loaded maps.
    pub map_handles: HashMap<AssetRef, MapHandle>,
}
