#[cfg(test)]
mod tests {
    use std::{iter::FromIterator, path::PathBuf};

    use amethyst::{
        ecs::{World, WorldExt},
        input::StringBindings,
        window::ScreenDimensions,
        Error,
    };
    use amethyst_test::AmethystApplication;
    use application::{AppDir, FindContext};
    use ron;
    use strum::IntoEnumIterator;

    use application_ui::{FontVariant, Theme, ThemeLoader};

    #[test]
    fn build_adds_theme_with_fonts_to_world() -> Result<(), Error> {
        let assertion = |world: &mut World| {
            ThemeLoader::load_internal(world, AppDir::RESOURCES, "font_config.ron").unwrap();

            let theme = world.read_resource::<Theme>();
            let fonts = &theme.fonts;

            FontVariant::iter().for_each(|variant| assert!(fonts.contains_key(&variant)));
        };

        AmethystApplication::ui_base::<StringBindings>()
            .with_assertion(assertion)
            .with_resource(ScreenDimensions::new(640, 480, 1.))
            .run()
    }

    #[test]
    fn fails_with_useful_error_when_font_config_does_not_exist() -> Result<(), Error> {
        let assertion = |world: &mut World| {
            if let Some(find_context) =
                ThemeLoader::load_internal(world, AppDir::RESOURCES, "non_existent.ron")
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

        AmethystApplication::ui_base::<StringBindings>()
            .with_assertion(assertion)
            .with_resource(ScreenDimensions::new(640, 480, 1.))
            .run()
    }

    #[test]
    fn fails_with_useful_error_when_font_config_fails_to_parse() -> Result<(), Error> {
        let assertion = |world: &mut World| {
            if let Some(_ron_error) = ThemeLoader::load_internal(
                world,
                PathBuf::from_iter(["src", "application_ui"].iter())
                    .to_str()
                    .expect("Expected path to be valid UTF8."),
                "bad_config.ron",
            )
            .unwrap_err()
            .as_error()
            .downcast_ref::<Box<ron::de::Error>>()
            {
                // pass
            } else {
                panic!("Expected resource deserialization error"); // kcov-ignore
            }
        };

        AmethystApplication::ui_base::<StringBindings>()
            .with_assertion(assertion)
            .with_resource(ScreenDimensions::new(640, 480, 1.))
            .run()
    }
}
