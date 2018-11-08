use amethyst::{
    assets::AssetStorage,
    ecs::{Read, Resources, System, SystemData},
};
use collision_model::config::CollisionFrame;

/// Adds a default `CollisionFrame` to the resources.
#[derive(Debug, Default, TypeName, new)]
pub(crate) struct CollisionLoadingSystem;

type CollisionLoadingSystemData<'s> = Read<'s, AssetStorage<CollisionFrame>>;

impl<'s> System<'s> for CollisionLoadingSystem {
    type SystemData = CollisionLoadingSystemData<'s>;

    fn run(&mut self, _collision_frames: Self::SystemData) {}

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
    }
}

#[cfg(test)]
mod test {
    use amethyst::{assets::AssetStorage, ecs::System};
    use amethyst_test::AmethystApplication;
    use collision_model::config::CollisionFrame;

    use super::CollisionLoadingSystem;

    #[test]
    fn setup_includes_collision_frame_asset_storage() {
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::ui_base::<String, String>()
                .with_setup(|world| CollisionLoadingSystem::new().setup(&mut world.res))
                .with_assertion(|world| {
                    assert!(world
                        .res
                        .try_fetch::<AssetStorage<CollisionFrame>>()
                        .is_some());
                })
                .run()
                .is_ok()
        );
    }
}
