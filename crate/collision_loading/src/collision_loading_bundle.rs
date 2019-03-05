use amethyst::{assets::Processor, core::bundle::SystemBundle, ecs::DispatcherBuilder, Error};
use collision_model::config::{Body, Interactions};
use derive_new::new;
use typename::TypeName;

use crate::CollisionLoadingSystem;

/// Adds `Body` and `Interactions` processors to the `World`.
///
/// * `Processor::<Body>` is added with id `"body_processor"`.
/// * `Processor::<Interactions>` is added with id `"interactions_processor"`.
#[derive(Debug, new)]
pub struct CollisionLoadingBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for CollisionLoadingBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<(), Error> {
        builder.add(
            CollisionLoadingSystem::new(),
            &CollisionLoadingSystem::type_name(),
            &[],
        );
        builder.add(
            Processor::<Body>::new(),
            "body_processor",
            &[&CollisionLoadingSystem::type_name()],
        );
        builder.add(
            Processor::<Interactions>::new(),
            "interactions_processor",
            &[&CollisionLoadingSystem::type_name()],
        );
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use amethyst::assets::AssetStorage;
    use amethyst_test::AmethystApplication;
    use collision_model::config::{Body, Interactions};

    use super::CollisionLoadingBundle;

    #[test]
    fn bundle_build_adds_body_and_interactions_processor() {
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::blank()
                .with_bundle(CollisionLoadingBundle::new())
                .with_assertion(|world| {
                    // Next line will panic if the Processors aren't added
                    world.read_resource::<AssetStorage<Body>>();
                    world.read_resource::<AssetStorage<Interactions>>();
                })
                .run()
                .is_ok()
        );
    }
}
