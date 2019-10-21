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
    use object_model::play::HealthPoints;

    use game_play_hud::{HpBarPrefab, HpBarUpdateSystem};

    #[test]
    fn sets_transform_x_and_scale() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_bundle(TransformBundle::new())
            .with_bundle(RenderEmptyBundle::<DefaultBackend>::new())
            .with_effect(|world| {
                <HpBarPrefab as PrefabData>::SystemData::setup(world);
                <HpBarUpdateSystem as System>::SystemData::setup(world);

                let mut transform = Transform::default();
                transform.set_translation_x(123.);
                transform.set_translation_y(456.);
                transform.set_translation_z(789.);
                let char_entity = {
                    world
                        .create_entity()
                        .with(transform)
                        .with(HealthPoints::new(20))
                        .build()
                };

                let hp_bar_entity = {
                    let hp_bar_entity = world.create_entity().build();

                    let mut hp_bar_prefab_system_data =
                        world.system_data::<<HpBarPrefab as PrefabData>::SystemData>();
                    let hp_bar_prefab = HpBarPrefab::new(char_entity);

                    hp_bar_prefab
                        .add_to_entity(hp_bar_entity, &mut hp_bar_prefab_system_data, &[], &[])
                        .expect("`HpBarPrefab` failed to augment entity.");

                    hp_bar_entity
                };

                world.insert(hp_bar_entity);
            })
            .with_system_single(HpBarUpdateSystem::new(), "", &[])
            .with_assertion(|world| {
                let hp_bar_entity = *world.read_resource::<Entity>();

                let transforms = world.read_storage::<Transform>();
                let transform = transforms
                    .get(hp_bar_entity)
                    .expect("Expected hp bar to have `Transform` component.");

                // 100 - 20 = 80 (80 HP)
                // -80 / 2  = -40 (half sprite width shift)
                // -40 + 123. = 83. (parent shift)
                assert_eq!(&Vector3::new(83., 446., 790.), transform.translation());
                assert_eq!(20., transform.scale()[0]);
            })
            .run_isolated()
    }
}
