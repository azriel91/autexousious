use amethyst::{
    core::bundle::{Result, SystemBundle}, ecs::prelude::DispatcherBuilder,
};

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

    use amethyst::{
        core::transform::TransformBundle, input::InputBundle, prelude::*, ui::UiBundle, Result,
    };
    use application_menu::MenuItem;

    use super::GameModeMenuBundle;
    use index::Index;

    fn setup<'a, 'b>() -> Result<Application<'a, GameData<'a, 'b>>> {
        env::set_var("APP_DIR", env!("CARGO_MANIFEST_DIR"));

        let game_data = GameDataBuilder::default()
            .with_bundle(TransformBundle::new())?
            .with_bundle(InputBundle::<String, String>::new())?
            .with_bundle(UiBundle::<String, String>::new())?
            .with_bundle(GameModeMenuBundle)?;
        let app = Application::new(
            format!("{}/assets", env!("CARGO_MANIFEST_DIR")),
            MockState,
            game_data,
        )?;

        Ok(app)
    } // kcov-ignore

    #[test]
    fn bundle_should_allow_menu_items_to_be_created() {
        let mut app = setup().expect("GameModeMenuBundle#build() should succeed");

        // If the system was not registered, this will panic
        app.run();
    }

    #[derive(Debug)]
    struct MockState;
    impl<'a, 'b> State<GameData<'a, 'b>> for MockState {
        fn update(&mut self, data: StateData<GameData>) -> Trans<GameData<'a, 'b>> {
            data.world
                .create_entity()
                .with(MenuItem {
                    index: Index::StartGame,
                })
                .build();

            Trans::Quit
        }
    }
}
