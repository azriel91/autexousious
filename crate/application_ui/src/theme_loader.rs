use std::collections::HashMap;

use amethyst::{
    assets::{AssetStorage, Loader},
    ecs::{World, WorldExt},
    ui::{FontAsset, FontHandle, TtfFormat},
    Error,
};

use crate::{FontConfig, FontVariant, Theme};

/// Provides functionality to load an application theme.
#[derive(Debug)]
pub struct ThemeLoader;

impl ThemeLoader {
    /// Loads the theme into the `World`
    ///
    /// # Parameters
    ///
    /// * `world`: `World` to load the theme into.
    /// * `font_config`: Loaded font configuration.
    pub fn load(world: &mut World, font_config: FontConfig) -> Result<(), Error> {
        let font_paths = vec![
            (FontVariant::Regular, font_config.regular),
            (FontVariant::Bold, font_config.bold),
            (FontVariant::Italic, font_config.italic),
            (FontVariant::BoldItalic, font_config.bold_italic),
        ];

        let fonts = font_paths
            .into_iter()
            .map(|(font_variant, font_path)| {
                let loader = world.read_resource::<Loader>();
                let font_storage = world.read_resource::<AssetStorage<FontAsset>>();
                let font_handle = loader.load(font_path, TtfFormat, (), &font_storage);
                (font_variant, font_handle)
            })
            .collect::<HashMap<FontVariant, FontHandle>>();

        world.insert(Theme::new(fonts));

        Ok(())
    }
}
