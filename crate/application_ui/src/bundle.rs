//! ECS input bundle for custom events

use std::error::Error;
use std::fs::File;

use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::bundle::{ECSBundle, Result};
use amethyst::ecs::{DispatcherBuilder, World};
use amethyst::ui::{FontAsset, TtfFormat};
use application::config::find_in;
use ron::de::from_reader;

use font_config::FontConfig;
use font_variant::FontVariant;

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
    fn build(
        self,
        world: &mut World,
        builder: DispatcherBuilder<'a, 'b>,
    ) -> Result<DispatcherBuilder<'a, 'b>> {
        let font_config_path = find_in(
            "resources",
            self.font_config_name,
            Some(development_base_dirs!()),
        )?;

        let font_config_file = File::open(font_config_path).map_err(|e| error_description(&e))?;
        let font_config: FontConfig =
            from_reader(font_config_file).map_err(|e| error_description(&e))?;

        // Order is important, this must align with `font_variant::FontVariant`
        let mut font_paths = vec![
            (FontVariant::Regular, font_config.regular),
            (FontVariant::Bold, font_config.bold),
            (FontVariant::Italic, font_config.italic),
            (FontVariant::BoldItalic, font_config.bold_italic),
        ];

        font_paths.drain(..).for_each(|(font_variant, font_path)| {
            let font_handle = {
                // `world` is borrowed immutably in here for `loader` and `font_storage`
                let loader = world.read_resource::<Loader>();
                let font_storage = world.read_resource::<AssetStorage<FontAsset>>();
                loader.load(font_path, TtfFormat, (), (), &font_storage)
            };

            // `world` is borrowed mutably here to add the font handle
            world.add_resource_with_id(font_handle, font_variant as usize);
        });

        Ok(builder)
    }
}

// The kcov-ignore lines are in odd places, but that's because the native code does not always line
// up with the source code.
// kcov-ignore-start
fn error_description<E: Error>(e: &E) -> String {
    // kcov-ignore-end
    e.description().to_string()
} // kcov-ignore

#[cfg(test)]
mod test {
    use amethyst::Result;
    use amethyst::prelude::*;
    use amethyst::ui::{FontHandle, UiBundle};

    use font_variant::FontVariant;
    use super::ApplicationUiBundle;

    fn setup<'a, 'b>(application_ui_bundle: ApplicationUiBundle) -> Result<Application<'a, 'b>> {
        // We need to instantiate an amethyst::Application because:
        //
        // * The `Loader` needs to be added to the world, and the code to do this is non-trivial
        // * The `AppBundle` in amethyst that does this is non-public
        Application::build(format!("{}/assets", env!("CARGO_MANIFEST_DIR")), MockState)?
            .with_bundle(UiBundle::new())?
            .with_bundle(application_ui_bundle)?
            .build()
    } // kcov-ignore

    #[test]
    fn build_adds_font_to_world() {
        let app = setup(ApplicationUiBundle::new())
            .expect("Failed to build Application, check the bundle initialization code.");

        let world = &app.world;

        // If the font was not added, the next line will panic
        let _font_handle_regular =
            world.read_resource_with_id::<FontHandle>(FontVariant::Regular as usize);
        let _font_handle_bold =
            world.read_resource_with_id::<FontHandle>(FontVariant::Bold as usize);
        let _font_handle_italic =
            world.read_resource_with_id::<FontHandle>(FontVariant::Italic as usize);
        let _font_handle_bold_italic =
            world.read_resource_with_id::<FontHandle>(FontVariant::BoldItalic as usize);

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
