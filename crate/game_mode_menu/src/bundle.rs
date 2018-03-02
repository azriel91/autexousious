use amethyst::core::bundle::{ECSBundle, Result};
use amethyst::ecs::{DispatcherBuilder, World};
use application_menu::MenuItem;

use index::Index;

/// This bundle prepares the world for a menu.
#[derive(Debug)]
pub struct Bundle;

impl<'a, 'b> ECSBundle<'a, 'b> for Bundle {
    fn build(
        self,
        world: &mut World,
        builder: DispatcherBuilder<'a, 'b>,
    ) -> Result<DispatcherBuilder<'a, 'b>> {
        world.register::<MenuItem<Index>>();

        Ok(builder)
    }
}
