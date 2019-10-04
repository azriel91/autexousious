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
