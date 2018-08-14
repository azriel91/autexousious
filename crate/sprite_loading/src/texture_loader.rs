use std::path::Path;

use amethyst::{
    assets::{AssetStorage, Loader},
    prelude::*,
    renderer::{PngFormat, Texture, TextureHandle},
};
use application::{self, ErrorKind};
use sprite_model::config::SpriteSheetDefinition;

#[derive(Debug)]
pub(crate) struct TextureLoader;

impl TextureLoader {
    /// Loads the sprite sheet images as textures and returns the texture handles.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` to store the sprite sheet textures.
    /// * `object_directory`: Object configuration base directory.
    /// * `sprite_sheet_definitions`: List of metadata for sprite sheets to load.
    pub(crate) fn load_textures(
        world: &World,
        object_directory: &Path,
        sprite_sheet_definitions: &[SpriteSheetDefinition],
    ) -> application::Result<Vec<TextureHandle>> {
        let texture_results = sprite_sheet_definitions
            .iter()
            .map(|sheet_definition| {
                let sprite_image_path = object_directory.join(&sheet_definition.path);

                let error_msg = format!(
                    "Failed to transform sprite image path to String: `{}`",
                    sprite_image_path.display()
                );

                let sprite_image_path = sprite_image_path.to_str().ok_or(error_msg)?;

                Ok(Self::load(world, String::from(sprite_image_path)))
            }).collect::<Vec<Result<TextureHandle, String>>>();

        {
            let failed_to_load = texture_results
                .iter()
                .filter(|result| result.is_err())
                .map(|result| result.as_ref().unwrap_err().as_str()) // kcov-ignore
                .collect::<Vec<&str>>();

            if !failed_to_load.is_empty() {
                // kcov-ignore-start
                let mut error_message = String::with_capacity(30 + failed_to_load.len() * 200);
                error_message.push_str("Failed to load textures:\n\n");
                failed_to_load.iter().for_each(|error| {
                    error_message.push_str("* ");
                    error_message.push_str(error);
                    error_message.push('\n');
                });
                error_message.push('\n');

                error!("{}", &error_message);

                return Err(ErrorKind::Msg(error_message).into());
            } // kcov-ignore-end
        }

        let texture_handles = texture_results
            .into_iter()
            .map(|result| result.unwrap())
            .collect::<Vec<TextureHandle>>();

        Ok(texture_handles)
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
