//! ECS input bundle for custom events

use amethyst::assets::Loader;
use amethyst::core::bundle::{ECSBundle, Result};
use amethyst::ecs::{DispatcherBuilder, World};
use amethyst::ui::TtfFormat;

/// Bundle that loads application UI assets.
///
/// Registers `FontHandle` resources in the world. See the [module level documentation](index.html)
/// for more details.
#[derive(Debug, Default)]
pub struct ApplicationUiBundle;

impl ApplicationUiBundle {
    /// Returns an application bundle.
    pub fn new() -> Self {
        Default::default()
    }
}

impl<'a, 'b> ECSBundle<'a, 'b> for ApplicationUiBundle {
    fn build(
        self,
        world: &mut World,
        builder: DispatcherBuilder<'a, 'b>,
    ) -> Result<DispatcherBuilder<'a, 'b>> {
        let _font = world.read_resource::<Loader>().load(
            "font/source-code-pro-2.030R-ro-1.050R-it/TTF/SourceCodePro-Regular.ttf",
            TtfFormat,
            Default::default(),
            (),
            &world.read_resource(),
        );

        Ok(builder)
    }
}

#[cfg(test)]
mod test {
    use amethyst::Result;
    use amethyst::prelude::*;
    use amethyst::ui::{FontHandle, UiBundle};

    use super::ApplicationUiBundle;

    fn setup<'a, 'b>() -> Result<Application<'a, 'b>> {
        // We need to instantiate an amethyst::Application because:
        //
        // * The `Loader` needs to be added to the world, and the code to do this is non-trivial
        // * The `AppBundle` in amethyst that does this is non-public
        Application::build(format!("{}/assets", env!("CARGO_MANIFEST_DIR")), MockState)?
            .with_bundle(UiBundle::new())?
            .with_bundle(ApplicationUiBundle::new())?
            .build()
    }

    #[test]
    fn build_adds_font_to_world() {
        let app =
            setup().expect("Failed to build Application, check the bundle initialization code.");

        // If the font was not added, the next line will panic
        let _font = app.world.read::<FontHandle>();
    }

    #[derive(Debug)]
    struct MockState;
    impl State for MockState {}
}
