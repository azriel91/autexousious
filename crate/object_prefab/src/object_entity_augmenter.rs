use amethyst::{core::transform::Transform, ecs::Entity, renderer::transparent::Transparent};
use asset_model::loaded::AssetId;
use kinematic_model::config::{Position, Velocity};
use object_model::play::{Grounding, Mirrored};
use sequence_model::{
    loaded::SequenceId,
    play::{FrameIndexClock, FrameWaitClock, SequenceStatus},
};

use crate::{ObjectComponentStorages, ObjectSpawningResources};

/// Placeholder constant for uninitialized frame index and wait clocks.
const UNINITIALIZED: usize = 99;

/// Augments an entity with `Object` components.
#[derive(Debug)]
pub struct ObjectEntityAugmenter;

impl ObjectEntityAugmenter {
    /// Augments an entity with `Object` components.
    ///
    /// # Parameters
    ///
    /// * `object_spawning_resources`: Resources needed to spawn the object.
    /// * `object_component_storages`: Character specific `Component` storages.
    /// * `asset_id`: ID of the object assets.
    /// * `entity`: The entity to augment.
    pub fn augment<'s>(
        ObjectSpawningResources {
            asset_sequence_end_transitions,
        }: &ObjectSpawningResources<'s>,
        ObjectComponentStorages {
            asset_ids,
            transparents,
            positions,
            velocities,
            transforms,
            mirroreds,
            groundings,
            sequence_end_transitionses,
            sequence_ids,
            sequence_statuses,
            frame_index_clocks,
            frame_wait_clocks,
        }: &mut ObjectComponentStorages<'s>,
        asset_id: AssetId,
        entity: Entity,
    ) {
        let sequence_end_transitions =
            asset_sequence_end_transitions
                .get(asset_id)
                .unwrap_or_else(|| {
                    panic!(
                        "Expected `SequenceEndTransitions` to exist for `{:?}`.",
                        asset_id
                    )
                });

        let sequence_id = SequenceId::default();

        asset_ids
            .insert(entity, asset_id)
            .expect("Failed to insert `AssetId` component.");
        if transparents.get(entity).is_none() {
            transparents
                .insert(entity, Transparent)
                .expect("Failed to insert `Transparent` component.");
        }
        if positions.get(entity).is_none() {
            positions
                .insert(entity, Position::default())
                .expect("Failed to insert `Position<f32>` component.");
        }
        if velocities.get(entity).is_none() {
            velocities
                .insert(entity, Velocity::default())
                .expect("Failed to insert `Velocity<f32>` component.");
        }
        if transforms.get(entity).is_none() {
            transforms
                .insert(entity, Transform::default())
                .expect("Failed to insert `Transform` component.");
        }
        if mirroreds.get(entity).is_none() {
            mirroreds
                .insert(entity, Mirrored::default())
                .expect("Failed to insert `Mirrored` component.");
        }
        if groundings.get(entity).is_none() {
            groundings
                .insert(entity, Grounding::Airborne)
                .expect("Failed to insert `Grounding` component.");
        }
        if sequence_end_transitionses.get(entity).is_none() {
            sequence_end_transitionses
                .insert(entity, sequence_end_transitions.clone())
                .expect("Failed to insert `SequenceEndTransitions` component.");
        }
        if sequence_ids.get(entity).is_none() {
            sequence_ids
                .insert(entity, sequence_id)
                .expect("Failed to insert `SequenceId` component.");
        }
        if sequence_statuses.get(entity).is_none() {
            sequence_statuses
                .insert(entity, SequenceStatus::default())
                .expect("Failed to insert `SequenceStatus` component.");
        }
        if frame_index_clocks.get(entity).is_none() {
            frame_index_clocks
                .insert(entity, FrameIndexClock::new(UNINITIALIZED))
                .expect("Failed to insert `FrameIndexClock` component.");
        }
        if frame_wait_clocks.get(entity).is_none() {
            frame_wait_clocks
                .insert(entity, FrameWaitClock::new(UNINITIALIZED))
                .expect("Failed to insert `FrameWaitClock` component.");
        }
    }
}

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

    use super::ObjectEntityAugmenter;
    use crate::{ObjectComponentStorages, ObjectSpawningResources};

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
            .run_isolated()
    }
}
