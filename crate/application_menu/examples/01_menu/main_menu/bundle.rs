use amethyst::core::bundle::{ECSBundle, Result};
use amethyst::ecs::{DispatcherBuilder, World};
use application_menu::MenuItem;

use main_menu;

/// This bundle prepares the world for a menu.
pub struct Bundle;

impl<'a, 'b> ECSBundle<'a, 'b> for Bundle {
    fn build(
        self,
        world: &mut World,
        builder: DispatcherBuilder<'a, 'b>,
    ) -> Result<DispatcherBuilder<'a, 'b>> {
        world.register::<MenuItem<main_menu::Index>>();

        Ok(builder)
    }
}
