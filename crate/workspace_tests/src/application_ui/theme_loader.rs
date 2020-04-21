#[cfg(test)]
mod tests {
    use amethyst::{
        ecs::{World, WorldExt},
        input::StringBindings,
        window::ScreenDimensions,
        Error,
    };
    use amethyst_test::AmethystApplication;

    use strum::IntoEnumIterator;

    use application_ui::{FontConfigLoader, FontVariant, Theme, ThemeLoader};

    #[test]
    fn build_adds_theme_with_fonts_to_world() -> Result<(), Error> {
        let font_config = FontConfigLoader::load()?;
        let assertion = |world: &mut World| {
            ThemeLoader::load(world, font_config).unwrap();

            let theme = world.read_resource::<Theme>();
            let fonts = &theme.fonts;

            FontVariant::iter().for_each(|variant| assert!(fonts.contains_key(&variant)));
        };

        AmethystApplication::ui_base::<StringBindings>()
            .with_assertion(assertion)
            .with_resource(ScreenDimensions::new(640, 480, 1.))
            .run()
    }
}
