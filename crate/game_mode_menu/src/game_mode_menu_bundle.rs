use amethyst::{
    core::bundle::{Result, SystemBundle},
    ecs::prelude::DispatcherBuilder,
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

    use amethyst_test_support::prelude::*;
    use application_menu::MenuItem;

    use super::GameModeMenuBundle;
    use Index;

    #[test]
    fn bundle_should_allow_menu_items_to_be_created() {
        env::set_var("APP_DIR", env!("CARGO_MANIFEST_DIR"));

        assert!(
            AmethystApplication::base()
                .with_bundle(GameModeMenuBundle)
                .with_effect(|world| {
                    world
                        .create_entity()
                        .with(MenuItem {
                            index: Index::StartGame,
                        })
                        .build();
                })
                .run()
                .is_ok()
        );
    }
}
