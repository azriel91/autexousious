#[cfg(test)]
mod tests {
    use amethyst::{
        assets::PrefabData,
        core::{math::Vector3, Transform, TransformBundle},
        ecs::{Builder, Entity, System, SystemData, WorldExt},
        renderer::{types::DefaultBackend, RenderEmptyBundle},
        Error,
    };
    use amethyst_test::AmethystApplication;
    use charge_model::play::ChargeTrackerClock;

    use game_play_hud::{CpBarPrefab, CpBarUpdateSystem};

    #[test]
    fn sets_transform_x_and_scale() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_bundle(TransformBundle::new())
            .with_bundle_event_fn(|event_loop| RenderEmptyBundle::<DefaultBackend>::new(event_loop))
            .with_effect(|world| {
                <CpBarPrefab as PrefabData>::SystemData::setup(world);
                <CpBarUpdateSystem as System>::SystemData::setup(world);

                let mut transform = Transform::default();
                transform.set_translation_x(123.);
                transform.set_translation_y(456.);
                transform.set_translation_z(789.);

                let charge_tracker_clock = ChargeTrackerClock::new_with_value(20, 18);
                let char_entity = {
                    world
                        .create_entity()
                        .with(transform)
                        .with(charge_tracker_clock)
                        .build()
                };

                let cp_bar_entity = {
                    let cp_bar_entity = world.create_entity().build();

                    let mut cp_bar_prefab_system_data =
                        world.system_data::<<CpBarPrefab as PrefabData>::SystemData>();
                    let cp_bar_prefab = CpBarPrefab::new(char_entity);

                    cp_bar_prefab
                        .add_to_entity(cp_bar_entity, &mut cp_bar_prefab_system_data, &[], &[])
                        .expect("`CpBarPrefab` failed to augment entity.");

                    cp_bar_entity
                };

                world.insert(cp_bar_entity);
            })
            .with_system_single(CpBarUpdateSystem::new(), "", &[])
            .with_assertion(|world| {
                let cp_bar_entity = *world.read_resource::<Entity>();

                let transforms = world.read_storage::<Transform>();
                let transform = transforms
                    .get(cp_bar_entity)
                    .expect("Expected cp bar to have `Transform` component.");

                //  20 -   18  =   2
                //   2. /  20. =  10. (10%)
                // -10  /   2. =  -5. (half sprite width shift)
                //  -5. + 123. = 118. (parent shift)
                assert_eq!(&Vector3::new(118., 442., 790.), transform.translation());
                assert_eq!(90., transform.scale()[0]);
            })
            .run_winit_loop()
    }
}
