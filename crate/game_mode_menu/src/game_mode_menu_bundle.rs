use amethyst::core::bundle::{Result, SystemBundle};
use amethyst::ecs::prelude::DispatcherBuilder;

use UiEventHandlerSystem;

/// This bundle prepares the world for a menu.
#[derive(Debug)]
pub struct GameModeMenuBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for GameModeMenuBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        builder.add(
            UiEventHandlerSystem::new(),
            "",
            &["ui_system", "ui_mouse_system"],
        );
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::env;

    use amethyst::core::transform::TransformBundle;
    use amethyst::input::InputBundle;
    use amethyst::prelude::*;
    use amethyst::ui::UiBundle;
    use amethyst::Result;
    use application_menu::MenuItem;

    use super::GameModeMenuBundle;
    use index::Index;

    fn setup<'a, 'b>() -> Result<Application<'a, 'b>> {
        env::set_var("APP_DIR", env!("CARGO_MANIFEST_DIR"));
        let app = Application::build(format!("{}/assets", env!("CARGO_MANIFEST_DIR")), MockState)?
            .with_bundle(TransformBundle::new())?
            .with_bundle(InputBundle::<String, String>::new())?
            .with_bundle(UiBundle::<String, String>::new())?
            .with_bundle(GameModeMenuBundle)?
            .build()?;

        Ok(app)
    } // kcov-ignore

    #[test]
    fn bundle_should_allow_menu_items_to_be_created() {
        let mut app = setup().expect("GameModeMenuBundle#build() should succeed");

        // If the system was not registered, this will panic
        &mut app
            .world
            .create_entity()
            .with(MenuItem {
                index: Index::StartGame,
            })
            .build();
    }

    #[derive(Debug)]
    struct MockState;
    impl<'a, 'b> State<GameData<'a, 'b>> for MockState {}
}
