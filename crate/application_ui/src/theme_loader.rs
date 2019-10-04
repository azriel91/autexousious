use std::collections::HashMap;

use amethyst::{
    assets::{AssetStorage, Loader},
    ecs::{World, WorldExt},
    ui::{FontAsset, FontHandle, TtfFormat},
    Error,
};
use application::{AppDir, AppFile, Format};

use crate::{FontConfig, FontVariant, Theme};

/// Privates functionality to load an application theme.
#[derive(Debug)]
pub struct ThemeLoader;

impl ThemeLoader {
    /// Loads the theme into the `World`
    ///
    /// # Parameters
    ///
    /// * `world`: `World` to load the theme into.
    pub fn load(world: &mut World) -> Result<(), Error> {
        Self::load_internal(world, AppDir::RESOURCES, "font_config.ron")
    }

    /// Visible for testing.
    #[inline]
    pub fn load_internal(
        world: &mut World,
        conf_dir: &str,
        font_config_name: &str,
    ) -> Result<(), Error> {
        let font_config: FontConfig = AppFile::load_in(conf_dir, font_config_name, Format::Ron)?;

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
