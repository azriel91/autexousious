use std::path::Path;

use amethyst::assets::{AssetStorage, Loader};
use amethyst::prelude::*;
use amethyst::renderer::{PngFormat, Texture, TextureHandle};
use object_model::config::SpriteSheetDefinition;

#[derive(Debug)]
pub(super) struct TextureLoader;

impl TextureLoader {
    /// Loads the sprite sheet images as textures and returns the texture handles.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` to store the sprite sheet textures.
    /// * `object_directory`: Object configuration base directory.
    /// * `sprite_sheet_definitions`: List of metadata for sprite sheets to load.
    pub(super) fn load_textures(
        world: &World,
        object_directory: &Path,
        sprite_sheet_definitions: &[SpriteSheetDefinition],
    ) -> Vec<TextureHandle> {
        sprite_sheet_definitions
            .iter()
            .map(|sheet_definition| {
                Self::load(
                    // TODO: resilient code
                    world,
                    String::from(
                        object_directory
                            .join(&sheet_definition.path)
                            .to_str()
                            .unwrap(),
                    ),
                )
            })
            .collect::<Vec<TextureHandle>>()
    }

    /// Returns a `TextureHandle` to the image.
    ///
    /// This function expects the image to be in PNG format.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` that stores resources.
    /// * `path`: Path to the sprite sheet.
    fn load<N>(world: &World, path: N) -> TextureHandle
    where
        N: Into<String>,
    {
        let loader = world.read_resource::<Loader>();
        loader.load(
            path,
            PngFormat,
            Default::default(),
            (),
            &world.read_resource::<AssetStorage<Texture>>(),
        )
    }
}
