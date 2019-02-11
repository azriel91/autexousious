use amethyst::{assets::Handle, renderer::SpriteSheetHandle};
use derive_new::new;

use crate::config::GameObjectDefinition;

/// Assets needed to load an object.
#[derive(Clone, Debug, PartialEq, new)]
pub struct ObjectAssetData<D>
where
    D: GameObjectDefinition,
{
    /// Handle to the `GameObjectDefinition` type.
    pub game_object_definition_handle: Handle<D>,
    /// Vector of the `SpriteSheetHandle`s for this object.
    pub sprite_sheet_handles: Vec<SpriteSheetHandle>,
}
