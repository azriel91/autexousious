use amethyst::{
    assets::AssetStorage,
    ecs::{Read, System},
};
use collision_model::config::{BodyFrame, InteractionFrame};
use derive_new::new;
use typename_derive::TypeName;

/// Adds a default `BodyFrame` to the resources.
#[derive(Debug, Default, TypeName, new)]
pub(crate) struct CollisionLoadingSystem;

type CollisionLoadingSystemData<'s> = (
    Read<'s, AssetStorage<BodyFrame>>,
    Read<'s, AssetStorage<InteractionFrame>>,
);

impl<'s> System<'s> for CollisionLoadingSystem {
    type SystemData = CollisionLoadingSystemData<'s>;

    fn run(&mut self, _: Self::SystemData) {}
}

#[cfg(test)]
mod test {
    use amethyst::{assets::AssetStorage, ecs::System};
    use amethyst_test::AmethystApplication;
    use collision_model::config::{BodyFrame, InteractionFrame};

    use super::CollisionLoadingSystem;

    #[test]
    fn setup_includes_body_frame_asset_storage() {
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::ui_base::<String, String>()
                .with_setup(|world| CollisionLoadingSystem::new().setup(&mut world.res))
                .with_assertion(|world| {
                    assert!(world.res.try_fetch::<AssetStorage<BodyFrame>>().is_some());
                    assert!(world
                        .res
                        .try_fetch::<AssetStorage<InteractionFrame>>()
                        .is_some());
                })
                .run()
                .is_ok()
        );
    }
}
