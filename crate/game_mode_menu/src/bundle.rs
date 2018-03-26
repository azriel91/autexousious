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

#[cfg(test)]
mod test {
    use amethyst::core::bundle::ECSBundle;
    use amethyst::ecs::{DispatcherBuilder, World};
    use application_menu::MenuItem;

    use super::Bundle;
    use index::Index;

    #[test]
    fn build_adds_application_event_channel_to_world() {
        let mut world = World::new();
        let builder = DispatcherBuilder::new();

        Bundle
            .build(&mut world, builder)
            .expect("game_mode_menu::Bundle#build() should succeed");

        // If the component was not registered, the next line will panic
        let _ = world
            .create_entity()
            .with(MenuItem {
                index: Index::StartGame,
            })
            .build();
    }
}
