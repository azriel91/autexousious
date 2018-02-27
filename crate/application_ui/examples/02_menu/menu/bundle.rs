use amethyst::core::bundle::{ECSBundle, Result};
use amethyst::ecs::{DispatcherBuilder, World};

use menu::main_menu;
use menu::{MenuItem, UiEventHandlerSystem};

/// This bundle prepares the world for a menu.
pub struct MenuBundle;

impl<'a, 'b> ECSBundle<'a, 'b> for MenuBundle {
    fn build(
        self,
        world: &mut World,
        builder: DispatcherBuilder<'a, 'b>,
    ) -> Result<DispatcherBuilder<'a, 'b>> {
        world.register::<MenuItem<main_menu::Index>>();

        Ok(builder.add(UiEventHandlerSystem::new(), "ui_event_handler", &[]))
    }
}
