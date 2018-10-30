use amethyst::{
    assets::{AssetStorage, Loader, ProgressCounter},
    ecs::{Read, Resources, System, SystemData, WriteExpect},
};
use collision_model::{
    animation::{CollisionDataSet, DEFAULT_COLLISION_FRAME_ID},
    config::CollisionFrame,
};

/// Adds a default `CollisionFrame` to the resources.
#[derive(Debug, Default, TypeName, new)]
pub(crate) struct CollisionLoadingSystem;

type CollisionLoadingSystemData<'s> = (
    Read<'s, AssetStorage<CollisionFrame>>,
    WriteExpect<'s, CollisionDataSet>,
);

impl<'s> System<'s> for CollisionLoadingSystem {
    type SystemData = CollisionLoadingSystemData<'s>;

    fn run(&mut self, (_collision_frames, mut _collision_data_set): Self::SystemData) {}

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);

        if !res.has_value::<CollisionDataSet>() {
            res.insert(CollisionDataSet::new());
        }

        // Default handle
        let collision_frame_handle = {
            let loader = res.fetch::<Loader>();
            loader.load_from_data(
                CollisionFrame::default(),
                &mut ProgressCounter::new(),
                &*res.fetch::<AssetStorage<_>>(),
            )
        };
        let mut collision_data_set = res.fetch_mut::<CollisionDataSet>();
        collision_data_set.insert(DEFAULT_COLLISION_FRAME_ID, collision_frame_handle);
    }
}

#[cfg(test)]
mod test {
    use amethyst::ecs::System;
    use amethyst_test_support::AmethystApplication;
    use collision_model::animation::{CollisionDataSet, DEFAULT_COLLISION_FRAME_ID};

    use super::CollisionLoadingSystem;

    #[test]
    fn setup_inserts_default_collision_frame() {
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::ui_base::<String, String>()
                .with_setup(|world| CollisionLoadingSystem::new().setup(&mut world.res))
                .with_assertion(|world| {
                    let collision_data_set = world.res.fetch::<CollisionDataSet>();

                    assert!(collision_data_set
                        .data(DEFAULT_COLLISION_FRAME_ID)
                        .is_some());
                })
                .run()
                .is_ok()
        );
    }
}
