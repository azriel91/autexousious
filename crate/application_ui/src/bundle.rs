//! ECS input bundle for custom events
use std::collections::HashMap;

use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::bundle::{ECSBundle, Result};
use amethyst::ecs::prelude::{DispatcherBuilder, World};
use amethyst::ui::{FontAsset, FontHandle, TtfFormat};
use application::resource;
use application::resource::dir;
use application::resource::load_in;

use FontConfig;
use FontVariant;
use Theme;

/// Bundle that loads application UI assets.
///
/// Registers `FontHandle` resources in the world. See the [module level documentation](index.html)
/// for more details.
#[derive(Debug)]
pub struct ApplicationUiBundle {
    font_config_name: &'static str,
}

impl ApplicationUiBundle {
    /// Returns an application bundle.
    pub fn new() -> Self {
        Default::default()
    }

    /// Returns an application bundle with a custom font configuration file name.
    ///
    /// For testing purposes, this allows you to override the font configuration file name.
    ///
    /// # Parameters
    ///
    /// * `font_config_name`: Name of the font configuration file.
    #[cfg(test)]
    fn internal_new(font_config_name: &'static str) -> Self {
        ApplicationUiBundle { font_config_name }
    }
}

impl Default for ApplicationUiBundle {
    fn default() -> Self {
        ApplicationUiBundle {
            font_config_name: "font_config.ron",
        }
    }
}

impl<'a, 'b> ECSBundle<'a, 'b> for ApplicationUiBundle {
    fn build(self, world: &mut World, _: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        let font_config: FontConfig = load_in(
            dir::RESOURCES,
            self.font_config_name,
            &resource::Format::Ron,
            Some(development_base_dirs!()),
        )?;

        // Order is important, this must align with `font_variant::FontVariant`
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
            })
            .collect::<HashMap<FontVariant, FontHandle>>();

        world.add_resource(Theme::new(fonts));

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use amethyst::core::transform::TransformBundle;
    use amethyst::input::InputBundle;
    use amethyst::prelude::*;
    use amethyst::ui::UiBundle;
    use amethyst::Result;
    use strum::IntoEnumIterator;

    use super::ApplicationUiBundle;
    use FontVariant;
    use Theme;

    fn setup<'a, 'b>(application_ui_bundle: ApplicationUiBundle) -> Result<Application<'a, 'b>> {
        // We need to instantiate an amethyst::Application because:
        //
        // * The `Loader` needs to be added to the world, and the code to do this is non-trivial
        // * The `AppBundle` in amethyst that does this is non-public
        Application::build(format!("{}/assets", env!("CARGO_MANIFEST_DIR")), MockState)?
            .with_bundle(TransformBundle::new())?
            .with_bundle(InputBundle::<String, String>::new())?
            .with_bundle(UiBundle::<String, String>::new())?
            .with_bundle(application_ui_bundle)?
            .build()
    } // kcov-ignore

    #[test]
    fn build_adds_theme_with_fonts_to_world() {
        let app = setup(ApplicationUiBundle::new())
            .expect("Failed to build Application, check the bundle initialization code.");

        let world = &app.world;

        let theme = world.read_resource::<Theme>();
        let fonts = &theme.fonts;
        FontVariant::iter().for_each(|variant| assert!(fonts.contains_key(&variant)));

        // TODO: The following verification relies on https://github.com/redox-os/rusttype/issues/86
        // Need to import the hamcrest crate

        // let font_storage = world.read_resource::<AssetStorage<FontAsset>>();
        // let font_asset_regular = font_storage
        //     .get(&font_handle_regular)
        //     .expect("Failed to get regular font handle.");
        // let font_regular = font_asset_regular.0;
        // assert_that!(font_regular.font_name_strings(), contains(vec!["Source Code Pro Regular"]));
    }

    #[test]
    #[should_panic(expected = "Failed to find \\'resources/non_existent.ron\\'")]
    #[cfg(not(windows))]
    fn fails_with_useful_error_when_font_config_does_not_exist() {
        let _app = setup(ApplicationUiBundle::internal_new("non_existent.ron")).unwrap();
    } // kcov-ignore

    #[test]
    #[should_panic(expected = "Failed to find \\'resources\\\\non_existent.ron\\'")]
    #[cfg(windows)]
    fn fails_with_useful_error_when_font_config_does_not_exist() {
        let _app = setup(ApplicationUiBundle::internal_new("non_existent.ron")).unwrap();
    } // kcov-ignore

    #[test]
    #[should_panic(expected = "missing field `bold`")]
    fn fails_with_useful_error_when_font_config_fails_to_parse() {
        let _app = setup(ApplicationUiBundle::internal_new("bad_config.ron")).unwrap();
    } // kcov-ignore

    #[derive(Debug)]
    struct MockState;
    impl State for MockState {}
}
