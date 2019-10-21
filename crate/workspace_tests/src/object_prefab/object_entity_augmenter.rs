#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use amethyst::{
        core::transform::{Transform, TransformBundle},
        ecs::{Builder, Read, ReadStorage, World, WorldExt},
        renderer::transparent::Transparent,
        shred::SystemData,
        Error,
    };
    use amethyst_test::AmethystApplication;
    use asset_model::{
        config::AssetSlug,
        loaded::{AssetId, AssetIdMappings},
    };
    use kinematic_model::config::{Position, Velocity};
    use object_model::play::{Grounding, Mirrored};
    use sequence_model::{
        loaded::{AssetSequenceEndTransitions, SequenceEndTransitions, SequenceId},
        play::{FrameIndexClock, FrameWaitClock, SequenceStatus},
    };

    use object_prefab::{ObjectComponentStorages, ObjectEntityAugmenter, ObjectSpawningResources};

    #[test]
    fn augments_entity_with_object_components() -> Result<(), Error> {
        run_test(|world| {
            let entity = world.create_entity().build();
            {
                let asset_id = *world.read_resource::<AssetId>();
                let (object_spawning_resources, mut object_component_storages) = world
                    .system_data::<(ObjectSpawningResources<'_>, ObjectComponentStorages<'_>)>();
                ObjectEntityAugmenter::augment(
                    &object_spawning_resources,
                    &mut object_component_storages,
                    asset_id,
                    entity,
                );
            }

            assert!(world.read_storage::<Transparent>().contains(entity));
            assert!(world.read_storage::<Position<f32>>().contains(entity));
            assert!(world.read_storage::<Velocity<f32>>().contains(entity));
            assert!(world.read_storage::<Transform>().contains(entity));
            assert!(world.read_storage::<Mirrored>().contains(entity));
            assert!(world.read_storage::<Grounding>().contains(entity));
            assert!(world
                .read_storage::<SequenceEndTransitions>()
                .contains(entity));
            assert!(world.read_storage::<SequenceId>().contains(entity));
            assert!(world.read_storage::<SequenceStatus>().contains(entity));
            assert!(world.read_storage::<FrameIndexClock>().contains(entity));
            assert!(world.read_storage::<FrameWaitClock>().contains(entity));
        })
    }

    #[test]
    fn does_not_overwrite_existing_component() -> Result<(), Error> {
        run_test(|world| {
            let position = Position::<f32>::new(1., 2., 3.);
            let velocity = Velocity::<f32>::new(1., 2., 3.);

            let entity = world.create_entity().with(position).with(velocity).build();
            {
                let asset_id = *world.read_resource::<AssetId>();
                let (object_spawning_resources, mut object_component_storages) = world
                    .system_data::<(ObjectSpawningResources<'_>, ObjectComponentStorages<'_>)>();
                ObjectEntityAugmenter::augment(
                    &object_spawning_resources,
                    &mut object_component_storages,
                    asset_id,
                    entity,
                );
            }

            let (positions, velocities) = world.system_data::<(
                ReadStorage<'_, Position<f32>>,
                ReadStorage<'_, Velocity<f32>>,
            )>();
            assert_eq!(
                &position,
                positions
                    .get(entity)
                    .expect("Expected entity to have `Position<f32>` component.")
            );
            assert_eq!(
                &velocity,
                velocities
                    .get(entity)
                    .expect("Expected entity to have `Velocity<f32>` component.")
            );
        })
    }

    fn run_test(assertion_fn: fn(&mut World)) -> Result<(), Error> {
        AmethystApplication::blank()
            .with_bundle(TransformBundle::new())
            .with_setup(|world| {
                <Read<'_, AssetIdMappings> as SystemData>::setup(world);
                <ObjectSpawningResources as SystemData>::setup(world);
                <ObjectComponentStorages as SystemData>::setup(world);
            })
            .with_effect(|world| {
                let asset_id = {
                    let mut asset_id_mappings = world.write_resource::<AssetIdMappings>();
                    let asset_slug =
                        AssetSlug::from_str("test/char").expect("Expected asset slug to be valid.");
                    asset_id_mappings.insert(asset_slug)
                };

                {
                    let mut asset_sequence_end_transitions =
                        world.write_resource::<AssetSequenceEndTransitions>();

                    asset_sequence_end_transitions
                        .insert(asset_id, SequenceEndTransitions::default());
                };

                world.insert(asset_id);
            })
            .with_assertion(assertion_fn)
            .run()
    }
}
