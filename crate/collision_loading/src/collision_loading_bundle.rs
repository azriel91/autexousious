use amethyst::{
    assets::Processor,
    core::bundle::{Result, SystemBundle},
    ecs::prelude::*,
};
use collision_model::config::CollisionFrame;
use typename::TypeName;

use CollisionLoadingSystem;

/// Adds `Processor::<CollisionFrame>` to the `World` with id `"collision_frame_processor"`.
#[derive(Debug, new)]
pub struct CollisionLoadingBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for CollisionLoadingBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        builder.add(
            CollisionLoadingSystem::new(),
            &CollisionLoadingSystem::type_name(),
            &[],
        );
        builder.add(
            Processor::<CollisionFrame>::new(),
            "collision_frame_processor",
            &[&CollisionLoadingSystem::type_name()],
        );
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use amethyst::assets::AssetStorage;
    use amethyst_test_support::AmethystApplication;
    use collision_model::config::CollisionFrame;

    use super::CollisionLoadingBundle;

    #[test]
    fn bundle_build_adds_collision_frame_processor() {
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::blank()
                .with_bundle(CollisionLoadingBundle::new())
                .with_assertion(|world| {
                    // Next line will panic if the Processor wasn't added
                    world.read_resource::<AssetStorage<CollisionFrame>>();
                })
                .run()
                .is_ok()
        );
    }
}
