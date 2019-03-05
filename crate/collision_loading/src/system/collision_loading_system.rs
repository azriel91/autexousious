use amethyst::{
    assets::AssetStorage,
    ecs::{Read, System},
};
use collision_model::config::{Body, Interactions};
use derive_new::new;
use typename_derive::TypeName;

/// Adds a default `Body` to the resources.
#[derive(Debug, Default, TypeName, new)]
pub(crate) struct CollisionLoadingSystem;

type CollisionLoadingSystemData<'s> = (
    Read<'s, AssetStorage<Body>>,
    Read<'s, AssetStorage<Interactions>>,
);

impl<'s> System<'s> for CollisionLoadingSystem {
    type SystemData = CollisionLoadingSystemData<'s>;

    fn run(&mut self, _: Self::SystemData) {}
}

#[cfg(test)]
mod test {
    use amethyst::{assets::AssetStorage, ecs::System};
    use amethyst_test::AmethystApplication;
    use collision_model::config::{Body, Interactions};

    use super::CollisionLoadingSystem;

    #[test]
    fn setup_includes_body_frame_asset_storage() {
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::ui_base::<String, String>()
                .with_setup(|world| CollisionLoadingSystem::new().setup(&mut world.res))
                .with_assertion(|world| {
                    assert!(world.res.try_fetch::<AssetStorage<Body>>().is_some());
                    assert!(world
                        .res
                        .try_fetch::<AssetStorage<Interactions>>()
                        .is_some());
                })
                .run()
                .is_ok()
        );
    }
}
