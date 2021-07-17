use std::path::{Component, Path};

use amethyst::{
    assets::{AssetStorage, Handle, Loader, ProgressCounter},
    renderer::{formats::texture::ImageFormat, Texture},
    Error,
};
use log::error;
use sprite_model::config::SpriteSheetDefinition;

/// Loads textures specified in the sprite sheet definitions.
#[derive(Debug)]
pub struct TextureLoader;

impl TextureLoader {
    /// Loads the sprite sheet images as textures and returns the texture
    /// handles.
    ///
    /// # Parameters
    ///
    /// * `progress_counter`: `ProgressCounter` to track loading.
    /// * `loader`: `Loader` to load assets.
    /// * `texture_assets`: `AssetStorage` for `Texture`s.
    /// * `object_directory`: Object configuration base directory.
    /// * `sprite_sheet_definitions`: List of metadata for sprite sheets to
    ///   load.
    pub fn load_textures(
        progress_counter: &mut ProgressCounter,
        loader: &Loader,
        texture_assets: &AssetStorage<Texture>,
        object_directory: &Path,
        sprite_sheet_definitions: &[SpriteSheetDefinition],
    ) -> Result<Vec<Handle<Texture>>, Error> {
        let texture_results = sprite_sheet_definitions
            .iter()
            .map(|sheet_definition| {
                // We need to do this to handle mixed slashes on Windows.
                let sheet_definition_path = Path::new(&sheet_definition.path);
                let sprite_image_path = if sheet_definition_path.is_absolute() {
                    sheet_definition_path.to_path_buf()
                } else {
                    sheet_definition_path.components().fold(
                        object_directory.to_path_buf(),
                        |mut sprite_image_path, sheet_definition_component| {
                            match sheet_definition_component {
                                Component::ParentDir => {
                                    sprite_image_path.pop();
                                }
                                Component::Normal(segment) => {
                                    sprite_image_path.push(segment);
                                }
                                Component::Prefix(_) | Component::RootDir | Component::CurDir => {}
                            }

                            sprite_image_path
                        },
                    )
                };

                let error_msg = format!(
                    "Failed to transform sprite image path to String: `{}`",
                    sprite_image_path.display()
                );

                let sprite_image_path = sprite_image_path.to_str().ok_or(error_msg)?;

                Ok(Self::load(
                    progress_counter,
                    loader,
                    texture_assets,
                    String::from(sprite_image_path),
                ))
            })
            .collect::<Vec<Result<Handle<Texture>, String>>>();

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

                return Err(Error::from_string(error_message));
            } // kcov-ignore-end
        }

        let texture_handles = texture_results
            .into_iter()
            .map(Result::unwrap)
            .collect::<Vec<Handle<Texture>>>();

        Ok(texture_handles)
    }

    /// Returns a `Handle<Texture>` to the image.
    ///
    /// This function expects the image to be in PNG format.
    ///
    /// # Parameters
    ///
    /// * `progress_counter`: `ProgressCounter` to track loading.
    /// * `loader`: `Loader` to load assets.
    /// * `texture_assets`: `AssetStorage` for `Texture`s.
    /// * `path`: Path to the sprite sheet.
    fn load<N>(
        progress_counter: &mut ProgressCounter,
        loader: &Loader,
        texture_assets: &AssetStorage<Texture>,
        path: N,
    ) -> Handle<Texture>
    where
        N: Into<String>,
    {
        loader.load(
            path,
            ImageFormat::default(),
            progress_counter,
            &texture_assets,
        )
    }
}
