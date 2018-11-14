use amethyst::{
    assets::Loader,
    prelude::*,
    renderer::{SpriteSheetHandle, TextureHandle},
};
use sprite_model::config::SpriteSheetDefinition;

use SpriteSheetMapper;

#[derive(Debug)]
pub(crate) struct SpriteSheetLoader;

impl SpriteSheetLoader {
    /// Loads Amethyst `SpriteSheet`s from configuration and returns their handles.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` to load the sprite sheets into.
    /// * `texture_handles`: Handles of the sprite sheets' textures.
    /// * `sprite_sheet_definitions`: List of metadata for sprite sheets to map.
    pub fn load(
        world: &World,
        texture_handles: &[TextureHandle],
        sprite_sheet_definitions: &[SpriteSheetDefinition],
    ) -> Vec<SpriteSheetHandle> {
        let sprite_sheets = SpriteSheetMapper::map(texture_handles, &sprite_sheet_definitions);

        let loader = world.read_resource::<Loader>();
        let sprite_sheet_store = world.read_resource();

        sprite_sheets
            .into_iter()
            .map(|sprite_sheet| loader.load_from_data(sprite_sheet, (), &sprite_sheet_store))
            .collect::<Vec<_>>()
    }
}
