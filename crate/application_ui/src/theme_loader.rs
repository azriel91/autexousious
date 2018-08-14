use std::collections::HashMap;

use amethyst::{
    assets::{AssetStorage, Loader},
    prelude::*,
    ui::{FontAsset, FontHandle, TtfFormat},
};
use application::{
    resource::{self, dir, load_in},
    Format,
};

use FontConfig;
use FontVariant;
use Theme;

/// Privates functionality to load an application theme.
#[derive(Debug)]
pub struct ThemeLoader;

impl ThemeLoader {
    /// Loads the theme into the `World`
    ///
    /// # Parameters
    ///
    /// * `world`: `World` to load the theme into.
    pub fn load(world: &mut World) -> Result<(), resource::Error> {
        Self::load_internal(world, "font_config.ron")
    }

    #[inline]
    fn load_internal(world: &mut World, font_config_name: &str) -> Result<(), resource::Error> {
        let font_config: FontConfig = load_in(
            dir::RESOURCES,
            font_config_name,
            Format::Ron,
            Some(development_base_dirs!()),
        )?;

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
                let font_handle = loader.load(font_path, TtfFormat, (), (), &font_storage);
                (font_variant, font_handle)
            }).collect::<HashMap<FontVariant, FontHandle>>();

        world.add_resource(Theme::new(fonts));

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use amethyst::{prelude::*, renderer::ScreenDimensions};
    use amethyst_test_support::prelude::*;
    use application::resource;
    use strum::IntoEnumIterator;

    use super::ThemeLoader;
    use FontVariant;
    use Theme;

    #[test]
    fn build_adds_theme_with_fonts_to_world() {
        let assertion = |world: &mut World| {
            ThemeLoader::load_internal(world, "font_config.ron").unwrap();

            let theme = world.read_resource::<Theme>();
            let fonts = &theme.fonts;
            debug!("Fonts: {:?}", &fonts);

            FontVariant::iter().for_each(|variant| assert!(fonts.contains_key(&variant)));
        };

        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::ui_base::<String, String>()
                .with_assertion(assertion)
                .with_resource(ScreenDimensions::new(640, 480, 1.))
                .run()
                .is_ok()
        );
    }

    #[test]
    fn fails_with_useful_error_when_font_config_does_not_exist() {
        let assertion = |world: &mut World| {
            if let Err(e) = ThemeLoader::load_internal(world, "non_existent.ron") {
                match *e.kind() {
                    resource::ErrorKind::Find(ref find_context) => {
                        assert_eq!("non_existent.ron", &find_context.file_name);
                        return; // pass
                    }
                    _ => {}
                }
            }

            panic!("Expected resource `Find` error containing `non_existent.ron`"); // kcov-ignore
        };

        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::ui_base::<String, String>()
                .with_assertion(assertion)
                .with_resource(ScreenDimensions::new(640, 480, 1.))
                .run()
                .is_ok()
        );
    }

    #[test]
    fn fails_with_useful_error_when_font_config_fails_to_parse() {
        let assertion = |world: &mut World| {
            if let Err(e) = ThemeLoader::load_internal(world, "bad_config.ron") {
                match *e.kind() {
                    resource::ErrorKind::RonDeserialization(ref _ron_error) => {
                        return; // pass
                    }
                    _ => {}
                }
            }

            panic!("Expected resource deserialization error"); // kcov-ignore
        };

        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::ui_base::<String, String>()
                .with_assertion(assertion)
                .with_resource(ScreenDimensions::new(640, 480, 1.))
                .run()
                .is_ok()
        );
    }
}
