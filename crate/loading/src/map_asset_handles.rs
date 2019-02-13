use amethyst::assets::Handle;
use derive_new::new;
use map_model::config::MapDefinition;
use sprite_model::config::SpritesDefinition;

/// Assets needed to load an object.
#[derive(Clone, Debug, PartialEq, new)]
pub struct MapAssetHandles {
    /// Handle to the `MapDefinition`.
    pub map_definition_handle: Handle<MapDefinition>,
    /// Handle to the `SpritesDefinition`s for this map, if any.
    pub sprites_definition_handle: Option<Handle<SpritesDefinition>>,
}
