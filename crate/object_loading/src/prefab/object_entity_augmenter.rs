use amethyst::{
    core::transform::Transform,
    ecs::Entity,
    renderer::{Flipped, Transparent},
};
use logic_clock::LogicClock;
use object_model::{
    config::object::FrameIndex,
    entity::{Mirrored, Position, SequenceStatus, Velocity},
    loaded::{ComponentSequence, ObjectWrapper},
};

use crate::{ObjectComponentStorages, ObjectFrameComponentStorages};

/// Augments an entity with `Object` components.
#[derive(Debug)]
pub struct ObjectEntityAugmenter;

impl ObjectEntityAugmenter {
    /// Augments an entity with `Object` components.
    ///
    /// # Parameters
    ///
    /// * `entity`: The entity to augment.
    /// * `object_component_storages`: Common `Component` storages for objects.
    /// * `object_animation_storages`: Common animation storages for objects.
    /// * `object_wrapper`: Slug and handle of the object to spawn.
    pub fn augment<'s, W>(
        entity: Entity,
        ObjectComponentStorages {
            ref mut flippeds,
            ref mut transparents,
            ref mut positions,
            ref mut velocities,
            ref mut transforms,
            ref mut mirroreds,
            ref mut sequence_end_transitionses,
            ref mut sequence_ids,
            ref mut sequence_statuses,
            ref mut frame_indicies,
            ref mut logic_clocks,
        }: &mut ObjectComponentStorages<'s, W::SequenceId>,
        ObjectFrameComponentStorages {
            ref mut waits,
            ref mut sprite_renders,
            ref mut bodies,
            ref mut interactionses,
        }: &mut ObjectFrameComponentStorages<'s>,
        object_wrapper: &W,
    ) where
        W: ObjectWrapper,
    {
        let sequence_end_transitions = &object_wrapper.inner().sequence_end_transitions;

        let sequence_id = W::SequenceId::default();

        flippeds
            .insert(entity, Flipped::None)
            .expect("Failed to insert flipped component.");
        transparents
            .insert(entity, Transparent)
            .expect("Failed to insert transparent component.");
        positions
            .insert(entity, Position::default())
            .expect("Failed to insert position component.");
        velocities
            .insert(entity, Velocity::default())
            .expect("Failed to insert velocity component.");
        transforms
            .insert(entity, Transform::default())
            .expect("Failed to insert transform component.");
        mirroreds
            .insert(entity, Mirrored::default())
            .expect("Failed to insert mirrored component.");
        sequence_end_transitionses
            .insert(entity, sequence_end_transitions.clone())
            .expect("Failed to insert sequence_end_transitions component.");
        sequence_ids
            .insert(entity, sequence_id)
            .expect("Failed to insert sequence_id component.");
        sequence_statuses
            .insert(entity, SequenceStatus::default())
            .expect("Failed to insert sequence_status component.");

        let frame_index = FrameIndex::default();
        frame_indicies
            .insert(entity, frame_index)
            .expect("Failed to insert frame_index component.");
        let mut logic_clock = LogicClock::new(1);

        let component_sequences = object_wrapper
            .inner()
            .component_sequences
            .get(&sequence_id)
            .unwrap_or_else(|| {
                panic!(
                    "Failed to get `ComponentSequences` for sequence ID: \
                     `{:?}`.",
                    sequence_id
                );
            });

        component_sequences
            .iter()
            .for_each(|component_sequence| match component_sequence {
                ComponentSequence::Wait(wait_sequence) => {
                    let wait = wait_sequence[*frame_index];
                    waits
                        .insert(entity, wait)
                        .expect("Failed to insert `Wait` component for object.");

                    logic_clock.limit = *wait;
                }
                ComponentSequence::SpriteRender(sprite_render_sequence) => {
                    let sprite_render = sprite_render_sequence[*frame_index].clone();
                    sprite_renders
                        .insert(entity, sprite_render)
                        .expect("Failed to insert `SpriteRender` component for object.");
                }
                ComponentSequence::Body(body_sequence) => {
                    let body_handle = body_sequence[*frame_index].clone();
                    bodies
                        .insert(entity, body_handle)
                        .expect("Failed to insert `Body` component for object.");
                }
                ComponentSequence::Interactions(interactions_sequence) => {
                    let interactions_handle = interactions_sequence[*frame_index].clone();
                    interactionses
                        .insert(entity, interactions_handle)
                        .expect("Failed to insert `Interactions` component for object.");
                }
            });

        logic_clocks
            .insert(entity, logic_clock)
            .expect("Failed to insert logic_clock component.");
    }
}

// See tests/object_entity_augmenter.rs
