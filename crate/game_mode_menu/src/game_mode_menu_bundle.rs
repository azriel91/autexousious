use amethyst::{
    core::bundle::{Result, SystemBundle},
    ecs::prelude::*,
};

use UiEventHandlerSystem;

/// Registers the game mode menu `UiEventHandlerSystem` to the dispatcher.
#[derive(Debug, new)]
pub(crate) struct GameModeMenuBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for GameModeMenuBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        builder.add(
            UiEventHandlerSystem::new(),
            "",
            &[], // "ui_keyboard_system", "ui_mouse_system"
        );
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::env;

    use amethyst::prelude::*;
    use amethyst_test_support::prelude::*;
    use application_menu::MenuItem;

    use super::GameModeMenuBundle;
    use Index;

    #[test]
    fn bundle_registration_enables_menu_items_to_be_created() {
        env::set_var("APP_DIR", env!("CARGO_MANIFEST_DIR"));

        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::ui_base::<String, String>()
                .with_bundle(GameModeMenuBundle)
                // kcov-ignore-start
                .with_effect(|world| {
                    world
                        .create_entity()
                        .with(MenuItem {
                            index: Index::StartGame,
                        })
                        .build();
                })
                // kcov-ignore-end
                .run()
                .is_ok()
        );
    }
}
