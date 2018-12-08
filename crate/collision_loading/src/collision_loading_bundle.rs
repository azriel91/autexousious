use amethyst::{
    assets::Processor,
    core::bundle::{Result, SystemBundle},
    ecs::prelude::*,
};
use collision_model::config::{BodyFrame, InteractionFrame};
use derive_new::new;
use typename::TypeName;

use crate::CollisionLoadingSystem;

/// Adds `BodyFrame` and `InteractionFrame` processors to the `World`.
///
/// * `Processor::<BodyFrame>` is added with id `"body_frame_processor"`.
/// * `Processor::<InteractionFrame>` is added with id `"interaction_frame_processor"`.
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
            Processor::<BodyFrame>::new(),
            "body_frame_processor",
            &[&CollisionLoadingSystem::type_name()],
        );
        builder.add(
            Processor::<InteractionFrame>::new(),
            "interaction_frame_processor",
            &[&CollisionLoadingSystem::type_name()],
        );
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use amethyst::assets::AssetStorage;
    use amethyst_test::AmethystApplication;
    use collision_model::config::{BodyFrame, InteractionFrame};

    use super::CollisionLoadingBundle;

    #[test]
    fn bundle_build_adds_body_frame_processor() {
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::blank()
                .with_bundle(CollisionLoadingBundle::new())
                .with_assertion(|world| {
                    // Next line will panic if the Processors aren't added
                    world.read_resource::<AssetStorage<BodyFrame>>();
                    world.read_resource::<AssetStorage<InteractionFrame>>();
                })
                .run()
                .is_ok()
        );
    }
}
