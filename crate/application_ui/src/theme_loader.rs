use std::collections::HashMap;

use amethyst::{
    assets::{AssetStorage, Loader},
    ecs::World,
    ui::{FontAsset, FontHandle, TtfFormat},
    Error,
};
use application::{
    development_base_dirs,
    resource::{dir, load_in},
    Format,
};

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
        Self::load_internal(world, "font_config.ron")
    }

    #[inline]
    fn load_internal(world: &mut World, font_config_name: &str) -> Result<(), Error> {
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
                let font_handle = loader.load(font_path, TtfFormat, (), &font_storage);
                (font_variant, font_handle)
            })
            .collect::<HashMap<FontVariant, FontHandle>>();

        world.insert(Theme::new(fonts));

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use amethyst::{input::StringBindings, prelude::*, window::ScreenDimensions};
    use amethyst_test::prelude::*;
    use application::FindContext;
    use ron;
    use strum::IntoEnumIterator;

    use super::ThemeLoader;
    use crate::{FontVariant, Theme};

    #[test]
    fn build_adds_theme_with_fonts_to_world() {
        let assertion = |world: &mut World| {
            ThemeLoader::load_internal(world, "font_config.ron").unwrap();

            let theme = world.read_resource::<Theme>();
            let fonts = &theme.fonts;

            FontVariant::iter().for_each(|variant| assert!(fonts.contains_key(&variant)));
        };

        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::ui_base::<StringBindings>()
                .with_assertion(assertion)
                .with_resource(ScreenDimensions::new(640, 480, 1.))
                .run()
                .is_ok()
        );
    }

    #[test]
    fn fails_with_useful_error_when_font_config_does_not_exist() {
        let assertion = |world: &mut World| {
            if let Some(find_context) = ThemeLoader::load_internal(world, "non_existent.ron")
                .unwrap_err()
                .as_error()
                .downcast_ref::<Box<FindContext>>()
            {
                assert_eq!("non_existent.ron", find_context.file_name);
            } else {
                // kcov-ignore-start
                panic!("Expected resource `Find` error containing `non_existent.ron`");
                // kcov-ignore-end
            }
        };

        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::ui_base::<StringBindings>()
                .with_assertion(assertion)
                .with_resource(ScreenDimensions::new(640, 480, 1.))
                .run()
                .is_ok()
        );
    }

    #[test]
    fn fails_with_useful_error_when_font_config_fails_to_parse() {
        let assertion = |world: &mut World| {
            if let Some(_ron_error) = ThemeLoader::load_internal(world, "bad_config.ron")
                .unwrap_err()
                .as_error()
                .downcast_ref::<Box<ron::de::Error>>()
            {
                // pass
            } else {
                panic!("Expected resource deserialization error"); // kcov-ignore
            }
        };

        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::ui_base::<StringBindings>()
                .with_assertion(assertion)
                .with_resource(ScreenDimensions::new(640, 480, 1.))
                .run()
                .is_ok()
        );
    }
}
