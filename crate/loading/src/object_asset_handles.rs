use amethyst::assets::Handle;
use derive_new::new;
use object_model::config::GameObjectDefinition;
use sprite_model::config::SpritesDefinition;

/// Assets needed to load an object.
#[derive(Clone, Debug, PartialEq, new)]
pub struct ObjectAssetHandles<D>
where
    D: GameObjectDefinition,
{
    /// Handle to the `GameObjectDefinition` type.
    pub game_object_definition_handle: Handle<D>,
    /// Vector of the `SpriteSheetHandle`s for this object.
    pub sprites_definition_handle: Handle<SpritesDefinition>,
}
